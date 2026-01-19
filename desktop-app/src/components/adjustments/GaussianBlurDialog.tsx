import { useMemo } from 'react'
import { useAppStore } from '../../stores/appStore'
import { t } from '../../i18n'
import { useAdjustmentDialog } from '../../hooks/useAdjustmentDialog'
import { BaseAdjustmentDialog, AdjustmentRow, SliderInput } from './BaseAdjustmentDialog'

interface GaussianBlurDialogProps {
  isOpen: boolean
  onClose: () => void
}

interface GaussianBlurValues {
  radius: number
}

export function GaussianBlurDialog({ isOpen, onClose }: GaussianBlurDialogProps) {
  const { filterGaussianBlur } = useAppStore()

  const initialValues = useMemo(() => ({ radius: 5 }), [])

  const { values, handleCancel, handleReset, updateValue } = useAdjustmentDialog<GaussianBlurValues>({
    isOpen,
    initialValues,
    isDefault: (v) => v.radius <= 0,
    applyEffect: async (v) => { await filterGaussianBlur(v.radius) },
  })

  const onCancel = async () => {
    await handleCancel()
    onClose()
  }

  const onReset = async () => {
    await handleReset()
  }

  return (
    <BaseAdjustmentDialog
      isOpen={isOpen}
      title={t('menu.gaussianBlur')}
      onCancel={onCancel}
      onOk={onClose}
      onReset={onReset}
    >
      <AdjustmentRow label={t('adjustment.radius')}>
        <SliderInput
          value={values.radius}
          onChange={(v) => updateValue('radius', v)}
          min={0.1}
          max={50}
          step={0.1}
        />
      </AdjustmentRow>
    </BaseAdjustmentDialog>
  )
}
