import { useState } from 'react'
import { save, open, message } from '@tauri-apps/api/dialog'
import { useAppStore } from '../stores/appStore'
import { t } from '../i18n'
import { NewCanvasDialog } from './NewCanvasDialog'
import { PluginManager } from './PluginManager'
import './MenuBar.css'

export function MenuBar() {
  const [showNewCanvasDialog, setShowNewCanvasDialog] = useState(false)
  const [showPluginManager, setShowPluginManager] = useState(false)

  const {
    appMode,
    setAppMode,
    undo,
    redo,
    canUndo,
    canRedo,
    saveFile,
    exportPng,
    importImage,
    importImageAsLayer,
    initializeCanvas,
    zoomIn,
    zoomOut,
    resetZoom,
    resetPan,
    importBrush,
    setActiveAdjustment,
    isInitialized,
    // Image adjustments
    adjustInvert,
    // Filters
    filterFindEdges,
    // Transforms
    transformRotate90CW,
    transformRotate90CCW,
    transformRotate180,
    transformFlipHorizontal,
    transformFlipVertical,
  } = useAppStore()

  const handleNewCanvas = async (width: number, height: number) => {
    await initializeCanvas(width, height)
    setShowNewCanvasDialog(false)
  }

  const handleBackToStart = () => {
    // Reset everything and go back to start screen
    window.location.reload()
  }

  const handleSave = async () => {
    const path = await save({
      filters: [{ name: 'DrawConnect', extensions: ['dcpaint'] }],
    })
    if (path) {
      await saveFile(path)
    }
  }

  const handleExportPng = async () => {
    const path = await save({
      filters: [{ name: 'PNG 图片', extensions: ['png'] }],
    })
    if (path) {
      await exportPng(path)
    }
  }

  const handleImportImage = async () => {
    const path = await open({
      filters: [{ name: '图片文件', extensions: ['png', 'jpg', 'jpeg', 'gif', 'bmp', 'webp', 'tiff', 'ico'] }],
      multiple: false,
    })
    if (path && typeof path === 'string') {
      try {
        await importImage(path)
      } catch (error) {
        await message(String(error), { title: '导入失败', type: 'error' })
      }
    }
  }

  const handleImportImageAsLayer = async () => {
    const path = await open({
      filters: [{ name: '图片文件', extensions: ['png', 'jpg', 'jpeg', 'gif', 'bmp', 'webp', 'tiff', 'ico'] }],
      multiple: false,
    })
    if (path && typeof path === 'string') {
      try {
        await importImageAsLayer(path)
      } catch (error) {
        await message(String(error), { title: '导入失败', type: 'error' })
      }
    }
  }

  const handleImportBrush = async () => {
    const path = await open({
      filters: [{ name: 'DrawConnect 画笔', extensions: ['dcbrush', 'json'] }],
      multiple: false,
    })
    if (path && typeof path === 'string') {
      try {
        await importBrush(path)
        await message('画笔导入成功', { title: '成功', type: 'info' })
      } catch (error) {
        await message(String(error), { title: '导入失败', type: 'error' })
      }
    }
  }

  return (
    <>
      <div className="menu-bar">
        <div className="menu-item">
          <span className="menu-title">{t('menu.file')}</span>
          <div className="menu-dropdown">
            <button onClick={handleBackToStart}>返回首页</button>
            <div className="menu-divider" />
            {appMode === 'draw' && (
              <>
                <button onClick={() => setShowNewCanvasDialog(true)}>{t('menu.newCanvas')}</button>
                <button onClick={handleSave}>{t('menu.save')}</button>
                <div className="menu-divider" />
              </>
            )}
            <button onClick={handleImportImage}>{t('menu.importImage')}</button>
            {appMode === 'draw' && (
              <>
                <button onClick={handleImportImageAsLayer}>{t('menu.importImageAsLayer')}</button>
                <button onClick={handleImportBrush}>{t('menu.importBrush')}</button>
              </>
            )}
            <div className="menu-divider" />
            <button onClick={handleExportPng}>{t('menu.exportPng')}</button>
          </div>
        </div>
        <div className="menu-item">
          <span className="menu-title">{t('menu.edit')}</span>
          <div className="menu-dropdown">
            <button onClick={undo} disabled={!canUndo}>
              {t('menu.undo')} <span className="shortcut">Ctrl+Z</span>
            </button>
            <button onClick={redo} disabled={!canRedo}>
              {t('menu.redo')} <span className="shortcut">Ctrl+Y</span>
            </button>
          </div>
        </div>
        <div className="menu-item">
          <span className="menu-title">{t('menu.view')}</span>
          <div className="menu-dropdown">
            <button onClick={zoomIn}>
              {t('menu.zoomIn')} <span className="shortcut">Ctrl++</span>
            </button>
            <button onClick={zoomOut}>
              {t('menu.zoomOut')} <span className="shortcut">Ctrl+-</span>
            </button>
            <button onClick={() => { resetZoom(); resetPan(); }}>
              {t('menu.fitToScreen')}
            </button>
            <button onClick={resetZoom}>
              {t('menu.actualSize')} <span className="shortcut">Ctrl+0</span>
            </button>
          </div>
        </div>
        <div className="menu-item">
          <span className="menu-title">{t('menu.image')}</span>
          <div className="menu-dropdown">
            <div className="menu-submenu">
              <span>{t('menu.adjustments')}</span>
              <div className="submenu-dropdown">
                <button onClick={() => setActiveAdjustment('brightness_contrast')}>{t('menu.brightnessContrast')}</button>
                <button onClick={() => setActiveAdjustment('levels')}>{t('menu.levels')}</button>
                <button onClick={() => setActiveAdjustment('hue_saturation')}>{t('menu.hueSaturation')}</button>
                <div className="menu-divider" />
                <button onClick={() => adjustInvert()}>{t('menu.invert')}</button>
                <button onClick={() => setActiveAdjustment('posterize')}>{t('menu.posterize')}</button>
                <button onClick={() => setActiveAdjustment('threshold')}>{t('menu.threshold')}</button>
              </div>
            </div>
            <div className="menu-divider" />
            <div className="menu-submenu">
              <span>{t('menu.rotate')}</span>
              <div className="submenu-dropdown">
                <button onClick={() => transformRotate90CW()}>{t('menu.rotate90CW')}</button>
                <button onClick={() => transformRotate90CCW()}>{t('menu.rotate90CCW')}</button>
                <button onClick={() => transformRotate180()}>{t('menu.rotate180')}</button>
              </div>
            </div>
            <button onClick={() => transformFlipHorizontal()}>{t('menu.flipHorizontal')}</button>
            <button onClick={() => transformFlipVertical()}>{t('menu.flipVertical')}</button>
          </div>
        </div>
        <div className="menu-item">
          <span className="menu-title">{t('menu.filter')}</span>
          <div className="menu-dropdown">
            <div className="menu-submenu">
              <span>{t('menu.blur')}</span>
              <div className="submenu-dropdown">
                <button onClick={() => setActiveAdjustment('gaussian_blur')}>{t('menu.gaussianBlur')}</button>
              </div>
            </div>
            <div className="menu-submenu">
              <span>{t('menu.stylize')}</span>
              <div className="submenu-dropdown">
                <button onClick={() => filterFindEdges()}>{t('menu.findEdges')}</button>
                <button onClick={() => setActiveAdjustment('emboss')}>{t('menu.emboss')}</button>
                <button onClick={() => setActiveAdjustment('oil_paint')}>{t('menu.oilPaint')}</button>
              </div>
            </div>
            <div className="menu-divider" />
            <button onClick={() => setActiveAdjustment('pixelate')}>{t('menu.pixelate')}</button>
          </div>
        </div>
        {appMode === 'draw' && (
          <div className="menu-item">
            <span className="menu-title">{t('menu.tools')}</span>
            <div className="menu-dropdown">
              <button onClick={() => setShowPluginManager(true)}>{t('menu.pluginManager')}</button>
            </div>
          </div>
        )}
        <div className="menu-item">
          <span className="menu-title">{t('menu.help')}</span>
          <div className="menu-dropdown">
            <button>{t('menu.about')}</button>
          </div>
        </div>
      </div>

      <NewCanvasDialog
        isOpen={showNewCanvasDialog}
        onClose={() => setShowNewCanvasDialog(false)}
        onCreate={handleNewCanvas}
      />

      <PluginManager
        isOpen={showPluginManager}
        onClose={() => setShowPluginManager(false)}
      />
    </>
  )
}
