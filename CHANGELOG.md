# Changelog

All notable changes to DrawConnect will be documented in this file.

## [0.1.4] - M1.4 - 2026-01-15

### Added
- **Eraser Tool**: Fully functional eraser tool using Porter-Duff "destination-out" compositing
  - Same pressure sensitivity as brush tool
  - Accessible via toolbar eraser icon
- **Proper Undo/Redo System**: Layer-based history management
  - Saves complete layer state before each stroke
  - Supports up to 100 undo steps (configurable)
  - Keyboard shortcuts: Ctrl+Z (Undo), Ctrl+Y (Redo)
- **Zoom and Pan**: Navigate large canvases easily
  - Ctrl+Scroll: Zoom in/out (10% to 1000%)
  - Space+Drag: Pan the canvas
  - Keyboard shortcuts: Ctrl++ (Zoom In), Ctrl+- (Zoom Out), Ctrl+0 (Reset)
  - Menu bar controls in View menu
- **Layer Operations**: Enhanced layer management
  - Move Layer Up/Down: Reorder layers in the stack
  - Duplicate Layer: Create a copy of the selected layer
  - Merge Down: Merge current layer with the one below

### Changed
- **History System**: Replaced canvas-tile based undo with layer-state based undo
  - More accurate restoration of drawing state
  - Better memory management for complex operations
- **BrushEngine**: Added BrushMode enum (Normal/Eraser) for tool switching

### Technical Details
- New `HistoryManager` struct for managing undo/redo state
- New `LayerSnapshot` and `HistoryState` types for history tracking
- Added `set_brush_mode` Tauri command for tool switching
- Added layer operation commands: `move_layer_up`, `move_layer_down`, `duplicate_layer`, `merge_layer_down`
- Canvas component now handles Space key for pan mode
- Zoom state stored in appStore with panX/panY offsets

## [0.1.3] - M1.3 - 2026-01-15

### Added
- **Image Import**: Import local images to the active layer or as a new layer
  - Supports PNG, JPEG, GIF, BMP, WebP, TIFF, ICO formats
  - Images are automatically centered on the canvas
  - Menu items: "Import Image" and "Import Image as Layer"
- **Chinese Localization (zh-CN)**: Complete Chinese translation for the UI
  - Menu bar, tool names, layer panel, brush panel
  - Blend modes, dialogs, status bar, error messages
- **Debug Command**: Added `debug_layer_info` for troubleshooting layer issues

### Fixed
- **Brush Color Sync**: Fixed brush color not changing - canvas preview now uses selected color
- **Export Rendering**: Fixed exported images not showing drawings/imported content
  - Root cause: Brush strokes were written to Canvas tiles but render read from Layer.pixels
  - Solution: Strokes now write directly to layer.pixels via `render_stroke_to_layer`
- **Layer Pixel Array**: Added validation and auto-resize for layer pixel arrays
- **Bounds Checking**: Improved pixel bounds checking with clearer `idx + 4 <= len` pattern

### Changed
- **Data Flow Architecture**: Unified data storage for brush strokes and imported images
  - Both now write to `layer.pixels` which is read by `render_cpu`
- **Error Handling**: Import errors now display dialog boxes to users instead of silent failures

### Technical Details
- New method `BrushEngine::render_stroke_to_layer()` for direct layer pixel manipulation
- New method `BrushEngine::render_point_to_layer()` with Porter-Duff alpha compositing
- Modified `DrawEngine::process_stroke()` to use layer-based rendering

## [0.1.2] - M1.2

### Added
- Basic brush engine with pressure sensitivity
- Layer system with visibility and opacity controls
- Canvas with tile-based storage
- Render pipeline with CPU fallback

## [0.1.1] - M1.1

### Added
- Tauri + React desktop application structure
- Basic UI components (MenuBar, Canvas, LayerPanel, BrushPanel)
- Zustand state management

## [0.1.0] - M1.0

### Added
- Initial project structure
- Core engine foundation in Rust
- Basic color management
