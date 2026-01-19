// 国际化入口文件
import zhCN from './zh-CN'

// 当前语言
export const currentLang = zhCN

// 翻译函数 - 支持模板参数
export function t(key: string, params?: Record<string, string | number>): string {
  const keys = key.split('.')
  let value: any = currentLang

  for (const k of keys) {
    if (value && typeof value === 'object' && k in value) {
      value = value[k]
    } else {
      return key // 返回原始 key 作为 fallback
    }
  }

  if (typeof value !== 'string') {
    return key
  }

  // 替换模板参数 {name} -> actual value
  if (params) {
    return value.replace(/\{(\w+)\}/g, (_, paramKey) => {
      return String(params[paramKey] ?? `{${paramKey}}`)
    })
  }

  return value
}

// 获取笔刷预设的中文名称
export function getBrushName(name: string): string {
  const presetNames = currentLang.brush.presetNames as Record<string, string>
  return presetNames[name] || name
}

// 获取混合模式的中文名称
export function getBlendModeName(mode: string): string {
  const blendModes = currentLang.blendModes as Record<string, string>
  return blendModes[mode] || mode
}

export { zhCN }
