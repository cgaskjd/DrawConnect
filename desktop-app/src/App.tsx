import { useState, useEffect, useCallback } from 'react'
import { open, message } from '@tauri-apps/api/dialog'
import { Toolbar } from './components/Toolbar'
import { Canvas } from './components/Canvas'
import { LayerPanel } from './components/LayerPanel'
import { ColorPicker } from './components/ColorPicker'
import { BrushPanel } from './components/BrushPanel'
import { MenuBar } from './components/MenuBar'
import { NewCanvasDialog } from './components/NewCanvasDialog'
import { StartScreen } from './components/StartScreen'
import { ImageEditor } from './components/ImageEditor'
import { useAppStore } from './stores/appStore'
import { t } from './i18n'
import './styles/app.css'

function App() {
  const [showNewCanvasDialog, setShowNewCanvasDialog] = useState(false)
  const {
    appMode,
    initializeCanvas,
    openImageAsCanvas,
    isInitialized,
    canvasWidth,
    canvasHeight,
    zoom,
    undo,
    redo,
    zoomIn,
    zoomOut,
    resetZoom,
    resetPan
  } = useAppStore()

  // When entering draw mode, show canvas dialog
  useEffect(() => {
    if (appMode === 'draw' && !isInitialized) {
      setShowNewCanvasDialog(true)
    }
  }, [appMode, isInitialized])

  // When entering edit mode, show file picker
  useEffect(() => {
    if (appMode === 'edit' && !isInitialized) {
      handleImportForEdit()
    }
  }, [appMode])

  const handleImportForEdit = async () => {
    const path = await open({
      filters: [{ name: '图片文件', extensions: ['png', 'jpg', 'jpeg', 'gif', 'bmp', 'webp', 'tiff', 'ico'] }],
      multiple: false,
    })
    if (path && typeof path === 'string') {
      try {
        await openImageAsCanvas(path)
      } catch (error) {
        await message(String(error), { title: '导入失败', type: 'error' })
        // Reset mode if import fails
        useAppStore.getState().setAppMode(null)
      }
    } else {
      // User cancelled, reset mode
      useAppStore.getState().setAppMode(null)
    }
  }

  const handleCreateCanvas = async (width: number, height: number) => {
    await initializeCanvas(width, height)
    setShowNewCanvasDialog(false)
  }

  // Keyboard shortcuts handler (only for draw mode)
  const handleKeyDown = useCallback((e: KeyboardEvent) => {
    if (appMode !== 'draw') return

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
  }, [appMode, undo, redo, zoomIn, zoomOut, resetZoom, resetPan])

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [handleKeyDown])

  // Show start screen if no mode selected
  if (!appMode) {
    return <StartScreen />
  }

  // Show ImageEditor for edit mode (completely independent)
  if (appMode === 'edit') {
    return <ImageEditor />
  }

  // Draw mode interface
  return (
    <div className="app">
      <MenuBar />
      <div className="main-content">
        <div className="left-sidebar">
          <Toolbar />
          <ColorPicker />
        </div>

        <div className="canvas-area">
          {isInitialized ? (
            <Canvas />
          ) : (
            <div className="welcome-screen">
              <div className="welcome-content">
                <h1>DrawConnect</h1>
                <p>创建画布开始绘画</p>
              </div>
            </div>
          )}
        </div>

        <div className="right-sidebar">
          <BrushPanel />
          <LayerPanel />
        </div>
      </div>
      <div className="status-bar">
        <span className="mode-indicator">画画模式</span>
        <span>{isInitialized ? t('status.ready') : '等待...'}</span>
        {isInitialized && (
          <>
            <span>{canvasWidth} x {canvasHeight} px</span>
            <span>{t('status.zoom')}: {Math.round(zoom * 100)}%</span>
          </>
        )}
      </div>

      {/* New canvas dialog for draw mode */}
      <NewCanvasDialog
        isOpen={showNewCanvasDialog}
        onClose={() => {
          if (!isInitialized) {
            // If cancelled without canvas, go back to start
            useAppStore.getState().setAppMode(null)
          }
          setShowNewCanvasDialog(false)
        }}
        onCreate={handleCreateCanvas}
      />
    </div>
  )
}

export default App
