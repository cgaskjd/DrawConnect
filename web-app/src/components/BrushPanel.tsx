// Note: Web version - file dialogs use browser-native APIs instead of Tauri
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
  } = useAppStore()

  // TODO: Web implementation - use browser file input for importing brushes
  const handleImportBrush = async () => {
    // Web implementation would use <input type="file"> or File System Access API
    console.log('Import brush - web implementation needed')
    alert('Brush import not yet implemented for web version')
  }

  // TODO: Web implementation - use browser file input for importing PS brushes
  const handleImportPsBrush = async () => {
    // Web implementation would use <input type="file"> or File System Access API
    console.log('Import PS brush - web implementation needed')
    alert('PS brush import not yet implemented for web version')
  }

  // TODO: Web implementation - use download link or File System Access API
  const handleExportBrush = async () => {
    if (!currentBrush) {
      alert(t('brush.selectFirst'))
      return
    }
    // Web implementation would create a Blob and trigger download
    console.log('Export brush - web implementation needed')
    alert('Brush export not yet implemented for web version')
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
