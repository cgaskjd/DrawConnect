import { useRef, useEffect, useCallback, useState } from 'react'
import { useAppStore } from '../stores/appStore'
import './Canvas.css'

/**
 * EditCanvas - 修图模式专用的画布组件
 * 特点：
 * - 鼠标左键拖动移动图片
 * - Ctrl+滚轮缩放
 * - 不支持绘画功能
 */
export function EditCanvas() {
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const containerRef = useRef<HTMLDivElement>(null)
  const wrapperRef = useRef<HTMLDivElement>(null)
  const isPanningRef = useRef(false)
  const lastPanPosRef = useRef({ x: 0, y: 0 })

  const {
    canvasImage,
    canvasWidth,
    canvasHeight,
    zoom,
    panX,
    panY,
    setZoom,
    setPan,
  } = useAppStore()

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

  const handlePointerDown = useCallback((e: React.PointerEvent) => {
    // 左键拖动移动图片
    if (e.button === 0) {
      isPanningRef.current = true
      lastPanPosRef.current = { x: e.clientX, y: e.clientY }
      wrapperRef.current?.setPointerCapture(e.pointerId)
    }
  }, [])

  const handlePointerMove = useCallback((e: React.PointerEvent) => {
    if (isPanningRef.current) {
      const dx = e.clientX - lastPanPosRef.current.x
      const dy = e.clientY - lastPanPosRef.current.y
      lastPanPosRef.current = { x: e.clientX, y: e.clientY }
      setPan(panX + dx, panY + dy)
    }
  }, [panX, panY, setPan])

  const handlePointerUp = useCallback((e: React.PointerEvent) => {
    if (isPanningRef.current) {
      isPanningRef.current = false
      wrapperRef.current?.releasePointerCapture(e.pointerId)
    }
  }, [])

  const handlePointerLeave = useCallback((e: React.PointerEvent) => {
    handlePointerUp(e)
  }, [handlePointerUp])

  // Determine cursor
  const getCursor = () => {
    if (isPanningRef.current) return 'grabbing'
    return 'grab'
  }

  return (
    <div
      ref={containerRef}
      className="canvas-container edit-canvas-container"
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
      >
        <canvas
          ref={canvasRef}
          width={canvasWidth}
          height={canvasHeight}
          className="drawing-canvas"
        />
      </div>
    </div>
  )
}
