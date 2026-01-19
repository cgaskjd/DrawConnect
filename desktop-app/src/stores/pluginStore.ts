import { create } from 'zustand'
import { invoke } from '@tauri-apps/api/tauri'

// Plugin types
export type PluginType = 'brush' | 'filter' | 'tool' | 'mixed'
export type PluginState = 'installed' | 'enabled' | 'disabled' | 'error'

export interface PluginInfo {
  id: string
  name: string
  version: string
  description: string
  author: string
  pluginType: string
  state: PluginState
  icon?: string
}

export interface PluginDetail {
  info: PluginInfo
  manifest: Record<string, unknown>
  settingsSchema?: Record<string, unknown>
  currentSettings: Record<string, unknown>
  readme?: string
  changelog?: string
}

export interface BrushCapability {
  id: string
  name: string
  category: string
  icon?: string
}

export interface FilterCapability {
  id: string
  name: string
  category: string
}

export interface ToolCapability {
  id: string
  name: string
  icon?: string
}

export interface PanelCapability {
  id: string
  title: string
  icon?: string
  position: string
}

export interface PluginCapabilities {
  brushes: BrushCapability[]
  filters: FilterCapability[]
  tools: ToolCapability[]
  panels: PanelCapability[]
}

// Store search result (for future store integration)
export interface StoreSearchResult {
  plugins: StorePluginInfo[]
  total: number
  page: number
  perPage: number
}

export interface StorePluginInfo {
  id: string
  name: string
  version: string
  description: string
  author: string
  downloads: number
  rating: number
  iconUrl?: string
  isInstalled: boolean
}

export interface PluginUpdateInfo {
  pluginId: string
  currentVersion: string
  newVersion: string
  changelog?: string
}

interface PluginStoreState {
  // State
  isInitialized: boolean
  isLoading: boolean
  error: string | null
  plugins: PluginInfo[]
  selectedPluginId: string | null
  selectedPluginDetail: PluginDetail | null
  capabilities: PluginCapabilities

  // Store state (for future)
  storeResults: StoreSearchResult | null
  storeSearchQuery: string
  availableUpdates: PluginUpdateInfo[]

  // Actions
  initialize: () => Promise<void>
  refreshPlugins: () => Promise<void>
  getPluginDetail: (id: string) => Promise<PluginDetail | null>
  selectPlugin: (id: string | null) => Promise<void>
  installPlugin: (path: string) => Promise<PluginInfo>
  uninstallPlugin: (id: string) => Promise<void>
  enablePlugin: (id: string) => Promise<void>
  disablePlugin: (id: string) => Promise<void>
  getPluginSettings: (id: string) => Promise<Record<string, unknown>>
  setPluginSetting: (id: string, key: string, value: unknown) => Promise<void>
  refreshCapabilities: () => Promise<void>
  openPluginsFolder: () => Promise<void>

  // Store actions (for future)
  searchStore: (query: string) => Promise<void>
  checkUpdates: () => Promise<void>

  // Utility
  clearError: () => void
}

