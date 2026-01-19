import { create } from 'zustand'
import { invoke } from '@tauri-apps/api/tauri'

export interface Layer {
  id: string
  name: string
  visible: boolean
  locked: boolean
  opacity: number
  blendMode: string
}

export interface Brush {
  name: string
  category: string
  size: number
  opacity: number
  hardness: number
}

export type Tool = 'brush' | 'eraser' | 'move' | 'select_rect' | 'select_lasso' | 'select_magic' | 'eyedropper' | 'fill'

export type AppMode = 'draw' | 'edit' | null

export type SelectionMode = 'replace' | 'add' | 'subtract' | 'intersect'

export type AdjustmentType =
  | 'brightness_contrast'
  | 'hue_saturation'
  | 'levels'
  | 'posterize'
  | 'threshold'
  | 'gaussian_blur'
  | 'pixelate'
  | 'emboss'
  | 'oil_paint'
  | null

export interface SelectionInfo {
  is_active: boolean
  bounds: [number, number, number, number] | null
  mode: SelectionMode
  feather: number
  shape_type: 'none' | 'rectangle' | 'lasso' | 'mask'
  points?: Array<[number, number]>
}

interface AppState {
  // App mode
  appMode: AppMode

  // Canvas state
  isInitialized: boolean
  canvasWidth: number
  canvasHeight: number
  canvasImage: string | null
  zoom: number
  panX: number
  panY: number

  // Tool state
  currentTool: Tool
  brushColor: string
  brushSize: number
  brushOpacity: number

  // Layers
  layers: Layer[]
  activeLayerId: string | null

  // Brushes
  brushes: Brush[]
  currentBrush: string | null

  // Undo/Redo
  canUndo: boolean
  canRedo: boolean

  // Selection
  selection: SelectionInfo | null
  selectionMode: SelectionMode

  // Active adjustment panel
  activeAdjustment: AdjustmentType

  // Actions
  setAppMode: (mode: AppMode) => void
  initializeCanvas: (width: number, height: number) => Promise<void>
  openImageAsCanvas: (path: string) => Promise<void>
  renderCanvas: () => Promise<void>
  setCurrentTool: (tool: Tool) => void
  setBrushColor: (color: string) => Promise<void>
  setBrushSize: (size: number) => Promise<void>
  setBrushOpacity: (opacity: number) => Promise<void>
  setCurrentBrush: (name: string) => Promise<void>
  importBrush: (path: string) => Promise<void>
  exportBrush: (brushName: string, path: string) => Promise<void>
  deleteCustomBrush: (name: string) => Promise<void>
  refreshBrushes: () => Promise<void>
  addLayer: (name: string) => Promise<void>
  deleteLayer: (id: string) => Promise<void>
  setActiveLayer: (id: string) => Promise<void>
  setLayerVisibility: (id: string, visible: boolean) => Promise<void>
  setLayerOpacity: (id: string, opacity: number) => Promise<void>
  moveLayerUp: (id: string) => Promise<void>
  moveLayerDown: (id: string) => Promise<void>
  duplicateLayer: (id: string) => Promise<void>
  mergeLayerDown: (id: string) => Promise<void>
  refreshLayers: () => Promise<void>
  processStroke: (points: StrokePoint[]) => Promise<void>
  // 增量笔触方法
  beginStroke: () => Promise<void>
  addStrokePoint: (point: StrokePoint) => Promise<void>
  endStroke: () => Promise<void>
  undo: () => Promise<void>
  redo: () => Promise<void>
  saveFile: (path: string) => Promise<void>
  exportPng: (path: string) => Promise<void>
  importImage: (path: string) => Promise<void>
  importImageAsLayer: (path: string) => Promise<void>
  // Selection
  selectRect: (x: number, y: number, width: number, height: number) => Promise<void>
  selectLasso: (points: Array<[number, number]>) => Promise<void>
  selectMagicWand: (x: number, y: number, tolerance?: number) => Promise<void>
  clearSelection: () => Promise<void>
  selectAll: () => Promise<void>
  invertSelection: () => Promise<void>
  setSelectionMode: (mode: SelectionMode) => Promise<void>
  // Eyedropper and Fill
  pickColor: (x: number, y: number) => Promise<string | null>
  floodFill: (x: number, y: number) => Promise<void>
  // Adjustment panel
  setActiveAdjustment: (type: AdjustmentType) => void
  // Zoom and Pan
  setZoom: (zoom: number) => void
  zoomIn: () => void
  zoomOut: () => void
  resetZoom: () => void
  setPan: (x: number, y: number) => void
  resetPan: () => void

