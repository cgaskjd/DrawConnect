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
  const clickHandledRef = useRef(false) // Track if single-click was handled
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
    processStroke,
    setZoom,
    setPan,
    selection,
    selectRect,
    selectLasso,
    selectMagicWand,
    pickColor,
    floodFill
  } = useAppStore()

  // 增量绘制笔触到预览层（高性能）
  const drawStrokeIncremental = useCallback((points: StrokePoint[]) => {
    const preview = previewRef.current
    if (!preview || points.length < 1) return

    const ctx = preview.getContext('2d')
    if (!ctx) return

    const strokeColor = currentTool === 'eraser' ? '#FFFFFF' : brushColor
    ctx.globalAlpha = brushOpacity

    // 从上次绘制位置开始（稍微回退一点确保连续性）
    const startIndex = Math.max(0, lastDrawnIndexRef.current - 1)

    if (points.length === 1 && startIndex === 0) {
      // 单点绘制一个圆
      const p = points[0]
      const radius = brushSize * (p.pressure + 0.5) / 2
      ctx.fillStyle = strokeColor
      ctx.beginPath()
      ctx.arc(p.x, p.y, radius, 0, Math.PI * 2)
      ctx.fill()
      lastDrawnIndexRef.current = 1
      return
    }

    // 使用圆点绘制法确保连续性
    for (let i = startIndex; i < points.length - 1; i++) {
      const p0 = points[i]
      const p1 = points[i + 1]

      // 计算两点间距离
      const dx = p1.x - p0.x
      const dy = p1.y - p0.y
      const dist = Math.sqrt(dx * dx + dy * dy)

      // 根据距离确定插值步数（最少1步，最多每2像素一步）
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

    // 更新已绘制位置
    lastDrawnIndexRef.current = points.length - 1
    ctx.globalAlpha = 1
  }, [currentTool, brushColor, brushSize, brushOpacity])

  // 清空预览层
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

      // Helper function to draw marching ants stroke
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

      // Draw based on shape type
      if (selection.shape_type === 'lasso' && selection.points && selection.points.length >= 3) {
        // Draw lasso polygon with marching ants
        drawMarchingAnts(() => {
          ctx.beginPath()
          ctx.moveTo(selection.points![0][0], selection.points![0][1])
          for (let i = 1; i < selection.points!.length; i++) {
            ctx.lineTo(selection.points![i][0], selection.points![i][1])
          }
          ctx.closePath()
        })
      } else {
        // Draw rectangle or mask bounds with marching ants
        drawMarchingAnts(() => {
          ctx.beginPath()
          ctx.rect(x, y, width, height)
        })
      }
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

  const handlePointerDown = useCallback((e: React.PointerEvent) => {
    // Reset click handled flag
    clickHandledRef.current = false

    if (isSpacePressed) {
      isPanningRef.current = true
      lastPanPosRef.current = { x: e.clientX, y: e.clientY }
      wrapperRef.current?.setPointerCapture(e.pointerId)
      return
    }

    const pos = getPointerPosition(e)

    // Alt+click to pick color - just prevent drawing, actual pick handled in onClick
    if (e.altKey) {
      return
    }

    // Single-click tools - prevent other actions, actual handling in onClick
    if (currentTool === 'eyedropper' || currentTool === 'fill' || currentTool === 'select_magic') {
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

    // Draw immediate preview for single click
    drawStrokeIncremental([pos])

    wrapperRef.current?.setPointerCapture(e.pointerId)
  }, [currentTool, getPointerPosition, isSpacePressed, drawStrokeIncremental])

  const handlePointerMove = useCallback((e: React.PointerEvent) => {
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

    // 使用 requestAnimationFrame 平滑绘制
    if (rafIdRef.current) {
      cancelAnimationFrame(rafIdRef.current)
    }
    rafIdRef.current = requestAnimationFrame(() => {
      drawStrokeIncremental(strokePointsRef.current)
    })
  }, [currentTool, getPointerPosition, panX, panY, setPan, selectionStart, lassoPoints, drawStrokeIncremental, isSpacePressed])

  const handlePointerUp = useCallback((e: React.PointerEvent) => {
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
        selectRect(x, y, width, height)
      }
      setSelectionStart(null)
      setSelectionCurrent(null)
      wrapperRef.current?.releasePointerCapture(e.pointerId)
      return
    }

    if (lassoPoints.length > 2 && currentTool === 'select_lasso') {
      selectLasso(lassoPoints)
      setLassoPoints([])
      wrapperRef.current?.releasePointerCapture(e.pointerId)
      return
    }

    if (!isDrawingRef.current) return

    isDrawingRef.current = false
    wrapperRef.current?.releasePointerCapture(e.pointerId)

    // 取消待处理的动画帧
    if (rafIdRef.current) {
      cancelAnimationFrame(rafIdRef.current)
      rafIdRef.current = null
    }

    // 发送完整笔触到后端处理
    if (strokePointsRef.current.length > 0) {
      // 清空预览层，等待后端渲染结果
      clearPreview()
      processStroke(strokePointsRef.current)
    }
    strokePointsRef.current = []
  }, [currentTool, processStroke, selectRect, selectLasso, selectionStart, selectionCurrent, lassoPoints, clearPreview])

  // Determine cursor based on tool and pan mode
  const getCursor = () => {
    if (isSpacePressed || isPanningRef.current) return 'grab'
    if (isAltPressed) return 'crosshair' // Eyedropper mode when Alt is pressed
    if (currentTool === 'brush' || currentTool === 'eraser') return 'none' // Hide default cursor, show custom brush cursor
    if (currentTool === 'move') return 'move'
    if (currentTool === 'eyedropper') return 'crosshair'
    if (currentTool === 'fill') return 'crosshair'
    if (currentTool === 'select_rect' || currentTool === 'select_lasso') return 'crosshair'
    if (currentTool === 'select_magic') return 'cell'
    return 'default'
  }

  // Hide brush cursor when leaving canvas
  const handlePointerLeave = useCallback((e: React.PointerEvent) => {
    setShowBrushCursor(false)
    handlePointerUp(e)
  }, [handlePointerUp])

  // Calculate brush cursor size in screen pixels
  const brushCursorSize = brushSize * zoom

  // Handle single click for eyedropper, fill, magic wand (more reliable than pointerdown for single clicks)
  const handleClick = useCallback((e: React.MouseEvent) => {
    // Skip if we were panning or drawing
    if (isPanningRef.current || isDrawingRef.current) return

    const canvas = canvasRef.current
    if (!canvas) return

    const rect = canvas.getBoundingClientRect()
    const scaleX = canvasWidth / rect.width
    const scaleY = canvasHeight / rect.height
    const x = (e.clientX - rect.left) * scaleX
    const y = (e.clientY - rect.top) * scaleY

    // Alt+click to pick color (works with any tool)
    if (e.altKey) {
      pickColor(x, y)
      return
    }

    if (currentTool === 'eyedropper') {
      pickColor(x, y)
      return
    }

    if (currentTool === 'fill') {
      floodFill(x, y)
      return
    }

    if (currentTool === 'select_magic') {
      selectMagicWand(Math.round(x), Math.round(y))
      return
    }
  }, [currentTool, canvasWidth, canvasHeight, pickColor, floodFill, selectMagicWand])

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
      {/* Brush cursor indicator */}
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
