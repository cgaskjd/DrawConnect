import { PluginCategory, PluginSortOption } from '../../api/client'
import { usePluginStore, CATEGORY_LABELS, SORT_OPTIONS } from '../../stores/pluginStore'

export default function PluginSearchBar() {
  const {
    searchQuery,
    setSearchQuery,
    selectedCategory,
    setSelectedCategory,
    sortOption,
    setSortOption,
    fetchPlugins,
  } = usePluginStore()

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault()
    fetchPlugins()
  }

  const handleSearchChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchQuery(e.target.value)
  }

  const handleCategoryChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setSelectedCategory(e.target.value as PluginCategory | 'all')
    fetchPlugins({ category: e.target.value as PluginCategory | 'all' })
  }

  const handleSortChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setSortOption(e.target.value as PluginSortOption)
    fetchPlugins({ sort: e.target.value as PluginSortOption })
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      fetchPlugins()
    }
  }

  return (
    <div className="plugin-search-bar">
      <div className="plugin-search-input-wrapper">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
          <circle cx="11" cy="11" r="8" />
          <path d="m21 21-4.35-4.35" />
        </svg>
        <input
          type="text"
          className="plugin-search-input"
          placeholder="Search plugins..."
          value={searchQuery}
          onChange={handleSearchChange}
          onKeyDown={handleKeyDown}
        />
      </div>
      <div className="plugin-filters">
        <select
          className="plugin-filter-select"
          value={selectedCategory}
          onChange={handleCategoryChange}
        >
          {Object.entries(CATEGORY_LABELS).map(([value, label]) => (
            <option key={value} value={value}>
              {label}
            </option>
          ))}
        </select>
        <select
          className="plugin-filter-select"
          value={sortOption}
          onChange={handleSortChange}
        >
          {SORT_OPTIONS.map(option => (
            <option key={option.value} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </div>
    </div>
  )
}
