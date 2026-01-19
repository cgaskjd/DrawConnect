import { useEffect, useCallback, useState } from 'react'
import { useAppStore } from '../stores/appStore'
import { Canvas } from './Canvas'
import { Toolbar } from './Toolbar'
import { LayerPanel } from './LayerPanel'
import { BrushPanel } from './BrushPanel'
import { ColorPicker } from './ColorPicker'
import { MenuBar } from './MenuBar'
import { NewCanvasDialog } from './NewCanvasDialog'
import './ImageEditor.css'

/**
 * ImageEditor - Main drawing/editing interface for web
 */
export default function ImageEditor() {
  const {
    appMode,
    isInitialized,
    canvasWidth,
    canvasHeight,
    zoom,
    undo,
    redo,
    zoomIn,
    zoomOut,
    resetZoom,
    resetPan,
    initializeCanvas,
    openImageFile,
    setAppMode,
  } = useAppStore()

  const [showNewCanvasDialog, setShowNewCanvasDialog] = useState(false)

  // Auto-show dialog for draw mode, or auto-open file for edit mode
  useEffect(() => {
    if (!isInitialized) {
      if (appMode === 'draw') {
        setShowNewCanvasDialog(true)
      } else if (appMode === 'edit') {
        // Auto-open file picker for edit mode
        openImageFile().catch(() => {
          // If user cancels, go back to start
          setAppMode(null)
        })
      }
    }
  }, [appMode, isInitialized, openImageFile, setAppMode])

  // Keyboard shortcuts handler
  const handleKeyDown = useCallback((e: KeyboardEvent) => {
    if (e.ctrlKey && e.key === 'z' && !e.shiftKey) {
      e.preventDefault()
      undo()
    }
    if ((e.ctrlKey && e.key === 'y') || (e.ctrlKey && e.shiftKey && e.key === 'Z')) {
      e.preventDefault()
      redo()
    }
    if (e.ctrlKey && (e.key === '+' || e.key === '=')) {
      e.preventDefault()
      zoomIn()
    }
    if (e.ctrlKey && e.key === '-') {
      e.preventDefault()
      zoomOut()
    }
    if (e.ctrlKey && e.key === '0') {
      e.preventDefault()
      resetZoom()
      resetPan()
    }
  }, [undo, redo, zoomIn, zoomOut, resetZoom, resetPan])

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [handleKeyDown])

  const handleCreateCanvas = async (width: number, height: number) => {
    await initializeCanvas(width, height)
    setShowNewCanvasDialog(false)
  }

  return (
    <div className="image-editor">
      <MenuBar />

      <div className="editor-content">
        {/* Left sidebar - Toolbar */}
        {isInitialized && appMode === 'draw' && (
          <div className="left-sidebar">
            <Toolbar />
          </div>
        )}

        {/* Main canvas area */}
        <div className="editor-canvas-area">
          {isInitialized ? (
            <Canvas />
          ) : (
            <div className="welcome-screen">
              <div className="welcome-content">
                <h1>{appMode === 'draw' ? 'Draw Mode' : 'Edit Mode'}</h1>
                <p>{appMode === 'draw' ? 'Create a new canvas to start drawing' : 'Open an image to start editing'}</p>
              </div>
            </div>
          )}
        </div>

        {/* Right sidebar - Panels */}
        {isInitialized && (
          <div className="right-sidebar">
            {appMode === 'draw' && (
              <>
                <ColorPicker />
                <BrushPanel />
              </>
            )}
            <LayerPanel />
          </div>
        )}
      </div>

      <div className="editor-status-bar">
        <span className="mode-indicator">{appMode === 'draw' ? 'Draw Mode' : 'Edit Mode'}</span>
        <span>{isInitialized ? 'Ready' : 'Waiting...'}</span>
        {isInitialized && (
          <>
            <span>{canvasWidth} x {canvasHeight} px</span>
            <span>Zoom: {Math.round(zoom * 100)}%</span>
            <span className="hint">Drag to pan | Ctrl+Scroll to zoom</span>
          </>
        )}
      </div>

      {/* New Canvas Dialog */}
      {showNewCanvasDialog && (
        <NewCanvasDialog
          onClose={() => {
            setShowNewCanvasDialog(false)
            if (!isInitialized) {
              setAppMode(null)
            }
          }}
          onCreate={handleCreateCanvas}
        />
      )}
    </div>
  )
}
