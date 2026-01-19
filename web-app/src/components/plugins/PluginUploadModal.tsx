import { useState, useRef } from 'react'
import { PluginCategory, uploadPlugin } from '../../api/client'
import { usePluginStore, CATEGORY_LABELS } from '../../stores/pluginStore'

interface PluginUploadModalProps {
  onClose: () => void
}

export default function PluginUploadModal({ onClose }: PluginUploadModalProps) {
  const { refreshPlugins, fetchMyPlugins } = usePluginStore()
  const fileInputRef = useRef<HTMLInputElement>(null)

  const [formData, setFormData] = useState({
    name: '',
    description: '',
    shortDescription: '',
    version: '1.0.0',
    category: 'other' as PluginCategory,
  })
  const [tags, setTags] = useState<string[]>([])
  const [tagInput, setTagInput] = useState('')
  const [file, setFile] = useState<File | null>(null)
  const [isUploading, setIsUploading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) => {
    const { name, value } = e.target
    setFormData(prev => ({ ...prev, [name]: value }))
  }

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFile = e.target.files?.[0]
    if (selectedFile) {
      if (!selectedFile.name.endsWith('.zip')) {
        setError('Only .zip files are allowed')
        return
      }
      setFile(selectedFile)
      setError(null)
    }
  }

  const handleFileDrop = (e: React.DragEvent) => {
    e.preventDefault()
    const droppedFile = e.dataTransfer.files[0]
    if (droppedFile) {
      if (!droppedFile.name.endsWith('.zip')) {
        setError('Only .zip files are allowed')
        return
      }
      setFile(droppedFile)
      setError(null)
    }
  }

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault()
  }

  const handleTagKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' || e.key === ',') {
      e.preventDefault()
      addTag()
    }
  }

  const addTag = () => {
    const tag = tagInput.trim().toLowerCase()
    if (tag && !tags.includes(tag) && tags.length < 10) {
      setTags([...tags, tag])
      setTagInput('')
    }
  }

  const removeTag = (tagToRemove: string) => {
    setTags(tags.filter(t => t !== tagToRemove))
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)

    if (!formData.name.trim()) {
      setError('Plugin name is required')
      return
    }

    if (!formData.description.trim()) {
      setError('Description is required')
      return
    }

    if (!file) {
      setError('Please select a plugin file')
      return
    }

    setIsUploading(true)

    try {
      const response = await uploadPlugin(file, {
        name: formData.name.trim(),
        description: formData.description.trim(),
        shortDescription: formData.shortDescription.trim() || undefined,
        version: formData.version.trim() || '1.0.0',
        category: formData.category,
        tags: tags.length > 0 ? tags : undefined,
      })

      if (response.success) {
        await refreshPlugins()
        await fetchMyPlugins()
        onClose()
      } else {
        setError(response.message || 'Upload failed')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Upload failed')
    } finally {
      setIsUploading(false)
    }
  }

  const handleOverlayClick = (e: React.MouseEvent) => {
    if (e.target === e.currentTarget) {
      onClose()
    }
  }

  const formatFileSize = (bytes: number): string => {
    if (bytes < 1024) return bytes + ' B'
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
    return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
  }

  const categories = Object.entries(CATEGORY_LABELS).filter(([key]) => key !== 'all')

  return (
    <div className="plugin-modal-overlay" onClick={handleOverlayClick}>
      <div className="plugin-modal">
        <div className="plugin-modal-header">
          <h2 className="plugin-modal-title">Upload Plugin</h2>
          <button className="plugin-modal-close" onClick={onClose}>
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M18 6L6 18M6 6l12 12" />
            </svg>
          </button>
        </div>

        <form className="plugin-modal-content" onSubmit={handleSubmit}>
          <div className="plugin-upload-form">
            {error && (
              <div className="plugin-error" style={{ padding: '12px', margin: 0, borderRadius: '8px' }}>
                <span className="plugin-error-text">{error}</span>
              </div>
            )}

            <div className="plugin-upload-field">
              <label>
                Plugin File <span>*</span>
              </label>
              <div
                className={`plugin-upload-file ${file ? 'has-file' : ''}`}
                onClick={() => fileInputRef.current?.click()}
                onDrop={handleFileDrop}
                onDragOver={handleDragOver}
              >
                <input
                  ref={fileInputRef}
                  type="file"
                  accept=".zip"
                  onChange={handleFileChange}
                />
                <span className="plugin-upload-file-icon">📦</span>
                <span className="plugin-upload-file-text">
                  {file ? 'Click to change file' : 'Click to select or drag and drop a .zip file'}
                </span>
                {file && (
                  <span className="plugin-upload-file-name">
                    {file.name} ({formatFileSize(file.size)})
                  </span>
                )}
              </div>
              <span className="plugin-upload-hint">Maximum file size: 50MB</span>
            </div>

            <div className="plugin-upload-field">
              <label>
                Plugin Name <span>*</span>
              </label>
              <input
                type="text"
                name="name"
                value={formData.name}
                onChange={handleInputChange}
                placeholder="Enter plugin name"
                maxLength={100}
              />
            </div>

            <div className="plugin-upload-field">
              <label>
                Short Description
              </label>
              <input
                type="text"
                name="shortDescription"
                value={formData.shortDescription}
                onChange={handleInputChange}
                placeholder="Brief description (displayed in card)"
                maxLength={200}
              />
            </div>

            <div className="plugin-upload-field">
              <label>
                Description <span>*</span>
              </label>
              <textarea
                name="description"
                value={formData.description}
                onChange={handleInputChange}
                placeholder="Detailed description of your plugin..."
                maxLength={5000}
              />
            </div>

            <div style={{ display: 'flex', gap: '16px' }}>
              <div className="plugin-upload-field" style={{ flex: 1 }}>
                <label>Version</label>
                <input
                  type="text"
                  name="version"
                  value={formData.version}
                  onChange={handleInputChange}
                  placeholder="1.0.0"
                />
              </div>

              <div className="plugin-upload-field" style={{ flex: 1 }}>
                <label>Category</label>
                <select
                  name="category"
                  value={formData.category}
                  onChange={handleInputChange}
                >
                  {categories.map(([value, label]) => (
                    <option key={value} value={value}>
                      {label}
                    </option>
                  ))}
                </select>
              </div>
            </div>

            <div className="plugin-upload-field">
              <label>Tags</label>
              <div className="plugin-upload-tags">
                {tags.map(tag => (
                  <span key={tag} className="plugin-upload-tag">
                    #{tag}
                    <span
                      className="plugin-upload-tag-remove"
                      onClick={() => removeTag(tag)}
                    >
                      &times;
                    </span>
                  </span>
                ))}
                <input
                  type="text"
                  value={tagInput}
                  onChange={e => setTagInput(e.target.value)}
                  onKeyDown={handleTagKeyDown}
                  onBlur={addTag}
                  placeholder={tags.length < 10 ? 'Add tags (press Enter)' : 'Max 10 tags'}
                  disabled={tags.length >= 10}
                />
              </div>
            </div>
          </div>
        </form>

        <div className="plugin-modal-footer">
          <button
            className="plugin-detail-btn plugin-detail-btn-secondary"
            onClick={onClose}
            disabled={isUploading}
          >
            Cancel
          </button>
          <button
            className="plugin-detail-btn plugin-detail-btn-primary"
            onClick={handleSubmit}
            disabled={isUploading}
          >
            {isUploading ? (
              <>
                <span className="plugin-loading-spinner" style={{ width: 16, height: 16, borderWidth: 2 }} />
                Uploading...
              </>
            ) : (
              <>
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                  <polyline points="17 8 12 3 7 8" />
                  <line x1="12" y1="3" x2="12" y2="15" />
                </svg>
                Upload Plugin
              </>
            )}
          </button>
        </div>
      </div>
    </div>
  )
}
