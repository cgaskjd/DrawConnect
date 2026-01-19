import { create } from 'zustand'
import {
  Plugin,
  PluginCategory,
  PluginSortOption,
  getPlugins,
  getMyPlugins,
  getPlugin,
  likePlugin,
  unlikePlugin,
  PluginListParams,
} from '../api/client'

// Sample plugins data for demo/fallback
const SAMPLE_PLUGINS: Plugin[] = [
  {
    id: 'sample-1',
    name: 'Skin Smoothing Filter',
    slug: 'skin-smoothing-filter',
    description: `Professional skin smoothing and beautification filter for portrait photos.

Features:
• Automatic Skin Detection - Uses YCbCr color space analysis to accurately detect skin tones
• Edge-Preserving Smoothing - Bilateral filter technology preserves important details while smoothing
• Multiple Skin Tone Presets - Supports light, medium, dark, and automatic skin tone detection
• Adjustable Strength - Fine-tune the smoothing intensity from 0-100%
• Detail Preservation - Option to blend with original for natural results

Usage:
1. Open a portrait image in DrawConnect
2. Go to Filters > Portrait > Skin Smoothing
3. Adjust settings and click Apply

Keyboard Shortcut: Ctrl+Shift+S`,
    shortDescription: 'Professional portrait skin smoothing filter with automatic skin detection',
    version: '1.0.0',
    category: 'filters',
    tags: ['portrait', 'skin', 'beauty', 'smoothing', 'retouch', 'filter'],
    authorId: null,
    authorName: 'DrawConnect Team',
    fileUrl: '/uploads/plugins/skin-smoothing-filter.zip',
    fileName: 'skin-smoothing-filter.zip',
    fileSize: 3981,
    screenshotUrls: [],
    downloadCount: 128,
    likeCount: 45,
    status: 'approved',
    liked: false,
    createdAt: '2024-01-15T10:00:00Z',
    updatedAt: '2024-01-15T10:00:00Z',
  },
  {
    id: 'sample-2',
    name: 'Watercolor Brush Pack',
    slug: 'watercolor-brush-pack',
    description: `A collection of 20 realistic watercolor brushes for digital painting.

Includes:
• Wet Wash - Soft, flowing strokes with natural color blending
• Dry Brush - Textured strokes for rough effects
• Splatter - Random ink splatter patterns
• Grunge - Distressed watercolor textures
• Round Soft - Smooth gradients for backgrounds
• And 15 more unique brushes!

Each brush features:
• Pressure sensitivity support
• Tilt recognition for natural strokes
• Color mixing simulation
• Paper texture interaction`,
    shortDescription: '20 realistic watercolor brushes for digital painting',
    version: '2.1.0',
    category: 'brushes',
    tags: ['watercolor', 'brush', 'painting', 'artistic', 'texture'],
    authorId: null,
    authorName: 'ArtistPro',
    fileUrl: '/uploads/plugins/watercolor-brush-pack.zip',
    fileName: 'watercolor-brush-pack.zip',
    fileSize: 15240,
    screenshotUrls: [],
    downloadCount: 356,
    likeCount: 89,
    status: 'approved',
    liked: false,
    createdAt: '2024-01-10T08:00:00Z',
    updatedAt: '2024-01-10T08:00:00Z',
  },
  {
    id: 'sample-3',
    name: 'Color Harmony Panel',
    slug: 'color-harmony-panel',
    description: `An advanced color theory panel that helps you create harmonious color palettes.

Features:
• Complementary Colors - Find perfect opposite colors
• Analogous Scheme - Colors next to each other on the wheel
• Triadic Colors - Three colors equally spaced
• Split-Complementary - Variation of complementary
• Tetradic (Square) - Four colors forming a square

Additional Tools:
• Color temperature adjustment
• Saturation/Brightness variations
• Export palettes as .aco or .ase files
• Quick apply to current brush`,
    shortDescription: 'Advanced color theory panel for creating harmonious palettes',
    version: '1.5.0',
    category: 'panels',
    tags: ['color', 'palette', 'harmony', 'theory', 'design'],
    authorId: null,
    authorName: 'ColorMaster',
    fileUrl: '/uploads/plugins/color-harmony-panel.zip',
    fileName: 'color-harmony-panel.zip',
    fileSize: 8920,
    screenshotUrls: [],
    downloadCount: 234,
    likeCount: 67,
    status: 'approved',
    liked: false,
    createdAt: '2024-01-08T14:00:00Z',
    updatedAt: '2024-01-08T14:00:00Z',
  },
  {
    id: 'sample-4',
    name: 'Dark Studio Theme',
    slug: 'dark-studio-theme',
    description: `A professional dark theme optimized for long drawing sessions.

Design Features:
• Ultra-dark background reduces eye strain
• High contrast UI elements for visibility
• Accent colors carefully chosen for accessibility
• Smooth transitions and animations

Customization:
• 5 accent color presets (Orange, Blue, Purple, Green, Pink)
• Adjustable panel opacity
• Custom icon set included`,
    shortDescription: 'Professional dark theme optimized for long drawing sessions',
    version: '1.2.0',
    category: 'themes',
    tags: ['theme', 'dark', 'ui', 'interface', 'design'],
    authorId: null,
    authorName: 'ThemeWorks',
    fileUrl: '/uploads/plugins/dark-studio-theme.zip',
    fileName: 'dark-studio-theme.zip',
    fileSize: 2450,
    screenshotUrls: [],
    downloadCount: 512,
    likeCount: 134,
    status: 'approved',
    liked: false,
    createdAt: '2024-01-05T12:00:00Z',
    updatedAt: '2024-01-05T12:00:00Z',
  },
  {
    id: 'sample-5',
    name: 'Auto Save & Backup',
    slug: 'auto-save-backup',
    description: `Automatically save your work and create backups at regular intervals.

Features:
• Configurable auto-save interval (1-60 minutes)
• Automatic backup creation
• Keep up to 10 backup versions
• Crash recovery support
• Cloud sync integration (optional)

Never lose your work again!`,
    shortDescription: 'Automatic saving and backup system for your projects',
    version: '1.0.2',
    category: 'automation',
    tags: ['autosave', 'backup', 'safety', 'recovery', 'automation'],
    authorId: null,
    authorName: 'SafetyFirst',
    fileUrl: '/uploads/plugins/auto-save-backup.zip',
    fileName: 'auto-save-backup.zip',
    fileSize: 4120,
    screenshotUrls: [],
    downloadCount: 678,
    likeCount: 201,
    status: 'approved',
    liked: false,
    createdAt: '2024-01-02T09:00:00Z',
    updatedAt: '2024-01-02T09:00:00Z',
  },
  {
    id: 'sample-6',
    name: 'Perspective Grid Tool',
    slug: 'perspective-grid-tool',
    description: `Professional perspective drawing assistance tool.

Grid Types:
• 1-Point Perspective
• 2-Point Perspective
• 3-Point Perspective
• Isometric Grid
• Custom Vanishing Points

Features:
• Snap to grid option
• Adjustable grid opacity
• Multiple grid colors
• Save/Load grid presets
• Keyboard shortcuts for quick toggling`,
    shortDescription: 'Professional perspective grid tool for accurate drawings',
    version: '2.0.0',
    category: 'tools',
    tags: ['perspective', 'grid', 'drawing', 'guide', 'architecture'],
    authorId: null,
    authorName: 'PerspectivePro',
    fileUrl: '/uploads/plugins/perspective-grid-tool.zip',
    fileName: 'perspective-grid-tool.zip',
    fileSize: 6780,
    screenshotUrls: [],
    downloadCount: 445,
    likeCount: 112,
    status: 'approved',
    liked: false,
    createdAt: '2024-01-01T16:00:00Z',
    updatedAt: '2024-01-01T16:00:00Z',
  },
  {
    id: 'sample-7',
    name: 'Vintage Film Filter',
    slug: 'vintage-film-filter',
    description: `Transform your digital photos into stunning vintage film looks.

Film Presets:
• Kodak Portra 400 - Warm skin tones, soft contrast
• Fuji Pro 400H - Cool shadows, natural greens
• Kodak Ektar 100 - Vibrant colors, high saturation
• Ilford HP5 - Classic B&W with rich grain
• Cinestill 800T - Tungsten-balanced cinema look
• Polaroid SX-70 - Instant film aesthetic

Adjustable Parameters:
• Grain intensity (0-100%)
• Color fade amount
• Vignette strength
• Light leak effects
• Dust & scratches overlay
• Film border frames

Perfect for photographers looking to add nostalgic charm to modern digital images.`,
    shortDescription: 'Classic film emulation with Kodak, Fuji, and Polaroid presets',
    version: '2.3.0',
    category: 'filters',
    tags: ['vintage', 'film', 'retro', 'photography', 'kodak', 'fuji', 'grain'],
    authorId: null,
    authorName: 'FilmLab Studio',
    fileUrl: '/uploads/plugins/vintage-film-filter.zip',
    fileName: 'vintage-film-filter.zip',
    fileSize: 12450,
    screenshotUrls: [],
    downloadCount: 892,
    likeCount: 267,
    status: 'approved',
    liked: false,
    createdAt: '2024-02-01T10:00:00Z',
    updatedAt: '2024-02-15T14:30:00Z',
  },
  {
    id: 'sample-8',
    name: 'Pixel Art Brush Pack',
    slug: 'pixel-art-brush-pack',
    description: `Complete toolkit for creating authentic pixel art in DrawConnect.

Brush Types:
• 1px Pencil - Single pixel precision drawing
• Dither Brush - Create smooth gradients with dithering patterns
• Tile Brush - Stamp repeating patterns
• Line Tool - Pixel-perfect straight lines (Bresenham algorithm)
• Circle Tool - Anti-aliased and hard-edge circles
• Fill Bucket - Smart fill with tolerance settings

Special Features:
• Grid overlay toggle (8x8, 16x16, 32x32)
• Indexed color palette support
• Export to GIF with animation frames
• Zoom to pixel level (800%+)
• Mirror drawing mode
• Onion skinning for animation

Includes 10 classic color palettes:
• NES, SNES, Game Boy, C64, PICO-8, and more!`,
    shortDescription: 'Complete pixel art toolkit with dithering and animation support',
    version: '1.8.0',
    category: 'brushes',
    tags: ['pixel', 'retro', '8bit', 'gamedev', 'sprite', 'animation', 'indie'],
    authorId: null,
    authorName: 'PixelMaster',
    fileUrl: '/uploads/plugins/pixel-art-brush-pack.zip',
    fileName: 'pixel-art-brush-pack.zip',
    fileSize: 8920,
    screenshotUrls: [],
    downloadCount: 1245,
    likeCount: 389,
    status: 'approved',
    liked: false,
    createdAt: '2024-01-20T08:00:00Z',
    updatedAt: '2024-02-10T16:00:00Z',
  },
  {
    id: 'sample-9',
    name: 'Layer Manager Pro',
    slug: 'layer-manager-pro',
    description: `Advanced layer management panel with powerful organization features.

Core Features:
• Layer Groups - Organize layers into collapsible folders
• Smart Tags - Color-code and tag layers for easy filtering
• Layer Search - Find layers by name or tag instantly
• Bulk Operations - Select multiple layers for batch actions
• Layer Locking - Lock position, transparency, or pixels separately

Advanced Tools:
• Layer Comps - Save and switch between layer visibility states
• Auto-Arrange - Distribute layers evenly with one click
• Merge Options - Flatten visible, merge down, or stamp visible
• Layer Effects - Quick access to blend modes and opacity
• Thumbnail Preview - Adjustable thumbnail sizes

Workflow Enhancements:
• Drag & drop reordering
• Keyboard shortcuts for all operations
• Layer history with undo per layer
• Export selected layers as separate files`,
    shortDescription: 'Professional layer organization with groups, tags, and batch operations',
    version: '3.0.0',
    category: 'panels',
    tags: ['layers', 'organization', 'workflow', 'productivity', 'groups'],
    authorId: null,
    authorName: 'WorkflowTools',
    fileUrl: '/uploads/plugins/layer-manager-pro.zip',
    fileName: 'layer-manager-pro.zip',
    fileSize: 15680,
    screenshotUrls: [],
    downloadCount: 567,
    likeCount: 178,
    status: 'approved',
    liked: false,
    createdAt: '2024-01-25T12:00:00Z',
    updatedAt: '2024-02-20T09:00:00Z',
  },
  {
    id: 'sample-10',
    name: 'Reference Guide Tool',
    slug: 'reference-guide-tool',
    description: `Professional guide and measurement tools for precise design work.

Guide Types:
• Ruler Guides - Draggable horizontal and vertical guides
• Smart Guides - Auto-snap to object edges and centers
• Margin Guides - Set document margins with presets
• Column Guides - Create multi-column layouts
• Golden Ratio - Overlay golden spiral and grid

Measurement Features:
• Distance Tool - Measure between any two points
• Angle Protractor - Measure and display angles
• Area Calculator - Calculate selection area in pixels/inches
• Density Checker - Verify print resolution (DPI/PPI)

Layout Presets:
• Rule of Thirds
• Golden Ratio Grid
• Diagonal Method
• Fibonacci Spiral
• Custom Grid Builder

All guides are non-printing and can be shown/hidden with a single shortcut.`,
    shortDescription: 'Precision guides, rulers, and measurement tools for design',
    version: '2.1.0',
    category: 'tools',
    tags: ['guides', 'rulers', 'measurement', 'layout', 'design', 'precision'],
    authorId: null,
    authorName: 'DesignPrecision',
    fileUrl: '/uploads/plugins/reference-guide-tool.zip',
    fileName: 'reference-guide-tool.zip',
    fileSize: 7340,
    screenshotUrls: [],
    downloadCount: 423,
    likeCount: 156,
    status: 'approved',
    liked: false,
    createdAt: '2024-02-05T14:00:00Z',
    updatedAt: '2024-02-18T11:00:00Z',
  },
  {
    id: 'sample-11',
    name: 'Neon Glow Theme',
    slug: 'neon-glow-theme',
    description: `Vibrant cyberpunk-inspired theme with glowing neon accents.

Color Schemes:
• Cyber Pink - Hot pink and electric blue
• Matrix Green - Classic hacker aesthetic
• Sunset Orange - Warm neon gradients
• Ice Blue - Cool cyan and white
• Purple Haze - Deep purple with pink highlights

Visual Effects:
• Subtle glow on active elements
• Animated hover states
• Gradient backgrounds
• Neon border highlights
• Custom cursor with glow trail

Customization Options:
• Glow intensity slider (0-100%)
• Background darkness level
• Accent color picker
• Animation speed control
• Icon style (outline/filled/neon)

Optimized for both light and dark environments with automatic contrast adjustment.`,
    shortDescription: 'Cyberpunk neon theme with glowing effects and 5 color schemes',
    version: '1.5.0',
    category: 'themes',
    tags: ['neon', 'cyberpunk', 'glow', 'dark', 'futuristic', 'aesthetic'],
    authorId: null,
    authorName: 'NeonDesigns',
    fileUrl: '/uploads/plugins/neon-glow-theme.zip',
    fileName: 'neon-glow-theme.zip',
    fileSize: 4560,
    screenshotUrls: [],
    downloadCount: 789,
    likeCount: 234,
    status: 'approved',
    liked: false,
    createdAt: '2024-01-18T16:00:00Z',
    updatedAt: '2024-02-12T10:00:00Z',
  },
  {
    id: 'sample-12',
    name: 'Batch Export Pro',
    slug: 'batch-export-pro',
    description: `Automate your export workflow with powerful batch processing.

Export Features:
• Multiple Formats - PNG, JPEG, WebP, SVG, PDF, PSD
• Resolution Presets - @1x, @2x, @3x for app development
• Custom Sizes - Export to specific dimensions
• Artboard Export - Export all artboards at once
• Layer Export - Export each layer as separate file

Naming Options:
• Custom prefix/suffix
• Sequential numbering
• Date/time stamps
• Layer name inclusion
• Folder structure templates

Automation:
• Save export presets for reuse
• Scheduled exports (hourly/daily)
• Watch folder for auto-processing
• Command line integration
• Action recording for complex workflows

Quality Settings:
• JPEG quality slider (1-100)
• PNG compression level
• WebP lossy/lossless toggle
• Color profile embedding
• Metadata preservation options`,
    shortDescription: 'Powerful batch export with multiple formats and automation',
    version: '2.5.0',
    category: 'automation',
    tags: ['export', 'batch', 'workflow', 'automation', 'productivity', 'assets'],
    authorId: null,
    authorName: 'AutomateStudio',
    fileUrl: '/uploads/plugins/batch-export-pro.zip',
    fileName: 'batch-export-pro.zip',
    fileSize: 9870,
    screenshotUrls: [],
    downloadCount: 934,
    likeCount: 312,
    status: 'approved',
    liked: false,
    createdAt: '2024-01-28T09:00:00Z',
    updatedAt: '2024-02-22T15:00:00Z',
  },
  {
    id: 'sample-13',
    name: 'Glitch Art Generator',
    slug: 'glitch-art-generator',
    description: `Create stunning digital glitch effects and data corruption art.

Glitch Effects:
• Pixel Sorting - Sort pixels by brightness, hue, or saturation
• Channel Shift - Offset RGB channels independently
• Scan Lines - CRT monitor simulation
• Data Moshing - Compression artifact simulation
• Bit Crushing - Reduce color depth dramatically
• Wave Distortion - Sine wave displacement

Corruption Modes:
• JPEG Artifact Amplifier
• PNG Chunk Corruption
• Interlace Glitch
• Header Manipulation
• Random Byte Insertion

Real-time Preview:
• Adjust parameters with live preview
• Randomize button for happy accidents
• Seed control for reproducible results
• Animation export (GIF/MP4)
• Blend modes for layered effects

Perfect for vaporwave aesthetics, music album covers, and experimental digital art.`,
    shortDescription: 'Digital glitch effects with pixel sorting and data corruption',
    version: '1.2.0',
    category: 'filters',
    tags: ['glitch', 'vaporwave', 'corruption', 'experimental', 'aesthetic', 'art'],
    authorId: null,
    authorName: 'GlitchLab',
    fileUrl: '/uploads/plugins/glitch-art-generator.zip',
    fileName: 'glitch-art-generator.zip',
    fileSize: 11230,
    screenshotUrls: [],
    downloadCount: 678,
    likeCount: 245,
    status: 'approved',
    liked: false,
    createdAt: '2024-02-08T11:00:00Z',
    updatedAt: '2024-02-25T14:00:00Z',
  },
  {
    id: 'sample-14',
    name: 'Calligraphy Master',
    slug: 'calligraphy-master',
    description: `Professional calligraphy brushes with realistic ink simulation.

Brush Styles:
• Western Calligraphy
  - Italic Nib (broad edge)
  - Copperplate (pointed pen)
  - Gothic Blackletter
  - Uncial Script

• East Asian Calligraphy
  - Chinese Maobi (毛笔)
  - Japanese Fude (筆)
  - Korean붓

• Arabic Calligraphy
  - Naskh
  - Thuluth
  - Diwani

Ink Simulation:
• Pressure-sensitive stroke width
• Ink flow and pooling effects
• Paper texture interaction
• Ink bleeding on wet areas
• Dry brush effects

Practice Features:
• Guideline templates
• Stroke order animations
• Letter spacing guides
• Baseline and x-height markers`,
    shortDescription: 'Realistic calligraphy brushes with ink simulation',
    version: '2.0.0',
    category: 'brushes',
    tags: ['calligraphy', 'ink', 'lettering', 'typography', 'brush', 'script'],
    authorId: null,
    authorName: 'InkMaster',
    fileUrl: '/uploads/plugins/calligraphy-master.zip',
    fileName: 'calligraphy-master.zip',
    fileSize: 18450,
    screenshotUrls: [],
    downloadCount: 567,
    likeCount: 198,
    status: 'approved',
    liked: false,
    createdAt: '2024-01-30T10:00:00Z',
    updatedAt: '2024-02-14T12:00:00Z',
  },
  {
    id: 'sample-15',
    name: 'HDR Tone Mapping',
    slug: 'hdr-tone-mapping',
    description: `Professional HDR processing and tone mapping for stunning dynamic range.

Tone Mapping Algorithms:
• Reinhard Global - Classic photographic tone mapping
• Reinhard Local - Adaptive local contrast
• Drago Logarithmic - Preserves bright highlights
• Durand Bilateral - Edge-preserving compression
• Mantiuk Contrast - Perceptually calibrated
• Fattal Gradient - Gradient domain compression

HDR Features:
• 32-bit floating point processing
• EXR and HDR file support
• Exposure bracketing merge
• Ghost removal for handheld shots
• Chromatic adaptation

Creative Controls:
• Highlight recovery
• Shadow boost
• Local contrast enhancement
• Color saturation preservation
• Micro-contrast adjustment
• Glow/bloom effects

Export Options:
• 8-bit/16-bit output
• sRGB/Adobe RGB/ProPhoto RGB
• HDR10 for displays
• Comparison split view`,
    shortDescription: 'Advanced HDR processing with multiple tone mapping algorithms',
    version: '3.1.0',
    category: 'filters',
    tags: ['hdr', 'tonemapping', 'photography', 'dynamic range', 'exposure'],
    authorId: null,
    authorName: 'HDRPro',
    fileUrl: '/uploads/plugins/hdr-tone-mapping.zip',
    fileName: 'hdr-tone-mapping.zip',
    fileSize: 14560,
    screenshotUrls: [],
    downloadCount: 445,
    likeCount: 167,
    status: 'approved',
    liked: false,
    createdAt: '2024-02-10T08:00:00Z',
    updatedAt: '2024-02-28T16:00:00Z',
  },
  {
    id: 'sample-16',
    name: 'Comic Book Style',
    slug: 'comic-book-style',
    description: `Transform photos into comic book and manga style artwork.

Style Presets:
• American Comics - Bold lines, halftone dots, vibrant colors
• Manga/Anime - Clean lines, screen tones, speed lines
• European BD - Ligne claire (clear line) style
• Pop Art - Warhol-inspired bold colors
• Noir - High contrast black and white
• Sketch - Pencil drawing simulation

Effect Components:
• Edge Detection - Adjustable line thickness
• Halftone Patterns - Dots, lines, or crosshatch
• Color Posterization - Reduce to comic palette
• Speech Bubbles - Add text with various styles
• Action Lines - Speed and impact effects
• Screen Tones - Gradient and pattern fills

Customization:
• Line color and weight
• Dot size and angle for halftones
• Color palette selection
• Posterization levels (4-32 colors)
• Blend with original photo`,
    shortDescription: 'Photo to comic/manga conversion with multiple artistic styles',
    version: '1.8.0',
    category: 'filters',
    tags: ['comic', 'manga', 'cartoon', 'popart', 'artistic', 'halftone'],
    authorId: null,
    authorName: 'ComicArtist',
    fileUrl: '/uploads/plugins/comic-book-style.zip',
    fileName: 'comic-book-style.zip',
    fileSize: 16780,
    screenshotUrls: [],
    downloadCount: 823,
    likeCount: 289,
    status: 'approved',
    liked: false,
    createdAt: '2024-01-22T14:00:00Z',
    updatedAt: '2024-02-16T11:00:00Z',
  },
  {
    id: 'sample-17',
    name: 'Shape Builder Tool',
    slug: 'shape-builder-tool',
    description: `Intuitive tool for creating and combining complex shapes.

Shape Operations:
• Union - Merge overlapping shapes
• Subtract - Cut shapes from each other
• Intersect - Keep only overlapping areas
• Exclude - Remove overlapping areas
• Divide - Split into separate parts

Drawing Tools:
• Rectangle & Rounded Rectangle
• Ellipse & Circle
• Polygon (3-12 sides)
• Star (adjustable points and inset)
• Line & Arrow
• Custom Path Builder

Smart Features:
• Snap to grid and guides
• Constrain proportions (hold Shift)
• Draw from center (hold Alt)
• Live dimensions display
• Corner radius editing
• Boolean preview before commit

Vector Output:
• Export as SVG
• Copy as CSS clip-path
• Convert to selection
• Create layer mask`,
    shortDescription: 'Advanced shape creation with boolean operations',
    version: '2.2.0',
    category: 'tools',
    tags: ['shapes', 'vector', 'boolean', 'design', 'geometry', 'drawing'],
    authorId: null,
    authorName: 'VectorWorks',
    fileUrl: '/uploads/plugins/shape-builder-tool.zip',
    fileName: 'shape-builder-tool.zip',
    fileSize: 8450,
    screenshotUrls: [],
    downloadCount: 612,
    likeCount: 201,
    status: 'approved',
    liked: false,
    createdAt: '2024-02-03T09:00:00Z',
    updatedAt: '2024-02-21T14:00:00Z',
  },
  {
    id: 'sample-18',
    name: 'History Panel Pro',
    slug: 'history-panel-pro',
    description: `Enhanced history management with branching and snapshots.

Core Features:
• Unlimited History - No more losing states
• History Branching - Explore different directions
• Named Snapshots - Save important states
• History Thumbnails - Visual preview of each state
• Selective Undo - Undo specific actions, not just last

Advanced Features:
• History Tree View - Visualize all branches
• Compare States - Side-by-side comparison
• Merge Branches - Combine different explorations
• Export History - Save as action script
• History Search - Find specific operations

Performance:
• Compressed history storage
• Configurable memory limits
• Auto-purge old states
• Background snapshot creation
• Instant state switching

Keyboard Shortcuts:
• Ctrl+Z / Ctrl+Y - Standard undo/redo
• Ctrl+Alt+Z - Step backward through history
• Ctrl+Shift+S - Create named snapshot
• F12 - Open history panel`,
    shortDescription: 'Unlimited history with branching, snapshots, and visual preview',
    version: '1.6.0',
    category: 'panels',
    tags: ['history', 'undo', 'snapshots', 'workflow', 'productivity'],
    authorId: null,
    authorName: 'UndoMaster',
    fileUrl: '/uploads/plugins/history-panel-pro.zip',
    fileName: 'history-panel-pro.zip',
    fileSize: 7890,
    screenshotUrls: [],
    downloadCount: 534,
    likeCount: 187,
    status: 'approved',
    liked: false,
    createdAt: '2024-02-06T11:00:00Z',
    updatedAt: '2024-02-24T10:00:00Z',
  },
  {
    id: 'sample-19',
    name: 'Minimal Light Theme',
    slug: 'minimal-light-theme',
    description: `Clean, distraction-free light theme for focused creative work.

Design Philosophy:
• Maximum canvas visibility
• Minimal chrome and borders
• Soft shadows instead of hard lines
• Reduced visual noise
• Accessibility-first color choices

Color Variants:
• Pure White - Clean white background
• Warm Paper - Slight cream tint, easy on eyes
• Cool Gray - Professional neutral tone
• Soft Blue - Calming blue-gray palette
• Sage Green - Nature-inspired relaxing green

Typography:
• SF Pro / Inter font family
• Optimized font sizes for readability
• Clear visual hierarchy
• Consistent spacing system

Special Features:
• Focus mode (hide all panels)
• Reading mode for documents
• Auto-switch with system theme
• Custom accent color picker
• Reduced motion option`,
    shortDescription: 'Clean minimalist light theme with 5 color variants',
    version: '1.3.0',
    category: 'themes',
    tags: ['light', 'minimal', 'clean', 'professional', 'accessibility'],
    authorId: null,
    authorName: 'MinimalUI',
    fileUrl: '/uploads/plugins/minimal-light-theme.zip',
    fileName: 'minimal-light-theme.zip',
    fileSize: 3240,
    screenshotUrls: [],
    downloadCount: 456,
    likeCount: 145,
    status: 'approved',
    liked: false,
    createdAt: '2024-02-12T13:00:00Z',
    updatedAt: '2024-02-26T09:00:00Z',
  },
  {
    id: 'sample-20',
    name: 'Watermark Batch Tool',
    slug: 'watermark-batch-tool',
    description: `Professional watermarking solution for protecting your artwork.

Watermark Types:
• Text Watermark - Custom text with fonts
• Image Watermark - Logo or signature overlay
• Tiled Pattern - Repeating watermark grid
• QR Code - Link to your portfolio
• Invisible Watermark - Steganographic embedding

Positioning:
• 9-point placement (corners, edges, center)
• Custom X/Y coordinates
• Percentage-based positioning
• Smart placement avoiding key areas
• Random position for batch

Styling Options:
• Opacity control (1-100%)
• Blend modes
• Drop shadow and glow
• Rotation angle
• Scale relative to image

Batch Processing:
• Process entire folders
• Preserve folder structure
• Skip already watermarked files
• Preview before processing
• Undo batch operations`,
    shortDescription: 'Batch watermarking with text, logo, and invisible options',
    version: '2.0.0',
    category: 'automation',
    tags: ['watermark', 'copyright', 'protection', 'batch', 'branding'],
    authorId: null,
    authorName: 'ProtectArt',
    fileUrl: '/uploads/plugins/watermark-batch-tool.zip',
    fileName: 'watermark-batch-tool.zip',
    fileSize: 6540,
    screenshotUrls: [],
    downloadCount: 712,
    likeCount: 234,
    status: 'approved',
    liked: false,
    createdAt: '2024-02-15T10:00:00Z',
    updatedAt: '2024-02-27T16:00:00Z',
  },
]

