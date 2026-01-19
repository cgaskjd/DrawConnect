import { useState } from 'react'
import { useNavigationStore } from '../stores/navigationStore'

export default function LandingPage() {
  const { setPageMode } = useNavigationStore()
  const [activeFeature, setActiveFeature] = useState(0)
  const [activePreview, setActivePreview] = useState(0)

  const previews = [
    { src: '/preview.png', alt: '界面预览 1' },
    { src: '/preview2.png', alt: '界面预览 2' },
    { src: '/preview3.png', alt: '界面预览 3' },
  ]

  const features = [
    {
      icon: '🎨',
      title: '专业画笔引擎',
      description: '内置 300+ 预设画笔，涵盖铅笔、水彩、油画、马克笔等多种风格。支持 8192 级压感感应，精准捕捉笔触轻重变化。自定义画笔参数包括大小、不透明度、流量、硬度、间距、抖动等，真实还原传统绘画手感。'
    },
    {
      icon: '🖼️',
      title: '强大图层系统',
      description: '支持无限图层创建，每个图层可独立调整不透明度和混合模式。提供 27 种专业混合模式（正片叠底、滤色、叠加、柔光等）。支持图层蒙版、剪贴蒙版、图层分组，满足复杂构图需求。'
    },
    {
      icon: '⚡',
      title: 'GPU 加速渲染',
      description: '核心渲染引擎基于 Rust + wgpu 构建，充分利用 GPU 并行计算能力。支持最高 16K×16K 分辨率画布，百层图层依然流畅。采用增量渲染技术，仅重绘变化区域，大幅降低性能开销。'
    },
    {
      icon: '🎛️',
      title: '丰富滤镜调整',
      description: '提供完整的图像调整工具：亮度/对比度、色阶、曲线、色相/饱和度、色彩平衡。内置多种滤镜效果：高斯模糊、锐化、噪点、马赛克、油画效果等。支持非破坏性编辑，随时调整参数。'
    },
    {
      icon: '📁',
      title: 'PS资源兼容',
      description: '无缝导入 Adobe Photoshop 资源文件。支持 .abr 画笔预设直接加载使用，.pat 图案文件作为填充和画笔纹理，.aco/.ase 色板文件导入色彩方案。让你的创作资源库即刻可用。'
    },
    {
      icon: '🔌',
      title: '插件扩展',
      description: '开放插件 API，支持 JavaScript 和 WebAssembly 两种开发方式。可扩展自定义滤镜、画笔、面板、快捷操作等。内置插件市场，一键安装社区优质插件，持续扩展软件功能边界。'
    }
  ]

  const screenshots = [
    { src: '/screenshots/draw-mode.png', alt: '绘画模式' },
    { src: '/screenshots/edit-mode.png', alt: '修图模式' },
    { src: '/screenshots/layers.png', alt: '图层面板' },
  ]

  return (
    <div className="landing-page">
      {/* Hero Section */}
      <header className="hero">
        <nav className="nav">
          <div className="logo">
            <span className="logo-icon">🎨</span>
            <span className="logo-text">DrawConnect</span>
          </div>
          <div className="nav-links">
            <a href="#features">功能特性</a>
            <a href="#download">下载</a>
            <a href="#" onClick={(e) => { e.preventDefault(); setPageMode('plugins'); }}>插件市场</a>
            <a href="https://github.com/drawconnect" target="_blank" rel="noopener noreferrer">GitHub</a>
          </div>
        </nav>

        <div className="hero-content">
          <h1>DrawConnect</h1>
          <p className="hero-subtitle">专业数字绘画与修图软件</p>
          <p className="hero-description">
            基于 Rust 构建的高性能跨平台绘画引擎，<br />
            为数字艺术家打造专业级创作工具
          </p>
          <div className="hero-buttons">
            <a href="/downloads/DrawConnect_0.1.6_x64-setup.exe" className="btn btn-primary btn-large" download>
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="7 10 12 15 17 10" />
                <line x1="12" y1="15" x2="12" y2="3" />
              </svg>
              <span>下载 Windows 版</span>
            </a>
            <a href="https://github.com/drawconnect" className="btn btn-secondary btn-large" target="_blank" rel="noopener noreferrer">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
              </svg>
              <span>查看源码</span>
            </a>
          </div>
          <div className="hero-version">
            <span className="version-badge">v0.1.6</span>
            <span className="version-text">最新版本 · 2026年1月发布</span>
          </div>
        </div>

        <div className="hero-visual">
          <div className="app-preview">
            <div className="preview-window">
              <div className="window-titlebar">
                <div className="window-buttons">
                  <span className="btn-close"></span>
                  <span className="btn-minimize"></span>
                  <span className="btn-maximize"></span>
                </div>
                <span className="window-title">DrawConnect</span>
              </div>
              <div className="window-content">
                <img src={previews[activePreview].src} alt={previews[activePreview].alt} />
              </div>
            </div>
            <div className="preview-nav">
              {previews.map((_, index) => (
                <button
                  key={index}
                  className={`preview-dot ${activePreview === index ? 'active' : ''}`}
                  onClick={() => setActivePreview(index)}
                />
              ))}
            </div>
          </div>
        </div>
      </header>

      {/* Features Section */}
      <section id="features" className="features-section">
        <h2>功能特性</h2>
        <p className="section-subtitle">专为数字艺术家设计的专业级功能</p>

        <div className="features-grid">
          {features.map((feature, index) => (
            <div
              key={index}
              className={`feature-card ${activeFeature === index ? 'active' : ''}`}
              onMouseEnter={() => setActiveFeature(index)}
            >
              <div className="feature-icon">{feature.icon}</div>
              <h3>{feature.title}</h3>
              <p>{feature.description}</p>
            </div>
          ))}
        </div>
      </section>

      {/* Tech Stack Section */}
      <section className="tech-section">
        <h2>技术架构</h2>
        <p className="section-subtitle">现代化技术栈，极致性能体验</p>

        <div className="tech-grid">
          <div className="tech-card">
            <div className="tech-logo">🦀</div>
            <h3>Rust Core</h3>
            <p>核心绘画引擎采用 Rust 语言编写，提供内存安全保证和零成本抽象。无 GC 停顿，性能媲美 C++，同时避免内存泄漏和数据竞争问题，确保软件长时间稳定运行。</p>
          </div>
          <div className="tech-card">
            <div className="tech-logo">⚛️</div>
            <h3>React + TypeScript</h3>
            <p>用户界面基于 React 18 构建，采用 TypeScript 实现完整类型检查。组件化架构便于维护扩展，配合 Zustand 状态管理，提供流畅响应的操作体验。</p>
          </div>
          <div className="tech-card">
            <div className="tech-logo">🖥️</div>
            <h3>Tauri</h3>
            <p>基于 Tauri 2.0 跨平台框架，打包体积仅 10MB 左右，远小于 Electron 方案。直接调用系统原生 API，启动速度快，内存占用低，带来接近原生应用的性能体验。</p>
          </div>
          <div className="tech-card">
            <div className="tech-logo">🎮</div>
            <h3>wgpu</h3>
            <p>图形渲染基于 wgpu —— Rust 实现的现代 GPU API 抽象层。自动适配 Vulkan (Linux/Windows)、Metal (macOS/iOS)、DX12 (Windows) 后端，充分发挥各平台 GPU 性能。</p>
          </div>
        </div>
      </section>

      {/* Download Section */}
      <section id="download" className="download-section">
        <h2>下载 DrawConnect</h2>
        <p className="section-subtitle">选择你的平台，开始创作之旅</p>

        <div className="download-grid">
          <div className="download-card download-card-featured">
            <div className="platform-icon">
              <svg width="48" height="48" viewBox="0 0 24 24" fill="currentColor">
                <path d="M0 3.449L9.75 2.1v9.451H0m10.949-9.602L24 0v11.4H10.949M0 12.6h9.75v9.451L0 20.699M10.949 12.6H24V24l-12.9-1.801"/>
              </svg>
            </div>
            <h3>Windows</h3>
            <p>Windows 10/11 (64-bit)</p>
            <a href="/downloads/DrawConnect_0.1.6_x64-setup.exe" className="btn btn-primary" download>
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="7 10 12 15 17 10" />
                <line x1="12" y1="15" x2="12" y2="3" />
              </svg>
              安装程序 (1.8 MB)
            </a>
            <a href="/downloads/DrawConnect_0.1.6_x64_en-US.msi" className="btn btn-secondary" download>
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="7 10 12 15 17 10" />
                <line x1="12" y1="15" x2="12" y2="3" />
              </svg>
              MSI 安装包 (2.5 MB)
            </a>
            <a href="/downloads/DrawConnect.exe" className="btn btn-outline" download>
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <rect x="2" y="3" width="20" height="14" rx="2" ry="2"/>
                <line x1="8" y1="21" x2="16" y2="21"/>
                <line x1="12" y1="17" x2="12" y2="21"/>
              </svg>
              便携版 (5.2 MB)
            </a>
          </div>

          <div className="download-card">
            <div className="platform-icon">
              <svg width="48" height="48" viewBox="0 0 24 24" fill="currentColor">
                <path d="M18.71 19.5c-.83 1.24-1.71 2.45-3.05 2.47-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.3-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.34 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z"/>
              </svg>
            </div>
            <h3>macOS</h3>
            <p>macOS 11+ (Apple Silicon / Intel)</p>
            <span className="btn btn-disabled">
              .dmg 安装包 - 即将推出
            </span>
          </div>

          <div className="download-card">
            <div className="platform-icon">
              <svg width="48" height="48" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12.504 0c-.155 0-.311.001-.466.004-.643.019-1.276.091-1.893.229-.616.137-1.217.344-1.793.624-.575.279-1.123.636-1.631 1.066-.508.43-.973.937-1.385 1.506-.412.569-.769 1.2-1.062 1.879-.293.678-.518 1.404-.67 2.165-.152.76-.228 1.555-.228 2.372 0 .817.076 1.612.228 2.373.152.76.377 1.486.67 2.164.293.679.65 1.31 1.062 1.879.412.569.877 1.076 1.385 1.506.508.43 1.056.787 1.631 1.066.576.28 1.177.487 1.793.624.617.138 1.25.21 1.893.229.155.003.311.004.466.004.155 0 .311-.001.466-.004.643-.019 1.276-.091 1.893-.229.616-.137 1.217-.344 1.793-.624.575-.279 1.123-.636 1.631-1.066.508-.43.973-.937 1.385-1.506.412-.569.769-1.2 1.062-1.879.293-.678.518-1.404.67-2.164.152-.761.228-1.556.228-2.373 0-.817-.076-1.612-.228-2.372-.152-.761-.377-1.487-.67-2.165-.293-.679-.65-1.31-1.062-1.879-.412-.569-.877-1.076-1.385-1.506-.508-.43-1.056-.787-1.631-1.066-.576-.28-1.177-.487-1.793-.624-.617-.138-1.25-.21-1.893-.229-.155-.003-.311-.004-.466-.004zm-2.034 5.036h4.059v3.058h3.058v4.059h-3.058v3.058h-4.059v-3.058h-3.058v-4.059h3.058z"/>
              </svg>
            </div>
            <h3>Linux</h3>
            <p>Ubuntu 20.04+ / Fedora 35+</p>
            <span className="btn btn-disabled">
              .AppImage - 即将推出
            </span>
            <span className="btn btn-disabled">
              .deb 包 - 即将推出
            </span>
          </div>
        </div>

        <div className="download-note">
          <p>💡 系统要求：4GB+ 内存，支持 Vulkan/Metal/DX12 的显卡</p>
          <p>📦 所有版本均已签名，请放心下载安装</p>
        </div>
      </section>

      {/* Footer */}
      <footer className="footer">
        <div className="footer-content">
          <div className="footer-brand">
            <span className="logo-icon">🎨</span>
            <span className="logo-text">DrawConnect</span>
            <p>专业数字绘画与修图软件</p>
          </div>
          <div className="footer-links">
            <div className="footer-column">
              <h4>产品</h4>
              <a href="#features">功能特性</a>
              <a href="#download">下载</a>
              <a href="/changelog">更新日志</a>
            </div>
            <div className="footer-column">
              <h4>资源</h4>
              <a href="/docs">文档</a>
              <a href="/tutorials">教程</a>
              <a href="#" onClick={(e) => { e.preventDefault(); setPageMode('plugins'); }}>插件市场</a>
            </div>
            <div className="footer-column">
              <h4>社区</h4>
              <a href="https://github.com/drawconnect" target="_blank" rel="noopener noreferrer">GitHub</a>
              <a href="https://discord.gg/drawconnect" target="_blank" rel="noopener noreferrer">Discord</a>
              <a href="mailto:contact@drawconnect.com">联系我们</a>
            </div>
          </div>
        </div>
        <div className="footer-bottom">
          <p>&copy; 2024 DrawConnect Team. MIT License.</p>
        </div>
      </footer>
    </div>
  )
}
