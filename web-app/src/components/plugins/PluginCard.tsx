import { Plugin, PluginAuthor } from '../../api/client'
import { CATEGORY_LABELS } from '../../stores/pluginStore'

interface PluginCardProps {
  plugin: Plugin
  onClick: () => void
  showStatus?: boolean
}

export default function PluginCard({ plugin, onClick, showStatus = false }: PluginCardProps) {
  const author = typeof plugin.authorId === 'object' && plugin.authorId ? plugin.authorId as PluginAuthor : null
  const authorName = author?.username || plugin.authorName || 'Anonymous'
  const authorInitial = authorName.charAt(0).toUpperCase()

  const formatFileSize = (bytes: number): string => {
    if (bytes < 1024) return bytes + ' B'
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
    return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
  }

  return (
    <div className="plugin-card" onClick={onClick}>
      <div className="plugin-card-thumbnail">
        {plugin.thumbnailUrl ? (
          <img src={plugin.thumbnailUrl} alt={plugin.name} />
        ) : (
          <span className="plugin-card-placeholder">
            {getCategoryIcon(plugin.category)}
          </span>
        )}
      </div>
      <div className="plugin-card-content">
        <span className="plugin-card-category">
          {CATEGORY_LABELS[plugin.category]}
        </span>
        {showStatus && plugin.status !== 'approved' && (
          <span className={`plugin-card-status ${plugin.status}`}>
            {plugin.status === 'pending' ? 'Pending Review' :
             plugin.status === 'rejected' ? 'Rejected' : 'Unpublished'}
          </span>
        )}
        <div className="plugin-card-header">
          <h3 className="plugin-card-name">{plugin.name}</h3>
          <span className="plugin-card-version">v{plugin.version}</span>
        </div>
        <p className="plugin-card-description">
          {plugin.shortDescription || plugin.description}
        </p>
        <div className="plugin-card-meta">
          <div className="plugin-card-author">
            <div className="plugin-card-avatar">
              {author?.avatarUrl ? (
                <img src={author.avatarUrl} alt={authorName} style={{ width: '100%', height: '100%', borderRadius: '50%' }} />
              ) : (
                authorInitial
              )}
            </div>
            <span className="plugin-card-author-name">{authorName}</span>
          </div>
          <div className="plugin-card-stats">
            <span className={`plugin-card-stat ${plugin.liked ? 'liked' : ''}`}>
              <svg viewBox="0 0 24 24" fill={plugin.liked ? 'currentColor' : 'none'} stroke="currentColor" strokeWidth="2">
                <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" />
              </svg>
              {plugin.likeCount}
            </span>
            <span className="plugin-card-stat">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="7 10 12 15 17 10" />
                <line x1="12" y1="15" x2="12" y2="3" />
              </svg>
              {plugin.downloadCount}
            </span>
          </div>
        </div>
      </div>
    </div>
  )
}

function getCategoryIcon(category: string): string {
  switch (category) {
    case 'brushes': return '🖌️'
    case 'filters': return '🎨'
    case 'tools': return '🔧'
    case 'panels': return '📐'
    case 'themes': return '🎭'
    case 'automation': return '⚡'
    default: return '📦'
  }
}