  // Image Adjustments
  adjustBrightnessContrast: (brightness: number, contrast: number) => Promise<void>
  adjustLevels: (inputBlack: number, inputWhite: number, gamma: number, outputBlack: number, outputWhite: number, channel?: string) => Promise<void>
  adjustCurves: (points: Array<[number, number]>, channel?: string) => Promise<void>
  adjustHueSaturation: (hue: number, saturation: number, lightness: number) => Promise<void>
  adjustColorBalance: (shadows: [number, number, number], midtones: [number, number, number], highlights: [number, number, number]) => Promise<void>
  adjustVibrance: (vibrance: number, saturation: number) => Promise<void>
  adjustExposure: (exposure: number, offset: number, gamma: number) => Promise<void>
  adjustBlackWhite: (red: number, yellow: number, green: number, cyan: number, blue: number, magenta: number) => Promise<void>
  adjustPhotoFilter: (color: string, density: number, preserveLuminosity: boolean) => Promise<void>
  adjustInvert: () => Promise<void>
  adjustPosterize: (levels: number) => Promise<void>
  adjustThreshold: (level: number) => Promise<void>

  // Filters
  filterGaussianBlur: (radius: number) => Promise<void>
  filterBoxBlur: (radius: number) => Promise<void>
  filterMotionBlur: (angle: number, distance: number) => Promise<void>
  filterRadialBlur: (amount: number, centerX: number, centerY: number, blurType?: string) => Promise<void>
  filterUnsharpMask: (amount: number, radius: number, threshold: number) => Promise<void>
  filterHighPass: (radius: number) => Promise<void>
  filterAddNoise: (amount: number, noiseType?: string, monochrome?: boolean) => Promise<void>
  filterReduceNoise: (strength: number, preserveDetails: number) => Promise<void>
  filterFindEdges: () => Promise<void>
  filterEmboss: (angle: number, height: number, amount: number) => Promise<void>
  filterPixelate: (cellSize: number) => Promise<void>
  filterOilPaint: (radius: number, levels: number) => Promise<void>
  // Distort Filters
  filterSpherize: (amount: number, mode?: string) => Promise<void>
  filterTwirl: (angle: number, radius?: number) => Promise<void>
  filterWave: (wavelength: number, amplitude: number, waveType?: string) => Promise<void>
  filterRipple: (amount: number, size?: string) => Promise<void>
  // Render Filters
  filterVignette: (amount: number, midpoint?: number, feather?: number) => Promise<void>
  filterLensFlare: (centerX: number, centerY: number, brightness?: number, style?: string) => Promise<void>
  filterClouds: (foreground?: string, background?: string, seed?: number) => Promise<void>

  // Transforms
  transformRotate90CW: () => Promise<void>
  transformRotate90CCW: () => Promise<void>
  transformRotate180: () => Promise<void>
  transformRotate: (angle: number) => Promise<void>
  transformFlipHorizontal: () => Promise<void>
  transformFlipVertical: () => Promise<void>
  transformCrop: (x: number, y: number, width: number, height: number) => Promise<void>
  transformCanvasResize: (width: number, height: number, anchor?: string, fillColor?: string) => Promise<void>
  transformImageResize: (width: number, height: number, interpolation?: string) => Promise<void>

