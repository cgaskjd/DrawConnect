import { useMemo } from 'react'
import { useAppStore } from '../../stores/appStore'
import { t } from '../../i18n'
import { useAdjustmentDialog } from '../../hooks/useAdjustmentDialog'
import { BaseAdjustmentDialog, AdjustmentRow, SliderInput } from './BaseAdjustmentDialog'

interface PixelateDialogProps {
  isOpen: boolean
  onClose: () => void
}

export function PixelateDialog({ isOpen, onClose }: PixelateDialogProps) {
  const { filterPixelate } = useAppStore()

  const initialValues = useMemo(() => ({ cellSize: 10 }), [])

  const { values, handleCancel, handleReset, updateValue } = useAdjustmentDialog({
    isOpen,
    initialValues,
    isDefault: (v) => v.cellSize <= 1,
    applyEffect: async (v) => { await filterPixelate(v.cellSize) },
  })

  const onCancel = async () => {
    await handleCancel()
    onClose()
  }

  return (
    <BaseAdjustmentDialog
      isOpen={isOpen}
      title={t('menu.pixelate')}
      onCancel={onCancel}
      onOk={onClose}
      onReset={handleReset}
    >
      <AdjustmentRow label={t('adjustment.cellSize')}>
        <SliderInput
          value={values.cellSize}
          onChange={(v) => updateValue('cellSize', v)}
          min={2}
          max={100}
          step={1}
        />
      </AdjustmentRow>
    </BaseAdjustmentDialog>
  )
}
