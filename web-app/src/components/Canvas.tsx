import { useRef, useEffect, useCallback, useState } from 'react'
import { useAppStore, StrokePoint } from '../stores/appStore'
import './Canvas.css'

export function Canvas() {
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const overlayRef = useRef<HTMLCanvasElement>(null)
  const previewRef = useRef<HTMLCanvasElement>(null)
  const containerRef = useRef<HTMLDivElement>(null)
  const wrapperRef = useRef<HTMLDivElement>(null)
  const isDrawingRef = useRef(false)
  const strokePointsRef = useRef<StrokePoint[]>([])
  const lastDrawnIndexRef = useRef(0)
  const isPanningRef = useRef(false)
  const lastPanPosRef = useRef({ x: 0, y: 0 })
  const rafIdRef = useRef<number | null>(null)
  const [isSpacePressed, setIsSpacePressed] = useState(false)
  const [isAltPressed, setIsAltPressed] = useState(false)

  // Brush cursor state
  const [cursorPos, setCursorPos] = useState<{ x: number; y: number } | null>(null)
  const [showBrushCursor, setShowBrushCursor] = useState(false)

  // Selection state
  const [selectionStart, setSelectionStart] = useState<{ x: number; y: number } | null>(null)
  const [selectionCurrent, setSelectionCurrent] = useState<{ x: number; y: number } | null>(null)
  const [lassoPoints, setLassoPoints] = useState<Array<[number, number]>>([])
  const [marchingAntsOffset, setMarchingAntsOffset] = useState(0)

  const {
    canvasImage,
    canvasWidth,
    canvasHeight,
    zoom,
    panX,
    panY,
    currentTool,
    brushColor,
    brushSize,
    brushOpacity,
    beginStroke,
    addStrokePoint,
    endStroke,
    renderCanvas,
    setZoom,
    setPan,
    selection,
    selectRect,
    selectLasso,
    pickColor,
    floodFill
  } = useAppStore()

  // Draw stroke incrementally to preview layer (high performance)
  const drawStrokeIncremental = useCallback((points: StrokePoint[]) => {
    const preview = previewRef.current
    if (!preview || points.length < 1) return

    const ctx = preview.getContext('2d')
    if (!ctx) return

    const strokeColor = currentTool === 'eraser' ? '#FFFFFF' : brushColor
    ctx.globalAlpha = brushOpacity

    const startIndex = Math.max(0, lastDrawnIndexRef.current - 1)

    if (points.length === 1 && startIndex === 0) {
      const p = points[0]
      const radius = brushSize * (p.pressure + 0.5) / 2
      ctx.fillStyle = strokeColor
      ctx.beginPath()
      ctx.arc(p.x, p.y, radius, 0, Math.PI * 2)
      ctx.fill()
      lastDrawnIndexRef.current = 1
      return
    }

    for (let i = startIndex; i < points.length - 1; i++) {
      const p0 = points[i]
      const p1 = points[i + 1]

      const dx = p1.x - p0.x
      const dy = p1.y - p0.y
      const dist = Math.sqrt(dx * dx + dy * dy)

      const steps = Math.max(1, Math.floor(dist / 2))

      for (let j = 0; j <= steps; j++) {
        const t = j / steps
        const x = p0.x + dx * t
        const y = p0.y + dy * t
        const pressure = p0.pressure + (p1.pressure - p0.pressure) * t
        const radius = brushSize * (pressure + 0.5) / 2

        ctx.fillStyle = strokeColor
        ctx.beginPath()
        ctx.arc(x, y, radius, 0, Math.PI * 2)
        ctx.fill()
      }
    }

    lastDrawnIndexRef.current = points.length - 1
    ctx.globalAlpha = 1
  }, [currentTool, brushColor, brushSize, brushOpacity])

  // Clear preview layer
  const clearPreview = useCallback(() => {
    const preview = previewRef.current
    if (!preview) return
    const ctx = preview.getContext('2d')
    if (!ctx) return
    ctx.clearRect(0, 0, canvasWidth, canvasHeight)
  }, [canvasWidth, canvasHeight])

  // Marching ants animation
  useEffect(() => {
    if (!selection?.is_active) return

    const interval = setInterval(() => {
      setMarchingAntsOffset((prev) => (prev + 1) % 16)
    }, 100)

    return () => clearInterval(interval)
  }, [selection?.is_active])

  // Draw selection overlay
  useEffect(() => {
    const overlay = overlayRef.current
    if (!overlay) return

    const ctx = overlay.getContext('2d')
    if (!ctx) return

    ctx.clearRect(0, 0, canvasWidth, canvasHeight)

    // Draw active selection (marching ants)
    if (selection?.is_active && selection.bounds) {
      const [x, y, width, height] = selection.bounds

      const drawMarchingAnts = (drawPath: () => void) => {
        ctx.save()
        ctx.strokeStyle = '#000000'
        ctx.lineWidth = 1
        ctx.setLineDash([4, 4])
        ctx.lineDashOffset = -marchingAntsOffset
        drawPath()
        ctx.stroke()

        ctx.strokeStyle = '#FFFFFF'
        ctx.lineDashOffset = -marchingAntsOffset + 4
        drawPath()
        ctx.stroke()
        ctx.restore()
        ctx.setLineDash([])
      }

      drawMarchingAnts(() => {
        ctx.beginPath()
        ctx.rect(x, y, width, height)
      })
    }

    // Draw selection preview during drag
    if (selectionStart && selectionCurrent && currentTool === 'select_rect') {
      const x = Math.min(selectionStart.x, selectionCurrent.x)
      const y = Math.min(selectionStart.y, selectionCurrent.y)
      const width = Math.abs(selectionCurrent.x - selectionStart.x)
      const height = Math.abs(selectionCurrent.y - selectionStart.y)

      ctx.strokeStyle = '#0088FF'
      ctx.lineWidth = 1
      ctx.setLineDash([4, 4])
      ctx.strokeRect(x, y, width, height)
      ctx.setLineDash([])

      ctx.fillStyle = 'rgba(0, 136, 255, 0.1)'
      ctx.fillRect(x, y, width, height)
    }

    // Draw lasso preview
    if (lassoPoints.length > 1 && currentTool === 'select_lasso') {
      ctx.beginPath()
      ctx.moveTo(lassoPoints[0][0], lassoPoints[0][1])
      for (let i = 1; i < lassoPoints.length; i++) {
        ctx.lineTo(lassoPoints[i][0], lassoPoints[i][1])
      }
      ctx.strokeStyle = '#0088FF'
      ctx.lineWidth = 1
      ctx.setLineDash([4, 4])
      ctx.stroke()
      ctx.setLineDash([])
    }
  }, [selection, selectionStart, selectionCurrent, lassoPoints, marchingAntsOffset, canvasWidth, canvasHeight, currentTool])

  // Render the canvas image
  useEffect(() => {
    if (!canvasImage || !canvasRef.current) return

    const ctx = canvasRef.current.getContext('2d')
    if (!ctx) return

    const img = new Image()
    img.onload = () => {
      ctx.clearRect(0, 0, canvasWidth, canvasHeight)
      ctx.drawImage(img, 0, 0)
    }
    img.src = `data:image/png;base64,${canvasImage}`
  }, [canvasImage, canvasWidth, canvasHeight])

  // Handle keyboard events for space key (pan mode) and alt key (eyedropper)
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.code === 'Space' && !e.repeat) {
        e.preventDefault()
        setIsSpacePressed(true)
      }
      if (e.code === 'AltLeft' || e.code === 'AltRight') {
        setIsAltPressed(true)
      }
    }

    const handleKeyUp = (e: KeyboardEvent) => {
      if (e.code === 'Space') {
        setIsSpacePressed(false)
        isPanningRef.current = false
      }
      if (e.code === 'AltLeft' || e.code === 'AltRight') {
        setIsAltPressed(false)
      }
    }

    window.addEventListener('keydown', handleKeyDown)
    window.addEventListener('keyup', handleKeyUp)

    return () => {
      window.removeEventListener('keydown', handleKeyDown)
      window.removeEventListener('keyup', handleKeyUp)
    }
  }, [])

  // Handle wheel events for zoom
  useEffect(() => {
    const container = containerRef.current
    if (!container) return

    const handleWheel = (e: WheelEvent) => {
      if (e.ctrlKey) {
        e.preventDefault()
        const delta = e.deltaY > 0 ? 0.9 : 1.1
        const newZoom = Math.max(0.1, Math.min(10, zoom * delta))
        setZoom(newZoom)
      }
    }

    container.addEventListener('wheel', handleWheel, { passive: false })

    return () => {
      container.removeEventListener('wheel', handleWheel)
    }
  }, [zoom, setZoom])

  const getPointerPosition = useCallback((e: React.PointerEvent): StrokePoint => {
    const canvas = canvasRef.current
    if (!canvas) return { x: 0, y: 0, pressure: 0.5, tilt_x: 0, tilt_y: 0, timestamp: Date.now() }

    const rect = canvas.getBoundingClientRect()
    const scaleX = canvasWidth / rect.width
    const scaleY = canvasHeight / rect.height

    return {
      x: (e.clientX - rect.left) * scaleX,
      y: (e.clientY - rect.top) * scaleY,
      pressure: e.pressure || 0.5,
      tilt_x: e.tiltX || 0,
      tilt_y: e.tiltY || 0,
      timestamp: Date.now(),
    }
  }, [canvasWidth, canvasHeight])

  const handlePointerDown = useCallback(async (e: React.PointerEvent) => {
    if (isSpacePressed) {
      isPanningRef.current = true
      lastPanPosRef.current = { x: e.clientX, y: e.clientY }
      wrapperRef.current?.setPointerCapture(e.pointerId)
      return
    }

    const pos = getPointerPosition(e)

    // Alt+click to pick color
    if (e.altKey) {
      return
    }

    // Single-click tools
    if (currentTool === 'eyedropper' || currentTool === 'fill') {
      return
    }

    if (currentTool === 'select_rect') {
      setSelectionStart({ x: pos.x, y: pos.y })
      setSelectionCurrent({ x: pos.x, y: pos.y })
      wrapperRef.current?.setPointerCapture(e.pointerId)
      return
    }

    if (currentTool === 'select_lasso') {
      setLassoPoints([[pos.x, pos.y]])
      wrapperRef.current?.setPointerCapture(e.pointerId)
      return
    }

    if (currentTool !== 'brush' && currentTool !== 'eraser') return

    isDrawingRef.current = true
    strokePointsRef.current = [pos]
    lastDrawnIndexRef.current = 0

    // Begin stroke in WASM engine
    await beginStroke()
    await addStrokePoint(pos)

    // Draw immediate preview
    drawStrokeIncremental([pos])

    wrapperRef.current?.setPointerCapture(e.pointerId)
  }, [currentTool, getPointerPosition, isSpacePressed, drawStrokeIncremental, beginStroke, addStrokePoint])

  const handlePointerMove = useCallback(async (e: React.PointerEvent) => {
    // Update brush cursor position
    if ((currentTool === 'brush' || currentTool === 'eraser') && !isSpacePressed) {
      const rect = containerRef.current?.getBoundingClientRect()
      if (rect) {
        setCursorPos({ x: e.clientX - rect.left, y: e.clientY - rect.top })
        setShowBrushCursor(true)
      }
    }

    if (isPanningRef.current) {
      const dx = e.clientX - lastPanPosRef.current.x
      const dy = e.clientY - lastPanPosRef.current.y
      lastPanPosRef.current = { x: e.clientX, y: e.clientY }
      setPan(panX + dx, panY + dy)
      return
    }

    const pos = getPointerPosition(e)

    if (selectionStart && currentTool === 'select_rect') {
      setSelectionCurrent({ x: pos.x, y: pos.y })
      return
    }

    if (lassoPoints.length > 0 && currentTool === 'select_lasso') {
      setLassoPoints(prev => [...prev, [pos.x, pos.y]])
      return
    }

    if (!isDrawingRef.current) return

    strokePointsRef.current.push(pos)

    // Add point to WASM engine
    await addStrokePoint(pos)

    // Use requestAnimationFrame for smooth drawing
    if (rafIdRef.current) {
      cancelAnimationFrame(rafIdRef.current)
    }
    rafIdRef.current = requestAnimationFrame(() => {
      drawStrokeIncremental(strokePointsRef.current)
    })
  }, [currentTool, getPointerPosition, panX, panY, setPan, selectionStart, lassoPoints, drawStrokeIncremental, isSpacePressed, addStrokePoint])

  const handlePointerUp = useCallback(async (e: React.PointerEvent) => {
    if (isPanningRef.current) {
      isPanningRef.current = false
      wrapperRef.current?.releasePointerCapture(e.pointerId)
      return
    }

    if (selectionStart && selectionCurrent && currentTool === 'select_rect') {
      const x = Math.min(selectionStart.x, selectionCurrent.x)
      const y = Math.min(selectionStart.y, selectionCurrent.y)
      const width = Math.abs(selectionCurrent.x - selectionStart.x)
      const height = Math.abs(selectionCurrent.y - selectionStart.y)
      if (width > 1 && height > 1) {
        await selectRect(x, y, width, height)
      }
      setSelectionStart(null)
      setSelectionCurrent(null)
      wrapperRef.current?.releasePointerCapture(e.pointerId)
      return
    }

    if (lassoPoints.length > 2 && currentTool === 'select_lasso') {
      await selectLasso(lassoPoints)
      setLassoPoints([])
      wrapperRef.current?.releasePointerCapture(e.pointerId)
      return
    }

    if (!isDrawingRef.current) return

    isDrawingRef.current = false
    wrapperRef.current?.releasePointerCapture(e.pointerId)

    if (rafIdRef.current) {
      cancelAnimationFrame(rafIdRef.current)
      rafIdRef.current = null
    }

    // End stroke in WASM engine and render
    if (strokePointsRef.current.length > 0) {
      clearPreview()
      await endStroke()
      await renderCanvas()
    }
    strokePointsRef.current = []
  }, [currentTool, selectRect, selectLasso, selectionStart, selectionCurrent, lassoPoints, clearPreview, endStroke, renderCanvas])

  // Determine cursor based on tool and pan mode
  const getCursor = () => {
    if (isSpacePressed || isPanningRef.current) return 'grab'
    if (isAltPressed) return 'crosshair'
    if (currentTool === 'brush' || currentTool === 'eraser') return 'none'
    if (currentTool === 'move') return 'move'
    if (currentTool === 'eyedropper') return 'crosshair'
    if (currentTool === 'fill') return 'crosshair'
    if (currentTool === 'select_rect' || currentTool === 'select_lasso') return 'crosshair'
    return 'default'
  }

  const handlePointerLeave = useCallback((e: React.PointerEvent) => {
    setShowBrushCursor(false)
    handlePointerUp(e)
  }, [handlePointerUp])

  const brushCursorSize = brushSize * zoom

  const handleClick = useCallback(async (e: React.MouseEvent) => {
    if (isPanningRef.current || isDrawingRef.current) return

    const canvas = canvasRef.current
    if (!canvas) return

    const rect = canvas.getBoundingClientRect()
    const scaleX = canvasWidth / rect.width
    const scaleY = canvasHeight / rect.height
    const x = (e.clientX - rect.left) * scaleX
    const y = (e.clientY - rect.top) * scaleY

    if (e.altKey) {
      await pickColor(x, y)
      return
    }

    if (currentTool === 'eyedropper') {
      await pickColor(x, y)
      return
    }

    if (currentTool === 'fill') {
      await floodFill(x, y)
      return
    }
  }, [currentTool, canvasWidth, canvasHeight, pickColor, floodFill])

  return (
    <div
      ref={containerRef}
      className="canvas-container"
      style={{ cursor: getCursor() }}
    >
      <div
        ref={wrapperRef}
        className="canvas-wrapper"
        style={{
          width: canvasWidth * zoom,
          height: canvasHeight * zoom,
          transform: `translate(${panX}px, ${panY}px)`,
          touchAction: 'none',
        }}
        onPointerDown={handlePointerDown}
        onPointerMove={handlePointerMove}
        onPointerUp={handlePointerUp}
        onPointerLeave={handlePointerLeave}
        onClick={handleClick}
      >
        <canvas
          ref={canvasRef}
          width={canvasWidth}
          height={canvasHeight}
          className="drawing-canvas"
        />
        <canvas
          ref={previewRef}
          width={canvasWidth}
          height={canvasHeight}
          className="preview-overlay"
        />
        <canvas
          ref={overlayRef}
          width={canvasWidth}
          height={canvasHeight}
          className="selection-overlay"
        />
      </div>
      {showBrushCursor && cursorPos && (currentTool === 'brush' || currentTool === 'eraser') && !isSpacePressed && !isAltPressed && (
        <div
          className="brush-cursor"
          style={{
            left: cursorPos.x,
            top: cursorPos.y,
            width: brushCursorSize,
            height: brushCursorSize,
            borderColor: currentTool === 'eraser' ? '#666' : brushColor,
          }}
        />
      )}
    </div>
  )
}
