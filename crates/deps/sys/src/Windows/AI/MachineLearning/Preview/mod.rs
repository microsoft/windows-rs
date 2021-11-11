#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn FeatureElementKindPreview();
    fn IImageVariableDescriptorPreview();
    fn IInferencingOptionsPreview();
    fn ILearningModelBindingPreview();
    fn ILearningModelBindingPreviewFactory();
    fn ILearningModelDescriptionPreview();
    fn ILearningModelEvaluationResultPreview();
    fn ILearningModelPreview();
    fn ILearningModelPreviewStatics();
    fn ILearningModelVariableDescriptorPreview();
    fn IMapVariableDescriptorPreview();
    fn ISequenceVariableDescriptorPreview();
    fn ITensorVariableDescriptorPreview();
    fn ImageVariableDescriptorPreview();
    fn InferencingOptionsPreview();
    fn LearningModelBindingPreview();
    fn LearningModelDescriptionPreview();
    fn LearningModelDeviceKindPreview();
    fn LearningModelEvaluationResultPreview();
    fn LearningModelFeatureKindPreview();
    fn LearningModelPreview();
    fn LearningModelVariableDescriptorPreview();
    fn MachineLearningPreviewContract();
    fn MapVariableDescriptorPreview();
    fn SequenceVariableDescriptorPreview();
    fn TensorVariableDescriptorPreview();
}