interface PluginStoreState {
  // Plugin list state
  plugins: Plugin[]
  myPlugins: Plugin[]
  selectedPlugin: Plugin | null
  isLoading: boolean
  error: string | null

  // Pagination
  currentPage: number
  totalPages: number
  totalPlugins: number

  // Filters
  searchQuery: string
  selectedCategory: PluginCategory | 'all'
  sortOption: PluginSortOption

  // Modals
  showDetailModal: boolean
  showUploadModal: boolean

  // Actions
  fetchPlugins: (params?: PluginListParams) => Promise<void>
  fetchMyPlugins: () => Promise<void>
  fetchPluginDetail: (id: string) => Promise<void>
  toggleLike: (id: string) => Promise<void>
  setSearchQuery: (query: string) => void
  setSelectedCategory: (category: PluginCategory | 'all') => void
  setSortOption: (sort: PluginSortOption) => void
  setCurrentPage: (page: number) => void
  openDetailModal: (plugin: Plugin) => void
  closeDetailModal: () => void
  openUploadModal: () => void
  closeUploadModal: () => void
  clearError: () => void
  refreshPlugins: () => Promise<void>
}

export const usePluginStore = create<PluginStoreState>((set, get) => ({
  // Initial state
  plugins: [],
  myPlugins: [],
  selectedPlugin: null,
  isLoading: false,
  error: null,

  currentPage: 1,
  totalPages: 1,
  totalPlugins: 0,

  searchQuery: '',
  selectedCategory: 'all',
  sortOption: 'newest',

  showDetailModal: false,
  showUploadModal: false,

  // Actions
  fetchPlugins: async (params?: PluginListParams) => {
    set({ isLoading: true, error: null })
    try {
      const { searchQuery, selectedCategory, sortOption, currentPage } = get()
      const response = await getPlugins({
        page: params?.page ?? currentPage,
        limit: params?.limit ?? 20,
        search: params?.search ?? (searchQuery || undefined),
        category: params?.category ?? selectedCategory,
        sort: params?.sort ?? sortOption,
        ...params,
      })

      if (response.success) {
        set({
          plugins: response.plugins,
          currentPage: response.pagination.page,
          totalPages: response.pagination.totalPages,
          totalPlugins: response.pagination.total,
          isLoading: false,
        })
      } else {
        // API returned error, use sample data as fallback
        const { searchQuery, selectedCategory } = get()
        let fallbackPlugins = SAMPLE_PLUGINS

        if (searchQuery) {
          const query = searchQuery.toLowerCase()
          fallbackPlugins = fallbackPlugins.filter(p =>
            p.name.toLowerCase().includes(query) ||
            p.description.toLowerCase().includes(query) ||
            p.tags.some(t => t.toLowerCase().includes(query))
          )
        }

        if (selectedCategory && selectedCategory !== 'all') {
          fallbackPlugins = fallbackPlugins.filter(p => p.category === selectedCategory)
        }

        set({
          plugins: fallbackPlugins,
          currentPage: 1,
          totalPages: 1,
          totalPlugins: fallbackPlugins.length,
          isLoading: false,
          error: null,
        })
      }
    } catch (error) {
      // Network error, use sample data as fallback
      const { searchQuery, selectedCategory } = get()
      let fallbackPlugins = SAMPLE_PLUGINS

      if (searchQuery) {
        const query = searchQuery.toLowerCase()
        fallbackPlugins = fallbackPlugins.filter(p =>
          p.name.toLowerCase().includes(query) ||
          p.description.toLowerCase().includes(query) ||
          p.tags.some(t => t.toLowerCase().includes(query))
        )
      }

      if (selectedCategory && selectedCategory !== 'all') {
        fallbackPlugins = fallbackPlugins.filter(p => p.category === selectedCategory)
      }

      set({
        plugins: fallbackPlugins,
        currentPage: 1,
        totalPages: 1,
        totalPlugins: fallbackPlugins.length,
        isLoading: false,
        error: null,
      })
    }
  },

  fetchMyPlugins: async () => {
    try {
      const response = await getMyPlugins()
      if (response.success) {
        set({ myPlugins: response.plugins })
      }
    } catch (error) {
      console.error('Failed to fetch user plugins:', error)
    }
  },

  fetchPluginDetail: async (id: string) => {
    set({ isLoading: true, error: null })
    try {
      const response = await getPlugin(id)
      if (response.success && response.plugin) {
        set({
          selectedPlugin: response.plugin,
          showDetailModal: true,
          isLoading: false,
        })
      } else {
        set({
          error: response.message || 'Failed to fetch plugin details',
          isLoading: false,
        })
      }
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : 'Network error',
        isLoading: false,
      })
    }
  },

  toggleLike: async (id: string) => {
    const { plugins, selectedPlugin } = get()
    const plugin = plugins.find(p => p.id === id) || selectedPlugin

    if (!plugin) return

    try {
      let response
      if (plugin.liked) {
        response = await unlikePlugin(id)
      } else {
        response = await likePlugin(id)
      }

      if (response.success && response.data) {
        const newLikeCount = response.data.likeCount
        const newLiked = !plugin.liked

        // Update plugin in list
        set({
          plugins: plugins.map(p =>
            p.id === id ? { ...p, liked: newLiked, likeCount: newLikeCount } : p
          ),
        })

        // Update selected plugin if it's the same
        if (selectedPlugin?.id === id) {
          set({
            selectedPlugin: {
              ...selectedPlugin,
              liked: newLiked,
              likeCount: newLikeCount,
            },
          })
        }
      }
    } catch (error) {
      console.error('Failed to toggle like:', error)
    }
  },

  setSearchQuery: (query: string) => {
    set({ searchQuery: query, currentPage: 1 })
  },

  setSelectedCategory: (category: PluginCategory | 'all') => {
    set({ selectedCategory: category, currentPage: 1 })
  },

  setSortOption: (sort: PluginSortOption) => {
    set({ sortOption: sort, currentPage: 1 })
  },

  setCurrentPage: (page: number) => {
    set({ currentPage: page })
  },

  openDetailModal: (plugin: Plugin) => {
    set({ selectedPlugin: plugin, showDetailModal: true })
  },

  closeDetailModal: () => {
    set({ showDetailModal: false, selectedPlugin: null })
  },

  openUploadModal: () => {
    set({ showUploadModal: true })
  },

  closeUploadModal: () => {
    set({ showUploadModal: false })
  },

  clearError: () => {
    set({ error: null })
  },

  refreshPlugins: async () => {
    const { fetchPlugins } = get()
    await fetchPlugins()
  },
}))

// Category labels for display
export const CATEGORY_LABELS: Record<PluginCategory | 'all', string> = {
  all: 'All',
  brushes: 'Brushes',
  filters: 'Filters',
  tools: 'Tools',
  panels: 'Panels',
  themes: 'Themes',
  automation: 'Automation',
  other: 'Other',
}

export const CATEGORY_LABELS_ZH: Record<PluginCategory | 'all', string> = {
  all: 'All',
  brushes: 'Painting Brushes',
  filters: 'Filters',
  tools: 'Tools',
  panels: 'Panels',
  themes: 'Themes',
  automation: 'Automation',
  other: 'Other',
}

export const SORT_OPTIONS: { value: PluginSortOption; label: string }[] = [
  { value: 'newest', label: 'Newest' },
  { value: 'popular', label: 'Most Popular' },
  { value: 'liked', label: 'Most Liked' },
  { value: 'oldest', label: 'Oldest' },
]

export const STATUS_LABELS: Record<string, string> = {
  pending: 'Pending Review',
  approved: 'Approved',
  rejected: 'Rejected',
  unpublished: 'Unpublished',
}
