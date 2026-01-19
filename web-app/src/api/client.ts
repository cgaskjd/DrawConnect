/**
 * DrawConnect Backend API Client
 *
 * This module provides functions to interact with the DrawConnect backend API
 * for user authentication and cloud storage features.
 */

const API_BASE = import.meta.env.VITE_API_URL || 'http://localhost:3000'

// Types
export interface User {
  id: string
  email: string
  name: string
  avatar?: string
}

export interface AuthResponse {
  success: boolean
  token?: string
  user?: User
  message?: string
}

export interface Artwork {
  id: string
  name: string
  description?: string
  thumbnail?: string
  createdAt: string
  updatedAt: string
  size: number
}

export interface ApiResponse<T> {
  success: boolean
  data?: T
  message?: string
}

// Token management
let authToken: string | null = null

export function setAuthToken(token: string | null): void {
  authToken = token
  if (token) {
    localStorage.setItem('drawconnect_token', token)
  } else {
    localStorage.removeItem('drawconnect_token')
  }
}

export function getAuthToken(): string | null {
  if (!authToken) {
    authToken = localStorage.getItem('drawconnect_token')
  }
  return authToken
}

// Helper function for API requests
async function apiRequest<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<ApiResponse<T>> {
  const token = getAuthToken()

  const headers: HeadersInit = {
    'Content-Type': 'application/json',
    ...options.headers,
  }

  if (token) {
    (headers as Record<string, string>)['Authorization'] = `Bearer ${token}`
  }

  try {
    const response = await fetch(`${API_BASE}${endpoint}`, {
      ...options,
      headers,
    })

    const data = await response.json()

    if (!response.ok) {
      return {
        success: false,
        message: data.message || `HTTP error ${response.status}`,
      }
    }

    return {
      success: true,
      data,
    }
  } catch (error) {
    return {
      success: false,
      message: error instanceof Error ? error.message : 'Network error',
    }
  }
}

// ============================================================================
// Authentication
// ============================================================================

/**
 * Register a new user
 */
export async function register(
  email: string,
  password: string,
  name: string
): Promise<AuthResponse> {
  const response = await apiRequest<AuthResponse>('/auth/register', {
    method: 'POST',
    body: JSON.stringify({ email, password, name }),
  })

  if (response.success && response.data?.token) {
    setAuthToken(response.data.token)
  }

  return response.data || { success: false, message: response.message }
}

/**
 * Login an existing user
 */
export async function login(email: string, password: string): Promise<AuthResponse> {
  const response = await apiRequest<AuthResponse>('/auth/login', {
    method: 'POST',
    body: JSON.stringify({ email, password }),
  })

  if (response.success && response.data?.token) {
    setAuthToken(response.data.token)
  }

  return response.data || { success: false, message: response.message }
}

/**
 * Logout the current user
 */
export function logout(): void {
  setAuthToken(null)
}

/**
 * Get the current user's profile
 */
export async function getProfile(): Promise<ApiResponse<User>> {
  return apiRequest<User>('/auth/profile')
}

/**
 * Update the current user's profile
 */
export async function updateProfile(updates: Partial<User>): Promise<ApiResponse<User>> {
  return apiRequest<User>('/auth/profile', {
    method: 'PUT',
    body: JSON.stringify(updates),
  })
}

// ============================================================================
// Cloud Storage
// ============================================================================

/**
 * Get all artworks for the current user
 */
export async function getArtworks(): Promise<ApiResponse<Artwork[]>> {
  return apiRequest<Artwork[]>('/cloud/artworks')
}

/**
 * Get a specific artwork
 */
export async function getArtwork(id: string): Promise<ApiResponse<Artwork>> {
  return apiRequest<Artwork>(`/cloud/artworks/${id}`)
}

/**
 * Upload an artwork
 */
export async function uploadArtwork(
  data: Blob,
  name: string,
  description?: string
): Promise<ApiResponse<Artwork>> {
  const token = getAuthToken()

  const formData = new FormData()
  formData.append('file', data, `${name}.png`)
  formData.append('name', name)
  if (description) {
    formData.append('description', description)
  }

  try {
    const response = await fetch(`${API_BASE}/cloud/artworks`, {
      method: 'POST',
      headers: token ? { Authorization: `Bearer ${token}` } : {},
      body: formData,
    })

    const result = await response.json()

    if (!response.ok) {
      return {
        success: false,
        message: result.message || `HTTP error ${response.status}`,
      }
    }

    return {
      success: true,
      data: result,
    }
  } catch (error) {
    return {
      success: false,
      message: error instanceof Error ? error.message : 'Upload failed',
    }
  }
}

/**
 * Delete an artwork
 */
export async function deleteArtwork(id: string): Promise<ApiResponse<void>> {
  return apiRequest<void>(`/cloud/artworks/${id}`, {
    method: 'DELETE',
  })
}

/**
 * Download an artwork
 */
export async function downloadArtwork(id: string): Promise<Blob | null> {
  const token = getAuthToken()

  try {
    const response = await fetch(`${API_BASE}/cloud/artworks/${id}/download`, {
      headers: token ? { Authorization: `Bearer ${token}` } : {},
    })

    if (!response.ok) {
      return null
    }

    return response.blob()
  } catch (error) {
    console.error('Download failed:', error)
    return null
  }
}

// ============================================================================
// Health Check
// ============================================================================

/**
 * Check if the API server is available
 */
