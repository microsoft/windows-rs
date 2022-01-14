#[cfg(feature = "Win32_Foundation")]
pub trait IMLOperatorAttributesImpl: Sized {
    fn GetAttributeElementCount(&mut self, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType) -> ::windows::core::Result<u32>;
    fn GetAttribute(&mut self, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetStringAttributeElementLength(&mut self, name: super::super::super::Foundation::PSTR, elementindex: u32) -> ::windows::core::Result<u32>;
    fn GetStringAttributeElement(&mut self, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMLOperatorAttributesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorAttributesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLOperatorAttributesVtbl {
        unsafe extern "system" fn GetAttributeElementCount<Impl: IMLOperatorAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributeElementCount(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *elementcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttribute<Impl: IMLOperatorAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttribute(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&elementcount), ::core::mem::transmute_copy(&elementbytesize), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetStringAttributeElementLength<Impl: IMLOperatorAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringAttributeElementLength(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&elementindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *attributeelementbytesize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringAttributeElement<Impl: IMLOperatorAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStringAttributeElement(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&elementindex), ::core::mem::transmute_copy(&attributeelementbytesize), ::core::mem::transmute_copy(&attributeelement)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAttributeElementCount: GetAttributeElementCount::<Impl, IMPL_OFFSET>,
            GetAttribute: GetAttribute::<Impl, IMPL_OFFSET>,
            GetStringAttributeElementLength: GetStringAttributeElementLength::<Impl, IMPL_OFFSET>,
            GetStringAttributeElement: GetStringAttributeElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorAttributes as ::windows::core::Interface>::IID
    }
}
pub trait IMLOperatorKernelImpl: Sized {
    fn Compute(&mut self, context: ::core::option::Option<IMLOperatorKernelContext>) -> ::windows::core::Result<()>;
}
impl IMLOperatorKernelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLOperatorKernelVtbl {
        unsafe extern "system" fn Compute<Impl: IMLOperatorKernelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Compute(::core::mem::transmute(&context)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Compute: Compute::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorKernel as ::windows::core::Interface>::IID
    }
}
pub trait IMLOperatorKernelContextImpl: Sized {
    fn GetInputTensor(&mut self, inputindex: u32) -> ::windows::core::Result<IMLOperatorTensor>;
    fn GetOutputTensor(&mut self, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32) -> ::windows::core::Result<IMLOperatorTensor>;
    fn GetOutputTensor2(&mut self, outputindex: u32) -> ::windows::core::Result<IMLOperatorTensor>;
    fn AllocateTemporaryData(&mut self, size: usize) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetExecutionInterface(&mut self, executionobject: *mut ::core::option::Option<::windows::core::IUnknown>);
}
impl IMLOperatorKernelContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLOperatorKernelContextVtbl {
        unsafe extern "system" fn GetInputTensor<Impl: IMLOperatorKernelContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, tensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputTensor(::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *tensor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputTensor<Impl: IMLOperatorKernelContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32, tensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputTensor(::core::mem::transmute_copy(&outputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensionsizes)) {
                ::core::result::Result::Ok(ok__) => {
                    *tensor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputTensor2<Impl: IMLOperatorKernelContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, tensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputTensor2(::core::mem::transmute_copy(&outputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *tensor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateTemporaryData<Impl: IMLOperatorKernelContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: usize, data: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllocateTemporaryData(::core::mem::transmute_copy(&size)) {
                ::core::result::Result::Ok(ok__) => {
                    *data = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExecutionInterface<Impl: IMLOperatorKernelContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executionobject: *mut *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetExecutionInterface(::core::mem::transmute_copy(&executionobject))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetInputTensor: GetInputTensor::<Impl, IMPL_OFFSET>,
            GetOutputTensor: GetOutputTensor::<Impl, IMPL_OFFSET>,
            GetOutputTensor: GetOutputTensor::<Impl, IMPL_OFFSET>,
            AllocateTemporaryData: AllocateTemporaryData::<Impl, IMPL_OFFSET>,
            GetExecutionInterface: GetExecutionInterface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorKernelContext as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMLOperatorKernelCreationContextImpl: Sized + IMLOperatorAttributesImpl {
    fn GetInputCount(&mut self) -> u32;
    fn GetOutputCount(&mut self) -> u32;
    fn IsInputValid(&mut self, inputindex: u32) -> bool;
    fn IsOutputValid(&mut self, outputindex: u32) -> bool;
    fn GetInputEdgeDescription(&mut self, inputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription>;
    fn GetOutputEdgeDescription(&mut self, outputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription>;
    fn HasTensorShapeDescription(&mut self) -> bool;
    fn GetTensorShapeDescription(&mut self) -> ::windows::core::Result<IMLOperatorTensorShapeDescription>;
    fn GetExecutionInterface(&mut self, executionobject: *mut ::core::option::Option<::windows::core::IUnknown>);
}
#[cfg(feature = "Win32_Foundation")]
impl IMLOperatorKernelCreationContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelCreationContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLOperatorKernelCreationContextVtbl {
        unsafe extern "system" fn GetInputCount<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInputCount()
        }
        unsafe extern "system" fn GetOutputCount<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputCount()
        }
        unsafe extern "system" fn IsInputValid<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsInputValid(::core::mem::transmute_copy(&inputindex))
        }
        unsafe extern "system" fn IsOutputValid<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsOutputValid(::core::mem::transmute_copy(&outputindex))
        }
        unsafe extern "system" fn GetInputEdgeDescription<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputEdgeDescription(::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *edgedescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputEdgeDescription<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputEdgeDescription(::core::mem::transmute_copy(&outputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *edgedescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasTensorShapeDescription<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HasTensorShapeDescription()
        }
        unsafe extern "system" fn GetTensorShapeDescription<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shapedescription: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTensorShapeDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *shapedescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExecutionInterface<Impl: IMLOperatorKernelCreationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executionobject: *mut *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetExecutionInterface(::core::mem::transmute_copy(&executionobject))
        }
        Self {
            base: IMLOperatorAttributesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetInputCount: GetInputCount::<Impl, IMPL_OFFSET>,
            GetOutputCount: GetOutputCount::<Impl, IMPL_OFFSET>,
            IsInputValid: IsInputValid::<Impl, IMPL_OFFSET>,
            IsOutputValid: IsOutputValid::<Impl, IMPL_OFFSET>,
            GetInputEdgeDescription: GetInputEdgeDescription::<Impl, IMPL_OFFSET>,
            GetOutputEdgeDescription: GetOutputEdgeDescription::<Impl, IMPL_OFFSET>,
            HasTensorShapeDescription: HasTensorShapeDescription::<Impl, IMPL_OFFSET>,
            GetTensorShapeDescription: GetTensorShapeDescription::<Impl, IMPL_OFFSET>,
            GetExecutionInterface: GetExecutionInterface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorKernelCreationContext as ::windows::core::Interface>::IID
    }
}
pub trait IMLOperatorKernelFactoryImpl: Sized {
    fn CreateKernel(&mut self, context: ::core::option::Option<IMLOperatorKernelCreationContext>) -> ::windows::core::Result<IMLOperatorKernel>;
}
impl IMLOperatorKernelFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLOperatorKernelFactoryVtbl {
        unsafe extern "system" fn CreateKernel<Impl: IMLOperatorKernelFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, kernel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateKernel(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    *kernel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateKernel: CreateKernel::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorKernelFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMLOperatorRegistryImpl: Sized {
    fn RegisterOperatorSetSchema(&mut self, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: ::core::option::Option<IMLOperatorTypeInferrer>, shapeinferrer: ::core::option::Option<IMLOperatorShapeInferrer>) -> ::windows::core::Result<()>;
    fn RegisterOperatorKernel(&mut self, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: ::core::option::Option<IMLOperatorKernelFactory>, shapeinferrer: ::core::option::Option<IMLOperatorShapeInferrer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMLOperatorRegistryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorRegistryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLOperatorRegistryVtbl {
        unsafe extern "system" fn RegisterOperatorSetSchema<Impl: IMLOperatorRegistryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: ::windows::core::RawPtr, shapeinferrer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterOperatorSetSchema(::core::mem::transmute_copy(&operatorsetid), ::core::mem::transmute_copy(&baselineversion), ::core::mem::transmute_copy(&schema), ::core::mem::transmute_copy(&schemacount), ::core::mem::transmute(&typeinferrer), ::core::mem::transmute(&shapeinferrer)).into()
        }
        unsafe extern "system" fn RegisterOperatorKernel<Impl: IMLOperatorRegistryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: ::windows::core::RawPtr, shapeinferrer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterOperatorKernel(::core::mem::transmute_copy(&operatorkernel), ::core::mem::transmute(&operatorkernelfactory), ::core::mem::transmute(&shapeinferrer)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterOperatorSetSchema: RegisterOperatorSetSchema::<Impl, IMPL_OFFSET>,
            RegisterOperatorKernel: RegisterOperatorKernel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorRegistry as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMLOperatorShapeInferenceContextImpl: Sized + IMLOperatorAttributesImpl {
    fn GetInputCount(&mut self) -> u32;
    fn GetOutputCount(&mut self) -> u32;
    fn IsInputValid(&mut self, inputindex: u32) -> bool;
    fn IsOutputValid(&mut self, outputindex: u32) -> bool;
    fn GetInputEdgeDescription(&mut self, inputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription>;
    fn GetInputTensorDimensionCount(&mut self, inputindex: u32) -> ::windows::core::Result<u32>;
    fn GetInputTensorShape(&mut self, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()>;
    fn SetOutputTensorShape(&mut self, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMLOperatorShapeInferenceContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorShapeInferenceContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLOperatorShapeInferenceContextVtbl {
        unsafe extern "system" fn GetInputCount<Impl: IMLOperatorShapeInferenceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInputCount()
        }
        unsafe extern "system" fn GetOutputCount<Impl: IMLOperatorShapeInferenceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputCount()
        }
        unsafe extern "system" fn IsInputValid<Impl: IMLOperatorShapeInferenceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsInputValid(::core::mem::transmute_copy(&inputindex))
        }
        unsafe extern "system" fn IsOutputValid<Impl: IMLOperatorShapeInferenceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsOutputValid(::core::mem::transmute_copy(&outputindex))
        }
        unsafe extern "system" fn GetInputEdgeDescription<Impl: IMLOperatorShapeInferenceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputEdgeDescription(::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *edgedescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTensorDimensionCount<Impl: IMLOperatorShapeInferenceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputTensorDimensionCount(::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *dimensioncount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTensorShape<Impl: IMLOperatorShapeInferenceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInputTensorShape(::core::mem::transmute_copy(&inputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into()
        }
        unsafe extern "system" fn SetOutputTensorShape<Impl: IMLOperatorShapeInferenceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutputTensorShape(::core::mem::transmute_copy(&outputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into()
        }
        Self {
            base: IMLOperatorAttributesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetInputCount: GetInputCount::<Impl, IMPL_OFFSET>,
            GetOutputCount: GetOutputCount::<Impl, IMPL_OFFSET>,
            IsInputValid: IsInputValid::<Impl, IMPL_OFFSET>,
            IsOutputValid: IsOutputValid::<Impl, IMPL_OFFSET>,
            GetInputEdgeDescription: GetInputEdgeDescription::<Impl, IMPL_OFFSET>,
            GetInputTensorDimensionCount: GetInputTensorDimensionCount::<Impl, IMPL_OFFSET>,
            GetInputTensorShape: GetInputTensorShape::<Impl, IMPL_OFFSET>,
            SetOutputTensorShape: SetOutputTensorShape::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorShapeInferenceContext as ::windows::core::Interface>::IID
    }
}
pub trait IMLOperatorShapeInferrerImpl: Sized {
    fn InferOutputShapes(&mut self, context: ::core::option::Option<IMLOperatorShapeInferenceContext>) -> ::windows::core::Result<()>;
}
impl IMLOperatorShapeInferrerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorShapeInferrerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLOperatorShapeInferrerVtbl {
        unsafe extern "system" fn InferOutputShapes<Impl: IMLOperatorShapeInferrerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InferOutputShapes(::core::mem::transmute(&context)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), InferOutputShapes: InferOutputShapes::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorShapeInferrer as ::windows::core::Interface>::IID
    }
}
pub trait IMLOperatorTensorImpl: Sized {
    fn GetDimensionCount(&mut self) -> u32;
    fn GetShape(&mut self, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()>;
    fn GetTensorDataType(&mut self) -> MLOperatorTensorDataType;
    fn IsCpuData(&mut self) -> bool;
    fn IsDataInterface(&mut self) -> bool;
    fn GetData(&mut self) -> *mut ::core::ffi::c_void;
    fn GetDataInterface(&mut self, datainterface: *mut ::core::option::Option<::windows::core::IUnknown>);
}
impl IMLOperatorTensorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTensorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLOperatorTensorVtbl {
        unsafe extern "system" fn GetDimensionCount<Impl: IMLOperatorTensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDimensionCount()
        }
        unsafe extern "system" fn GetShape<Impl: IMLOperatorTensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetShape(::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into()
        }
        unsafe extern "system" fn GetTensorDataType<Impl: IMLOperatorTensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> MLOperatorTensorDataType {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTensorDataType()
        }
        unsafe extern "system" fn IsCpuData<Impl: IMLOperatorTensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsCpuData()
        }
        unsafe extern "system" fn IsDataInterface<Impl: IMLOperatorTensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsDataInterface()
        }
        unsafe extern "system" fn GetData<Impl: IMLOperatorTensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetData()
        }
        unsafe extern "system" fn GetDataInterface<Impl: IMLOperatorTensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datainterface: *mut *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDataInterface(::core::mem::transmute_copy(&datainterface))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDimensionCount: GetDimensionCount::<Impl, IMPL_OFFSET>,
            GetShape: GetShape::<Impl, IMPL_OFFSET>,
            GetTensorDataType: GetTensorDataType::<Impl, IMPL_OFFSET>,
            IsCpuData: IsCpuData::<Impl, IMPL_OFFSET>,
            IsDataInterface: IsDataInterface::<Impl, IMPL_OFFSET>,
            GetData: GetData::<Impl, IMPL_OFFSET>,
            GetDataInterface: GetDataInterface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorTensor as ::windows::core::Interface>::IID
    }
}
pub trait IMLOperatorTensorShapeDescriptionImpl: Sized {
    fn GetInputTensorDimensionCount(&mut self, inputindex: u32) -> ::windows::core::Result<u32>;
    fn GetInputTensorShape(&mut self, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()>;
    fn HasOutputShapeDescription(&mut self) -> bool;
    fn GetOutputTensorDimensionCount(&mut self, outputindex: u32) -> ::windows::core::Result<u32>;
    fn GetOutputTensorShape(&mut self, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()>;
}
impl IMLOperatorTensorShapeDescriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTensorShapeDescriptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLOperatorTensorShapeDescriptionVtbl {
        unsafe extern "system" fn GetInputTensorDimensionCount<Impl: IMLOperatorTensorShapeDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputTensorDimensionCount(::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *dimensioncount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTensorShape<Impl: IMLOperatorTensorShapeDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInputTensorShape(::core::mem::transmute_copy(&inputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into()
        }
        unsafe extern "system" fn HasOutputShapeDescription<Impl: IMLOperatorTensorShapeDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HasOutputShapeDescription()
        }
        unsafe extern "system" fn GetOutputTensorDimensionCount<Impl: IMLOperatorTensorShapeDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputTensorDimensionCount(::core::mem::transmute_copy(&outputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *dimensioncount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputTensorShape<Impl: IMLOperatorTensorShapeDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputTensorShape(::core::mem::transmute_copy(&outputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetInputTensorDimensionCount: GetInputTensorDimensionCount::<Impl, IMPL_OFFSET>,
            GetInputTensorShape: GetInputTensorShape::<Impl, IMPL_OFFSET>,
            HasOutputShapeDescription: HasOutputShapeDescription::<Impl, IMPL_OFFSET>,
            GetOutputTensorDimensionCount: GetOutputTensorDimensionCount::<Impl, IMPL_OFFSET>,
            GetOutputTensorShape: GetOutputTensorShape::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorTensorShapeDescription as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMLOperatorTypeInferenceContextImpl: Sized + IMLOperatorAttributesImpl {
    fn GetInputCount(&mut self) -> u32;
    fn GetOutputCount(&mut self) -> u32;
    fn IsInputValid(&mut self, inputindex: u32) -> bool;
    fn IsOutputValid(&mut self, outputindex: u32) -> bool;
    fn GetInputEdgeDescription(&mut self, inputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription>;
    fn SetOutputEdgeDescription(&mut self, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMLOperatorTypeInferenceContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTypeInferenceContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLOperatorTypeInferenceContextVtbl {
        unsafe extern "system" fn GetInputCount<Impl: IMLOperatorTypeInferenceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInputCount()
        }
        unsafe extern "system" fn GetOutputCount<Impl: IMLOperatorTypeInferenceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputCount()
        }
        unsafe extern "system" fn IsInputValid<Impl: IMLOperatorTypeInferenceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsInputValid(::core::mem::transmute_copy(&inputindex))
        }
        unsafe extern "system" fn IsOutputValid<Impl: IMLOperatorTypeInferenceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsOutputValid(::core::mem::transmute_copy(&outputindex))
        }
        unsafe extern "system" fn GetInputEdgeDescription<Impl: IMLOperatorTypeInferenceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputEdgeDescription(::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *edgedescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputEdgeDescription<Impl: IMLOperatorTypeInferenceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutputEdgeDescription(::core::mem::transmute_copy(&outputindex), ::core::mem::transmute_copy(&edgedescription)).into()
        }
        Self {
            base: IMLOperatorAttributesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetInputCount: GetInputCount::<Impl, IMPL_OFFSET>,
            GetOutputCount: GetOutputCount::<Impl, IMPL_OFFSET>,
            IsInputValid: IsInputValid::<Impl, IMPL_OFFSET>,
            IsOutputValid: IsOutputValid::<Impl, IMPL_OFFSET>,
            GetInputEdgeDescription: GetInputEdgeDescription::<Impl, IMPL_OFFSET>,
            SetOutputEdgeDescription: SetOutputEdgeDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorTypeInferenceContext as ::windows::core::Interface>::IID
    }
}
pub trait IMLOperatorTypeInferrerImpl: Sized {
    fn InferOutputTypes(&mut self, context: ::core::option::Option<IMLOperatorTypeInferenceContext>) -> ::windows::core::Result<()>;
}
impl IMLOperatorTypeInferrerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTypeInferrerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLOperatorTypeInferrerVtbl {
        unsafe extern "system" fn InferOutputTypes<Impl: IMLOperatorTypeInferrerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InferOutputTypes(::core::mem::transmute(&context)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), InferOutputTypes: InferOutputTypes::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorTypeInferrer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait IWinMLEvaluationContextImpl: Sized {
    fn BindValue(&mut self, pdescriptor: *const WINML_BINDING_DESC) -> ::windows::core::Result<()>;
    fn GetValueByName(&mut self, name: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<*mut WINML_BINDING_DESC>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl IWinMLEvaluationContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLEvaluationContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWinMLEvaluationContextVtbl {
        unsafe extern "system" fn BindValue<Impl: IWinMLEvaluationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescriptor: *const WINML_BINDING_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindValue(::core::mem::transmute_copy(&pdescriptor)).into()
        }
        unsafe extern "system" fn GetValueByName<Impl: IWinMLEvaluationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR, pdescriptor: *mut *mut WINML_BINDING_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValueByName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdescriptor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IWinMLEvaluationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BindValue: BindValue::<Impl, IMPL_OFFSET>,
            GetValueByName: GetValueByName::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinMLEvaluationContext as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWinMLModelImpl: Sized {
    fn GetDescription(&mut self) -> ::windows::core::Result<*mut WINML_MODEL_DESC>;
    fn EnumerateMetadata(&mut self, index: u32, pkey: *mut super::super::super::Foundation::PWSTR, pvalue: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn EnumerateModelInputs(&mut self, index: u32) -> ::windows::core::Result<*mut WINML_VARIABLE_DESC>;
    fn EnumerateModelOutputs(&mut self, index: u32) -> ::windows::core::Result<*mut WINML_VARIABLE_DESC>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWinMLModelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLModelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWinMLModelVtbl {
        unsafe extern "system" fn GetDescription<Impl: IWinMLModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdescription: *mut *mut WINML_MODEL_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateMetadata<Impl: IWinMLModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pkey: *mut super::super::super::Foundation::PWSTR, pvalue: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateMetadata(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn EnumerateModelInputs<Impl: IWinMLModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppinputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateModelInputs(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinputdescriptor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateModelOutputs<Impl: IWinMLModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppoutputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateModelOutputs(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoutputdescriptor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
            EnumerateMetadata: EnumerateMetadata::<Impl, IMPL_OFFSET>,
            EnumerateModelInputs: EnumerateModelInputs::<Impl, IMPL_OFFSET>,
            EnumerateModelOutputs: EnumerateModelOutputs::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinMLModel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait IWinMLRuntimeImpl: Sized {
    fn LoadModel(&mut self, path: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<IWinMLModel>;
    fn CreateEvaluationContext(&mut self, device: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Device>) -> ::windows::core::Result<IWinMLEvaluationContext>;
    fn EvaluateModel(&mut self, pcontext: ::core::option::Option<IWinMLEvaluationContext>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl IWinMLRuntimeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLRuntimeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWinMLRuntimeVtbl {
        unsafe extern "system" fn LoadModel<Impl: IWinMLRuntimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::super::Foundation::PWSTR, ppmodel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadModel(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmodel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEvaluationContext<Impl: IWinMLRuntimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEvaluationContext(::core::mem::transmute(&device)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EvaluateModel<Impl: IWinMLRuntimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EvaluateModel(::core::mem::transmute(&pcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            LoadModel: LoadModel::<Impl, IMPL_OFFSET>,
            CreateEvaluationContext: CreateEvaluationContext::<Impl, IMPL_OFFSET>,
            EvaluateModel: EvaluateModel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinMLRuntime as ::windows::core::Interface>::IID
    }
}
pub trait IWinMLRuntimeFactoryImpl: Sized {
    fn CreateRuntime(&mut self, runtimetype: WINML_RUNTIME_TYPE) -> ::windows::core::Result<IWinMLRuntime>;
}
impl IWinMLRuntimeFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLRuntimeFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWinMLRuntimeFactoryVtbl {
        unsafe extern "system" fn CreateRuntime<Impl: IWinMLRuntimeFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runtimetype: WINML_RUNTIME_TYPE, ppruntime: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRuntime(::core::mem::transmute_copy(&runtimetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppruntime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateRuntime: CreateRuntime::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinMLRuntimeFactory as ::windows::core::Interface>::IID
    }
}
