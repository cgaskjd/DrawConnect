import { useEffect, useState } from 'react'
import { open, message, confirm } from '@tauri-apps/api/dialog'
import {
  Package,
  Power,
  PowerOff,
  Trash2,
  Download,
  FolderOpen,
  RefreshCw,
  X,
  AlertCircle,
  Check,
  Settings,
  Info
} from 'lucide-react'
import { usePluginStore, PluginInfo } from '../stores/pluginStore'
import { t } from '../i18n'
import './PluginManager.css'

interface PluginManagerProps {
  isOpen: boolean
  onClose: () => void
}

export function PluginManager({ isOpen, onClose }: PluginManagerProps) {
  const {
    isInitialized,
    isLoading,
    error,
    plugins,
    selectedPluginId,
    selectedPluginDetail,
    initialize,
    refreshPlugins,
    selectPlugin,
    installPlugin,
    uninstallPlugin,
    enablePlugin,
    disablePlugin,
    openPluginsFolder,
    clearError,
  } = usePluginStore()

  const [activeTab, setActiveTab] = useState<'installed' | 'store'>('installed')

  useEffect(() => {
    if (isOpen && !isInitialized) {
      initialize()
    }
  }, [isOpen, isInitialized, initialize])

  if (!isOpen) return null

  const handleInstallPlugin = async () => {
    const path = await open({
      filters: [
        { name: t('plugin.fileType'), extensions: ['zip', 'dcplugin'] },
      ],
      multiple: false,
      directory: false,
    })

    if (path && typeof path === 'string') {
      try {
        const plugin = await installPlugin(path)
        await message(t('plugin.installSuccess', { name: plugin.name }), {
          title: t('plugin.success'),
          type: 'info',
        })
      } catch (error) {
        await message(String(error), {
          title: t('plugin.installFailed'),
          type: 'error',
        })
      }
    }
  }

  const handleInstallFromFolder = async () => {
    const path = await open({
      directory: true,
      multiple: false,
    })

    if (path && typeof path === 'string') {
      try {
        const plugin = await installPlugin(path)
        await message(t('plugin.installSuccess', { name: plugin.name }), {
          title: t('plugin.success'),
          type: 'info',
        })
      } catch (error) {
        await message(String(error), {
          title: t('plugin.installFailed'),
          type: 'error',
        })
      }
    }
  }

  const handleUninstall = async (plugin: PluginInfo) => {
    const confirmed = await confirm(
      t('plugin.confirmUninstall', { name: plugin.name }),
      {
        title: t('plugin.uninstall'),
        type: 'warning',
      }
    )

    if (confirmed) {
      try {
        await uninstallPlugin(plugin.id)
        await message(t('plugin.uninstallSuccess'), {
          title: t('plugin.success'),
          type: 'info',
        })
      } catch (error) {
        await message(String(error), {
          title: t('plugin.uninstallFailed'),
          type: 'error',
        })
      }
    }
  }

  const handleToggleEnable = async (plugin: PluginInfo) => {
    try {
      if (plugin.state === 'enabled') {
        await disablePlugin(plugin.id)
      } else {
        await enablePlugin(plugin.id)
      }
    } catch (error) {
      await message(String(error), {
        title: t('plugin.operationFailed'),
        type: 'error',
      })
    }
  }

  const getStateIcon = (state: PluginInfo['state']) => {
    switch (state) {
      case 'enabled':
        return <Check size={14} className="state-icon enabled" />
      case 'disabled':
        return <PowerOff size={14} className="state-icon disabled" />
      case 'error':
        return <AlertCircle size={14} className="state-icon error" />
      default:
        return <Info size={14} className="state-icon installed" />
    }
  }

  const getStateLabel = (state: PluginInfo['state']) => {
    switch (state) {
      case 'enabled':
        return t('plugin.state.enabled')
      case 'disabled':
        return t('plugin.state.disabled')
      case 'error':
        return t('plugin.state.error')
      default:
        return t('plugin.state.installed')
    }
  }

  const getPluginTypeLabel = (type: string) => {
    switch (type.toLowerCase()) {
      case 'brush':
        return t('plugin.type.brush')
      case 'filter':
        return t('plugin.type.filter')
      case 'tool':
        return t('plugin.type.tool')
      case 'mixed':
        return t('plugin.type.mixed')
      default:
        return type
    }
  }

  return (
    <div className="plugin-manager-overlay" onClick={onClose}>
      <div className="plugin-manager" onClick={(e) => e.stopPropagation()}>
        {/* Header */}
        <div className="plugin-manager-header">
          <div className="header-title">
            <Package size={20} />
            <h2>{t('plugin.manager')}</h2>
          </div>
          <button className="icon-btn" onClick={onClose}>
            <X size={18} />
          </button>
        </div>

        {/* Tabs */}
        <div className="plugin-tabs">
          <button
            className={`tab ${activeTab === 'installed' ? 'active' : ''}`}
            onClick={() => setActiveTab('installed')}
          >
            {t('plugin.installed')} ({plugins.length})
          </button>
          <button
            className={`tab ${activeTab === 'store' ? 'active' : ''}`}
            onClick={() => setActiveTab('store')}
            disabled
          >
            {t('plugin.store')}
            <span className="coming-soon">{t('plugin.comingSoon')}</span>
          </button>
        </div>

        {/* Content */}
        <div className="plugin-manager-content">
          {/* Error Banner */}
          {error && (
            <div className="error-banner">
              <AlertCircle size={16} />
              <span>{error}</span>
              <button onClick={clearError}>
                <X size={14} />
              </button>
            </div>
          )}

          {/* Loading State */}
          {isLoading && (
            <div className="loading-overlay">
              <RefreshCw size={24} className="spin" />
              <span>{t('plugin.loading')}</span>
            </div>
          )}

          {activeTab === 'installed' && (
            <div className="plugin-list-container">
              {/* Plugin List */}
              <div className="plugin-list">
                {plugins.length === 0 ? (
                  <div className="empty-state">
                    <Package size={48} />
                    <p>{t('plugin.noPlugins')}</p>
                    <p className="hint">{t('plugin.installHint')}</p>
                  </div>
                ) : (
                  plugins.map((plugin) => (
                    <div
                      key={plugin.id}
                      className={`plugin-item ${selectedPluginId === plugin.id ? 'selected' : ''}`}
                      onClick={() => selectPlugin(plugin.id)}
                    >
                      <div className="plugin-icon">
                        {plugin.icon ? (
                          <img src={plugin.icon} alt="" />
                        ) : (
                          <Package size={24} />
                        )}
                      </div>
                      <div className="plugin-info">
                        <div className="plugin-name">{plugin.name}</div>
                        <div className="plugin-meta">
                          <span className="version">v{plugin.version}</span>
                          <span className="type">{getPluginTypeLabel(plugin.pluginType)}</span>
                        </div>
                      </div>
                      <div className={`plugin-state ${plugin.state}`}>
                        {getStateIcon(plugin.state)}
                        <span>{getStateLabel(plugin.state)}</span>
                      </div>
                    </div>
                  ))
                )}
              </div>

              {/* Plugin Detail */}
              {selectedPluginDetail && (
                <div className="plugin-detail">
                  <div className="detail-header">
                    <div className="detail-icon">
                      {selectedPluginDetail.info.icon ? (
                        <img src={selectedPluginDetail.info.icon} alt="" />
                      ) : (
                        <Package size={32} />
                      )}
                    </div>
                    <div className="detail-title">
                      <h3>{selectedPluginDetail.info.name}</h3>
                      <div className="detail-meta">
                        <span>v{selectedPluginDetail.info.version}</span>
                        <span>{selectedPluginDetail.info.author}</span>
                      </div>
                    </div>
                  </div>

                  <p className="detail-description">
                    {selectedPluginDetail.info.description}
                  </p>

                  <div className="detail-actions">
                    <button
                      className={`btn ${selectedPluginDetail.info.state === 'enabled' ? 'secondary' : 'primary'}`}
                      onClick={() => handleToggleEnable(selectedPluginDetail.info)}
                    >
                      {selectedPluginDetail.info.state === 'enabled' ? (
                        <>
                          <PowerOff size={14} />
                          {t('plugin.disable')}
                        </>
                      ) : (
                        <>
                          <Power size={14} />
                          {t('plugin.enable')}
                        </>
                      )}
                    </button>
                    <button
                      className="btn danger"
                      onClick={() => handleUninstall(selectedPluginDetail.info)}
                    >
                      <Trash2 size={14} />
                      {t('plugin.uninstall')}
                    </button>
                  </div>

                  {selectedPluginDetail.readme && (
                    <div className="detail-readme">
                      <h4>{t('plugin.readme')}</h4>
                      <div className="readme-content">
                        {selectedPluginDetail.readme}
                      </div>
                    </div>
                  )}
                </div>
              )}
            </div>
          )}

          {activeTab === 'store' && (
            <div className="store-placeholder">
              <Package size={64} />
              <h3>{t('plugin.storeComingSoon')}</h3>
              <p>{t('plugin.storeDescription')}</p>
            </div>
          )}
        </div>

        {/* Footer Actions */}
        <div className="plugin-manager-footer">
          <div className="footer-left">
            <button className="btn secondary" onClick={openPluginsFolder}>
              <FolderOpen size={14} />
              {t('plugin.openFolder')}
            </button>
            <button className="btn secondary" onClick={refreshPlugins}>
              <RefreshCw size={14} />
              {t('plugin.refresh')}
            </button>
          </div>
          <div className="footer-right">
            <button className="btn secondary" onClick={handleInstallFromFolder}>
              <FolderOpen size={14} />
              {t('plugin.installFromFolder')}
            </button>
            <button className="btn primary" onClick={handleInstallPlugin}>
              <Download size={14} />
              {t('plugin.install')}
            </button>
          </div>
        </div>
      </div>
    </div>
  )
}
