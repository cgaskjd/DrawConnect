import { useMemo } from 'react'
import { useAppStore } from '../../stores/appStore'
import { t } from '../../i18n'
import { useAdjustmentDialog } from '../../hooks/useAdjustmentDialog'
import { BaseAdjustmentDialog, AdjustmentRow, SliderInput } from './BaseAdjustmentDialog'

interface LevelsDialogProps {
  isOpen: boolean
  onClose: () => void
}

export function LevelsDialog({ isOpen, onClose }: LevelsDialogProps) {
  const { adjustLevels } = useAppStore()

  const initialValues = useMemo(() => ({
    inputBlack: 0,
    inputWhite: 1,
    gamma: 1,
    outputBlack: 0,
    outputWhite: 1,
  }), [])

  const { values, handleCancel, handleReset, updateValue } = useAdjustmentDialog({
    isOpen,
    initialValues,
    isDefault: (v) => v.inputBlack === 0 && v.inputWhite === 1 && v.gamma === 1 && v.outputBlack === 0 && v.outputWhite === 1,
    applyEffect: async (v) => { await adjustLevels(v.inputBlack, v.inputWhite, v.gamma, v.outputBlack, v.outputWhite) },
  })

  const onCancel = async () => {
    await handleCancel()
    onClose()
  }

  return (
    <BaseAdjustmentDialog
      isOpen={isOpen}
      title={t('menu.levels')}
      onCancel={onCancel}
      onOk={onClose}
      onReset={handleReset}
    >
      <div className="section-label">{t('adjustment.inputLevels')}</div>
      <AdjustmentRow label={t('adjustment.black')}>
        <SliderInput
          value={values.inputBlack}
          onChange={(v) => updateValue('inputBlack', v)}
          min={0}
          max={1}
          step={0.01}
          displayMultiplier={255}
        />
      </AdjustmentRow>
      <AdjustmentRow label={t('adjustment.gamma')}>
        <SliderInput
          value={values.gamma}
          onChange={(v) => updateValue('gamma', v)}
          min={0.1}
          max={10}
          step={0.01}
        />
      </AdjustmentRow>
      <AdjustmentRow label={t('adjustment.white')}>
        <SliderInput
          value={values.inputWhite}
          onChange={(v) => updateValue('inputWhite', v)}
          min={0}
          max={1}
          step={0.01}
          displayMultiplier={255}
        />
      </AdjustmentRow>

      <div className="section-label">{t('adjustment.outputLevels')}</div>
      <AdjustmentRow label={t('adjustment.black')}>
        <SliderInput
          value={values.outputBlack}
          onChange={(v) => updateValue('outputBlack', v)}
          min={0}
          max={1}
          step={0.01}
          displayMultiplier={255}
        />
      </AdjustmentRow>
      <AdjustmentRow label={t('adjustment.white')}>
        <SliderInput
          value={values.outputWhite}
          onChange={(v) => updateValue('outputWhite', v)}
          min={0}
          max={1}
          step={0.01}
          displayMultiplier={255}
        />
      </AdjustmentRow>
    </BaseAdjustmentDialog>
  )
}
