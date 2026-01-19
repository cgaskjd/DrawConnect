import { useState, useEffect, useCallback } from 'react'
import { useAppStore } from '../stores/appStore'

interface UseAdjustmentDialogOptions<T> {
  isOpen: boolean
  initialValues: T
  isDefault: (values: T) => boolean
  applyEffect: (values: T) => Promise<void>
}

export function useAdjustmentDialog<T>({
  isOpen,
  initialValues,
  isDefault,
  applyEffect,
}: UseAdjustmentDialogOptions<T>) {
  const [values, setValues] = useState<T>(initialValues)
  const [hasPreview, setHasPreview] = useState(false)
  const { undo } = useAppStore()

  // Reset when dialog opens
  useEffect(() => {
    if (isOpen) {
      setValues(initialValues)
      setHasPreview(false)
    }
  }, [isOpen, initialValues])

  const applyPreview = useCallback(async (newValues: T) => {
    if (hasPreview) {
      await undo()
    }
    if (!isDefault(newValues)) {
      await applyEffect(newValues)
      setHasPreview(true)
    } else {
      setHasPreview(false)
    }
  }, [hasPreview, undo, applyEffect, isDefault])

  const handleCancel = useCallback(async () => {
    if (hasPreview) {
      await undo()
    }
  }, [hasPreview, undo])

  const handleReset = useCallback(async () => {
    if (hasPreview) {
      await undo()
    }
    setValues(initialValues)
    setHasPreview(false)
  }, [hasPreview, undo, initialValues])

  const updateValue = useCallback(<K extends keyof T>(key: K, value: T[K]) => {
    const newValues = { ...values, [key]: value }
    setValues(newValues)
    applyPreview(newValues)
  }, [values, applyPreview])

  return {
    values,
    setValues,
    hasPreview,
    applyPreview,
    handleCancel,
    handleReset,
    updateValue,
  }
}