export const usePluginStore = create<PluginStoreState>((set, get) => ({
  // Initial state
  isInitialized: false,
  isLoading: false,
  error: null,
  plugins: [],
  selectedPluginId: null,
  selectedPluginDetail: null,
  capabilities: {
    brushes: [],
    filters: [],
    tools: [],
    panels: [],
  },

  // Store state
  storeResults: null,
  storeSearchQuery: '',
  availableUpdates: [],

  // Actions
  initialize: async () => {
    if (get().isInitialized) return

    set({ isLoading: true, error: null })

    try {
      // Initialize the plugin system
      await invoke('init_plugin_system')

      // Load plugins
      const plugins = await invoke<PluginInfo[]>('get_plugins')

      // Load capabilities from enabled plugins
      const capabilities = await invoke<PluginCapabilities>('get_plugin_contributions')

      set({
        isInitialized: true,
        isLoading: false,
        plugins,
        capabilities,
      })
    } catch (error) {
      console.error('Failed to initialize plugin system:', error)
      set({
        isLoading: false,
        error: `Failed to initialize plugin system: ${error}`,
      })
    }
  },

  refreshPlugins: async () => {
    set({ isLoading: true, error: null })

    try {
      const plugins = await invoke<PluginInfo[]>('refresh_plugins')
      const capabilities = await invoke<PluginCapabilities>('get_plugin_contributions')

      set({
        isLoading: false,
        plugins,
        capabilities,
      })
    } catch (error) {
      console.error('Failed to refresh plugins:', error)
      set({
        isLoading: false,
        error: `Failed to refresh plugins: ${error}`,
      })
    }
  },

  getPluginDetail: async (id: string) => {
    try {
      const detail = await invoke<PluginDetail>('get_plugin_detail', { pluginId: id })
      return detail
    } catch (error) {
      console.error('Failed to get plugin detail:', error)
      return null
    }
  },

  selectPlugin: async (id: string | null) => {
    set({ selectedPluginId: id, selectedPluginDetail: null })

    if (id) {
      const detail = await get().getPluginDetail(id)
      set({ selectedPluginDetail: detail })
    }
  },

  installPlugin: async (path: string) => {
    set({ isLoading: true, error: null })

    try {
      const pluginInfo = await invoke<PluginInfo>('install_plugin', { path })

      // Refresh plugins list
      const plugins = await invoke<PluginInfo[]>('get_plugins')
      const capabilities = await invoke<PluginCapabilities>('get_plugin_contributions')

      set({
        isLoading: false,
        plugins,
        capabilities,
      })

      return pluginInfo
    } catch (error) {
      console.error('Failed to install plugin:', error)
      set({
        isLoading: false,
        error: `Failed to install plugin: ${error}`,
      })
      throw error
    }
  },

  uninstallPlugin: async (id: string) => {
    set({ isLoading: true, error: null })

    try {
      await invoke('uninstall_plugin', { pluginId: id })

      // Clear selection if uninstalling selected plugin
      if (get().selectedPluginId === id) {
        set({ selectedPluginId: null, selectedPluginDetail: null })
      }

      // Refresh plugins list
      const plugins = await invoke<PluginInfo[]>('get_plugins')
      const capabilities = await invoke<PluginCapabilities>('get_plugin_contributions')

      set({
        isLoading: false,
        plugins,
        capabilities,
      })
    } catch (error) {
      console.error('Failed to uninstall plugin:', error)
      set({
        isLoading: false,
        error: `Failed to uninstall plugin: ${error}`,
      })
      throw error
    }
  },

  enablePlugin: async (id: string) => {
    set({ isLoading: true, error: null })

    try {
      await invoke('enable_plugin', { pluginId: id })

      // Refresh plugins and capabilities
      const plugins = await invoke<PluginInfo[]>('get_plugins')
      const capabilities = await invoke<PluginCapabilities>('get_plugin_contributions')

      // Refresh selected plugin detail if it's the one being enabled
      let selectedPluginDetail = get().selectedPluginDetail
      if (get().selectedPluginId === id) {
        selectedPluginDetail = await get().getPluginDetail(id)
      }

      set({
        isLoading: false,
        plugins,
        capabilities,
        selectedPluginDetail,
      })
    } catch (error) {
      console.error('Failed to enable plugin:', error)
      set({
        isLoading: false,
        error: `Failed to enable plugin: ${error}`,
      })
      throw error
    }
  },

  disablePlugin: async (id: string) => {
    set({ isLoading: true, error: null })

    try {
      await invoke('disable_plugin', { pluginId: id })

      // Refresh plugins and capabilities
      const plugins = await invoke<PluginInfo[]>('get_plugins')
      const capabilities = await invoke<PluginCapabilities>('get_plugin_contributions')

      // Refresh selected plugin detail if it's the one being disabled
      let selectedPluginDetail = get().selectedPluginDetail
      if (get().selectedPluginId === id) {
        selectedPluginDetail = await get().getPluginDetail(id)
      }

      set({
        isLoading: false,
        plugins,
        capabilities,
        selectedPluginDetail,
      })
    } catch (error) {
      console.error('Failed to disable plugin:', error)
      set({
        isLoading: false,
        error: `Failed to disable plugin: ${error}`,
      })
      throw error
    }
  },

  getPluginSettings: async (id: string) => {
    try {
      const settings = await invoke<Record<string, unknown>>('get_plugin_settings', { pluginId: id })
      return settings
    } catch (error) {
      console.error('Failed to get plugin settings:', error)
      return {}
    }
  },

  setPluginSetting: async (id: string, key: string, value: unknown) => {
    try {
      await invoke('set_plugin_setting', { pluginId: id, key, value })

      // Refresh selected plugin detail if it's the one being modified
      if (get().selectedPluginId === id) {
        const detail = await get().getPluginDetail(id)
        set({ selectedPluginDetail: detail })
      }
    } catch (error) {
      console.error('Failed to set plugin setting:', error)
      throw error
    }
  },

  refreshCapabilities: async () => {
    try {
      const capabilities = await invoke<PluginCapabilities>('get_plugin_contributions')
      set({ capabilities })
    } catch (error) {
      console.error('Failed to refresh capabilities:', error)
    }
  },

  openPluginsFolder: async () => {
    try {
      await invoke('open_plugins_folder')
    } catch (error) {
      console.error('Failed to open plugins folder:', error)
    }
  },

  // Store actions (placeholder for future)
  searchStore: async (query: string) => {
    set({ storeSearchQuery: query, isLoading: true })

    try {
      const results = await invoke<StoreSearchResult>('search_store_plugins', {
        query,
        page: 1,
        perPage: 20,
      })

      set({
        isLoading: false,
        storeResults: results,
      })
    } catch (error) {
      console.error('Failed to search store:', error)
      set({
        isLoading: false,
        error: `Failed to search store: ${error}`,
      })
    }
  },

  checkUpdates: async () => {
    try {
      const updates = await invoke<PluginUpdateInfo[]>('check_plugin_updates')
      set({ availableUpdates: updates })
    } catch (error) {
      console.error('Failed to check updates:', error)
    }
  },

  // Utility
  clearError: () => {
    set({ error: null })
  },
}))

// Selector hooks for convenience
export const usePlugins = () => usePluginStore((state) => state.plugins)
export const usePluginCapabilities = () => usePluginStore((state) => state.capabilities)
export const useSelectedPlugin = () => usePluginStore((state) => state.selectedPluginDetail)
export const usePluginLoading = () => usePluginStore((state) => state.isLoading)
export const usePluginError = () => usePluginStore((state) => state.error)
