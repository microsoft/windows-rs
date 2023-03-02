#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"implement\"`*"]
pub trait IMLOperatorAttributes_Impl: Sized {
    fn GetAttributeElementCount(&self, name: &::windows::core::PCSTR, r#type: MLOperatorAttributeType) -> ::windows::core::Result<u32>;
    fn GetAttribute(&self, name: &::windows::core::PCSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetStringAttributeElementLength(&self, name: &::windows::core::PCSTR, elementindex: u32) -> ::windows::core::Result<u32>;
    fn GetStringAttributeElement(&self, name: &::windows::core::PCSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: ::windows::core::PSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMLOperatorAttributes {}
impl IMLOperatorAttributes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorAttributes_Impl, const OFFSET: isize>() -> IMLOperatorAttributes_Vtbl {
        unsafe extern "system" fn GetAttributeElementCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttributeElementCount(::core::mem::transmute(&name), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(elementcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttribute(::core::mem::transmute(&name), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&elementcount), ::core::mem::transmute_copy(&elementbytesize), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetStringAttributeElementLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStringAttributeElementLength(::core::mem::transmute(&name), ::core::mem::transmute_copy(&elementindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributeelementbytesize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringAttributeElement<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: ::windows::core::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStringAttributeElement(::core::mem::transmute(&name), ::core::mem::transmute_copy(&elementindex), ::core::mem::transmute_copy(&attributeelementbytesize), ::core::mem::transmute_copy(&attributeelement)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAttributeElementCount: GetAttributeElementCount::<Identity, Impl, OFFSET>,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
            GetStringAttributeElementLength: GetStringAttributeElementLength::<Identity, Impl, OFFSET>,
            GetStringAttributeElement: GetStringAttributeElement::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorAttributes as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"implement\"`*"]
pub trait IMLOperatorKernel_Impl: Sized {
    fn Compute(&self, context: ::core::option::Option<&IMLOperatorKernelContext>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMLOperatorKernel {}
impl IMLOperatorKernel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernel_Impl, const OFFSET: isize>() -> IMLOperatorKernel_Vtbl {
        unsafe extern "system" fn Compute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Compute(::windows::core::from_raw_borrowed(&context)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Compute: Compute::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorKernel as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"implement\"`*"]
pub trait IMLOperatorKernelContext_Impl: Sized {
    fn GetInputTensor(&self, inputindex: u32) -> ::windows::core::Result<IMLOperatorTensor>;
    fn GetOutputTensor(&self, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32) -> ::windows::core::Result<IMLOperatorTensor>;
    fn GetOutputTensor2(&self, outputindex: u32) -> ::windows::core::Result<IMLOperatorTensor>;
    fn AllocateTemporaryData(&self, size: usize) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetExecutionInterface(&self, executionobject: *mut ::core::option::Option<::windows::core::IUnknown>) -> ();
}
impl ::windows::core::RuntimeName for IMLOperatorKernelContext {}
impl IMLOperatorKernelContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelContext_Impl, const OFFSET: isize>() -> IMLOperatorKernelContext_Vtbl {
        unsafe extern "system" fn GetInputTensor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, tensor: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputTensor(::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tensor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputTensor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32, tensor: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputTensor(::core::mem::transmute_copy(&outputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensionsizes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tensor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputTensor2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, tensor: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputTensor2(::core::mem::transmute_copy(&outputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tensor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateTemporaryData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: usize, data: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllocateTemporaryData(::core::mem::transmute_copy(&size)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(data, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExecutionInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executionobject: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetExecutionInterface(::core::mem::transmute_copy(&executionobject))
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInputTensor: GetInputTensor::<Identity, Impl, OFFSET>,
            GetOutputTensor: GetOutputTensor::<Identity, Impl, OFFSET>,
            GetOutputTensor2: GetOutputTensor2::<Identity, Impl, OFFSET>,
            AllocateTemporaryData: AllocateTemporaryData::<Identity, Impl, OFFSET>,
            GetExecutionInterface: GetExecutionInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorKernelContext as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"implement\"`*"]
pub trait IMLOperatorKernelCreationContext_Impl: Sized + IMLOperatorAttributes_Impl {
    fn GetInputCount(&self) -> u32;
    fn GetOutputCount(&self) -> u32;
    fn IsInputValid(&self, inputindex: u32) -> bool;
    fn IsOutputValid(&self, outputindex: u32) -> bool;
    fn GetInputEdgeDescription(&self, inputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription>;
    fn GetOutputEdgeDescription(&self, outputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription>;
    fn HasTensorShapeDescription(&self) -> bool;
    fn GetTensorShapeDescription(&self) -> ::windows::core::Result<IMLOperatorTensorShapeDescription>;
    fn GetExecutionInterface(&self, executionobject: *mut ::core::option::Option<::windows::core::IUnknown>) -> ();
}
impl ::windows::core::RuntimeName for IMLOperatorKernelCreationContext {}
impl IMLOperatorKernelCreationContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>() -> IMLOperatorKernelCreationContext_Vtbl {
        unsafe extern "system" fn GetInputCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInputCount()
        }
        unsafe extern "system" fn GetOutputCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutputCount()
        }
        unsafe extern "system" fn IsInputValid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsInputValid(::core::mem::transmute_copy(&inputindex))
        }
        unsafe extern "system" fn IsOutputValid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsOutputValid(::core::mem::transmute_copy(&outputindex))
        }
        unsafe extern "system" fn GetInputEdgeDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputEdgeDescription(::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(edgedescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputEdgeDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputEdgeDescription(::core::mem::transmute_copy(&outputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(edgedescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasTensorShapeDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HasTensorShapeDescription()
        }
        unsafe extern "system" fn GetTensorShapeDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shapedescription: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTensorShapeDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(shapedescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExecutionInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executionobject: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetExecutionInterface(::core::mem::transmute_copy(&executionobject))
        }
        Self {
            base__: IMLOperatorAttributes_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMLOperatorKernelCreationContext as ::windows::core::ComInterface>::IID || iid == &<IMLOperatorAttributes as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"implement\"`*"]
pub trait IMLOperatorKernelFactory_Impl: Sized {
    fn CreateKernel(&self, context: ::core::option::Option<&IMLOperatorKernelCreationContext>) -> ::windows::core::Result<IMLOperatorKernel>;
}
impl ::windows::core::RuntimeName for IMLOperatorKernelFactory {}
impl IMLOperatorKernelFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelFactory_Impl, const OFFSET: isize>() -> IMLOperatorKernelFactory_Vtbl {
        unsafe extern "system" fn CreateKernel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, kernel: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateKernel(::windows::core::from_raw_borrowed(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(kernel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateKernel: CreateKernel::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorKernelFactory as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"implement\"`*"]
pub trait IMLOperatorRegistry_Impl: Sized {
    fn RegisterOperatorSetSchema(&self, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: ::core::option::Option<&IMLOperatorTypeInferrer>, shapeinferrer: ::core::option::Option<&IMLOperatorShapeInferrer>) -> ::windows::core::Result<()>;
    fn RegisterOperatorKernel(&self, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: ::core::option::Option<&IMLOperatorKernelFactory>, shapeinferrer: ::core::option::Option<&IMLOperatorShapeInferrer>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMLOperatorRegistry {}
impl IMLOperatorRegistry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorRegistry_Impl, const OFFSET: isize>() -> IMLOperatorRegistry_Vtbl {
        unsafe extern "system" fn RegisterOperatorSetSchema<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorRegistry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: *mut ::core::ffi::c_void, shapeinferrer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterOperatorSetSchema(::core::mem::transmute_copy(&operatorsetid), ::core::mem::transmute_copy(&baselineversion), ::core::mem::transmute_copy(&schema), ::core::mem::transmute_copy(&schemacount), ::windows::core::from_raw_borrowed(&typeinferrer), ::windows::core::from_raw_borrowed(&shapeinferrer)).into()
        }
        unsafe extern "system" fn RegisterOperatorKernel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorRegistry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: *mut ::core::ffi::c_void, shapeinferrer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterOperatorKernel(::core::mem::transmute_copy(&operatorkernel), ::windows::core::from_raw_borrowed(&operatorkernelfactory), ::windows::core::from_raw_borrowed(&shapeinferrer)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterOperatorSetSchema: RegisterOperatorSetSchema::<Identity, Impl, OFFSET>,
            RegisterOperatorKernel: RegisterOperatorKernel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorRegistry as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"implement\"`*"]
pub trait IMLOperatorShapeInferenceContext_Impl: Sized + IMLOperatorAttributes_Impl {
    fn GetInputCount(&self) -> u32;
    fn GetOutputCount(&self) -> u32;
    fn IsInputValid(&self, inputindex: u32) -> bool;
    fn IsOutputValid(&self, outputindex: u32) -> bool;
    fn GetInputEdgeDescription(&self, inputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription>;
    fn GetInputTensorDimensionCount(&self, inputindex: u32) -> ::windows::core::Result<u32>;
    fn GetInputTensorShape(&self, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()>;
    fn SetOutputTensorShape(&self, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMLOperatorShapeInferenceContext {}
impl IMLOperatorShapeInferenceContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>() -> IMLOperatorShapeInferenceContext_Vtbl {
        unsafe extern "system" fn GetInputCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInputCount()
        }
        unsafe extern "system" fn GetOutputCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutputCount()
        }
        unsafe extern "system" fn IsInputValid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsInputValid(::core::mem::transmute_copy(&inputindex))
        }
        unsafe extern "system" fn IsOutputValid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsOutputValid(::core::mem::transmute_copy(&outputindex))
        }
        unsafe extern "system" fn GetInputEdgeDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputEdgeDescription(::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(edgedescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTensorDimensionCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputTensorDimensionCount(::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dimensioncount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTensorShape<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInputTensorShape(::core::mem::transmute_copy(&inputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into()
        }
        unsafe extern "system" fn SetOutputTensorShape<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutputTensorShape(::core::mem::transmute_copy(&outputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into()
        }
        Self {
            base__: IMLOperatorAttributes_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMLOperatorShapeInferenceContext as ::windows::core::ComInterface>::IID || iid == &<IMLOperatorAttributes as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"implement\"`*"]
pub trait IMLOperatorShapeInferrer_Impl: Sized {
    fn InferOutputShapes(&self, context: ::core::option::Option<&IMLOperatorShapeInferenceContext>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMLOperatorShapeInferrer {}
impl IMLOperatorShapeInferrer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferrer_Impl, const OFFSET: isize>() -> IMLOperatorShapeInferrer_Vtbl {
        unsafe extern "system" fn InferOutputShapes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferrer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InferOutputShapes(::windows::core::from_raw_borrowed(&context)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InferOutputShapes: InferOutputShapes::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorShapeInferrer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"implement\"`*"]
pub trait IMLOperatorTensor_Impl: Sized {
    fn GetDimensionCount(&self) -> u32;
    fn GetShape(&self, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()>;
    fn GetTensorDataType(&self) -> MLOperatorTensorDataType;
    fn IsCpuData(&self) -> bool;
    fn IsDataInterface(&self) -> bool;
    fn GetData(&self) -> *mut ::core::ffi::c_void;
    fn GetDataInterface(&self, datainterface: *mut ::core::option::Option<::windows::core::IUnknown>) -> ();
}
impl ::windows::core::RuntimeName for IMLOperatorTensor {}
impl IMLOperatorTensor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>() -> IMLOperatorTensor_Vtbl {
        unsafe extern "system" fn GetDimensionCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDimensionCount()
        }
        unsafe extern "system" fn GetShape<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetShape(::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into()
        }
        unsafe extern "system" fn GetTensorDataType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> MLOperatorTensorDataType {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTensorDataType()
        }
        unsafe extern "system" fn IsCpuData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsCpuData()
        }
        unsafe extern "system" fn IsDataInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsDataInterface()
        }
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetData()
        }
        unsafe extern "system" fn GetDataInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datainterface: *mut *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDataInterface(::core::mem::transmute_copy(&datainterface))
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IMLOperatorTensor as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"implement\"`*"]
pub trait IMLOperatorTensorShapeDescription_Impl: Sized {
    fn GetInputTensorDimensionCount(&self, inputindex: u32) -> ::windows::core::Result<u32>;
    fn GetInputTensorShape(&self, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()>;
    fn HasOutputShapeDescription(&self) -> bool;
    fn GetOutputTensorDimensionCount(&self, outputindex: u32) -> ::windows::core::Result<u32>;
    fn GetOutputTensorShape(&self, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMLOperatorTensorShapeDescription {}
impl IMLOperatorTensorShapeDescription_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>() -> IMLOperatorTensorShapeDescription_Vtbl {
        unsafe extern "system" fn GetInputTensorDimensionCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputTensorDimensionCount(::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dimensioncount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTensorShape<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInputTensorShape(::core::mem::transmute_copy(&inputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into()
        }
        unsafe extern "system" fn HasOutputShapeDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HasOutputShapeDescription()
        }
        unsafe extern "system" fn GetOutputTensorDimensionCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputTensorDimensionCount(::core::mem::transmute_copy(&outputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dimensioncount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputTensorShape<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutputTensorShape(::core::mem::transmute_copy(&outputindex), ::core::mem::transmute_copy(&dimensioncount), ::core::mem::transmute_copy(&dimensions)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInputTensorDimensionCount: GetInputTensorDimensionCount::<Identity, Impl, OFFSET>,
            GetInputTensorShape: GetInputTensorShape::<Identity, Impl, OFFSET>,
            HasOutputShapeDescription: HasOutputShapeDescription::<Identity, Impl, OFFSET>,
            GetOutputTensorDimensionCount: GetOutputTensorDimensionCount::<Identity, Impl, OFFSET>,
            GetOutputTensorShape: GetOutputTensorShape::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorTensorShapeDescription as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"implement\"`*"]
pub trait IMLOperatorTypeInferenceContext_Impl: Sized + IMLOperatorAttributes_Impl {
    fn GetInputCount(&self) -> u32;
    fn GetOutputCount(&self) -> u32;
    fn IsInputValid(&self, inputindex: u32) -> bool;
    fn IsOutputValid(&self, outputindex: u32) -> bool;
    fn GetInputEdgeDescription(&self, inputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription>;
    fn SetOutputEdgeDescription(&self, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMLOperatorTypeInferenceContext {}
impl IMLOperatorTypeInferenceContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>() -> IMLOperatorTypeInferenceContext_Vtbl {
        unsafe extern "system" fn GetInputCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInputCount()
        }
        unsafe extern "system" fn GetOutputCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutputCount()
        }
        unsafe extern "system" fn IsInputValid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsInputValid(::core::mem::transmute_copy(&inputindex))
        }
        unsafe extern "system" fn IsOutputValid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsOutputValid(::core::mem::transmute_copy(&outputindex))
        }
        unsafe extern "system" fn GetInputEdgeDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputEdgeDescription(::core::mem::transmute_copy(&inputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(edgedescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputEdgeDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutputEdgeDescription(::core::mem::transmute_copy(&outputindex), ::core::mem::transmute_copy(&edgedescription)).into()
        }
        Self {
            base__: IMLOperatorAttributes_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetInputCount: GetInputCount::<Identity, Impl, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, Impl, OFFSET>,
            IsInputValid: IsInputValid::<Identity, Impl, OFFSET>,
            IsOutputValid: IsOutputValid::<Identity, Impl, OFFSET>,
            GetInputEdgeDescription: GetInputEdgeDescription::<Identity, Impl, OFFSET>,
            SetOutputEdgeDescription: SetOutputEdgeDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorTypeInferenceContext as ::windows::core::ComInterface>::IID || iid == &<IMLOperatorAttributes as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"implement\"`*"]
pub trait IMLOperatorTypeInferrer_Impl: Sized {
    fn InferOutputTypes(&self, context: ::core::option::Option<&IMLOperatorTypeInferenceContext>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMLOperatorTypeInferrer {}
impl IMLOperatorTypeInferrer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTypeInferrer_Impl, const OFFSET: isize>() -> IMLOperatorTypeInferrer_Vtbl {
        unsafe extern "system" fn InferOutputTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTypeInferrer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InferOutputTypes(::windows::core::from_raw_borrowed(&context)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InferOutputTypes: InferOutputTypes::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLOperatorTypeInferrer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"Win32_Graphics_Direct3D12\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IWinMLEvaluationContext_Impl: Sized {
    fn BindValue(&self, pdescriptor: *const WINML_BINDING_DESC) -> ::windows::core::Result<()>;
    fn GetValueByName(&self, name: &::windows::core::PCWSTR) -> ::windows::core::Result<*mut WINML_BINDING_DESC>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::RuntimeName for IWinMLEvaluationContext {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IWinMLEvaluationContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinMLEvaluationContext_Impl, const OFFSET: isize>() -> IWinMLEvaluationContext_Vtbl {
        unsafe extern "system" fn BindValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinMLEvaluationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescriptor: *const WINML_BINDING_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindValue(::core::mem::transmute_copy(&pdescriptor)).into()
        }
        unsafe extern "system" fn GetValueByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinMLEvaluationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, pdescriptor: *mut *mut WINML_BINDING_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValueByName(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdescriptor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinMLEvaluationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BindValue: BindValue::<Identity, Impl, OFFSET>,
            GetValueByName: GetValueByName::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinMLEvaluationContext as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWinMLModel_Impl: Sized {
    fn GetDescription(&self) -> ::windows::core::Result<*mut WINML_MODEL_DESC>;
    fn EnumerateMetadata(&self, index: u32, pkey: *mut ::windows::core::PWSTR, pvalue: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn EnumerateModelInputs(&self, index: u32) -> ::windows::core::Result<*mut WINML_VARIABLE_DESC>;
    fn EnumerateModelOutputs(&self, index: u32) -> ::windows::core::Result<*mut WINML_VARIABLE_DESC>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWinMLModel {}
#[cfg(feature = "Win32_Foundation")]
impl IWinMLModel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinMLModel_Impl, const OFFSET: isize>() -> IWinMLModel_Vtbl {
        unsafe extern "system" fn GetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinMLModel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdescription: *mut *mut WINML_MODEL_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateMetadata<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinMLModel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pkey: *mut ::windows::core::PWSTR, pvalue: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumerateMetadata(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn EnumerateModelInputs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinMLModel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppinputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateModelInputs(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinputdescriptor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateModelOutputs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinMLModel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppoutputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateModelOutputs(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoutputdescriptor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            EnumerateMetadata: EnumerateMetadata::<Identity, Impl, OFFSET>,
            EnumerateModelInputs: EnumerateModelInputs::<Identity, Impl, OFFSET>,
            EnumerateModelOutputs: EnumerateModelOutputs::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinMLModel as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"Win32_Graphics_Direct3D12\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IWinMLRuntime_Impl: Sized {
    fn LoadModel(&self, path: &::windows::core::PCWSTR) -> ::windows::core::Result<IWinMLModel>;
    fn CreateEvaluationContext(&self, device: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Device>) -> ::windows::core::Result<IWinMLEvaluationContext>;
    fn EvaluateModel(&self, pcontext: ::core::option::Option<&IWinMLEvaluationContext>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::RuntimeName for IWinMLRuntime {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IWinMLRuntime_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinMLRuntime_Impl, const OFFSET: isize>() -> IWinMLRuntime_Vtbl {
        unsafe extern "system" fn LoadModel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinMLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, ppmodel: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadModel(::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmodel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEvaluationContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinMLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEvaluationContext(::windows::core::from_raw_borrowed(&device)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EvaluateModel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinMLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EvaluateModel(::windows::core::from_raw_borrowed(&pcontext)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadModel: LoadModel::<Identity, Impl, OFFSET>,
            CreateEvaluationContext: CreateEvaluationContext::<Identity, Impl, OFFSET>,
            EvaluateModel: EvaluateModel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinMLRuntime as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"implement\"`*"]
pub trait IWinMLRuntimeFactory_Impl: Sized {
    fn CreateRuntime(&self, runtimetype: WINML_RUNTIME_TYPE) -> ::windows::core::Result<IWinMLRuntime>;
}
impl ::windows::core::RuntimeName for IWinMLRuntimeFactory {}
impl IWinMLRuntimeFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinMLRuntimeFactory_Impl, const OFFSET: isize>() -> IWinMLRuntimeFactory_Vtbl {
        unsafe extern "system" fn CreateRuntime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinMLRuntimeFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runtimetype: WINML_RUNTIME_TYPE, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRuntime(::core::mem::transmute_copy(&runtimetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppruntime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateRuntime: CreateRuntime::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinMLRuntimeFactory as ::windows::core::ComInterface>::IID
    }
}
