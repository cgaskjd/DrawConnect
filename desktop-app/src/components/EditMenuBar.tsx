import { save, open, message } from '@tauri-apps/api/dialog'
import { useAppStore } from '../stores/appStore'
import { usePluginStore } from '../stores/pluginStore'
import { t } from '../i18n'
import './MenuBar.css'

interface EditMenuBarProps {
  onOpenPluginManager?: () => void
}

/**
 * EditMenuBar - 修图模式专用的菜单栏
 * 只包含修图相关的功能，不包含画笔、图层等绘画功能
 */
export function EditMenuBar({ onOpenPluginManager }: EditMenuBarProps) {
  const {
    setAppMode,
    undo,
    redo,
    canUndo,
    canRedo,
    exportPng,
    importImage,
    zoomIn,
    zoomOut,
    resetZoom,
    resetPan,
    setActiveAdjustment,
    isInitialized,
    // Image adjustments
    adjustInvert,
    // Filters
    filterFindEdges,
    // Distort filters
    filterSpherize,
    filterTwirl,
    filterWave,
    filterRipple,
    // Render filters
    filterVignette,
    filterLensFlare,
    filterClouds,
    // Transforms
    transformRotate90CW,
    transformRotate90CCW,
    transformRotate180,
    transformFlipHorizontal,
    transformFlipVertical,
    // PS imports
    importAbrBrushes,
    importPatPatterns,
    importColorSwatches,
  } = useAppStore()

  const { capabilities } = usePluginStore()

  const handleBackToStart = () => {
    // Reset everything and go back to start screen
    window.location.reload()
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
        await message(String(error), { title: t('import.failed'), type: 'error' })
      }
    }
  }

  // PS 资源导入处理函数
  const handleImportPsBrushes = async () => {
    const path = await open({
      filters: [{ name: t('import.abrBrush'), extensions: ['abr'] }],
      multiple: false,
    })
    if (path && typeof path === 'string') {
      try {
        const brushes = await importAbrBrushes(path)
        await message(
          t('import.success', { count: brushes.length, type: t('import.brushType') }),
          { title: t('common.success'), type: 'info' }
        )
      } catch (error) {
        await message(String(error), { title: t('import.failed'), type: 'error' })
      }
    }
  }

  const handleImportPsPatterns = async () => {
    const path = await open({
      filters: [{ name: t('import.patPattern'), extensions: ['pat'] }],
      multiple: false,
    })
    if (path && typeof path === 'string') {
      try {
        const patterns = await importPatPatterns(path)
        await message(
          t('import.success', { count: patterns.length, type: t('import.patternType') }),
          { title: t('common.success'), type: 'info' }
        )
      } catch (error) {
        await message(String(error), { title: t('import.failed'), type: 'error' })
      }
    }
  }

  const handleImportPsSwatches = async () => {
    const path = await open({
      filters: [{ name: t('import.colorSwatch'), extensions: ['aco', 'ase'] }],
      multiple: false,
    })
    if (path && typeof path === 'string') {
      try {
        const swatches = await importColorSwatches(path)
        await message(
          t('import.success', { count: swatches.length, type: t('import.swatchType') }),
          { title: t('common.success'), type: 'info' }
        )
      } catch (error) {
        await message(String(error), { title: t('import.failed'), type: 'error' })
      }
    }
  }

  return (
    <div className="menu-bar">
      <div className="menu-item">
        <span className="menu-title">{t('menu.file')}</span>
        <div className="menu-dropdown">
          <button onClick={handleBackToStart}>返回首页</button>
          <div className="menu-divider" />
          <button onClick={handleImportImage}>{t('menu.importImage')}</button>
          <div className="menu-divider" />
          <button onClick={handleExportPng} disabled={!isInitialized}>{t('menu.exportPng')}</button>
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
              <button onClick={() => setActiveAdjustment('brightness_contrast')} disabled={!isInitialized}>{t('menu.brightnessContrast')}</button>
              <button onClick={() => setActiveAdjustment('levels')} disabled={!isInitialized}>{t('menu.levels')}</button>
              <button onClick={() => setActiveAdjustment('hue_saturation')} disabled={!isInitialized}>{t('menu.hueSaturation')}</button>
              <div className="menu-divider" />
              <button onClick={() => adjustInvert()} disabled={!isInitialized}>{t('menu.invert')}</button>
              <button onClick={() => setActiveAdjustment('posterize')} disabled={!isInitialized}>{t('menu.posterize')}</button>
              <button onClick={() => setActiveAdjustment('threshold')} disabled={!isInitialized}>{t('menu.threshold')}</button>
            </div>
          </div>
          <div className="menu-divider" />
          <div className="menu-submenu">
            <span>{t('menu.rotate')}</span>
            <div className="submenu-dropdown">
              <button onClick={() => transformRotate90CW()} disabled={!isInitialized}>{t('menu.rotate90CW')}</button>
              <button onClick={() => transformRotate90CCW()} disabled={!isInitialized}>{t('menu.rotate90CCW')}</button>
              <button onClick={() => transformRotate180()} disabled={!isInitialized}>{t('menu.rotate180')}</button>
            </div>
          </div>
          <button onClick={() => transformFlipHorizontal()} disabled={!isInitialized}>{t('menu.flipHorizontal')}</button>
          <button onClick={() => transformFlipVertical()} disabled={!isInitialized}>{t('menu.flipVertical')}</button>
        </div>
      </div>
      <div className="menu-item">
        <span className="menu-title">{t('menu.filter')}</span>
        <div className="menu-dropdown">
          <div className="menu-submenu">
            <span>{t('menu.blur')}</span>
            <div className="submenu-dropdown">
              <button onClick={() => setActiveAdjustment('gaussian_blur')} disabled={!isInitialized}>{t('menu.gaussianBlur')}</button>
            </div>
          </div>
          <div className="menu-submenu">
            <span>{t('menu.distort')}</span>
            <div className="submenu-dropdown">
              <button onClick={() => filterSpherize(50)} disabled={!isInitialized}>{t('menu.spherize')}</button>
              <button onClick={() => filterTwirl(90)} disabled={!isInitialized}>{t('menu.twirl')}</button>
              <button onClick={() => filterWave(120, 35)} disabled={!isInitialized}>{t('menu.wave')}</button>
              <button onClick={() => filterRipple(100)} disabled={!isInitialized}>{t('menu.ripple')}</button>
            </div>
          </div>
          <div className="menu-submenu">
            <span>{t('menu.stylize')}</span>
            <div className="submenu-dropdown">
              <button onClick={() => filterFindEdges()} disabled={!isInitialized}>{t('menu.findEdges')}</button>
              <button onClick={() => setActiveAdjustment('emboss')} disabled={!isInitialized}>{t('menu.emboss')}</button>
              <button onClick={() => setActiveAdjustment('oil_paint')} disabled={!isInitialized}>{t('menu.oilPaint')}</button>
            </div>
          </div>
          <div className="menu-submenu">
            <span>{t('menu.render')}</span>
            <div className="submenu-dropdown">
              <button onClick={() => filterVignette(-50)} disabled={!isInitialized}>{t('menu.vignette')}</button>
              <button onClick={() => filterLensFlare(50, 50, 100)} disabled={!isInitialized}>{t('menu.lensFlare')}</button>
              <button onClick={() => filterClouds()} disabled={!isInitialized}>{t('menu.clouds')}</button>
            </div>
          </div>
          <div className="menu-divider" />
          <button onClick={() => setActiveAdjustment('pixelate')} disabled={!isInitialized}>{t('menu.pixelate')}</button>
          {/* Plugin Filters */}
          {capabilities.filters.length > 0 && (
            <>
              <div className="menu-divider" />
              <div className="menu-submenu">
                <span>{t('menu.pluginFilters')}</span>
                <div className="submenu-dropdown">
                  {capabilities.filters.map((filter) => (
                    <button key={filter.id} disabled={!isInitialized}>
                      {filter.name}
                    </button>
                  ))}
                </div>
              </div>
            </>
          )}
        </div>
      </div>
      <div className="menu-item">
        <span className="menu-title">{t('menu.tools')}</span>
        <div className="menu-dropdown">
          <div className="menu-submenu">
            <span>{t('menu.importPsResources')}</span>
            <div className="submenu-dropdown">
              <button onClick={handleImportPsBrushes}>{t('import.abrBrush')}</button>
              <button onClick={handleImportPsPatterns}>{t('import.patPattern')}</button>
              <button onClick={handleImportPsSwatches}>{t('import.colorSwatch')}</button>
            </div>
          </div>
          <div className="menu-divider" />
          <button onClick={onOpenPluginManager}>{t('menu.pluginManager')}</button>
        </div>
      </div>
      <div className="menu-item">
        <span className="menu-title">{t('menu.help')}</span>
        <div className="menu-dropdown">
          <button>{t('menu.about')}</button>
        </div>
      </div>
    </div>
  )
}
