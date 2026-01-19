import { useAppStore, Tool, SelectionMode } from '../stores/appStore'
import { t } from '../i18n'
import {
  Brush,
  Eraser,
  Move,
  Square,
  Lasso,
  Wand2,
  Pipette,
  PaintBucket,
  Replace,
  Plus,
  Minus,
  Layers,
} from 'lucide-react'
import './Toolbar.css'

// 绘画工具组
const drawingTools: { id: Tool; icon: typeof Brush; labelKey: string }[] = [
  { id: 'brush', icon: Brush, labelKey: 'tools.brush' },
  { id: 'eraser', icon: Eraser, labelKey: 'tools.eraser' },
]

// 选择工具组
const selectionTools: { id: Tool; icon: typeof Brush; labelKey: string }[] = [
  { id: 'select_rect', icon: Square, labelKey: 'tools.selectRect' },
  { id: 'select_lasso', icon: Lasso, labelKey: 'tools.selectLasso' },
  { id: 'select_magic', icon: Wand2, labelKey: 'tools.selectMagic' },
]

// 辅助工具组
const utilityTools: { id: Tool; icon: typeof Brush; labelKey: string }[] = [
  { id: 'move', icon: Move, labelKey: 'tools.move' },
  { id: 'eyedropper', icon: Pipette, labelKey: 'tools.eyedropper' },
  { id: 'fill', icon: PaintBucket, labelKey: 'tools.fill' },
]

// 选区模式按钮
const selectionModeButtons: { mode: SelectionMode; icon: typeof Replace; labelKey: string }[] = [
  { mode: 'replace', icon: Replace, labelKey: 'selection.modeReplace' },
  { mode: 'add', icon: Plus, labelKey: 'selection.modeAdd' },
  { mode: 'subtract', icon: Minus, labelKey: 'selection.modeSubtract' },
  { mode: 'intersect', icon: Layers, labelKey: 'selection.modeIntersect' },
]

export function Toolbar() {
  const { currentTool, setCurrentTool, selectionMode, setSelectionMode } = useAppStore()

  const isSelectionTool = currentTool === 'select_rect' || currentTool === 'select_lasso' || currentTool === 'select_magic'

  const renderToolGroup = (tools: typeof drawingTools) => (
    <>
      {tools.map((tool) => {
        const Icon = tool.icon
        return (
          <button
            key={tool.id}
            className={`toolbar-btn ${currentTool === tool.id ? 'active' : ''}`}
            onClick={() => setCurrentTool(tool.id)}
            title={t(tool.labelKey)}
          >
            <Icon size={20} strokeWidth={1.5} />
          </button>
        )
      })}
    </>
  )

  return (
    <div className="toolbar">
      {renderToolGroup(drawingTools)}
      <div className="toolbar-divider" />
      {renderToolGroup(selectionTools)}
      {isSelectionTool && (
        <>
          <div className="toolbar-divider small" />
          <div className="selection-mode-group">
            {selectionModeButtons.map(({ mode, icon: Icon, labelKey }) => (
              <button
                key={mode}
                className={`toolbar-btn small ${selectionMode === mode ? 'active' : ''}`}
                onClick={() => setSelectionMode(mode)}
                title={t(labelKey)}
              >
                <Icon size={14} strokeWidth={1.5} />
              </button>
            ))}
          </div>
        </>
      )}
      <div className="toolbar-divider" />
      {renderToolGroup(utilityTools)}
    </div>
  )
}
