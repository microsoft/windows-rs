pub trait IMLOperatorAttributesImpl: Sized {
    fn GetAttributeElementCount();
    fn GetAttribute();
    fn GetStringAttributeElementLength();
    fn GetStringAttributeElement();
}
impl ::windows::core::RuntimeName for IMLOperatorAttributes {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.WinML.IMLOperatorAttributes";
}
impl IMLOperatorAttributesVtbl {
    pub const fn new<Impl: IMLOperatorAttributesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMLOperatorAttributesVtbl {
        unsafe extern "system" fn GetAttributeElementCount<Impl: IMLOperatorAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributeElementCount(&*(&name as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), r#type, ::core::mem::transmute_copy(&elementcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttribute<Impl: IMLOperatorAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttribute(&*(&name as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), r#type, elementcount, elementbytesize, ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringAttributeElementLength<Impl: IMLOperatorAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStringAttributeElementLength(&*(&name as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), elementindex, ::core::mem::transmute_copy(&attributeelementbytesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringAttributeElement<Impl: IMLOperatorAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStringAttributeElement(&*(&name as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), elementindex, attributeelementbytesize, ::core::mem::transmute_copy(&attributeelement)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMLOperatorAttributes>, base.5, GetAttributeElementCount::<Impl, OFFSET>, GetAttribute::<Impl, OFFSET>, GetStringAttributeElementLength::<Impl, OFFSET>, GetStringAttributeElement::<Impl, OFFSET>)
    }
}
pub trait IMLOperatorKernelImpl: Sized {
    fn Compute();
}
impl ::windows::core::RuntimeName for IMLOperatorKernel {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.WinML.IMLOperatorKernel";
}
impl IMLOperatorKernelVtbl {
    pub const fn new<Impl: IMLOperatorKernelImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMLOperatorKernelVtbl {
        unsafe extern "system" fn Compute<Impl: IMLOperatorKernelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Compute(&*(&context as *const <IMLOperatorKernelContext as ::windows::core::Abi>::Abi as *const <IMLOperatorKernelContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMLOperatorKernel>, base.5, Compute::<Impl, OFFSET>)
    }
}
pub trait IMLOperatorKernelContextImpl: Sized {
    fn GetInputTensor();
    fn GetOutputTensor();
    fn GetOutputTensor();
    fn AllocateTemporaryData();
    fn GetExecutionInterface();
}
impl ::windows::core::RuntimeName for IMLOperatorKernelContext {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.WinML.IMLOperatorKernelContext";
}
impl IMLOperatorKernelContextVtbl {
    pub const fn new<Impl: IMLOperatorKernelContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMLOperatorKernelContextVtbl {
        unsafe extern "system" fn GetInputTensor<Impl: IMLOperatorKernelContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, tensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputTensor(inputindex, ::core::mem::transmute_copy(&tensor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputTensor<Impl: IMLOperatorKernelContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32, tensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputTensor(outputindex, dimensioncount, dimensionsizes, ::core::mem::transmute_copy(&tensor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputTensor<Impl: IMLOperatorKernelContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32, tensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputTensor(outputindex, ::core::mem::transmute_copy(&tensor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateTemporaryData<Impl: IMLOperatorKernelContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: usize, data: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllocateTemporaryData(size, ::core::mem::transmute_copy(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExecutionInterface<Impl: IMLOperatorKernelContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, executionobject: *mut *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetExecutionInterface(::core::mem::transmute_copy(&executionobject)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMLOperatorKernelContext>, base.5, GetInputTensor::<Impl, OFFSET>, GetOutputTensor::<Impl, OFFSET>, GetOutputTensor::<Impl, OFFSET>, AllocateTemporaryData::<Impl, OFFSET>, GetExecutionInterface::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IMLOperatorKernelCreationContext {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.WinML.IMLOperatorKernelCreationContext";
}
impl IMLOperatorKernelCreationContextVtbl {
    pub const fn new<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMLOperatorKernelCreationContextVtbl {
        unsafe extern "system" fn GetInputCount<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputCount<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInputValid<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInputValid(inputindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOutputValid<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOutputValid(outputindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputEdgeDescription<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputEdgeDescription(inputindex, ::core::mem::transmute_copy(&edgedescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputEdgeDescription<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputEdgeDescription(outputindex, ::core::mem::transmute_copy(&edgedescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasTensorShapeDescription<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasTensorShapeDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTensorShapeDescription<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shapedescription: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTensorShapeDescription(::core::mem::transmute_copy(&shapedescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExecutionInterface<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, executionobject: *mut *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetExecutionInterface(::core::mem::transmute_copy(&executionobject)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMLOperatorKernelCreationContext>, base.5, GetInputCount::<Impl, OFFSET>, GetOutputCount::<Impl, OFFSET>, IsInputValid::<Impl, OFFSET>, IsOutputValid::<Impl, OFFSET>, GetInputEdgeDescription::<Impl, OFFSET>, GetOutputEdgeDescription::<Impl, OFFSET>, HasTensorShapeDescription::<Impl, OFFSET>, GetTensorShapeDescription::<Impl, OFFSET>, GetExecutionInterface::<Impl, OFFSET>)
    }
}
pub trait IMLOperatorKernelFactoryImpl: Sized {
    fn CreateKernel();
}
impl ::windows::core::RuntimeName for IMLOperatorKernelFactory {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.WinML.IMLOperatorKernelFactory";
}
impl IMLOperatorKernelFactoryVtbl {
    pub const fn new<Impl: IMLOperatorKernelFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMLOperatorKernelFactoryVtbl {
        unsafe extern "system" fn CreateKernel<Impl: IMLOperatorKernelFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, kernel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateKernel(&*(&context as *const <IMLOperatorKernelCreationContext as ::windows::core::Abi>::Abi as *const <IMLOperatorKernelCreationContext as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&kernel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMLOperatorKernelFactory>, base.5, CreateKernel::<Impl, OFFSET>)
    }
}
pub trait IMLOperatorRegistryImpl: Sized {
    fn RegisterOperatorSetSchema();
    fn RegisterOperatorKernel();
}
impl ::windows::core::RuntimeName for IMLOperatorRegistry {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.WinML.IMLOperatorRegistry";
}
impl IMLOperatorRegistryVtbl {
    pub const fn new<Impl: IMLOperatorRegistryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMLOperatorRegistryVtbl {
        unsafe extern "system" fn RegisterOperatorSetSchema<Impl: IMLOperatorRegistryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: ::windows::core::RawPtr, shapeinferrer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterOperatorSetSchema(
                &*(&operatorsetid as *const <MLOperatorSetId as ::windows::core::Abi>::Abi as *const <MLOperatorSetId as ::windows::core::DefaultType>::DefaultType),
                baselineversion,
                &*(&schema as *const <MLOperatorSchemaDescription as ::windows::core::Abi>::Abi as *const <MLOperatorSchemaDescription as ::windows::core::DefaultType>::DefaultType),
                schemacount,
                &*(&typeinferrer as *const <IMLOperatorTypeInferrer as ::windows::core::Abi>::Abi as *const <IMLOperatorTypeInferrer as ::windows::core::DefaultType>::DefaultType),
                &*(&shapeinferrer as *const <IMLOperatorShapeInferrer as ::windows::core::Abi>::Abi as *const <IMLOperatorShapeInferrer as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterOperatorKernel<Impl: IMLOperatorRegistryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: ::windows::core::RawPtr, shapeinferrer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterOperatorKernel(
                &*(&operatorkernel as *const <MLOperatorKernelDescription as ::windows::core::Abi>::Abi as *const <MLOperatorKernelDescription as ::windows::core::DefaultType>::DefaultType),
                &*(&operatorkernelfactory as *const <IMLOperatorKernelFactory as ::windows::core::Abi>::Abi as *const <IMLOperatorKernelFactory as ::windows::core::DefaultType>::DefaultType),
                &*(&shapeinferrer as *const <IMLOperatorShapeInferrer as ::windows::core::Abi>::Abi as *const <IMLOperatorShapeInferrer as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMLOperatorRegistry>, base.5, RegisterOperatorSetSchema::<Impl, OFFSET>, RegisterOperatorKernel::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IMLOperatorShapeInferenceContext {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.WinML.IMLOperatorShapeInferenceContext";
}
impl IMLOperatorShapeInferenceContextVtbl {
    pub const fn new<Impl: IMLOperatorShapeInferenceContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMLOperatorShapeInferenceContextVtbl {
        unsafe extern "system" fn GetInputCount<Impl: IMLOperatorShapeInferenceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputCount<Impl: IMLOperatorShapeInferenceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInputValid<Impl: IMLOperatorShapeInferenceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInputValid(inputindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOutputValid<Impl: IMLOperatorShapeInferenceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOutputValid(outputindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputEdgeDescription<Impl: IMLOperatorShapeInferenceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputEdgeDescription(inputindex, ::core::mem::transmute_copy(&edgedescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTensorDimensionCount<Impl: IMLOperatorShapeInferenceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputTensorDimensionCount(inputindex, ::core::mem::transmute_copy(&dimensioncount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTensorShape<Impl: IMLOperatorShapeInferenceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputTensorShape(inputindex, dimensioncount, ::core::mem::transmute_copy(&dimensions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputTensorShape<Impl: IMLOperatorShapeInferenceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOutputTensorShape(outputindex, dimensioncount, dimensions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMLOperatorShapeInferenceContext>, base.5, GetInputCount::<Impl, OFFSET>, GetOutputCount::<Impl, OFFSET>, IsInputValid::<Impl, OFFSET>, IsOutputValid::<Impl, OFFSET>, GetInputEdgeDescription::<Impl, OFFSET>, GetInputTensorDimensionCount::<Impl, OFFSET>, GetInputTensorShape::<Impl, OFFSET>, SetOutputTensorShape::<Impl, OFFSET>)
    }
}
pub trait IMLOperatorShapeInferrerImpl: Sized {
    fn InferOutputShapes();
}
impl ::windows::core::RuntimeName for IMLOperatorShapeInferrer {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.WinML.IMLOperatorShapeInferrer";
}
impl IMLOperatorShapeInferrerVtbl {
    pub const fn new<Impl: IMLOperatorShapeInferrerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMLOperatorShapeInferrerVtbl {
        unsafe extern "system" fn InferOutputShapes<Impl: IMLOperatorShapeInferrerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InferOutputShapes(&*(&context as *const <IMLOperatorShapeInferenceContext as ::windows::core::Abi>::Abi as *const <IMLOperatorShapeInferenceContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMLOperatorShapeInferrer>, base.5, InferOutputShapes::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IMLOperatorTensor {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.WinML.IMLOperatorTensor";
}
impl IMLOperatorTensorVtbl {
    pub const fn new<Impl: IMLOperatorTensorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMLOperatorTensorVtbl {
        unsafe extern "system" fn GetDimensionCount<Impl: IMLOperatorTensorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDimensionCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetShape<Impl: IMLOperatorTensorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetShape(dimensioncount, ::core::mem::transmute_copy(&dimensions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTensorDataType<Impl: IMLOperatorTensorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> MLOperatorTensorDataType {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTensorDataType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCpuData<Impl: IMLOperatorTensorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCpuData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDataInterface<Impl: IMLOperatorTensorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDataInterface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetData<Impl: IMLOperatorTensorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataInterface<Impl: IMLOperatorTensorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datainterface: *mut *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetDataInterface(::core::mem::transmute_copy(&datainterface)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMLOperatorTensor>, base.5, GetDimensionCount::<Impl, OFFSET>, GetShape::<Impl, OFFSET>, GetTensorDataType::<Impl, OFFSET>, IsCpuData::<Impl, OFFSET>, IsDataInterface::<Impl, OFFSET>, GetData::<Impl, OFFSET>, GetDataInterface::<Impl, OFFSET>)
    }
}
pub trait IMLOperatorTensorShapeDescriptionImpl: Sized {
    fn GetInputTensorDimensionCount();
    fn GetInputTensorShape();
    fn HasOutputShapeDescription();
    fn GetOutputTensorDimensionCount();
    fn GetOutputTensorShape();
}
impl ::windows::core::RuntimeName for IMLOperatorTensorShapeDescription {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.WinML.IMLOperatorTensorShapeDescription";
}
impl IMLOperatorTensorShapeDescriptionVtbl {
    pub const fn new<Impl: IMLOperatorTensorShapeDescriptionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMLOperatorTensorShapeDescriptionVtbl {
        unsafe extern "system" fn GetInputTensorDimensionCount<Impl: IMLOperatorTensorShapeDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputTensorDimensionCount(inputindex, ::core::mem::transmute_copy(&dimensioncount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTensorShape<Impl: IMLOperatorTensorShapeDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputTensorShape(inputindex, dimensioncount, ::core::mem::transmute_copy(&dimensions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasOutputShapeDescription<Impl: IMLOperatorTensorShapeDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasOutputShapeDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputTensorDimensionCount<Impl: IMLOperatorTensorShapeDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputTensorDimensionCount(outputindex, ::core::mem::transmute_copy(&dimensioncount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputTensorShape<Impl: IMLOperatorTensorShapeDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputTensorShape(outputindex, dimensioncount, ::core::mem::transmute_copy(&dimensions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMLOperatorTensorShapeDescription>, base.5, GetInputTensorDimensionCount::<Impl, OFFSET>, GetInputTensorShape::<Impl, OFFSET>, HasOutputShapeDescription::<Impl, OFFSET>, GetOutputTensorDimensionCount::<Impl, OFFSET>, GetOutputTensorShape::<Impl, OFFSET>)
    }
}
pub trait IMLOperatorTypeInferenceContextImpl: Sized + IMLOperatorAttributesImpl {
    fn GetInputCount();
    fn GetOutputCount();
    fn IsInputValid();
    fn IsOutputValid();
    fn GetInputEdgeDescription();
    fn SetOutputEdgeDescription();
}
impl ::windows::core::RuntimeName for IMLOperatorTypeInferenceContext {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.WinML.IMLOperatorTypeInferenceContext";
}
impl IMLOperatorTypeInferenceContextVtbl {
    pub const fn new<Impl: IMLOperatorTypeInferenceContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMLOperatorTypeInferenceContextVtbl {
        unsafe extern "system" fn GetInputCount<Impl: IMLOperatorTypeInferenceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputCount<Impl: IMLOperatorTypeInferenceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInputValid<Impl: IMLOperatorTypeInferenceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInputValid(inputindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOutputValid<Impl: IMLOperatorTypeInferenceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOutputValid(outputindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputEdgeDescription<Impl: IMLOperatorTypeInferenceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputEdgeDescription(inputindex, ::core::mem::transmute_copy(&edgedescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputEdgeDescription<Impl: IMLOperatorTypeInferenceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOutputEdgeDescription(outputindex, &*(&edgedescription as *const <MLOperatorEdgeDescription as ::windows::core::Abi>::Abi as *const <MLOperatorEdgeDescription as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMLOperatorTypeInferenceContext>, base.5, GetInputCount::<Impl, OFFSET>, GetOutputCount::<Impl, OFFSET>, IsInputValid::<Impl, OFFSET>, IsOutputValid::<Impl, OFFSET>, GetInputEdgeDescription::<Impl, OFFSET>, SetOutputEdgeDescription::<Impl, OFFSET>)
    }
}
pub trait IMLOperatorTypeInferrerImpl: Sized {
    fn InferOutputTypes();
}
impl ::windows::core::RuntimeName for IMLOperatorTypeInferrer {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.WinML.IMLOperatorTypeInferrer";
}
impl IMLOperatorTypeInferrerVtbl {
    pub const fn new<Impl: IMLOperatorTypeInferrerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMLOperatorTypeInferrerVtbl {
        unsafe extern "system" fn InferOutputTypes<Impl: IMLOperatorTypeInferrerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InferOutputTypes(&*(&context as *const <IMLOperatorTypeInferenceContext as ::windows::core::Abi>::Abi as *const <IMLOperatorTypeInferenceContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMLOperatorTypeInferrer>, base.5, InferOutputTypes::<Impl, OFFSET>)
    }
}
pub trait IWinMLEvaluationContextImpl: Sized {
    fn BindValue();
    fn GetValueByName();
    fn Clear();
}
impl ::windows::core::RuntimeName for IWinMLEvaluationContext {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.WinML.IWinMLEvaluationContext";
}
impl IWinMLEvaluationContextVtbl {
    pub const fn new<Impl: IWinMLEvaluationContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWinMLEvaluationContextVtbl {
        unsafe extern "system" fn BindValue<Impl: IWinMLEvaluationContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdescriptor: *const WINML_BINDING_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BindValue(&*(&pdescriptor as *const <WINML_BINDING_DESC as ::windows::core::Abi>::Abi as *const <WINML_BINDING_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueByName<Impl: IWinMLEvaluationContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR, pdescriptor: *mut *mut WINML_BINDING_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValueByName(&*(&name as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdescriptor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IWinMLEvaluationContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWinMLEvaluationContext>, base.5, BindValue::<Impl, OFFSET>, GetValueByName::<Impl, OFFSET>, Clear::<Impl, OFFSET>)
    }
}
pub trait IWinMLModelImpl: Sized {
    fn GetDescription();
    fn EnumerateMetadata();
    fn EnumerateModelInputs();
    fn EnumerateModelOutputs();
}
impl ::windows::core::RuntimeName for IWinMLModel {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.WinML.IWinMLModel";
}
impl IWinMLModelVtbl {
    pub const fn new<Impl: IWinMLModelImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWinMLModelVtbl {
        unsafe extern "system" fn GetDescription<Impl: IWinMLModelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdescription: *mut *mut WINML_MODEL_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDescription(::core::mem::transmute_copy(&ppdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateMetadata<Impl: IWinMLModelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pkey: *mut super::super::super::Foundation::PWSTR, pvalue: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateMetadata(index, ::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateModelInputs<Impl: IWinMLModelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, ppinputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateModelInputs(index, ::core::mem::transmute_copy(&ppinputdescriptor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateModelOutputs<Impl: IWinMLModelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, ppoutputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateModelOutputs(index, ::core::mem::transmute_copy(&ppoutputdescriptor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWinMLModel>, base.5, GetDescription::<Impl, OFFSET>, EnumerateMetadata::<Impl, OFFSET>, EnumerateModelInputs::<Impl, OFFSET>, EnumerateModelOutputs::<Impl, OFFSET>)
    }
}
pub trait IWinMLRuntimeImpl: Sized {
    fn LoadModel();
    fn CreateEvaluationContext();
    fn EvaluateModel();
}
impl ::windows::core::RuntimeName for IWinMLRuntime {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.WinML.IWinMLRuntime";
}
impl IWinMLRuntimeVtbl {
    pub const fn new<Impl: IWinMLRuntimeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWinMLRuntimeVtbl {
        unsafe extern "system" fn LoadModel<Impl: IWinMLRuntimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: super::super::super::Foundation::PWSTR, ppmodel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadModel(&*(&path as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppmodel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEvaluationContext<Impl: IWinMLRuntimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateEvaluationContext(&*(&device as *const <super::super::super::Graphics::Direct3D12::ID3D12Device as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Device as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EvaluateModel<Impl: IWinMLRuntimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EvaluateModel(&*(&pcontext as *const <IWinMLEvaluationContext as ::windows::core::Abi>::Abi as *const <IWinMLEvaluationContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWinMLRuntime>, base.5, LoadModel::<Impl, OFFSET>, CreateEvaluationContext::<Impl, OFFSET>, EvaluateModel::<Impl, OFFSET>)
    }
}
pub trait IWinMLRuntimeFactoryImpl: Sized {
    fn CreateRuntime();
}
impl ::windows::core::RuntimeName for IWinMLRuntimeFactory {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.WinML.IWinMLRuntimeFactory";
}
impl IWinMLRuntimeFactoryVtbl {
    pub const fn new<Impl: IWinMLRuntimeFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWinMLRuntimeFactoryVtbl {
        unsafe extern "system" fn CreateRuntime<Impl: IWinMLRuntimeFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, runtimetype: WINML_RUNTIME_TYPE, ppruntime: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRuntime(runtimetype, ::core::mem::transmute_copy(&ppruntime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWinMLRuntimeFactory>, base.5, CreateRuntime::<Impl, OFFSET>)
    }
}
