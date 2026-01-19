import { useEffect, useCallback, useState } from 'react'
import { EditCanvas } from './EditCanvas'
import { EditMenuBar } from './EditMenuBar'
import { AdjustmentPanel } from './AdjustmentPanel'
import { PluginManager } from './PluginManager'
import { useAppStore } from '../stores/appStore'
import { usePluginStore } from '../stores/pluginStore'
import { t } from '../i18n'
import './ImageEditor.css'

/**
 * ImageEditor - 修图模式的独立界面
 * 功能：
 * - 图片查看和缩放
 * - 图像调整（亮度、对比度、色相等）
 * - 滤镜效果
 * - 变换操作（旋转、翻转等）
 * - 插件管理
 */
export function ImageEditor() {
  const {
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
  } = useAppStore()

  const { initialize: initPluginSystem } = usePluginStore()

  const [isPluginManagerOpen, setPluginManagerOpen] = useState(false)

  // Initialize plugin system when editor mounts
  useEffect(() => {
    initPluginSystem()
  }, [initPluginSystem])

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

  return (
    <div className="image-editor">
      <EditMenuBar onOpenPluginManager={() => setPluginManagerOpen(true)} />
      <div className="editor-content">
        <div className="editor-canvas-area">
          {isInitialized ? (
            <>
              <EditCanvas />
              <AdjustmentPanel />
            </>
          ) : (
            <div className="welcome-screen">
              <div className="welcome-content">
                <h1>修图模式</h1>
                <p>导入图片开始修图</p>
              </div>
            </div>
          )}
        </div>
      </div>
      <div className="editor-status-bar">
        <span className="mode-indicator">修图模式</span>
        <span>{isInitialized ? t('status.ready') : '等待导入图片...'}</span>
        {isInitialized && (
          <>
            <span>{canvasWidth} x {canvasHeight} px</span>
            <span>{t('status.zoom')}: {Math.round(zoom * 100)}%</span>
            <span className="hint">拖动移动 | Ctrl+滚轮缩放</span>
          </>
        )}
      </div>

      {/* Plugin Manager Modal */}
      <PluginManager
        isOpen={isPluginManagerOpen}
        onClose={() => setPluginManagerOpen(false)}
      />
    </div>
  )
}
