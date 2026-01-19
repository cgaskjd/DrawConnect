import { useState } from 'react'
import { t } from '../i18n'
import './NewCanvasDialog.css'

interface NewCanvasDialogProps {
  isOpen: boolean
  onClose: () => void
  onCreate: (width: number, height: number) => void
}

// 预设尺寸
const presets = [
  { name: 'HD (1920×1080)', width: 1920, height: 1080 },
  { name: '2K (2560×1440)', width: 2560, height: 1440 },
  { name: '4K (3840×2160)', width: 3840, height: 2160 },
  { name: 'A4 横向 (3508×2480)', width: 3508, height: 2480 },
  { name: 'A4 纵向 (2480×3508)', width: 2480, height: 3508 },
  { name: '正方形 (2048×2048)', width: 2048, height: 2048 },
  { name: '社交媒体 (1080×1080)', width: 1080, height: 1080 },
  { name: '手机壁纸 (1080×1920)', width: 1080, height: 1920 },
]

export function NewCanvasDialog({ isOpen, onClose, onCreate }: NewCanvasDialogProps) {
  const [width, setWidth] = useState(1920)
  const [height, setHeight] = useState(1080)
  const [selectedPreset, setSelectedPreset] = useState(0)

  if (!isOpen) return null

  const handlePresetChange = (index: number) => {
    setSelectedPreset(index)
    setWidth(presets[index].width)
    setHeight(presets[index].height)
  }

  const handleWidthChange = (value: number) => {
    setWidth(Math.max(1, Math.min(16384, value)))
    setSelectedPreset(-1)
  }

  const handleHeightChange = (value: number) => {
    setHeight(Math.max(1, Math.min(16384, value)))
    setSelectedPreset(-1)
  }

  const handleCreate = () => {
    onCreate(width, height)
    // 不要在这里调用 onClose，让父组件处理关闭
  }

  const handleSwapDimensions = () => {
    setWidth(height)
    setHeight(width)
    setSelectedPreset(-1)
  }

  return (
    <div className="dialog-overlay" onClick={onClose}>
      <div className="dialog-content" onClick={(e) => e.stopPropagation()}>
        <div className="dialog-header">
          <h2>{t('dialog.newCanvas')}</h2>
          <button className="dialog-close" onClick={onClose}>×</button>
        </div>

        <div className="dialog-body">
          {/* 预设选择 */}
          <div className="form-group">
            <label>预设尺寸</label>
            <div className="preset-grid">
              {presets.map((preset, index) => (
                <button
                  key={index}
                  className={`preset-btn ${selectedPreset === index ? 'active' : ''}`}
                  onClick={() => handlePresetChange(index)}
                >
                  {preset.name}
                </button>
              ))}
            </div>
          </div>

          {/* 自定义尺寸 */}
          <div className="form-group">
            <label>自定义尺寸</label>
            <div className="dimension-inputs">
              <div className="dimension-input">
                <span className="dimension-label">{t('dialog.width')}</span>
                <input
                  type="number"
                  value={width}
                  onChange={(e) => handleWidthChange(parseInt(e.target.value) || 0)}
                  min={1}
                  max={16384}
                />
                <span className="dimension-unit">px</span>
              </div>
              <button className="swap-btn" onClick={handleSwapDimensions} title="交换宽高">
                ⇄
              </button>
              <div className="dimension-input">
                <span className="dimension-label">{t('dialog.height')}</span>
                <input
                  type="number"
                  value={height}
                  onChange={(e) => handleHeightChange(parseInt(e.target.value) || 0)}
                  min={1}
                  max={16384}
                />
                <span className="dimension-unit">px</span>
              </div>
            </div>
          </div>

          {/* 预览 */}
          <div className="form-group">
            <label>预览</label>
            <div className="canvas-preview-container">
              <div
                className="canvas-preview"
                style={{
                  aspectRatio: `${width} / ${height}`,
                  maxWidth: width > height ? '200px' : `${200 * width / height}px`,
                  maxHeight: height > width ? '150px' : `${150 * height / width}px`,
                }}
              />
              <span className="preview-size">{width} × {height} 像素</span>
            </div>
          </div>
        </div>

        <div className="dialog-footer">
          <button className="btn btn-secondary" onClick={onClose}>
            {t('dialog.cancel')}
          </button>
          <button className="btn btn-primary" onClick={handleCreate}>
            {t('dialog.create')}
          </button>
        </div>
      </div>
    </div>
  )
}
