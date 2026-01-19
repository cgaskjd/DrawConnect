import { create } from 'zustand'
import * as wasmEngine from '../wasm/engine'

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

export type AppMode = 'draw' | 'edit' | 'plugins' | null

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
  // Engine state
  isEngineReady: boolean

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
  initializeEngine: () => Promise<void>
  setAppMode: (mode: AppMode) => void
  initializeCanvas: (width: number, height: number) => Promise<void>
  openImageFile: () => Promise<void>
  renderCanvas: () => Promise<void>
  setCurrentTool: (tool: Tool) => void
  setBrushColor: (color: string) => Promise<void>
  setBrushSize: (size: number) => Promise<void>
  setBrushOpacity: (opacity: number) => Promise<void>
  setCurrentBrush: (name: string) => Promise<void>
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
  // Incremental stroke methods
  beginStroke: () => Promise<void>
  addStrokePoint: (point: StrokePoint) => Promise<void>
  endStroke: () => Promise<void>
  undo: () => Promise<void>
  redo: () => Promise<void>
  exportPng: (filename?: string) => Promise<void>
  exportJpeg: (filename?: string, quality?: number) => Promise<void>
  // Selection
  selectRect: (x: number, y: number, width: number, height: number) => Promise<void>
  selectLasso: (points: Array<[number, number]>) => Promise<void>
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
}

export interface StrokePoint {
  x: number
  y: number
  pressure: number
  tilt_x: number
  tilt_y: number
  timestamp: number
}