export async function healthCheck(): Promise<boolean> {
  try {
    const response = await fetch(`${API_BASE}/health`)
    return response.ok
  } catch {
    return false
  }
}

// ============================================================================
// Plugin Community
// ============================================================================

export type PluginCategory = 'brushes' | 'filters' | 'tools' | 'panels' | 'themes' | 'automation' | 'other'
export type PluginStatus = 'pending' | 'approved' | 'rejected' | 'unpublished'
export type PluginSortOption = 'newest' | 'oldest' | 'popular' | 'liked'

export interface PluginAuthor {
  _id: string
  username: string
  avatarUrl?: string
}

export interface Plugin {
  id: string
  name: string
  slug: string
  description: string
  shortDescription?: string
  version: string
  category: PluginCategory
  tags: string[]
  authorId: PluginAuthor | string | null
  authorName?: string
  fileUrl: string
  fileName: string
  fileSize: number
  thumbnailUrl?: string
  screenshotUrls: string[]
  downloadCount: number
  likeCount: number
  status: PluginStatus
  liked: boolean
  createdAt: string
  updatedAt: string
}

export interface PluginListResponse {
  success: boolean
  plugins: Plugin[]
  pagination: {
    page: number
    limit: number
    total: number
    totalPages: number
  }
  message?: string
}

export interface PluginResponse {
  success: boolean
  plugin?: Plugin
  message?: string
}

export interface PluginListParams {
  page?: number
  limit?: number
  search?: string
  category?: PluginCategory | 'all'
  sort?: PluginSortOption
  status?: PluginStatus
}

/**
 * Get plugin list with pagination, search, and filtering
 */
export async function getPlugins(params: PluginListParams = {}): Promise<PluginListResponse> {
  const queryParams = new URLSearchParams()
  if (params.page) queryParams.set('page', String(params.page))
  if (params.limit) queryParams.set('limit', String(params.limit))
  if (params.search) queryParams.set('search', params.search)
  if (params.category && params.category !== 'all') queryParams.set('category', params.category)
  if (params.sort) queryParams.set('sort', params.sort)
  if (params.status) queryParams.set('status', params.status)

  const response = await apiRequest<PluginListResponse>(`/plugins?${queryParams.toString()}`)
  return response.data || { success: false, plugins: [], pagination: { page: 1, limit: 20, total: 0, totalPages: 0 }, message: response.message }
}

/**
 * Get current user's plugins
 */
export async function getMyPlugins(): Promise<PluginListResponse> {
  const response = await apiRequest<{ success: boolean; plugins: Plugin[] }>('/plugins/user/me')
  return {
    success: response.success,
    plugins: response.data?.plugins || [],
    pagination: { page: 1, limit: 100, total: response.data?.plugins.length || 0, totalPages: 1 },
    message: response.message
  }
}

/**
 * Get plugin details by ID
 */
export async function getPlugin(id: string): Promise<PluginResponse> {
  const response = await apiRequest<PluginResponse>(`/plugins/${id}`)
  return response.data || { success: false, message: response.message }
}

/**
 * Upload a new plugin
 */
export async function uploadPlugin(
  file: File,
  data: {
    name: string
    description: string
    shortDescription?: string
    version?: string
    category?: PluginCategory
    tags?: string[]
  }
): Promise<PluginResponse> {
  const token = getAuthToken()

  const formData = new FormData()
  formData.append('file', file)
  formData.append('name', data.name)
  formData.append('description', data.description)
  if (data.shortDescription) formData.append('shortDescription', data.shortDescription)
  if (data.version) formData.append('version', data.version)
  if (data.category) formData.append('category', data.category)
  if (data.tags) formData.append('tags', JSON.stringify(data.tags))

  try {
    const response = await fetch(`${API_BASE}/plugins`, {
      method: 'POST',
      headers: token ? { Authorization: `Bearer ${token}` } : {},
      body: formData,
    })

    const result = await response.json()
    return result
  } catch (error) {
    return {
      success: false,
      message: error instanceof Error ? error.message : 'Upload failed',
    }
  }
}

/**
 * Update plugin details
 */
export async function updatePlugin(
  id: string,
  data: {
    name?: string
    description?: string
    shortDescription?: string
    version?: string
    category?: PluginCategory
    tags?: string[]
  }
): Promise<PluginResponse> {
  const response = await apiRequest<PluginResponse>(`/plugins/${id}`, {
    method: 'PUT',
    body: JSON.stringify(data),
  })
  return response.data || { success: false, message: response.message }
}

/**
 * Delete a plugin
 */
export async function deletePlugin(id: string): Promise<ApiResponse<void>> {
  return apiRequest<void>(`/plugins/${id}`, {
    method: 'DELETE',
  })
}

/**
 * Like a plugin
 */
export async function likePlugin(id: string): Promise<ApiResponse<{ likeCount: number }>> {
  return apiRequest<{ likeCount: number }>(`/plugins/${id}/like`, {
    method: 'POST',
  })
}

/**
 * Unlike a plugin
 */
export async function unlikePlugin(id: string): Promise<ApiResponse<{ likeCount: number }>> {
  return apiRequest<{ likeCount: number }>(`/plugins/${id}/like`, {
    method: 'DELETE',
  })
}

/**
 * Get plugin download URL
 */
export function getPluginDownloadUrl(id: string): string {
  return `${API_BASE}/plugins/${id}/download`
}