  // PS Resource Import
  importAbrBrushes: (path: string) => Promise<ImportedBrushInfo[]>
  importPatPatterns: (path: string) => Promise<ImportedPatternInfo[]>
  importColorSwatches: (path: string) => Promise<ImportedSwatchInfo[]>
}

export interface StrokePoint {
  x: number
  y: number
  pressure: number
  tilt_x: number
  tilt_y: number
  timestamp: number
}

// PS Import types
export interface ImportedBrushInfo {
  name: string
  diameter: number
  hardness: number
  spacing: number
  angle: number
  roundness: number
  has_tip_image: boolean
}

export interface ImportedPatternInfo {
  name: string
  width: number
  height: number
}

export interface ImportedSwatchInfo {
  name: string
  hex: string
  r: number
  g: number
  b: number
  a: number
}

export const useAppStore = create<AppState>((set, get) => ({
  // Initial state
  appMode: null,

  isInitialized: false,
  canvasWidth: 1920,
  canvasHeight: 1080,
  canvasImage: null,
  zoom: 1,
  panX: 0,
  panY: 0,

  currentTool: 'brush',
  brushColor: '#000000',
  brushSize: 10,
  brushOpacity: 1,

  layers: [],
  activeLayerId: null,

  brushes: [],
  currentBrush: null,

  canUndo: false,
  canRedo: false,

  selection: null,
  selectionMode: 'replace' as SelectionMode,

  activeAdjustment: null,

  // Actions
  setAppMode: (mode: AppMode) => {
    set({ appMode: mode })
  },

  initializeCanvas: async (width: number, height: number) => {
    try {
      await invoke('create_canvas', {
        width,
        height,
        dpi: 300,
        background: '#FFFFFF',
      })

      // Load brushes
      const brushes = await invoke<Brush[]>('get_brushes')

      // Get layers
      const layers = await invoke<Layer[]>('get_layers')

      // Sync initial brush color to backend
      const initialColor = get().brushColor
      await invoke('set_brush_color', { hex: initialColor })

      // Render initial canvas
      const image = await invoke<string>('render_canvas')

      set({
        isInitialized: true,
        canvasWidth: width,
        canvasHeight: height,
        canvasImage: image,
        brushes,
        layers,
        activeLayerId: layers.length > 0 ? layers[layers.length - 1].id : null,
        currentBrush: brushes.length > 0 ? brushes[0].name : null,
      })
    } catch (error) {
      console.error('Failed to initialize canvas:', error)
    }
  },

  openImageAsCanvas: async (path: string) => {
    try {
      const result = await invoke<{ width: number; height: number }>('open_image_as_canvas', { path })

      // Load brushes
      const brushes = await invoke<Brush[]>('get_brushes')

      // Get layers
      const layers = await invoke<Layer[]>('get_layers')

      // Get the first layer (Background with the image) and set it as active
      const activeLayerId = layers.length > 0 ? layers[0].id : null
      if (activeLayerId) {
        await invoke('set_active_layer', { layerId: activeLayerId })
      }

      // Render canvas
      const image = await invoke<string>('render_canvas')

      set({
        isInitialized: true,
        canvasWidth: result.width,
        canvasHeight: result.height,
        canvasImage: image,
        brushes,
        layers,
        activeLayerId,
        currentBrush: brushes.length > 0 ? brushes[0].name : null,
      })
    } catch (error) {
      console.error('Failed to open image as canvas:', error)
      throw error
    }
  },

  renderCanvas: async () => {
    try {
      const image = await invoke<string>('render_canvas')
      set({ canvasImage: image })
    } catch (error) {
      console.error('Failed to render canvas:', error)
    }
  },

  setCurrentTool: (tool: Tool) => {
    set({ currentTool: tool })
    // Sync brush mode with backend when switching between brush and eraser
    const mode = tool === 'eraser' ? 'eraser' : 'normal'
    invoke('set_brush_mode', { mode }).catch(console.error)
  },

  setBrushColor: async (color: string) => {
    try {
      await invoke('set_brush_color', { hex: color })
      set({ brushColor: color })
    } catch (error) {
      console.error('Failed to set brush color:', error)
    }
  },

  setBrushSize: async (size: number) => {
    try {
      await invoke('set_brush_size', { size })
      set({ brushSize: size })
    } catch (error) {
      console.error('Failed to set brush size:', error)
    }
  },

  setBrushOpacity: async (opacity: number) => {
    try {
      await invoke('set_brush_opacity', { opacity })
      set({ brushOpacity: opacity })
    } catch (error) {
      console.error('Failed to set brush opacity:', error)
    }
  },

  setCurrentBrush: async (name: string) => {
    try {
      await invoke('set_brush', { brushName: name })
      set({ currentBrush: name })
    } catch (error) {
      console.error('Failed to set brush:', error)
    }
  },

  importBrush: async (path: string) => {
    try {
      await invoke('import_brush', { path })
      await get().refreshBrushes()
    } catch (error) {
      console.error('Failed to import brush:', error)
      throw error
    }
  },

  exportBrush: async (brushName: string, path: string) => {
    try {
      await invoke('export_brush', { brushName, path })
    } catch (error) {
      console.error('Failed to export brush:', error)
      throw error
    }
  },

  deleteCustomBrush: async (name: string) => {
    try {
      await invoke('delete_custom_brush', { brushName: name })
      await get().refreshBrushes()
    } catch (error) {
      console.error('Failed to delete brush:', error)
      throw error
    }
  },

  refreshBrushes: async () => {
    try {
      const brushes = await invoke<Brush[]>('get_brushes')
      set({ brushes })
    } catch (error) {
      console.error('Failed to refresh brushes:', error)
    }
  },

  addLayer: async (name: string) => {
    try {
      await invoke('add_layer', { name })
      await get().refreshLayers()
    } catch (error) {
      console.error('Failed to add layer:', error)
    }
  },

  deleteLayer: async (id: string) => {
    try {
      await invoke('delete_layer', { layerId: id })
      await get().refreshLayers()
    } catch (error) {
      console.error('Failed to delete layer:', error)
    }
  },

  setActiveLayer: async (id: string) => {
    try {
      await invoke('set_active_layer', { layerId: id })
      set({ activeLayerId: id })
    } catch (error) {
      console.error('Failed to set active layer:', error)
    }
  },

  setLayerVisibility: async (id: string, visible: boolean) => {
    try {
      await invoke('set_layer_visibility', { layerId: id, visible })
      await get().refreshLayers()
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to set layer visibility:', error)
    }
  },

  setLayerOpacity: async (id: string, opacity: number) => {
    try {
      await invoke('set_layer_opacity', { layerId: id, opacity })
      await get().refreshLayers()
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to set layer opacity:', error)
    }
  },

  moveLayerUp: async (id: string) => {
    try {
      await invoke('move_layer_up', { layerId: id })
      await get().refreshLayers()
    } catch (error) {
      console.error('Failed to move layer up:', error)
    }
  },

  moveLayerDown: async (id: string) => {
    try {
      await invoke('move_layer_down', { layerId: id })
      await get().refreshLayers()
    } catch (error) {
      console.error('Failed to move layer down:', error)
    }
  },

  duplicateLayer: async (id: string) => {
    try {
      await invoke('duplicate_layer', { layerId: id })
      await get().refreshLayers()
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to duplicate layer:', error)
    }
  },

  mergeLayerDown: async (id: string) => {
    try {
      await invoke('merge_layer_down', { layerId: id })
      await get().refreshLayers()
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to merge layer down:', error)
    }
  },

  refreshLayers: async () => {
    try {
      const layers = await invoke<Layer[]>('get_layers')
      set({ layers })
    } catch (error) {
      console.error('Failed to refresh layers:', error)
    }
  },

  processStroke: async (points: StrokePoint[]) => {
    try {
      await invoke('process_stroke', { points })
      await get().renderCanvas()

      // Update undo/redo state
      const canUndo = await invoke<boolean>('can_undo')
      const canRedo = await invoke<boolean>('can_redo')
      set({ canUndo, canRedo })
    } catch (error) {
      console.error('Failed to process stroke:', error)
    }
  },

  // 增量笔触方法 - 实时渲染
  beginStroke: async () => {
    try {
      await invoke('begin_stroke')
    } catch (error) {
      console.error('Failed to begin stroke:', error)
    }
  },

  addStrokePoint: async (point: StrokePoint) => {
    try {
      await invoke('add_stroke_point', { point })
    } catch (error) {
      console.error('Failed to add stroke point:', error)
    }
  },

  endStroke: async () => {
    try {
      await invoke('end_stroke')
      await get().renderCanvas()

      // Update undo/redo state
      const canUndo = await invoke<boolean>('can_undo')
      const canRedo = await invoke<boolean>('can_redo')
      set({ canUndo, canRedo })
    } catch (error) {
      console.error('Failed to end stroke:', error)
    }
  },

  undo: async () => {
    try {
      await invoke('undo')
      await get().renderCanvas()

      const canUndo = await invoke<boolean>('can_undo')
      const canRedo = await invoke<boolean>('can_redo')
      set({ canUndo, canRedo })
    } catch (error) {
      console.error('Failed to undo:', error)
    }
  },

  redo: async () => {
    try {
      await invoke('redo')
      await get().renderCanvas()

      const canUndo = await invoke<boolean>('can_undo')
      const canRedo = await invoke<boolean>('can_redo')
      set({ canUndo, canRedo })
    } catch (error) {
      console.error('Failed to redo:', error)
    }
  },

  saveFile: async (path: string) => {
    try {
      await invoke('save_file', { path })
    } catch (error) {
      console.error('Failed to save file:', error)
    }
  },

  exportPng: async (path: string) => {
    try {
      await invoke('export_png', { path })
    } catch (error) {
      console.error('Failed to export PNG:', error)
    }
  },

  importImage: async (path: string) => {
    try {
      await invoke('import_image', { path })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to import image:', error)
      throw error // Re-throw to show error to user
    }
  },

  importImageAsLayer: async (path: string) => {
    try {
      await invoke('import_image_as_layer', { path })
      await get().refreshLayers()
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to import image as layer:', error)
      throw error // Re-throw to show error to user
    }
  },

  // Selection actions
  selectRect: async (x: number, y: number, width: number, height: number) => {
    try {
      const selection = await invoke<SelectionInfo>('select_rect', { x, y, width, height })
      set({ selection })
    } catch (error) {
      console.error('Failed to create rectangle selection:', error)
    }
  },

  selectLasso: async (points: Array<[number, number]>) => {
    try {
      const selection = await invoke<SelectionInfo>('select_lasso', { points })
      set({ selection })
    } catch (error) {
      console.error('Failed to create lasso selection:', error)
    }
  },

  selectMagicWand: async (x: number, y: number, tolerance?: number) => {
    try {
      const selection = await invoke<SelectionInfo>('select_magic_wand', { x, y, tolerance })
      set({ selection })
    } catch (error) {
      console.error('Failed to create magic wand selection:', error)
    }
  },

  clearSelection: async () => {
    try {
      await invoke('clear_selection')
      set({ selection: null })
    } catch (error) {
      console.error('Failed to clear selection:', error)
    }
  },

  selectAll: async () => {
    try {
      const selection = await invoke<SelectionInfo>('select_all')
      set({ selection })
    } catch (error) {
      console.error('Failed to select all:', error)
    }
  },

  invertSelection: async () => {
    try {
      const selection = await invoke<SelectionInfo>('invert_selection')
      set({ selection })
    } catch (error) {
      console.error('Failed to invert selection:', error)
    }
  },

  setSelectionMode: async (mode: SelectionMode) => {
    try {
      await invoke('set_selection_mode', { mode })
      set({ selectionMode: mode })
    } catch (error) {
      console.error('Failed to set selection mode:', error)
    }
  },

  // Eyedropper and Fill actions
  pickColor: async (x: number, y: number) => {
    try {
      const { canvasWidth, canvasHeight } = get()
      // Clamp coordinates to valid canvas range
      const clampedX = Math.max(0, Math.min(canvasWidth - 1, Math.round(x)))
      const clampedY = Math.max(0, Math.min(canvasHeight - 1, Math.round(y)))
      const result = await invoke<{ hex: string }>('pick_color', { x: clampedX, y: clampedY })
      // Set the picked color as the current brush color
      await get().setBrushColor(result.hex)
      return result.hex
    } catch (error) {
      console.error('Failed to pick color:', error)
      return null
    }
  },

  floodFill: async (x: number, y: number) => {
    try {
      const { brushColor, canvasWidth, canvasHeight } = get()
      // Clamp coordinates to valid canvas range
      const clampedX = Math.max(0, Math.min(canvasWidth - 1, Math.round(x)))
      const clampedY = Math.max(0, Math.min(canvasHeight - 1, Math.round(y)))
      await invoke('flood_fill', { x: clampedX, y: clampedY, hex: brushColor, tolerance: 0.1 })
      await get().renderCanvas()
      // Update undo/redo state
      const canUndo = await invoke<boolean>('can_undo')
      const canRedo = await invoke<boolean>('can_redo')
      set({ canUndo, canRedo })
    } catch (error) {
      console.error('Failed to flood fill:', error)
    }
  },

  // Adjustment panel
  setActiveAdjustment: (type: AdjustmentType) => {
    set({ activeAdjustment: type })
  },

  // Zoom and Pan actions
  setZoom: (zoom: number) => {
    // Clamp zoom between 10% and 1000%
    const clampedZoom = Math.max(0.1, Math.min(10, zoom))
    set({ zoom: clampedZoom })
  },

  zoomIn: () => {
    const currentZoom = get().zoom
    const newZoom = Math.min(10, currentZoom * 1.25)
    set({ zoom: newZoom })
  },

  zoomOut: () => {
    const currentZoom = get().zoom
    const newZoom = Math.max(0.1, currentZoom / 1.25)
    set({ zoom: newZoom })
  },

  resetZoom: () => {
    set({ zoom: 1 })
  },

  setPan: (x: number, y: number) => {
    set({ panX: x, panY: y })
  },

  resetPan: () => {
    set({ panX: 0, panY: 0 })
  },

  // Image Adjustments
  adjustBrightnessContrast: async (brightness: number, contrast: number) => {
    try {
      await invoke('adjust_brightness_contrast', { brightness, contrast })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to adjust brightness/contrast:', error)
      throw error
    }
  },

  adjustLevels: async (inputBlack: number, inputWhite: number, gamma: number, outputBlack: number, outputWhite: number, channel?: string) => {
    try {
      await invoke('adjust_levels', { inputBlack, inputWhite, gamma, outputBlack, outputWhite, channel })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to adjust levels:', error)
      throw error
    }
  },

  adjustCurves: async (points: Array<[number, number]>, channel?: string) => {
    try {
      await invoke('adjust_curves', { points, channel })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to adjust curves:', error)
      throw error
    }
  },

  adjustHueSaturation: async (hue: number, saturation: number, lightness: number) => {
    try {
      await invoke('adjust_hue_saturation', { hue, saturation, lightness })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to adjust hue/saturation:', error)
      throw error
    }
  },

  adjustColorBalance: async (shadows: [number, number, number], midtones: [number, number, number], highlights: [number, number, number]) => {
    try {
      await invoke('adjust_color_balance', { shadows, midtones, highlights })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to adjust color balance:', error)
      throw error
    }
  },

  adjustVibrance: async (vibrance: number, saturation: number) => {
    try {
      await invoke('adjust_vibrance', { vibrance, saturation })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to adjust vibrance:', error)
      throw error
    }
  },

  adjustExposure: async (exposure: number, offset: number, gamma: number) => {
    try {
      await invoke('adjust_exposure', { exposure, offset, gamma })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to adjust exposure:', error)
      throw error
    }
  },

  adjustBlackWhite: async (red: number, yellow: number, green: number, cyan: number, blue: number, magenta: number) => {
    try {
      await invoke('adjust_black_white', { red, yellow, green, cyan, blue, magenta })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to convert to black and white:', error)
      throw error
    }
  },

  adjustPhotoFilter: async (color: string, density: number, preserveLuminosity: boolean) => {
    try {
      await invoke('adjust_photo_filter', { color, density, preserveLuminosity })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to apply photo filter:', error)
      throw error
    }
  },

  adjustInvert: async () => {
    try {
      await invoke('adjust_invert')
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to invert colors:', error)
      throw error
    }
  },

  adjustPosterize: async (levels: number) => {
    try {
      await invoke('adjust_posterize', { levels })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to posterize:', error)
      throw error
    }
  },

  adjustThreshold: async (level: number) => {
    try {
      await invoke('adjust_threshold', { level })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to apply threshold:', error)
      throw error
    }
  },

  // Filters
  filterGaussianBlur: async (radius: number) => {
    try {
      await invoke('filter_gaussian_blur', { radius })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to apply Gaussian blur:', error)
      throw error
    }
  },

  filterBoxBlur: async (radius: number) => {
    try {
      await invoke('filter_box_blur', { radius })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to apply box blur:', error)
      throw error
    }
  },

  filterMotionBlur: async (angle: number, distance: number) => {
    try {
      await invoke('filter_motion_blur', { angle, distance })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to apply motion blur:', error)
      throw error
    }
  },

  filterRadialBlur: async (amount: number, centerX: number, centerY: number, blurType?: string) => {
    try {
      await invoke('filter_radial_blur', { amount, centerX, centerY, blurType })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to apply radial blur:', error)
      throw error
    }
  },

  filterUnsharpMask: async (amount: number, radius: number, threshold: number) => {
    try {
      await invoke('filter_unsharp_mask', { amount, radius, threshold })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to apply unsharp mask:', error)
      throw error
    }
  },

  filterHighPass: async (radius: number) => {
    try {
      await invoke('filter_high_pass', { radius })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to apply high pass filter:', error)
      throw error
    }
  },

  filterAddNoise: async (amount: number, noiseType?: string, monochrome: boolean = false) => {
    try {
      await invoke('filter_add_noise', { amount, noiseType, monochrome })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to add noise:', error)
      throw error
    }
  },

  filterReduceNoise: async (strength: number, preserveDetails: number) => {
    try {
      await invoke('filter_reduce_noise', { strength, preserveDetails })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to reduce noise:', error)
      throw error
    }
  },

  filterFindEdges: async () => {
    try {
      await invoke('filter_find_edges')
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to find edges:', error)
      throw error
    }
  },

  filterEmboss: async (angle: number, height: number, amount: number) => {
    try {
      await invoke('filter_emboss', { angle, height, amount })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to apply emboss:', error)
      throw error
    }
  },

  filterPixelate: async (cellSize: number) => {
    try {
      await invoke('filter_pixelate', { cellSize })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to pixelate:', error)
      throw error
    }
  },

  filterOilPaint: async (radius: number, levels: number) => {
    try {
      await invoke('filter_oil_paint', { radius, levels })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to apply oil paint effect:', error)
      throw error
    }
  },

  // Distort Filters
  filterSpherize: async (amount: number, mode?: string) => {
    try {
      await invoke('filter_spherize', { amount, mode })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to apply spherize:', error)
      throw error
    }
  },

  filterTwirl: async (angle: number, radius?: number) => {
    try {
      await invoke('filter_twirl', { angle, radius })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to apply twirl:', error)
      throw error
    }
  },

  filterWave: async (wavelength: number, amplitude: number, waveType?: string) => {
    try {
      await invoke('filter_wave', { wavelength, amplitude, wave_type: waveType })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to apply wave:', error)
      throw error
    }
  },

  filterRipple: async (amount: number, size?: string) => {
    try {
      await invoke('filter_ripple', { amount, size })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to apply ripple:', error)
      throw error
    }
  },

  // Render Filters
  filterVignette: async (amount: number, midpoint?: number, feather?: number) => {
    try {
      await invoke('filter_vignette', { amount, midpoint, feather })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to apply vignette:', error)
      throw error
    }
  },

  filterLensFlare: async (centerX: number, centerY: number, brightness?: number, style?: string) => {
    try {
      await invoke('filter_lens_flare', { center_x: centerX, center_y: centerY, brightness, style })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to apply lens flare:', error)
      throw error
    }
  },

  filterClouds: async (foreground?: string, background?: string, seed?: number) => {
    try {
      await invoke('filter_clouds', { foreground, background, seed })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to generate clouds:', error)
      throw error
    }
  },

  // Transforms
  transformRotate90CW: async () => {
    try {
      await invoke('transform_rotate_90_cw')
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to rotate 90° CW:', error)
      throw error
    }
  },

  transformRotate90CCW: async () => {
    try {
      await invoke('transform_rotate_90_ccw')
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to rotate 90° CCW:', error)
      throw error
    }
  },

  transformRotate180: async () => {
    try {
      await invoke('transform_rotate_180')
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to rotate 180°:', error)
      throw error
    }
  },

  transformRotate: async (angle: number) => {
    try {
      await invoke('transform_rotate', { angle })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to rotate:', error)
      throw error
    }
  },

  transformFlipHorizontal: async () => {
    try {
      await invoke('transform_flip_horizontal')
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to flip horizontal:', error)
      throw error
    }
  },

  transformFlipVertical: async () => {
    try {
      await invoke('transform_flip_vertical')
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to flip vertical:', error)
      throw error
    }
  },

  transformCrop: async (x: number, y: number, width: number, height: number) => {
    try {
      await invoke('transform_crop', { x, y, width, height })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to crop:', error)
      throw error
    }
  },

  transformCanvasResize: async (width: number, height: number, anchor?: string, fillColor?: string) => {
    try {
      await invoke('transform_canvas_resize', { width, height, anchor, fillColor })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to resize canvas:', error)
      throw error
    }
  },

  transformImageResize: async (width: number, height: number, interpolation?: string) => {
    try {
      await invoke('transform_image_resize', { width, height, interpolation })
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to resize image:', error)
      throw error
    }
  },

  // PS Resource Import
  importAbrBrushes: async (path: string) => {
    try {
      const brushes = await invoke<ImportedBrushInfo[]>('import_abr_brushes', { path })
      return brushes
    } catch (error) {
      console.error('Failed to import ABR brushes:', error)
      throw error
    }
  },

  importPatPatterns: async (path: string) => {
    try {
      const patterns = await invoke<ImportedPatternInfo[]>('import_pat_patterns', { path })
      return patterns
    } catch (error) {
      console.error('Failed to import PAT patterns:', error)
      throw error
    }
  },

  importColorSwatches: async (path: string) => {
    try {
      const swatches = await invoke<ImportedSwatchInfo[]>('import_color_swatches', { path })
      return swatches
    } catch (error) {
      console.error('Failed to import color swatches:', error)
      throw error
    }
  },
}))