export const useAppStore = create<AppState>((set, get) => ({
  // Initial state
  isEngineReady: false,
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
  initializeEngine: async () => {
    try {
      await wasmEngine.initEngine()
      set({ isEngineReady: true })
      console.log('WASM Engine initialized successfully')
    } catch (error) {
      console.error('Failed to initialize WASM engine:', error)
      throw error
    }
  },

  setAppMode: (mode: AppMode) => {
    set({ appMode: mode })
  },

  initializeCanvas: async (width: number, height: number) => {
    try {
      await wasmEngine.createCanvas(width, height, 300, '#FFFFFF')

      // Load brushes
      const brushes = await wasmEngine.getBrushes() as Brush[]

      // Get layers
      const layers = await wasmEngine.getLayers() as Layer[]

      // Sync initial brush color to backend
      const initialColor = get().brushColor
      await wasmEngine.setBrushColor(initialColor)

      // Render initial canvas
      const imageBase64 = await wasmEngine.renderCanvas()

      set({
        isInitialized: true,
        canvasWidth: width,
        canvasHeight: height,
        canvasImage: imageBase64,
        brushes,
        layers,
        activeLayerId: layers.length > 0 ? layers[layers.length - 1].id : null,
        currentBrush: brushes.length > 0 ? brushes[0].name : null,
      })
    } catch (error) {
      console.error('Failed to initialize canvas:', error)
    }
  },

  openImageFile: async () => {
    try {
      const result = await wasmEngine.openImageFile()
      if (!result) return

      // Load brushes
      const brushes = await wasmEngine.getBrushes() as Brush[]

      // Get layers
      const layers = await wasmEngine.getLayers() as Layer[]

      // Get the first layer (Background with the image) and set it as active
      const activeLayerId = layers.length > 0 ? layers[0].id : null
      if (activeLayerId) {
        await wasmEngine.setActiveLayer(activeLayerId)
      }

      // Render canvas
      const imageBase64 = await wasmEngine.renderCanvas()

      set({
        isInitialized: true,
        canvasWidth: result.width,
        canvasHeight: result.height,
        canvasImage: imageBase64,
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
      const imageBase64 = await wasmEngine.renderCanvas()
      set({ canvasImage: imageBase64 })
    } catch (error) {
      console.error('Failed to render canvas:', error)
    }
  },

  setCurrentTool: (tool: Tool) => {
    set({ currentTool: tool })
    // Sync brush mode with backend when switching between brush and eraser
    const mode = tool === 'eraser' ? 'eraser' : 'normal'
    wasmEngine.setBrushMode(mode).catch(console.error)
  },

  setBrushColor: async (color: string) => {
    try {
      await wasmEngine.setBrushColor(color)
      set({ brushColor: color })
    } catch (error) {
      console.error('Failed to set brush color:', error)
    }
  },

  setBrushSize: async (size: number) => {
    try {
      await wasmEngine.setBrushSize(size)
      set({ brushSize: size })
    } catch (error) {
      console.error('Failed to set brush size:', error)
    }
  },

  setBrushOpacity: async (opacity: number) => {
    try {
      await wasmEngine.setBrushOpacity(opacity)
      set({ brushOpacity: opacity })
    } catch (error) {
      console.error('Failed to set brush opacity:', error)
    }
  },

  setCurrentBrush: async (name: string) => {
    try {
      await wasmEngine.setBrush(name)
      set({ currentBrush: name })
    } catch (error) {
      console.error('Failed to set brush:', error)
    }
  },

  refreshBrushes: async () => {
    try {
      const brushes = await wasmEngine.getBrushes() as Brush[]
      set({ brushes })
    } catch (error) {
      console.error('Failed to refresh brushes:', error)
    }
  },

  addLayer: async (name: string) => {
    try {
      await wasmEngine.addLayer(name)
      await get().refreshLayers()
    } catch (error) {
      console.error('Failed to add layer:', error)
    }
  },

  deleteLayer: async (id: string) => {
    try {
      await wasmEngine.deleteLayer(id)
      await get().refreshLayers()
    } catch (error) {
      console.error('Failed to delete layer:', error)
    }
  },

  setActiveLayer: async (id: string) => {
    try {
      await wasmEngine.setActiveLayer(id)
      set({ activeLayerId: id })
    } catch (error) {
      console.error('Failed to set active layer:', error)
    }
  },

  setLayerVisibility: async (id: string, visible: boolean) => {
    try {
      await wasmEngine.setLayerVisibility(id, visible)
      await get().refreshLayers()
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to set layer visibility:', error)
    }
  },

  setLayerOpacity: async (id: string, opacity: number) => {
    try {
      await wasmEngine.setLayerOpacity(id, opacity)
      await get().refreshLayers()
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to set layer opacity:', error)
    }
  },

  moveLayerUp: async (id: string) => {
    try {
      await wasmEngine.moveLayerUp(id)
      await get().refreshLayers()
    } catch (error) {
      console.error('Failed to move layer up:', error)
    }
  },

  moveLayerDown: async (id: string) => {
    try {
      await wasmEngine.moveLayerDown(id)
      await get().refreshLayers()
    } catch (error) {
      console.error('Failed to move layer down:', error)
    }
  },

  duplicateLayer: async (id: string) => {
    try {
      await wasmEngine.duplicateLayer(id)
      await get().refreshLayers()
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to duplicate layer:', error)
    }
  },

  mergeLayerDown: async (id: string) => {
    try {
      await wasmEngine.mergeLayerDown(id)
      await get().refreshLayers()
      await get().renderCanvas()
    } catch (error) {
      console.error('Failed to merge layer down:', error)
    }
  },

  refreshLayers: async () => {
    try {
      const layers = await wasmEngine.getLayers() as Layer[]
      set({ layers })
    } catch (error) {
      console.error('Failed to refresh layers:', error)
    }
  },

  // Incremental stroke methods - real-time rendering
  beginStroke: async () => {
    try {
      await wasmEngine.beginStroke()
    } catch (error) {
      console.error('Failed to begin stroke:', error)
    }
  },

  addStrokePoint: async (point: StrokePoint) => {
    try {
      await wasmEngine.addStrokePoint(
        point.x,
        point.y,
        point.pressure,
        point.tilt_x,
        point.tilt_y,
        point.timestamp
      )
    } catch (error) {
      console.error('Failed to add stroke point:', error)
    }
  },

  endStroke: async () => {
    try {
      await wasmEngine.endStroke()
      await get().renderCanvas()

      // Update undo/redo state
      set({
        canUndo: wasmEngine.canUndo(),
        canRedo: wasmEngine.canRedo(),
      })
    } catch (error) {
      console.error('Failed to end stroke:', error)
    }
  },

  undo: async () => {
    try {
      await wasmEngine.undo()
      await get().renderCanvas()

      set({
        canUndo: wasmEngine.canUndo(),
        canRedo: wasmEngine.canRedo(),
      })
    } catch (error) {
      console.error('Failed to undo:', error)
    }
  },

  redo: async () => {
    try {
      await wasmEngine.redo()
      await get().renderCanvas()

      set({
        canUndo: wasmEngine.canUndo(),
        canRedo: wasmEngine.canRedo(),
      })
    } catch (error) {
      console.error('Failed to redo:', error)
    }
  },

  exportPng: async (filename: string = 'drawing.png') => {
    try {
      await wasmEngine.saveAsPng(filename)
    } catch (error) {
      console.error('Failed to export PNG:', error)
    }
  },

  exportJpeg: async (filename: string = 'drawing.jpg', quality: number = 90) => {
    try {
      await wasmEngine.saveAsJpeg(filename, quality)
    } catch (error) {
      console.error('Failed to export JPEG:', error)
    }
  },

  // Selection actions
  selectRect: async (x: number, y: number, width: number, height: number) => {
    try {
      const selection = await wasmEngine.selectRect(x, y, width, height)
      set({ selection: selection as SelectionInfo })
    } catch (error) {
      console.error('Failed to create rectangle selection:', error)
    }
  },

  selectLasso: async (points: Array<[number, number]>) => {
    try {
      const selection = await wasmEngine.selectLasso(points)
      set({ selection: selection as SelectionInfo })
    } catch (error) {
      console.error('Failed to create lasso selection:', error)
    }
  },

  clearSelection: async () => {
    try {
      await wasmEngine.clearSelection()
      set({ selection: null })
    } catch (error) {
      console.error('Failed to clear selection:', error)
    }
  },

  selectAll: async () => {
    try {
      const selection = await wasmEngine.selectAll()
      set({ selection: selection as SelectionInfo })
    } catch (error) {
      console.error('Failed to select all:', error)
    }
  },

  invertSelection: async () => {
    try {
      const selection = await wasmEngine.invertSelection()
      set({ selection: selection as SelectionInfo })
    } catch (error) {
      console.error('Failed to invert selection:', error)
    }
  },

  setSelectionMode: async (mode: SelectionMode) => {
    try {
      await wasmEngine.setSelectionMode(mode)
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
      const result = await wasmEngine.pickColor(clampedX, clampedY)
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
      await wasmEngine.floodFill(clampedX, clampedY, brushColor, 0.1)
      await get().renderCanvas()
      // Update undo/redo state
      set({
        canUndo: wasmEngine.canUndo(),
        canRedo: wasmEngine.canRedo(),
      })
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
}))
