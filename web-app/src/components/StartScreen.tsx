import { useAppStore } from '../stores/appStore'
import './StartScreen.css'

export default function StartScreen() {
  const { setAppMode } = useAppStore()

  return (
    <div className="start-screen">
      <div className="start-content">
        <h1 className="start-title">DrawConnect</h1>
        <p className="start-subtitle">Professional Digital Painting & Image Editing - Web Edition</p>

        <div className="mode-selection">
          <div className="mode-card" onClick={() => setAppMode('draw')}>
            <div className="mode-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <path d="M12 19l7-7 3 3-7 7-3-3z" />
                <path d="M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z" />
                <path d="M2 2l7.586 7.586" />
                <circle cx="11" cy="11" r="2" />
              </svg>
            </div>
            <h2>Draw</h2>
            <p>Create a new canvas and start digital painting with professional brushes</p>
          </div>

          <div className="mode-card" onClick={() => setAppMode('edit')}>
            <div className="mode-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
                <circle cx="8.5" cy="8.5" r="1.5" />
                <polyline points="21 15 16 10 5 21" />
              </svg>
            </div>
            <h2>Edit</h2>
            <p>Import images and apply professional adjustments, filters, and effects</p>
          </div>
        </div>

        <p className="start-version">Web Version 0.1.0</p>
      </div>
    </div>
  )
}
