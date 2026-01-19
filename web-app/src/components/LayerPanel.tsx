import { Eye, EyeOff, Plus, Trash2, Copy, ChevronUp, ChevronDown, Merge } from 'lucide-react'
import { useAppStore } from '../stores/appStore'
import { t, getBlendModeName } from '../i18n'
import './LayerPanel.css'

export function LayerPanel() {
  const {
    layers,
    activeLayerId,
    addLayer,
    deleteLayer,
    setActiveLayer,
    setLayerVisibility,
    moveLayerUp,
    moveLayerDown,
    duplicateLayer,
    mergeLayerDown,
  } = useAppStore()

  const handleAddLayer = () => {
    addLayer(`${t('layer.layerName')} ${layers.length + 1}`)
  }

  const handleDeleteLayer = () => {
    if (activeLayerId && layers.length > 1) {
      deleteLayer(activeLayerId)
    }
  }

  const handleMoveUp = () => {
    if (activeLayerId) {
      moveLayerUp(activeLayerId)
    }
  }

  const handleMoveDown = () => {
    if (activeLayerId) {
      moveLayerDown(activeLayerId)
    }
  }

  const handleDuplicate = () => {
    if (activeLayerId) {
      duplicateLayer(activeLayerId)
    }
  }

  const handleMergeDown = () => {
    if (activeLayerId) {
      // Find the index of active layer
      const activeIndex = layers.findIndex(l => l.id === activeLayerId)
      // Can only merge if not the bottom layer
      if (activeIndex > 0) {
        mergeLayerDown(activeLayerId)
      }
    }
  }

  // Check if active layer can be moved up or down
  const activeIndex = layers.findIndex(l => l.id === activeLayerId)
  const canMoveUp = activeIndex >= 0 && activeIndex < layers.length - 1
  const canMoveDown = activeIndex > 0
  const canMergeDown = activeIndex > 0

  return (
    <div className="panel layer-panel">
      <div className="panel-header">
        {t('layer.title')}
        <div className="layer-actions">
          <button className="icon-btn small" onClick={handleAddLayer} title={t('layer.addLayer')}>
            <Plus size={14} />
          </button>
          <button
            className="icon-btn small"
            onClick={handleDuplicate}
            disabled={!activeLayerId}
            title={t('layer.duplicateLayer')}
          >
            <Copy size={14} />
          </button>
          <button
            className="icon-btn small"
            onClick={handleMergeDown}
            disabled={!canMergeDown}
            title={t('layer.mergeDown')}
          >
            <Merge size={14} />
          </button>
          <button
            className="icon-btn small"
            onClick={handleDeleteLayer}
            disabled={layers.length <= 1}
            title={t('layer.deleteLayer')}
          >
            <Trash2 size={14} />
          </button>
        </div>
      </div>
      <div className="layer-order-actions">
        <button
          className="icon-btn small"
          onClick={handleMoveUp}
          disabled={!canMoveUp}
          title={t('layer.moveUp')}
        >
          <ChevronUp size={14} />
        </button>
        <button
          className="icon-btn small"
          onClick={handleMoveDown}
          disabled={!canMoveDown}
          title={t('layer.moveDown')}
        >
          <ChevronDown size={14} />
        </button>
      </div>
      <div className="layer-list">
        {[...layers].reverse().map((layer) => (
          <div
            key={layer.id}
            className={`layer-item ${activeLayerId === layer.id ? 'active' : ''}`}
            onClick={() => setActiveLayer(layer.id)}
          >
            <button
              className="layer-visibility"
              onClick={(e) => {
                e.stopPropagation()
                setLayerVisibility(layer.id, !layer.visible)
              }}
              title={t('layer.visible')}
            >
              {layer.visible ? <Eye size={14} /> : <EyeOff size={14} />}
            </button>
            <div className="layer-thumbnail" />
            <div className="layer-info">
              <span className="layer-name">{layer.name}</span>
              <span className="layer-blend">{getBlendModeName(layer.blendMode)}</span>
            </div>
            <div className="layer-opacity">
              {Math.round(layer.opacity * 100)}%
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}
