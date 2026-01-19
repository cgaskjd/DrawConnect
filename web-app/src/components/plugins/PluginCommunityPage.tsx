import { useEffect, useState } from 'react'
import { usePluginStore, CATEGORY_LABELS } from '../../stores/pluginStore'
import { useNavigationStore } from '../../stores/navigationStore'
import { getAuthToken, PluginCategory } from '../../api/client'
import PluginSearchBar from './PluginSearchBar'
import PluginGrid from './PluginGrid'
import PluginDetailModal from './PluginDetailModal'
import PluginUploadModal from './PluginUploadModal'
import '../../styles/plugins.css'

type TabType = 'browse' | 'my-plugins'

export default function PluginCommunityPage() {
  const { setPageMode } = useNavigationStore()
  const {
    plugins,
    myPlugins,
    selectedPlugin,
    isLoading,
    error,
    currentPage,
    totalPages,
    selectedCategory,
    setSelectedCategory,
    showDetailModal,
    showUploadModal,
    fetchPlugins,
    fetchMyPlugins,
    openDetailModal,
    closeDetailModal,
    openUploadModal,
    closeUploadModal,
    setCurrentPage,
    clearError,
  } = usePluginStore()

  const [activeTab, setActiveTab] = useState<TabType>('browse')
  const [showTutorial, setShowTutorial] = useState(true)
  const isAuthenticated = !!getAuthToken()

  useEffect(() => {
    fetchPlugins()
    if (isAuthenticated) {
      fetchMyPlugins()
    }
  }, [])

  const handleBack = () => {
    setPageMode('landing')
  }

  const handleCategoryClick = (category: PluginCategory | 'all') => {
    setSelectedCategory(category)
    fetchPlugins({ category })
  }

  const handlePageChange = (page: number) => {
    setCurrentPage(page)
    fetchPlugins({ page })
  }

  const handleUploadClick = () => {
    if (!isAuthenticated) {
      alert('Please login to upload plugins')
      return
    }
    openUploadModal()
  }

  const handleTabChange = (tab: TabType) => {
    setActiveTab(tab)
    if (tab === 'my-plugins' && isAuthenticated) {
      fetchMyPlugins()
    }
  }

  const categories: (PluginCategory | 'all')[] = [
    'all', 'brushes', 'filters', 'tools', 'panels', 'themes', 'automation', 'other'
  ]

  return (
    <div className="plugin-community-page">
      {/* Header */}
      <header className="plugin-header">
        <div className="plugin-header-left">
          <button className="plugin-back-btn" onClick={handleBack}>
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M19 12H5M12 19l-7-7 7-7" />
            </svg>
            Back
          </button>
          <div className="plugin-header-title">
            <h1>Plugin Community</h1>
          </div>
        </div>
        <div className="plugin-header-right">
          {isAuthenticated && (
            <button className="plugin-upload-btn" onClick={handleUploadClick}>
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="17 8 12 3 7 8" />
                <line x1="12" y1="3" x2="12" y2="15" />
              </svg>
              Upload Plugin
            </button>
          )}
        </div>
      </header>

      {/* Developer Tutorial Banner */}
      {showTutorial && (
        <div className="plugin-tutorial-banner">
          <div className="plugin-tutorial-content">
            <div className="plugin-tutorial-icon">🛠️</div>
            <div className="plugin-tutorial-text">
              <h3>Create Your Own Plugin</h3>
              <p>Learn how to develop plugins for DrawConnect in just a few steps!</p>
            </div>
            <div className="plugin-tutorial-steps">
              <div className="plugin-tutorial-step">
                <span className="step-number">1</span>
                <div className="step-content">
                  <strong>Create manifest.json</strong>
                  <code>{`{
  "id": "my-plugin",
  "name": "My Plugin",
  "version": "1.0.0",
  "main": "main.js",
  "category": "filters"
}`}</code>
                </div>
              </div>
              <div className="plugin-tutorial-step">
                <span className="step-number">2</span>
                <div className="step-content">
                  <strong>Write main.js</strong>
                  <code>{`class MyPlugin {
  async init(api) {
    api.registerFilter({
      id: 'my-filter',
      name: 'My Filter',
      execute: this.apply
    });
  }
  async apply() { /* ... */ }
}
export default MyPlugin;`}</code>
                </div>
              </div>
              <div className="plugin-tutorial-step">
                <span className="step-number">3</span>
                <div className="step-content">
                  <strong>Package & Upload</strong>
                  <code>{`my-plugin.zip
├── manifest.json
├── main.js
├── icon.svg (optional)
└── README.md (optional)`}</code>
                </div>
              </div>
            </div>
            <div className="plugin-tutorial-actions">
              <a href="https://docs.drawconnect.com/plugins" target="_blank" rel="noopener noreferrer" className="plugin-tutorial-btn primary">
                View Full Documentation
              </a>
              <a href="/uploads/plugins/skin-smoothing-filter.zip" download className="plugin-tutorial-btn secondary">
                Download Example Plugin
              </a>
            </div>
          </div>
          <button className="plugin-tutorial-close" onClick={() => setShowTutorial(false)}>
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M18 6L6 18M6 6l12 12" />
            </svg>
          </button>
        </div>
      )}

      {/* Tabs */}
      <div className="plugin-category-tabs">
        <button
          className={`plugin-category-tab ${activeTab === 'browse' ? 'active' : ''}`}
          onClick={() => handleTabChange('browse')}
        >
          Browse Plugins
        </button>
        {isAuthenticated && (
          <button
            className={`plugin-category-tab ${activeTab === 'my-plugins' ? 'active' : ''}`}
            onClick={() => handleTabChange('my-plugins')}
          >
            My Plugins ({myPlugins.length})
          </button>
        )}
      </div>

      {/* Search and Filter */}
      {activeTab === 'browse' && <PluginSearchBar />}

      {/* Category Tabs */}
      {activeTab === 'browse' && (
        <div className="plugin-category-tabs" style={{ background: 'transparent', borderBottom: 'none' }}>
          {categories.map(category => (
            <button
              key={category}
              className={`plugin-category-tab ${selectedCategory === category ? 'active' : ''}`}
              onClick={() => handleCategoryClick(category)}
            >
              {CATEGORY_LABELS[category]}
            </button>
          ))}
        </div>
      )}

      {/* Error Message */}
      {error && (
        <div className="plugin-error">
          <span className="plugin-error-text">{error}</span>
          <button className="plugin-error-close" onClick={clearError}>
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M18 6L6 18M6 6l12 12" />
            </svg>
          </button>
        </div>
      )}

      {/* Content */}
      <div className="plugin-content">
        {isLoading ? (
          <div className="plugin-loading">
            <div className="plugin-loading-spinner" />
            <p className="plugin-loading-text">Loading plugins...</p>
          </div>
        ) : activeTab === 'browse' ? (
          <>
            <PluginGrid
              plugins={plugins}
              onPluginClick={openDetailModal}
            />

            {/* Pagination */}
            {totalPages > 1 && (
              <div className="plugin-pagination">
                <button
                  className="plugin-pagination-btn"
                  onClick={() => handlePageChange(currentPage - 1)}
                  disabled={currentPage <= 1}
                >
                  Previous
                </button>
                <span className="plugin-pagination-info">
                  Page {currentPage} of {totalPages}
                </span>
                <button
                  className="plugin-pagination-btn"
                  onClick={() => handlePageChange(currentPage + 1)}
                  disabled={currentPage >= totalPages}
                >
                  Next
                </button>
              </div>
            )}
          </>
        ) : (
          <PluginGrid
            plugins={myPlugins}
            onPluginClick={openDetailModal}
            showStatus
          />
        )}
      </div>

      {/* Detail Modal */}
      {showDetailModal && selectedPlugin && (
        <PluginDetailModal
          plugin={selectedPlugin}
          onClose={closeDetailModal}
        />
      )}

      {/* Upload Modal */}
      {showUploadModal && (
        <PluginUploadModal
          onClose={closeUploadModal}
        />
      )}
    </div>
  )
}
