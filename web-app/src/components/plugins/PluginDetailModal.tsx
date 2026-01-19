import { Plugin, PluginAuthor, getPluginDownloadUrl } from '../../api/client'
import { usePluginStore, CATEGORY_LABELS } from '../../stores/pluginStore'
import { getAuthToken } from '../../api/client'

interface PluginDetailModalProps {
  plugin: Plugin
  onClose: () => void
}

export default function PluginDetailModal({ plugin, onClose }: PluginDetailModalProps) {
  const { toggleLike } = usePluginStore()
  const isAuthenticated = !!getAuthToken()

  const author = typeof plugin.authorId === 'object' && plugin.authorId ? plugin.authorId as PluginAuthor : null
  const authorName = author?.username || plugin.authorName || 'Anonymous'
  const authorInitial = authorName.charAt(0).toUpperCase()

  const formatFileSize = (bytes: number): string => {
    if (bytes < 1024) return bytes + ' B'
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
    return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
  }

  const formatDate = (dateStr: string): string => {
    const date = new Date(dateStr)
    return date.toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric'
    })
  }

  const handleDownload = () => {
    if (plugin.status !== 'approved') {
      alert('This plugin is not available for download')
      return
    }
    window.open(getPluginDownloadUrl(plugin.id), '_blank')
  }

  const handleLike = () => {
    if (!isAuthenticated) {
      alert('Please login to like plugins')
      return
    }
    toggleLike(plugin.id)
  }

  const handleOverlayClick = (e: React.MouseEvent) => {
    if (e.target === e.currentTarget) {
      onClose()
    }
  }

  return (
    <div className="plugin-modal-overlay" onClick={handleOverlayClick}>
      <div className="plugin-modal">
        <div className="plugin-modal-header">
          <h2 className="plugin-modal-title">Plugin Details</h2>
          <button className="plugin-modal-close" onClick={onClose}>
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M18 6L6 18M6 6l12 12" />
            </svg>
          </button>
        </div>

        <div className="plugin-modal-content">
          <div className="plugin-detail-info">
            <div className="plugin-detail-thumbnail">
              {plugin.thumbnailUrl ? (
                <img src={plugin.thumbnailUrl} alt={plugin.name} />
              ) : (
                <span className="plugin-detail-thumbnail-placeholder">
                  {getCategoryIcon(plugin.category)}
                </span>
              )}
            </div>

            <div className="plugin-detail-main">
              <h3 className="plugin-detail-name">{plugin.name}</h3>
              <p className="plugin-detail-version">
                Version {plugin.version} &middot; {CATEGORY_LABELS[plugin.category]} &middot; {formatFileSize(plugin.fileSize)}
              </p>

              <div className="plugin-detail-author">
                <div className="plugin-detail-author-avatar">
                  {author?.avatarUrl ? (
                    <img src={author.avatarUrl} alt={authorName} style={{ width: '100%', height: '100%', borderRadius: '50%' }} />
                  ) : (
                    authorInitial
                  )}
                </div>
                <span className="plugin-detail-author-name">By {authorName}</span>
              </div>

              <div className="plugin-detail-stats">
                <div className="plugin-detail-stat">
                  <svg viewBox="0 0 24 24" fill={plugin.liked ? 'currentColor' : 'none'} stroke="currentColor" strokeWidth="2">
                    <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" />
                  </svg>
                  {plugin.likeCount} likes
                </div>
                <div className="plugin-detail-stat">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                    <polyline points="7 10 12 15 17 10" />
                    <line x1="12" y1="15" x2="12" y2="3" />
                  </svg>
                  {plugin.downloadCount} downloads
                </div>
                <div className="plugin-detail-stat">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                    <rect x="3" y="4" width="18" height="18" rx="2" ry="2" />
                    <line x1="16" y1="2" x2="16" y2="6" />
                    <line x1="8" y1="2" x2="8" y2="6" />
                    <line x1="3" y1="10" x2="21" y2="10" />
                  </svg>
                  {formatDate(plugin.createdAt)}
                </div>
              </div>
            </div>
          </div>

          <div className="plugin-detail-description">
            <h3>Description</h3>
            <p>{plugin.description}</p>
          </div>

          {plugin.tags.length > 0 && (
            <div className="plugin-detail-tags">
              {plugin.tags.map((tag, index) => (
                <span key={index} className="plugin-detail-tag">
                  #{tag}
                </span>
              ))}
            </div>
          )}

          <div className="plugin-detail-actions">
            <button
              className="plugin-detail-btn plugin-detail-btn-primary"
              onClick={handleDownload}
              disabled={plugin.status !== 'approved'}
            >
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="7 10 12 15 17 10" />
                <line x1="12" y1="15" x2="12" y2="3" />
              </svg>
              Download
            </button>
            <button
              className={`plugin-detail-btn plugin-detail-btn-like ${plugin.liked ? 'liked' : ''}`}
              onClick={handleLike}
            >
              <svg width="18" height="18" viewBox="0 0 24 24" fill={plugin.liked ? 'currentColor' : 'none'} stroke="currentColor" strokeWidth="2">
                <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" />
              </svg>
              {plugin.liked ? 'Liked' : 'Like'}
            </button>
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
