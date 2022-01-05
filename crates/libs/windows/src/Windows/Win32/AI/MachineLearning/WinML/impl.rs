pub trait IMLOperatorAttributesImpl: Sized {
    fn GetAttributeElementCount();
    fn GetAttribute();
    fn GetStringAttributeElementLength();
    fn GetStringAttributeElement();
}
pub trait IMLOperatorKernelImpl: Sized {
    fn Compute();
}
pub trait IMLOperatorKernelContextImpl: Sized {
    fn GetInputTensor();
    fn GetOutputTensor();
    fn GetOutputTensor();
    fn AllocateTemporaryData();
    fn GetExecutionInterface();
}
pub trait IMLOperatorKernelCreationContextImpl: Sized + IMLOperatorAttributesImpl {
    fn GetInputCount();
    fn GetOutputCount();
    fn IsInputValid();
    fn IsOutputValid();
    fn GetInputEdgeDescription();
    fn GetOutputEdgeDescription();
    fn HasTensorShapeDescription();
    fn GetTensorShapeDescription();
    fn GetExecutionInterface();
}
pub trait IMLOperatorKernelFactoryImpl: Sized {
    fn CreateKernel();
}
pub trait IMLOperatorRegistryImpl: Sized {
    fn RegisterOperatorSetSchema();
    fn RegisterOperatorKernel();
}
pub trait IMLOperatorShapeInferenceContextImpl: Sized + IMLOperatorAttributesImpl {
    fn GetInputCount();
    fn GetOutputCount();
    fn IsInputValid();
    fn IsOutputValid();
    fn GetInputEdgeDescription();
    fn GetInputTensorDimensionCount();
    fn GetInputTensorShape();
    fn SetOutputTensorShape();
}
pub trait IMLOperatorShapeInferrerImpl: Sized {
    fn InferOutputShapes();
}
pub trait IMLOperatorTensorImpl: Sized {
    fn GetDimensionCount();
    fn GetShape();
    fn GetTensorDataType();
    fn IsCpuData();
    fn IsDataInterface();
    fn GetData();
    fn GetDataInterface();
}
pub trait IMLOperatorTensorShapeDescriptionImpl: Sized {
    fn GetInputTensorDimensionCount();
    fn GetInputTensorShape();
    fn HasOutputShapeDescription();
    fn GetOutputTensorDimensionCount();
    fn GetOutputTensorShape();
}
pub trait IMLOperatorTypeInferenceContextImpl: Sized + IMLOperatorAttributesImpl {
    fn GetInputCount();
    fn GetOutputCount();
    fn IsInputValid();
    fn IsOutputValid();
    fn GetInputEdgeDescription();
    fn SetOutputEdgeDescription();
}
pub trait IMLOperatorTypeInferrerImpl: Sized {
    fn InferOutputTypes();
}
pub trait IWinMLEvaluationContextImpl: Sized {
    fn BindValue();
    fn GetValueByName();
    fn Clear();
}
pub trait IWinMLModelImpl: Sized {
    fn GetDescription();
    fn EnumerateMetadata();
    fn EnumerateModelInputs();
    fn EnumerateModelOutputs();
}
pub trait IWinMLRuntimeImpl: Sized {
    fn LoadModel();
    fn CreateEvaluationContext();
    fn EvaluateModel();
}
pub trait IWinMLRuntimeFactoryImpl: Sized {
    fn CreateRuntime();
}
