import { open, save, message } from '@tauri-apps/api/dialog'
import { Download, Upload, FileImage } from 'lucide-react'
import { useAppStore } from '../stores/appStore'
import { t, getBrushName } from '../i18n'
import './BrushPanel.css'

export function BrushPanel() {
  const {
    brushes,
    currentBrush,
    brushSize,
    brushOpacity,
    setBrushSize,
    setBrushOpacity,
    setCurrentBrush,
    importBrush,
    exportBrush,
    importAbrBrushes,
  } = useAppStore()

  const handleImportBrush = async () => {
    const path = await open({
      filters: [{ name: 'DrawConnect 画笔', extensions: ['dcbrush', 'json'] }],
      multiple: false,
    })
    if (path && typeof path === 'string') {
      try {
        await importBrush(path)
        await message(t('import.success').replace('{count}', '1').replace('{type}', t('brush.title')), { title: t('common.success'), type: 'info' })
      } catch (error) {
        await message(String(error), { title: t('import.failed'), type: 'error' })
      }
    }
  }

  const handleImportPsBrush = async () => {
    const path = await open({
      filters: [{ name: t('import.abrBrush'), extensions: ['abr'] }],
      multiple: false,
    })
    if (path && typeof path === 'string') {
      try {
        const brushes = await importAbrBrushes(path)
        const count = brushes.length
        await message(
          t('import.success').replace('{count}', String(count)).replace('{type}', t('brush.title')),
          { title: t('common.success'), type: 'info' }
        )
      } catch (error) {
        await message(String(error), { title: t('import.failed'), type: 'error' })
      }
    }
  }

  const handleExportBrush = async () => {
    if (!currentBrush) {
      await message(t('brush.selectFirst'), { title: t('common.tip'), type: 'warning' })
      return
    }
    const path = await save({
      filters: [{ name: 'DrawConnect 画笔', extensions: ['dcbrush'] }],
      defaultPath: `${currentBrush}.dcbrush`,
    })
    if (path) {
      try {
        await exportBrush(currentBrush, path)
        await message(t('brush.exportSuccess'), { title: t('common.success'), type: 'info' })
      } catch (error) {
        await message(String(error), { title: t('brush.exportFailed'), type: 'error' })
      }
    }
  }

  return (
    <div className="panel brush-panel">
      <div className="panel-header">
        {t('brush.title')}
        <div className="brush-actions">
          <button
            className="icon-btn small"
            onClick={handleImportBrush}
            title={t('brush.import')}
          >
            <Download size={14} />
          </button>
          <button
            className="icon-btn small"
            onClick={handleImportPsBrush}
            title={t('import.abrBrush')}
          >
            <FileImage size={14} />
          </button>
          <button
            className="icon-btn small"
            onClick={handleExportBrush}
            title={t('brush.export')}
          >
            <Upload size={14} />
          </button>
        </div>
      </div>
      <div className="panel-content">
        <div className="brush-setting">
          <label>
            {t('brush.size')}
            <div className="setting-input-group">
              <input
                type="number"
                className="setting-input"
                min="1"
                max="500"
                value={brushSize}
                onChange={(e) => {
                  const value = Math.max(1, Math.min(500, Number(e.target.value) || 1))
                  setBrushSize(value)
                }}
              />
              <span className="setting-unit">px</span>
            </div>
          </label>
          <input
            type="range"
            className="slider"
            min="1"
            max="500"
            value={brushSize}
            onChange={(e) => setBrushSize(Number(e.target.value))}
          />
        </div>
        <div className="brush-setting">
          <label>
            {t('brush.opacity')}
            <div className="setting-input-group">
              <input
                type="number"
                className="setting-input"
                min="0"
                max="100"
                value={Math.round(brushOpacity * 100)}
                onChange={(e) => {
                  const value = Math.max(0, Math.min(100, Number(e.target.value) || 0))
                  setBrushOpacity(value / 100)
                }}
              />
              <span className="setting-unit">%</span>
            </div>
          </label>
          <input
            type="range"
            className="slider"
            min="0"
            max="100"
            value={brushOpacity * 100}
            onChange={(e) => setBrushOpacity(Number(e.target.value) / 100)}
          />
        </div>
        <div className="brush-presets">
          <div className="panel-header" style={{ padding: '8px 0', background: 'none' }}>
            {t('brush.presets')}
          </div>
          <div className="brush-list">
            {brushes.map((brush) => (
              <button
                key={brush.name}
                className={`brush-item ${currentBrush === brush.name ? 'active' : ''}`}
                onClick={() => setCurrentBrush(brush.name)}
              >
                <div className="brush-preview" />
                <span>{getBrushName(brush.name)}</span>
              </button>
            ))}
          </div>
        </div>
      </div>
    </div>
  )
}
