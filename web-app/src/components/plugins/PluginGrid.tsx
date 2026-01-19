import { Plugin } from '../../api/client'
import PluginCard from './PluginCard'

interface PluginGridProps {
  plugins: Plugin[]
  onPluginClick: (plugin: Plugin) => void
  showStatus?: boolean
}

export default function PluginGrid({ plugins, onPluginClick, showStatus = false }: PluginGridProps) {
  if (plugins.length === 0) {
    return (
      <div className="plugin-empty">
        <span className="plugin-empty-icon">📦</span>
        <p className="plugin-empty-text">No plugins found</p>
      </div>
    )
  }

  return (
    <div className="plugin-grid">
      {plugins.map(plugin => (
        <PluginCard
          key={plugin.id}
          plugin={plugin}
          onClick={() => onPluginClick(plugin)}
          showStatus={showStatus}
        />
      ))}
    </div>
  )
}
