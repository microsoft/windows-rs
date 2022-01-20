#[cfg(feature = "Win32_Foundation")]
pub trait IMLOperatorAttributes_Impl: Sized {
    fn GetAttributeElementCount(&mut self, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType) -> ::windows::core::Result<u32>;
    fn GetAttribute(&mut self, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetStringAttributeElementLength(&mut self, name: super::super::super::Foundation::PSTR, elementindex: u32) -> ::windows::core::Result<u32>;
    fn GetStringAttributeElement(&mut self, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMLOperatorAttributes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorAttributes_Impl, const OFFSET: isize>() -> IMLOperatorAttributes_Vtbl {
        unsafe extern "system" fn GetAttributeElementCount<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAttributeElementCount(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *elementcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAttribute(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&elementcount), ::core::mem::transmute_copy(&elementbytesize), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetStringAttributeElementLength<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStringAttributeElementLength(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&elementindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *attributeelementbytesize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringAttributeElement<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStringAttributeElement(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&elementindex), ::core::mem::transmute_copy(&attributeelementbytesize), ::core::mem::transmute_copy(&attributeelement)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetAttributeElementCount: GetAttributeElementCount::<Identity, Impl, OFFSET>,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
            GetStringAttributeElementLength: GetStringAttributeElementLength::<Identity, Impl, OFFSET>,
            GetStringAttributeElement: GetStringAttributeElement::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorAttributes as ::windows::core::Interface>::IID
    }
}
pub trait IMLOperatorKernel_Impl: Sized {
    fn Compute(&mut self, context: &::core::option::Option<IMLOperatorKernelContext>) -> ::windows::core::Result<()>;
}
impl IMLOperatorKernel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernel_Impl, const OFFSET: isize>() -> IMLOperatorKernel_Vtbl {
        unsafe extern "system" fn Compute<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Compute(::core::mem::transmute(&context)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Compute: Compute::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorKernel as ::windows::core::Interface>::IID
    }
}
pub trait IMLOperatorKernelContext_Impl: Sized {
    fn GetInputTensor(&mut self, inputindex: u32) -> ::windows::core::Result<IMLOperatorTensor>;
    fn GetOutputTensor(&mut self, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32) -> ::windows::core::Result<IMLOperatorTensor>;
    fn GetOutputTensor2(&mut self, outputindex: u32) -> ::windows::core::Result<IMLOperatorTensor>;
    fn AllocateTemporaryData(&mut self, size: usize) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetExecutionInterface(&mut self, executionobject: *mut ::core::option::Option<::windows::core::IUnknown>);
}
impl IMLOperatorKernelContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelContext_Impl, const OFFSET: isize>() -> IMLOperatorKernelContext_Vtbl {
        unsafe extern "system" fn GetInputTensor<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, tensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputTensor(::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *tensor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputTensor<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32, tensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputTensor(::core::mem::transmute_copy(&outputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensionsizes)) {
                ::core::result::Result::Ok(ok__) => {
                    *tensor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputTensor2<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, tensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputTensor2(::core::mem::transmute_copy(&outputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *tensor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateTemporaryData<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: usize, data: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllocateTemporaryData(::core::mem::transmute_copy(&size)) {
                ::core::result::Result::Ok(ok__) => {
                    *data = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExecutionInterface<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executionobject: *mut *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetExecutionInterface(::core::mem::transmute_copy(&executionobject))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetInputTensor: GetInputTensor::<Identity, Impl, OFFSET>,
            GetOutputTensor: GetOutputTensor::<Identity, Impl, OFFSET>,
            GetOutputTensor2: GetOutputTensor2::<Identity, Impl, OFFSET>,
            AllocateTemporaryData: AllocateTemporaryData::<Identity, Impl, OFFSET>,
            GetExecutionInterface: GetExecutionInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorKernelContext as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMLOperatorKernelCreationContext_Impl: Sized + IMLOperatorAttributes_Impl {
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
impl IMLOperatorKernelCreationContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>() -> IMLOperatorKernelCreationContext_Vtbl {
        unsafe extern "system" fn GetInputCount<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInputCount()
        }
        unsafe extern "system" fn GetOutputCount<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOutputCount()
        }
        unsafe extern "system" fn IsInputValid<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsInputValid(::core::mem::transmute_copy(&inputindex))
        }
        unsafe extern "system" fn IsOutputValid<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsOutputValid(::core::mem::transmute_copy(&outputindex))
        }
        unsafe extern "system" fn GetInputEdgeDescription<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputEdgeDescription(::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *edgedescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputEdgeDescription<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputEdgeDescription(::core::mem::transmute_copy(&outputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *edgedescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasTensorShapeDescription<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HasTensorShapeDescription()
        }
        unsafe extern "system" fn GetTensorShapeDescription<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shapedescription: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTensorShapeDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *shapedescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExecutionInterface<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executionobject: *mut *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetExecutionInterface(::core::mem::transmute_copy(&executionobject))
        }
        Self {
            base: IMLOperatorAttributes_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetInputCount: GetInputCount::<Identity, Impl, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, Impl, OFFSET>,
            IsInputValid: IsInputValid::<Identity, Impl, OFFSET>,
            IsOutputValid: IsOutputValid::<Identity, Impl, OFFSET>,
            GetInputEdgeDescription: GetInputEdgeDescription::<Identity, Impl, OFFSET>,
            GetOutputEdgeDescription: GetOutputEdgeDescription::<Identity, Impl, OFFSET>,
            HasTensorShapeDescription: HasTensorShapeDescription::<Identity, Impl, OFFSET>,
            GetTensorShapeDescription: GetTensorShapeDescription::<Identity, Impl, OFFSET>,
            GetExecutionInterface: GetExecutionInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorKernelCreationContext as ::windows::core::Interface>::IID || iid == &<IMLOperatorAttributes as ::windows::core::Interface>::IID
    }
}
pub trait IMLOperatorKernelFactory_Impl: Sized {
    fn CreateKernel(&mut self, context: &::core::option::Option<IMLOperatorKernelCreationContext>) -> ::windows::core::Result<IMLOperatorKernel>;
}
impl IMLOperatorKernelFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelFactory_Impl, const OFFSET: isize>() -> IMLOperatorKernelFactory_Vtbl {
        unsafe extern "system" fn CreateKernel<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorKernelFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, kernel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateKernel(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    *kernel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateKernel: CreateKernel::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorKernelFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMLOperatorRegistry_Impl: Sized {
    fn RegisterOperatorSetSchema(&mut self, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: &::core::option::Option<IMLOperatorTypeInferrer>, shapeinferrer: &::core::option::Option<IMLOperatorShapeInferrer>) -> ::windows::core::Result<()>;
    fn RegisterOperatorKernel(&mut self, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: &::core::option::Option<IMLOperatorKernelFactory>, shapeinferrer: &::core::option::Option<IMLOperatorShapeInferrer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMLOperatorRegistry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorRegistry_Impl, const OFFSET: isize>() -> IMLOperatorRegistry_Vtbl {
        unsafe extern "system" fn RegisterOperatorSetSchema<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorRegistry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: ::windows::core::RawPtr, shapeinferrer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterOperatorSetSchema(::core::mem::transmute_copy(&operatorsetid), ::core::mem::transmute_copy(&baselineversion), ::core::mem::transmute_copy(&schema), ::core::mem::transmute_copy(&schemacount), ::core::mem::transmute(&typeinferrer), ::core::mem::transmute(&shapeinferrer)).into()
        }
        unsafe extern "system" fn RegisterOperatorKernel<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorRegistry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: ::windows::core::RawPtr, shapeinferrer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterOperatorKernel(::core::mem::transmute_copy(&operatorkernel), ::core::mem::transmute(&operatorkernelfactory), ::core::mem::transmute(&shapeinferrer)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterOperatorSetSchema: RegisterOperatorSetSchema::<Identity, Impl, OFFSET>,
            RegisterOperatorKernel: RegisterOperatorKernel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorRegistry as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMLOperatorShapeInferenceContext_Impl: Sized + IMLOperatorAttributes_Impl {
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
impl IMLOperatorShapeInferenceContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>() -> IMLOperatorShapeInferenceContext_Vtbl {
        unsafe extern "system" fn GetInputCount<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInputCount()
        }
        unsafe extern "system" fn GetOutputCount<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOutputCount()
        }
        unsafe extern "system" fn IsInputValid<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsInputValid(::core::mem::transmute_copy(&inputindex))
        }
        unsafe extern "system" fn IsOutputValid<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsOutputValid(::core::mem::transmute_copy(&outputindex))
        }
        unsafe extern "system" fn GetInputEdgeDescription<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputEdgeDescription(::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *edgedescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTensorDimensionCount<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputTensorDimensionCount(::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *dimensioncount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTensorShape<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInputTensorShape(::core::mem::transmute_copy(&inputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into()
        }
        unsafe extern "system" fn SetOutputTensorShape<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutputTensorShape(::core::mem::transmute_copy(&outputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into()
        }
        Self {
            base: IMLOperatorAttributes_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetInputCount: GetInputCount::<Identity, Impl, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, Impl, OFFSET>,
            IsInputValid: IsInputValid::<Identity, Impl, OFFSET>,
            IsOutputValid: IsOutputValid::<Identity, Impl, OFFSET>,
            GetInputEdgeDescription: GetInputEdgeDescription::<Identity, Impl, OFFSET>,
            GetInputTensorDimensionCount: GetInputTensorDimensionCount::<Identity, Impl, OFFSET>,
            GetInputTensorShape: GetInputTensorShape::<Identity, Impl, OFFSET>,
            SetOutputTensorShape: SetOutputTensorShape::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorShapeInferenceContext as ::windows::core::Interface>::IID || iid == &<IMLOperatorAttributes as ::windows::core::Interface>::IID
    }
}
pub trait IMLOperatorShapeInferrer_Impl: Sized {
    fn InferOutputShapes(&mut self, context: &::core::option::Option<IMLOperatorShapeInferenceContext>) -> ::windows::core::Result<()>;
}
impl IMLOperatorShapeInferrer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorShapeInferrer_Impl, const OFFSET: isize>() -> IMLOperatorShapeInferrer_Vtbl {
        unsafe extern "system" fn InferOutputShapes<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorShapeInferrer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InferOutputShapes(::core::mem::transmute(&context)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), InferOutputShapes: InferOutputShapes::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorShapeInferrer as ::windows::core::Interface>::IID
    }
}
pub trait IMLOperatorTensor_Impl: Sized {
    fn GetDimensionCount(&mut self) -> u32;
    fn GetShape(&mut self, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()>;
    fn GetTensorDataType(&mut self) -> MLOperatorTensorDataType;
    fn IsCpuData(&mut self) -> bool;
    fn IsDataInterface(&mut self) -> bool;
    fn GetData(&mut self) -> *mut ::core::ffi::c_void;
    fn GetDataInterface(&mut self, datainterface: *mut ::core::option::Option<::windows::core::IUnknown>);
}
impl IMLOperatorTensor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>() -> IMLOperatorTensor_Vtbl {
        unsafe extern "system" fn GetDimensionCount<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDimensionCount()
        }
        unsafe extern "system" fn GetShape<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetShape(::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into()
        }
        unsafe extern "system" fn GetTensorDataType<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> MLOperatorTensorDataType {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTensorDataType()
        }
        unsafe extern "system" fn IsCpuData<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsCpuData()
        }
        unsafe extern "system" fn IsDataInterface<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsDataInterface()
        }
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetData()
        }
        unsafe extern "system" fn GetDataInterface<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datainterface: *mut *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDataInterface(::core::mem::transmute_copy(&datainterface))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDimensionCount: GetDimensionCount::<Identity, Impl, OFFSET>,
            GetShape: GetShape::<Identity, Impl, OFFSET>,
            GetTensorDataType: GetTensorDataType::<Identity, Impl, OFFSET>,
            IsCpuData: IsCpuData::<Identity, Impl, OFFSET>,
            IsDataInterface: IsDataInterface::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            GetDataInterface: GetDataInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorTensor as ::windows::core::Interface>::IID
    }
}
pub trait IMLOperatorTensorShapeDescription_Impl: Sized {
    fn GetInputTensorDimensionCount(&mut self, inputindex: u32) -> ::windows::core::Result<u32>;
    fn GetInputTensorShape(&mut self, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()>;
    fn HasOutputShapeDescription(&mut self) -> bool;
    fn GetOutputTensorDimensionCount(&mut self, outputindex: u32) -> ::windows::core::Result<u32>;
    fn GetOutputTensorShape(&mut self, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()>;
}
impl IMLOperatorTensorShapeDescription_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>() -> IMLOperatorTensorShapeDescription_Vtbl {
        unsafe extern "system" fn GetInputTensorDimensionCount<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputTensorDimensionCount(::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *dimensioncount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTensorShape<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInputTensorShape(::core::mem::transmute_copy(&inputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into()
        }
        unsafe extern "system" fn HasOutputShapeDescription<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HasOutputShapeDescription()
        }
        unsafe extern "system" fn GetOutputTensorDimensionCount<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputTensorDimensionCount(::core::mem::transmute_copy(&outputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *dimensioncount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputTensorShape<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOutputTensorShape(::core::mem::transmute_copy(&outputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetInputTensorDimensionCount: GetInputTensorDimensionCount::<Identity, Impl, OFFSET>,
            GetInputTensorShape: GetInputTensorShape::<Identity, Impl, OFFSET>,
            HasOutputShapeDescription: HasOutputShapeDescription::<Identity, Impl, OFFSET>,
            GetOutputTensorDimensionCount: GetOutputTensorDimensionCount::<Identity, Impl, OFFSET>,
            GetOutputTensorShape: GetOutputTensorShape::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorTensorShapeDescription as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMLOperatorTypeInferenceContext_Impl: Sized + IMLOperatorAttributes_Impl {
    fn GetInputCount(&mut self) -> u32;
    fn GetOutputCount(&mut self) -> u32;
    fn IsInputValid(&mut self, inputindex: u32) -> bool;
    fn IsOutputValid(&mut self, outputindex: u32) -> bool;
    fn GetInputEdgeDescription(&mut self, inputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription>;
    fn SetOutputEdgeDescription(&mut self, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMLOperatorTypeInferenceContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>() -> IMLOperatorTypeInferenceContext_Vtbl {
        unsafe extern "system" fn GetInputCount<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInputCount()
        }
        unsafe extern "system" fn GetOutputCount<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOutputCount()
        }
        unsafe extern "system" fn IsInputValid<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsInputValid(::core::mem::transmute_copy(&inputindex))
        }
        unsafe extern "system" fn IsOutputValid<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsOutputValid(::core::mem::transmute_copy(&outputindex))
        }
        unsafe extern "system" fn GetInputEdgeDescription<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputEdgeDescription(::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *edgedescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputEdgeDescription<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutputEdgeDescription(::core::mem::transmute_copy(&outputindex), ::core::mem::transmute_copy(&edgedescription)).into()
        }
        Self {
            base: IMLOperatorAttributes_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetInputCount: GetInputCount::<Identity, Impl, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, Impl, OFFSET>,
            IsInputValid: IsInputValid::<Identity, Impl, OFFSET>,
            IsOutputValid: IsOutputValid::<Identity, Impl, OFFSET>,
            GetInputEdgeDescription: GetInputEdgeDescription::<Identity, Impl, OFFSET>,
            SetOutputEdgeDescription: SetOutputEdgeDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorTypeInferenceContext as ::windows::core::Interface>::IID || iid == &<IMLOperatorAttributes as ::windows::core::Interface>::IID
    }
}
pub trait IMLOperatorTypeInferrer_Impl: Sized {
    fn InferOutputTypes(&mut self, context: &::core::option::Option<IMLOperatorTypeInferenceContext>) -> ::windows::core::Result<()>;
}
impl IMLOperatorTypeInferrer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTypeInferrer_Impl, const OFFSET: isize>() -> IMLOperatorTypeInferrer_Vtbl {
        unsafe extern "system" fn InferOutputTypes<Identity: ::windows::core::IUnknownImpl, Impl: IMLOperatorTypeInferrer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InferOutputTypes(::core::mem::transmute(&context)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), InferOutputTypes: InferOutputTypes::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorTypeInferrer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait IWinMLEvaluationContext_Impl: Sized {
    fn BindValue(&mut self, pdescriptor: *const WINML_BINDING_DESC) -> ::windows::core::Result<()>;
    fn GetValueByName(&mut self, name: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<*mut WINML_BINDING_DESC>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl IWinMLEvaluationContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLEvaluationContext_Impl, const OFFSET: isize>() -> IWinMLEvaluationContext_Vtbl {
        unsafe extern "system" fn BindValue<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLEvaluationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescriptor: *const WINML_BINDING_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BindValue(::core::mem::transmute_copy(&pdescriptor)).into()
        }
        unsafe extern "system" fn GetValueByName<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLEvaluationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR, pdescriptor: *mut *mut WINML_BINDING_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetValueByName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdescriptor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLEvaluationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            BindValue: BindValue::<Identity, Impl, OFFSET>,
            GetValueByName: GetValueByName::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinMLEvaluationContext as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWinMLModel_Impl: Sized {
    fn GetDescription(&mut self) -> ::windows::core::Result<*mut WINML_MODEL_DESC>;
    fn EnumerateMetadata(&mut self, index: u32, pkey: *mut super::super::super::Foundation::PWSTR, pvalue: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn EnumerateModelInputs(&mut self, index: u32) -> ::windows::core::Result<*mut WINML_VARIABLE_DESC>;
    fn EnumerateModelOutputs(&mut self, index: u32) -> ::windows::core::Result<*mut WINML_VARIABLE_DESC>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWinMLModel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLModel_Impl, const OFFSET: isize>() -> IWinMLModel_Vtbl {
        unsafe extern "system" fn GetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLModel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdescription: *mut *mut WINML_MODEL_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateMetadata<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLModel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pkey: *mut super::super::super::Foundation::PWSTR, pvalue: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumerateMetadata(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn EnumerateModelInputs<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLModel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppinputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateModelInputs(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinputdescriptor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateModelOutputs<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLModel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppoutputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateModelOutputs(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoutputdescriptor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            EnumerateMetadata: EnumerateMetadata::<Identity, Impl, OFFSET>,
            EnumerateModelInputs: EnumerateModelInputs::<Identity, Impl, OFFSET>,
            EnumerateModelOutputs: EnumerateModelOutputs::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinMLModel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait IWinMLRuntime_Impl: Sized {
    fn LoadModel(&mut self, path: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<IWinMLModel>;
    fn CreateEvaluationContext(&mut self, device: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Device>) -> ::windows::core::Result<IWinMLEvaluationContext>;
    fn EvaluateModel(&mut self, pcontext: &::core::option::Option<IWinMLEvaluationContext>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl IWinMLRuntime_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLRuntime_Impl, const OFFSET: isize>() -> IWinMLRuntime_Vtbl {
        unsafe extern "system" fn LoadModel<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::super::Foundation::PWSTR, ppmodel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoadModel(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmodel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEvaluationContext<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateEvaluationContext(::core::mem::transmute(&device)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EvaluateModel<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EvaluateModel(::core::mem::transmute(&pcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            LoadModel: LoadModel::<Identity, Impl, OFFSET>,
            CreateEvaluationContext: CreateEvaluationContext::<Identity, Impl, OFFSET>,
            EvaluateModel: EvaluateModel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinMLRuntime as ::windows::core::Interface>::IID
    }
}
pub trait IWinMLRuntimeFactory_Impl: Sized {
    fn CreateRuntime(&mut self, runtimetype: WINML_RUNTIME_TYPE) -> ::windows::core::Result<IWinMLRuntime>;
}
impl IWinMLRuntimeFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLRuntimeFactory_Impl, const OFFSET: isize>() -> IWinMLRuntimeFactory_Vtbl {
        unsafe extern "system" fn CreateRuntime<Identity: ::windows::core::IUnknownImpl, Impl: IWinMLRuntimeFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runtimetype: WINML_RUNTIME_TYPE, ppruntime: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRuntime(::core::mem::transmute_copy(&runtimetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppruntime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateRuntime: CreateRuntime::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinMLRuntimeFactory as ::windows::core::Interface>::IID
    }
}
