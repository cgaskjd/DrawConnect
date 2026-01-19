import { useAppStore } from '../stores/appStore'
import './StartScreen.css'

export function StartScreen() {
  const { setAppMode } = useAppStore()

  return (
    <div className="start-screen">
      <div className="start-content">
        <h1 className="start-title">DrawConnect</h1>
        <p className="start-subtitle">专业数字绘画与修图软件</p>

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
            <h2>画画</h2>
            <p>创建新画布，使用画笔工具进行数字绘画创作</p>
          </div>

          <div className="mode-card" onClick={() => setAppMode('edit')}>
            <div className="mode-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
                <circle cx="8.5" cy="8.5" r="1.5" />
                <polyline points="21 15 16 10 5 21" />
              </svg>
            </div>
            <h2>修图</h2>
            <p>导入图片，进行亮度、对比度、滤镜等专业调整</p>
          </div>
        </div>

        <p className="start-version">版本 0.1.6</p>
      </div>
    </div>
  )
}
