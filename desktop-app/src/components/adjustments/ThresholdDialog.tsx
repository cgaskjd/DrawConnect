import { useMemo } from 'react'
import { useAppStore } from '../../stores/appStore'
import { t } from '../../i18n'
import { useAdjustmentDialog } from '../../hooks/useAdjustmentDialog'
import { BaseAdjustmentDialog, AdjustmentRow, SliderInput } from './BaseAdjustmentDialog'

interface ThresholdDialogProps {
  isOpen: boolean
  onClose: () => void
}

export function ThresholdDialog({ isOpen, onClose }: ThresholdDialogProps) {
  const { adjustThreshold } = useAppStore()

  const initialValues = useMemo(() => ({ level: 128 }), [])

  const { values, handleCancel, handleReset, updateValue } = useAdjustmentDialog({
    isOpen,
    initialValues,
    isDefault: () => false, // Threshold always applies
    applyEffect: async (v) => { await adjustThreshold(v.level) },
  })

  const onCancel = async () => {
    await handleCancel()
    onClose()
  }

  return (
    <BaseAdjustmentDialog
      isOpen={isOpen}
      title={t('menu.threshold')}
      onCancel={onCancel}
      onOk={onClose}
      onReset={handleReset}
    >
      <AdjustmentRow label={t('adjustment.threshold')}>
        <SliderInput
          value={values.level}
          onChange={(v) => updateValue('level', v)}
          min={0}
          max={255}
          step={1}
        />
      </AdjustmentRow>
    </BaseAdjustmentDialog>
  )
}
