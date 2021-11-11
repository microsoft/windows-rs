#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMLOperatorAttributes(pub ::windows::core::IUnknown);
impl IMLOperatorAttributes {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributeElementCount<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(r#type), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn GetAttribute<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(elementcount), ::core::mem::transmute(elementbytesize), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn GetStringAttributeElementLength<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(elementindex), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn GetStringAttributeElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(elementindex), ::core::mem::transmute(attributeelementbytesize), ::core::mem::transmute(attributeelement)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorAttributes {
    type Vtable = IMLOperatorAttributes_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b1b1759_ec40_466c_aab4_beb5347fd24c);
}
impl ::core::convert::From<IMLOperatorAttributes> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorAttributes) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMLOperatorAttributes> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorAttributes) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMLOperatorAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorAttributes_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMLOperatorKernel(pub ::windows::core::IUnknown);
impl IMLOperatorKernel {
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn Compute<'a, Param0: ::windows::core::IntoParam<'a, IMLOperatorKernelContext>>(&self, context: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), context.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorKernel {
    type Vtable = IMLOperatorKernel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11c4b4a0_b467_4eaa_a1a6_b961d8d0ed79);
}
impl ::core::convert::From<IMLOperatorKernel> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorKernel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMLOperatorKernel> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorKernel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorKernel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMLOperatorKernel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernel_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMLOperatorKernelContext(pub ::windows::core::IUnknown);
impl IMLOperatorKernelContext {
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetInputTensor(&self, inputindex: u32) -> ::windows::core::Result<IMLOperatorTensor> {
        let mut result__: <IMLOperatorTensor as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex), &mut result__).from_abi::<IMLOperatorTensor>(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetOutputTensor(&self, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32) -> ::windows::core::Result<IMLOperatorTensor> {
        let mut result__: <IMLOperatorTensor as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex), ::core::mem::transmute(dimensioncount), ::core::mem::transmute(dimensionsizes), &mut result__).from_abi::<IMLOperatorTensor>(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetOutputTensor2(&self, outputindex: u32) -> ::windows::core::Result<IMLOperatorTensor> {
        let mut result__: <IMLOperatorTensor as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex), &mut result__).from_abi::<IMLOperatorTensor>(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn AllocateTemporaryData(&self, size: usize) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(size), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetExecutionInterface(&self, executionobject: *mut ::core::option::Option<::windows::core::IUnknown>) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(executionobject)))
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorKernelContext {
    type Vtable = IMLOperatorKernelContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82536a28_f022_4769_9d3f_8b278f84c0c3);
}
impl ::core::convert::From<IMLOperatorKernelContext> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorKernelContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMLOperatorKernelContext> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorKernelContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorKernelContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMLOperatorKernelContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernelContext_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputindex: u32, tensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32, tensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputindex: u32, tensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, size: usize, data: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, executionobject: *mut ::windows::core::RawPtr),
);
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMLOperatorKernelCreationContext(pub ::windows::core::IUnknown);
impl IMLOperatorKernelCreationContext {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributeElementCount<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(r#type), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn GetAttribute<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(elementcount), ::core::mem::transmute(elementbytesize), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn GetStringAttributeElementLength<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(elementindex), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn GetStringAttributeElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(elementindex), ::core::mem::transmute(attributeelementbytesize), ::core::mem::transmute(attributeelement)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetInputCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetOutputCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn IsInputValid(&self, inputindex: u32) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn IsOutputValid(&self, outputindex: u32) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetInputEdgeDescription(&self, inputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription> {
        let mut result__: <MLOperatorEdgeDescription as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex), &mut result__).from_abi::<MLOperatorEdgeDescription>(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetOutputEdgeDescription(&self, outputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription> {
        let mut result__: <MLOperatorEdgeDescription as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex), &mut result__).from_abi::<MLOperatorEdgeDescription>(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn HasTensorShapeDescription(&self) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetTensorShapeDescription(&self) -> ::windows::core::Result<IMLOperatorTensorShapeDescription> {
        let mut result__: <IMLOperatorTensorShapeDescription as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMLOperatorTensorShapeDescription>(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetExecutionInterface(&self, executionobject: *mut ::core::option::Option<::windows::core::IUnknown>) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(executionobject)))
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorKernelCreationContext {
    type Vtable = IMLOperatorKernelCreationContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5459b53d_a0fc_4665_addd_70171ef7e631);
}
impl ::core::convert::From<IMLOperatorKernelCreationContext> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorKernelCreationContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMLOperatorKernelCreationContext> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorKernelCreationContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorKernelCreationContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMLOperatorKernelCreationContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMLOperatorKernelCreationContext> for IMLOperatorAttributes {
    fn from(value: IMLOperatorKernelCreationContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMLOperatorKernelCreationContext> for IMLOperatorAttributes {
    fn from(value: &IMLOperatorKernelCreationContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMLOperatorAttributes> for IMLOperatorKernelCreationContext {
    fn into_param(self) -> ::windows::core::Param<'a, IMLOperatorAttributes> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMLOperatorAttributes> for &IMLOperatorKernelCreationContext {
    fn into_param(self) -> ::windows::core::Param<'a, IMLOperatorAttributes> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernelCreationContext_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputindex: u32) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputindex: u32) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, shapedescription: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, executionobject: *mut ::windows::core::RawPtr),
);
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMLOperatorKernelFactory(pub ::windows::core::IUnknown);
impl IMLOperatorKernelFactory {
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn CreateKernel<'a, Param0: ::windows::core::IntoParam<'a, IMLOperatorKernelCreationContext>>(&self, context: Param0) -> ::windows::core::Result<IMLOperatorKernel> {
        let mut result__: <IMLOperatorKernel as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), context.into_param().abi(), &mut result__).from_abi::<IMLOperatorKernel>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorKernelFactory {
    type Vtable = IMLOperatorKernelFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef15ad6f_0dc9_4908_ab35_a575a30dfbf8);
}
impl ::core::convert::From<IMLOperatorKernelFactory> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorKernelFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMLOperatorKernelFactory> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorKernelFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorKernelFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMLOperatorKernelFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernelFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::windows::core::RawPtr, kernel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMLOperatorRegistry(pub ::windows::core::IUnknown);
impl IMLOperatorRegistry {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn RegisterOperatorSetSchema<'a, Param4: ::windows::core::IntoParam<'a, IMLOperatorTypeInferrer>, Param5: ::windows::core::IntoParam<'a, IMLOperatorShapeInferrer>>(&self, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: Param4, shapeinferrer: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(operatorsetid), ::core::mem::transmute(baselineversion), ::core::mem::transmute(schema), ::core::mem::transmute(schemacount), typeinferrer.into_param().abi(), shapeinferrer.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn RegisterOperatorKernel<'a, Param1: ::windows::core::IntoParam<'a, IMLOperatorKernelFactory>, Param2: ::windows::core::IntoParam<'a, IMLOperatorShapeInferrer>>(&self, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: Param1, shapeinferrer: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(operatorkernel), operatorkernelfactory.into_param().abi(), shapeinferrer.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorRegistry {
    type Vtable = IMLOperatorRegistry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2af9dd2d_b516_4672_9ab5_530c208493ad);
}
impl ::core::convert::From<IMLOperatorRegistry> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorRegistry) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMLOperatorRegistry> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorRegistry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorRegistry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMLOperatorRegistry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorRegistry_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: ::windows::core::RawPtr, shapeinferrer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: ::windows::core::RawPtr, shapeinferrer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMLOperatorShapeInferenceContext(pub ::windows::core::IUnknown);
impl IMLOperatorShapeInferenceContext {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributeElementCount<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(r#type), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn GetAttribute<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(elementcount), ::core::mem::transmute(elementbytesize), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn GetStringAttributeElementLength<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(elementindex), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn GetStringAttributeElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(elementindex), ::core::mem::transmute(attributeelementbytesize), ::core::mem::transmute(attributeelement)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetInputCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetOutputCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn IsInputValid(&self, inputindex: u32) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn IsOutputValid(&self, outputindex: u32) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetInputEdgeDescription(&self, inputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription> {
        let mut result__: <MLOperatorEdgeDescription as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex), &mut result__).from_abi::<MLOperatorEdgeDescription>(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetInputTensorDimensionCount(&self, inputindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetInputTensorShape(&self, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex), ::core::mem::transmute(dimensioncount), ::core::mem::transmute(dimensions)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn SetOutputTensorShape(&self, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex), ::core::mem::transmute(dimensioncount), ::core::mem::transmute(dimensions)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorShapeInferenceContext {
    type Vtable = IMLOperatorShapeInferenceContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x105b6b29_5408_4a68_9959_09b5955a3492);
}
impl ::core::convert::From<IMLOperatorShapeInferenceContext> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorShapeInferenceContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMLOperatorShapeInferenceContext> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorShapeInferenceContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorShapeInferenceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMLOperatorShapeInferenceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMLOperatorShapeInferenceContext> for IMLOperatorAttributes {
    fn from(value: IMLOperatorShapeInferenceContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMLOperatorShapeInferenceContext> for IMLOperatorAttributes {
    fn from(value: &IMLOperatorShapeInferenceContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMLOperatorAttributes> for IMLOperatorShapeInferenceContext {
    fn into_param(self) -> ::windows::core::Param<'a, IMLOperatorAttributes> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMLOperatorAttributes> for &IMLOperatorShapeInferenceContext {
    fn into_param(self) -> ::windows::core::Param<'a, IMLOperatorAttributes> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorShapeInferenceContext_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputindex: u32) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputindex: u32) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMLOperatorShapeInferrer(pub ::windows::core::IUnknown);
impl IMLOperatorShapeInferrer {
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn InferOutputShapes<'a, Param0: ::windows::core::IntoParam<'a, IMLOperatorShapeInferenceContext>>(&self, context: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), context.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorShapeInferrer {
    type Vtable = IMLOperatorShapeInferrer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x540be5be_a6c9_40ee_83f6_d2b8b40a7798);
}
impl ::core::convert::From<IMLOperatorShapeInferrer> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorShapeInferrer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMLOperatorShapeInferrer> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorShapeInferrer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorShapeInferrer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMLOperatorShapeInferrer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorShapeInferrer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMLOperatorTensor(pub ::windows::core::IUnknown);
impl IMLOperatorTensor {
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetDimensionCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetShape(&self, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dimensioncount), ::core::mem::transmute(dimensions)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetTensorDataType(&self) -> MLOperatorTensorDataType {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn IsCpuData(&self) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn IsDataInterface(&self) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetData(&self) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetDataInterface(&self, datainterface: *mut ::core::option::Option<::windows::core::IUnknown>) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(datainterface)))
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorTensor {
    type Vtable = IMLOperatorTensor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fe41f41_f430_440e_aece_54416dc8b9db);
}
impl ::core::convert::From<IMLOperatorTensor> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorTensor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMLOperatorTensor> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorTensor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorTensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMLOperatorTensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTensor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> MLOperatorTensorDataType,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> *mut ::core::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, datainterface: *mut ::windows::core::RawPtr),
);
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMLOperatorTensorShapeDescription(pub ::windows::core::IUnknown);
impl IMLOperatorTensorShapeDescription {
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetInputTensorDimensionCount(&self, inputindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetInputTensorShape(&self, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex), ::core::mem::transmute(dimensioncount), ::core::mem::transmute(dimensions)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn HasOutputShapeDescription(&self) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetOutputTensorDimensionCount(&self, outputindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetOutputTensorShape(&self, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex), ::core::mem::transmute(dimensioncount), ::core::mem::transmute(dimensions)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorTensorShapeDescription {
    type Vtable = IMLOperatorTensorShapeDescription_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf20e8cbe_3b28_4248_be95_f96fbc6e4643);
}
impl ::core::convert::From<IMLOperatorTensorShapeDescription> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorTensorShapeDescription) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMLOperatorTensorShapeDescription> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorTensorShapeDescription) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorTensorShapeDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMLOperatorTensorShapeDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTensorShapeDescription_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMLOperatorTypeInferenceContext(pub ::windows::core::IUnknown);
impl IMLOperatorTypeInferenceContext {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributeElementCount<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(r#type), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn GetAttribute<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(elementcount), ::core::mem::transmute(elementbytesize), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn GetStringAttributeElementLength<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(elementindex), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn GetStringAttributeElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(elementindex), ::core::mem::transmute(attributeelementbytesize), ::core::mem::transmute(attributeelement)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetInputCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetOutputCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn IsInputValid(&self, inputindex: u32) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn IsOutputValid(&self, outputindex: u32) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetInputEdgeDescription(&self, inputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription> {
        let mut result__: <MLOperatorEdgeDescription as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex), &mut result__).from_abi::<MLOperatorEdgeDescription>(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn SetOutputEdgeDescription(&self, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex), ::core::mem::transmute(edgedescription)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorTypeInferenceContext {
    type Vtable = IMLOperatorTypeInferenceContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec893bb1_f938_427b_8488_c8dcf775f138);
}
impl ::core::convert::From<IMLOperatorTypeInferenceContext> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorTypeInferenceContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMLOperatorTypeInferenceContext> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorTypeInferenceContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorTypeInferenceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMLOperatorTypeInferenceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMLOperatorTypeInferenceContext> for IMLOperatorAttributes {
    fn from(value: IMLOperatorTypeInferenceContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMLOperatorTypeInferenceContext> for IMLOperatorAttributes {
    fn from(value: &IMLOperatorTypeInferenceContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMLOperatorAttributes> for IMLOperatorTypeInferenceContext {
    fn into_param(self) -> ::windows::core::Param<'a, IMLOperatorAttributes> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMLOperatorAttributes> for &IMLOperatorTypeInferenceContext {
    fn into_param(self) -> ::windows::core::Param<'a, IMLOperatorAttributes> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTypeInferenceContext_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputindex: u32) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputindex: u32) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMLOperatorTypeInferrer(pub ::windows::core::IUnknown);
impl IMLOperatorTypeInferrer {
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn InferOutputTypes<'a, Param0: ::windows::core::IntoParam<'a, IMLOperatorTypeInferenceContext>>(&self, context: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), context.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorTypeInferrer {
    type Vtable = IMLOperatorTypeInferrer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x781aeb48_9bcb_4797_bf77_8bf455217beb);
}
impl ::core::convert::From<IMLOperatorTypeInferrer> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorTypeInferrer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMLOperatorTypeInferrer> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorTypeInferrer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorTypeInferrer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMLOperatorTypeInferrer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTypeInferrer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWinMLEvaluationContext(pub ::windows::core::IUnknown);
impl IWinMLEvaluationContext {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn BindValue(&self, pdescriptor: *const WINML_BINDING_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdescriptor)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn GetValueByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<*mut WINML_BINDING_DESC> {
        let mut result__: <*mut WINML_BINDING_DESC as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<*mut WINML_BINDING_DESC>(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWinMLEvaluationContext {
    type Vtable = IWinMLEvaluationContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95848f9e_583d_4054_af12_916387cd8426);
}
impl ::core::convert::From<IWinMLEvaluationContext> for ::windows::core::IUnknown {
    fn from(value: IWinMLEvaluationContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWinMLEvaluationContext> for ::windows::core::IUnknown {
    fn from(value: &IWinMLEvaluationContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWinMLEvaluationContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWinMLEvaluationContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLEvaluationContext_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdescriptor: *const ::core::mem::ManuallyDrop<WINML_BINDING_DESC>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PWSTR, pdescriptor: *mut *mut WINML_BINDING_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWinMLModel(pub ::windows::core::IUnknown);
impl IWinMLModel {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<*mut WINML_MODEL_DESC> {
        let mut result__: <*mut WINML_MODEL_DESC as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WINML_MODEL_DESC>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn EnumerateMetadata(&self, index: u32, pkey: *mut super::super::super::Foundation::PWSTR, pvalue: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(pkey), ::core::mem::transmute(pvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn EnumerateModelInputs(&self, index: u32) -> ::windows::core::Result<*mut WINML_VARIABLE_DESC> {
        let mut result__: <*mut WINML_VARIABLE_DESC as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<*mut WINML_VARIABLE_DESC>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn EnumerateModelOutputs(&self, index: u32) -> ::windows::core::Result<*mut WINML_VARIABLE_DESC> {
        let mut result__: <*mut WINML_VARIABLE_DESC as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<*mut WINML_VARIABLE_DESC>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWinMLModel {
    type Vtable = IWinMLModel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2eeb6a9_f31f_4055_a521_e30b5b33664a);
}
impl ::core::convert::From<IWinMLModel> for ::windows::core::IUnknown {
    fn from(value: IWinMLModel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWinMLModel> for ::windows::core::IUnknown {
    fn from(value: &IWinMLModel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWinMLModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWinMLModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLModel_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdescription: *mut *mut WINML_MODEL_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, pkey: *mut super::super::super::Foundation::PWSTR, pvalue: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, ppinputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, ppoutputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWinMLRuntime(pub ::windows::core::IUnknown);
impl IWinMLRuntime {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
    pub unsafe fn LoadModel<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, path: Param0) -> ::windows::core::Result<IWinMLModel> {
        let mut result__: <IWinMLModel as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<IWinMLModel>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CreateEvaluationContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>>(&self, device: Param0) -> ::windows::core::Result<IWinMLEvaluationContext> {
        let mut result__: <IWinMLEvaluationContext as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), device.into_param().abi(), &mut result__).from_abi::<IWinMLEvaluationContext>(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn EvaluateModel<'a, Param0: ::windows::core::IntoParam<'a, IWinMLEvaluationContext>>(&self, pcontext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pcontext.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IWinMLRuntime {
    type Vtable = IWinMLRuntime_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0425329_40ae_48d9_bce3_829ef7b8a41a);
}
impl ::core::convert::From<IWinMLRuntime> for ::windows::core::IUnknown {
    fn from(value: IWinMLRuntime) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWinMLRuntime> for ::windows::core::IUnknown {
    fn from(value: &IWinMLRuntime) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWinMLRuntime {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWinMLRuntime {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLRuntime_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: super::super::super::Foundation::PWSTR, ppmodel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, device: ::windows::core::RawPtr, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWinMLRuntimeFactory(pub ::windows::core::IUnknown);
impl IWinMLRuntimeFactory {
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn CreateRuntime(&self, runtimetype: WINML_RUNTIME_TYPE) -> ::windows::core::Result<IWinMLRuntime> {
        let mut result__: <IWinMLRuntime as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(runtimetype), &mut result__).from_abi::<IWinMLRuntime>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWinMLRuntimeFactory {
    type Vtable = IWinMLRuntimeFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa807b84d_4ae5_4bc0_a76a_941aa246bd41);
}
impl ::core::convert::From<IWinMLRuntimeFactory> for ::windows::core::IUnknown {
    fn from(value: IWinMLRuntimeFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWinMLRuntimeFactory> for ::windows::core::IUnknown {
    fn from(value: &IWinMLRuntimeFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWinMLRuntimeFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWinMLRuntimeFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLRuntimeFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, runtimetype: WINML_RUNTIME_TYPE, ppruntime: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[inline]
pub unsafe fn MLCreateOperatorRegistry() -> ::windows::core::Result<IMLOperatorRegistry> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MLCreateOperatorRegistry(registry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IMLOperatorRegistry as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        MLCreateOperatorRegistry(&mut result__).from_abi::<IMLOperatorRegistry>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
pub struct MLOperatorAttribute {
    pub name: super::super::super::Foundation::PSTR,
    pub r#type: MLOperatorAttributeType,
    pub required: bool,
}
#[cfg(feature = "Win32_Foundation")]
impl MLOperatorAttribute {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MLOperatorAttribute {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MLOperatorAttribute {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MLOperatorAttribute").field("name", &self.name).field("r#type", &self.r#type).field("required", &self.required).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MLOperatorAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.r#type == other.r#type && self.required == other.required
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MLOperatorAttribute {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MLOperatorAttribute {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
pub struct MLOperatorAttributeNameValue {
    pub name: super::super::super::Foundation::PSTR,
    pub r#type: MLOperatorAttributeType,
    pub valueCount: u32,
    pub Anonymous: MLOperatorAttributeNameValue_0,
}
#[cfg(feature = "Win32_Foundation")]
impl MLOperatorAttributeNameValue {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MLOperatorAttributeNameValue {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MLOperatorAttributeNameValue {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MLOperatorAttributeNameValue {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MLOperatorAttributeNameValue {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union MLOperatorAttributeNameValue_0 {
    pub reserved: *mut ::core::ffi::c_void,
    pub ints: *mut i64,
    pub strings: *mut *mut i8,
    pub floats: *mut f32,
}
#[cfg(feature = "Win32_Foundation")]
impl MLOperatorAttributeNameValue_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MLOperatorAttributeNameValue_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MLOperatorAttributeNameValue_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MLOperatorAttributeNameValue_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MLOperatorAttributeNameValue_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MLOperatorAttributeType(pub u32);
impl MLOperatorAttributeType {
    pub const Undefined: MLOperatorAttributeType = MLOperatorAttributeType(0u32);
    pub const Float: MLOperatorAttributeType = MLOperatorAttributeType(2u32);
    pub const Int: MLOperatorAttributeType = MLOperatorAttributeType(3u32);
    pub const String: MLOperatorAttributeType = MLOperatorAttributeType(4u32);
    pub const FloatArray: MLOperatorAttributeType = MLOperatorAttributeType(7u32);
    pub const IntArray: MLOperatorAttributeType = MLOperatorAttributeType(8u32);
    pub const StringArray: MLOperatorAttributeType = MLOperatorAttributeType(9u32);
}
impl ::core::convert::From<u32> for MLOperatorAttributeType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MLOperatorAttributeType {
    type Abi = Self;
}
impl ::core::ops::BitOr for MLOperatorAttributeType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for MLOperatorAttributeType {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for MLOperatorAttributeType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for MLOperatorAttributeType {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for MLOperatorAttributeType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
pub struct MLOperatorEdgeDescription {
    pub edgeType: MLOperatorEdgeType,
    pub Anonymous: MLOperatorEdgeDescription_0,
}
impl MLOperatorEdgeDescription {}
impl ::core::default::Default for MLOperatorEdgeDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MLOperatorEdgeDescription {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MLOperatorEdgeDescription {}
unsafe impl ::windows::core::Abi for MLOperatorEdgeDescription {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
pub union MLOperatorEdgeDescription_0 {
    pub reserved: u64,
    pub tensorDataType: MLOperatorTensorDataType,
}
impl MLOperatorEdgeDescription_0 {}
impl ::core::default::Default for MLOperatorEdgeDescription_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MLOperatorEdgeDescription_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MLOperatorEdgeDescription_0 {}
unsafe impl ::windows::core::Abi for MLOperatorEdgeDescription_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MLOperatorEdgeType(pub u32);
impl MLOperatorEdgeType {
    pub const Undefined: MLOperatorEdgeType = MLOperatorEdgeType(0u32);
    pub const Tensor: MLOperatorEdgeType = MLOperatorEdgeType(1u32);
}
impl ::core::convert::From<u32> for MLOperatorEdgeType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MLOperatorEdgeType {
    type Abi = Self;
}
impl ::core::ops::BitOr for MLOperatorEdgeType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for MLOperatorEdgeType {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for MLOperatorEdgeType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for MLOperatorEdgeType {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for MLOperatorEdgeType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
pub struct MLOperatorEdgeTypeConstraint {
    pub typeLabel: super::super::super::Foundation::PSTR,
    pub allowedTypes: *mut MLOperatorEdgeDescription,
    pub allowedTypeCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MLOperatorEdgeTypeConstraint {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MLOperatorEdgeTypeConstraint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MLOperatorEdgeTypeConstraint {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MLOperatorEdgeTypeConstraint").field("typeLabel", &self.typeLabel).field("allowedTypes", &self.allowedTypes).field("allowedTypeCount", &self.allowedTypeCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MLOperatorEdgeTypeConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.typeLabel == other.typeLabel && self.allowedTypes == other.allowedTypes && self.allowedTypeCount == other.allowedTypeCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MLOperatorEdgeTypeConstraint {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MLOperatorEdgeTypeConstraint {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MLOperatorExecutionType(pub u32);
impl MLOperatorExecutionType {
    pub const Undefined: MLOperatorExecutionType = MLOperatorExecutionType(0u32);
    pub const Cpu: MLOperatorExecutionType = MLOperatorExecutionType(1u32);
    pub const D3D12: MLOperatorExecutionType = MLOperatorExecutionType(2u32);
}
impl ::core::convert::From<u32> for MLOperatorExecutionType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MLOperatorExecutionType {
    type Abi = Self;
}
impl ::core::ops::BitOr for MLOperatorExecutionType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for MLOperatorExecutionType {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for MLOperatorExecutionType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for MLOperatorExecutionType {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for MLOperatorExecutionType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
pub struct MLOperatorKernelDescription {
    pub domain: super::super::super::Foundation::PSTR,
    pub name: super::super::super::Foundation::PSTR,
    pub minimumOperatorSetVersion: i32,
    pub executionType: MLOperatorExecutionType,
    pub typeConstraints: *mut MLOperatorEdgeTypeConstraint,
    pub typeConstraintCount: u32,
    pub defaultAttributes: *mut MLOperatorAttributeNameValue,
    pub defaultAttributeCount: u32,
    pub options: MLOperatorKernelOptions,
    pub executionOptions: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MLOperatorKernelDescription {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MLOperatorKernelDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MLOperatorKernelDescription {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MLOperatorKernelDescription")
            .field("domain", &self.domain)
            .field("name", &self.name)
            .field("minimumOperatorSetVersion", &self.minimumOperatorSetVersion)
            .field("executionType", &self.executionType)
            .field("typeConstraints", &self.typeConstraints)
            .field("typeConstraintCount", &self.typeConstraintCount)
            .field("defaultAttributes", &self.defaultAttributes)
            .field("defaultAttributeCount", &self.defaultAttributeCount)
            .field("options", &self.options)
            .field("executionOptions", &self.executionOptions)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MLOperatorKernelDescription {
    fn eq(&self, other: &Self) -> bool {
        self.domain == other.domain && self.name == other.name && self.minimumOperatorSetVersion == other.minimumOperatorSetVersion && self.executionType == other.executionType && self.typeConstraints == other.typeConstraints && self.typeConstraintCount == other.typeConstraintCount && self.defaultAttributes == other.defaultAttributes && self.defaultAttributeCount == other.defaultAttributeCount && self.options == other.options && self.executionOptions == other.executionOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MLOperatorKernelDescription {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MLOperatorKernelDescription {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MLOperatorKernelOptions(pub u32);
impl MLOperatorKernelOptions {
    pub const None: MLOperatorKernelOptions = MLOperatorKernelOptions(0u32);
    pub const AllowDynamicInputShapes: MLOperatorKernelOptions = MLOperatorKernelOptions(1u32);
}
impl ::core::convert::From<u32> for MLOperatorKernelOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MLOperatorKernelOptions {
    type Abi = Self;
}
impl ::core::ops::BitOr for MLOperatorKernelOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for MLOperatorKernelOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for MLOperatorKernelOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for MLOperatorKernelOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for MLOperatorKernelOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MLOperatorParameterOptions(pub u32);
impl MLOperatorParameterOptions {
    pub const Single: MLOperatorParameterOptions = MLOperatorParameterOptions(0u32);
    pub const Optional: MLOperatorParameterOptions = MLOperatorParameterOptions(1u32);
    pub const Variadic: MLOperatorParameterOptions = MLOperatorParameterOptions(2u32);
}
impl ::core::convert::From<u32> for MLOperatorParameterOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MLOperatorParameterOptions {
    type Abi = Self;
}
impl ::core::ops::BitOr for MLOperatorParameterOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for MLOperatorParameterOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for MLOperatorParameterOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for MLOperatorParameterOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for MLOperatorParameterOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
pub struct MLOperatorSchemaDescription {
    pub name: super::super::super::Foundation::PSTR,
    pub operatorSetVersionAtLastChange: i32,
    pub inputs: *mut MLOperatorSchemaEdgeDescription,
    pub inputCount: u32,
    pub outputs: *mut MLOperatorSchemaEdgeDescription,
    pub outputCount: u32,
    pub typeConstraints: *mut MLOperatorEdgeTypeConstraint,
    pub typeConstraintCount: u32,
    pub attributes: *mut MLOperatorAttribute,
    pub attributeCount: u32,
    pub defaultAttributes: *mut MLOperatorAttributeNameValue,
    pub defaultAttributeCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MLOperatorSchemaDescription {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MLOperatorSchemaDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MLOperatorSchemaDescription {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MLOperatorSchemaDescription")
            .field("name", &self.name)
            .field("operatorSetVersionAtLastChange", &self.operatorSetVersionAtLastChange)
            .field("inputs", &self.inputs)
            .field("inputCount", &self.inputCount)
            .field("outputs", &self.outputs)
            .field("outputCount", &self.outputCount)
            .field("typeConstraints", &self.typeConstraints)
            .field("typeConstraintCount", &self.typeConstraintCount)
            .field("attributes", &self.attributes)
            .field("attributeCount", &self.attributeCount)
            .field("defaultAttributes", &self.defaultAttributes)
            .field("defaultAttributeCount", &self.defaultAttributeCount)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MLOperatorSchemaDescription {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.operatorSetVersionAtLastChange == other.operatorSetVersionAtLastChange
            && self.inputs == other.inputs
            && self.inputCount == other.inputCount
            && self.outputs == other.outputs
            && self.outputCount == other.outputCount
            && self.typeConstraints == other.typeConstraints
            && self.typeConstraintCount == other.typeConstraintCount
            && self.attributes == other.attributes
            && self.attributeCount == other.attributeCount
            && self.defaultAttributes == other.defaultAttributes
            && self.defaultAttributeCount == other.defaultAttributeCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MLOperatorSchemaDescription {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MLOperatorSchemaDescription {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
pub struct MLOperatorSchemaEdgeDescription {
    pub options: MLOperatorParameterOptions,
    pub typeFormat: MLOperatorSchemaEdgeTypeFormat,
    pub Anonymous: MLOperatorSchemaEdgeDescription_0,
}
#[cfg(feature = "Win32_Foundation")]
impl MLOperatorSchemaEdgeDescription {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MLOperatorSchemaEdgeDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MLOperatorSchemaEdgeDescription {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MLOperatorSchemaEdgeDescription {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MLOperatorSchemaEdgeDescription {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union MLOperatorSchemaEdgeDescription_0 {
    pub reserved: *mut ::core::ffi::c_void,
    pub typeLabel: super::super::super::Foundation::PSTR,
    pub edgeDescription: MLOperatorEdgeDescription,
}
#[cfg(feature = "Win32_Foundation")]
impl MLOperatorSchemaEdgeDescription_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MLOperatorSchemaEdgeDescription_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MLOperatorSchemaEdgeDescription_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MLOperatorSchemaEdgeDescription_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MLOperatorSchemaEdgeDescription_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MLOperatorSchemaEdgeTypeFormat(pub i32);
impl MLOperatorSchemaEdgeTypeFormat {
    pub const EdgeDescription: MLOperatorSchemaEdgeTypeFormat = MLOperatorSchemaEdgeTypeFormat(0i32);
    pub const Label: MLOperatorSchemaEdgeTypeFormat = MLOperatorSchemaEdgeTypeFormat(1i32);
}
impl ::core::convert::From<i32> for MLOperatorSchemaEdgeTypeFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MLOperatorSchemaEdgeTypeFormat {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
pub struct MLOperatorSetId {
    pub domain: super::super::super::Foundation::PSTR,
    pub version: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl MLOperatorSetId {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MLOperatorSetId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MLOperatorSetId {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MLOperatorSetId").field("domain", &self.domain).field("version", &self.version).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MLOperatorSetId {
    fn eq(&self, other: &Self) -> bool {
        self.domain == other.domain && self.version == other.version
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MLOperatorSetId {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MLOperatorSetId {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MLOperatorTensorDataType(pub u32);
impl MLOperatorTensorDataType {
    pub const Undefined: MLOperatorTensorDataType = MLOperatorTensorDataType(0u32);
    pub const Float: MLOperatorTensorDataType = MLOperatorTensorDataType(1u32);
    pub const UInt8: MLOperatorTensorDataType = MLOperatorTensorDataType(2u32);
    pub const Int8: MLOperatorTensorDataType = MLOperatorTensorDataType(3u32);
    pub const UInt16: MLOperatorTensorDataType = MLOperatorTensorDataType(4u32);
    pub const Int16: MLOperatorTensorDataType = MLOperatorTensorDataType(5u32);
    pub const Int32: MLOperatorTensorDataType = MLOperatorTensorDataType(6u32);
    pub const Int64: MLOperatorTensorDataType = MLOperatorTensorDataType(7u32);
    pub const String: MLOperatorTensorDataType = MLOperatorTensorDataType(8u32);
    pub const Bool: MLOperatorTensorDataType = MLOperatorTensorDataType(9u32);
    pub const Float16: MLOperatorTensorDataType = MLOperatorTensorDataType(10u32);
    pub const Double: MLOperatorTensorDataType = MLOperatorTensorDataType(11u32);
    pub const UInt32: MLOperatorTensorDataType = MLOperatorTensorDataType(12u32);
    pub const UInt64: MLOperatorTensorDataType = MLOperatorTensorDataType(13u32);
    pub const Complex64: MLOperatorTensorDataType = MLOperatorTensorDataType(14u32);
    pub const Complex128: MLOperatorTensorDataType = MLOperatorTensorDataType(15u32);
}
impl ::core::convert::From<u32> for MLOperatorTensorDataType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MLOperatorTensorDataType {
    type Abi = Self;
}
impl ::core::ops::BitOr for MLOperatorTensorDataType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for MLOperatorTensorDataType {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for MLOperatorTensorDataType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for MLOperatorTensorDataType {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for MLOperatorTensorDataType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::clone::Clone for WINML_BINDING_DESC {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`, `Win32_Graphics_Direct3D12`*"]
pub struct WINML_BINDING_DESC {
    pub Name: super::super::super::Foundation::PWSTR,
    pub BindType: WINML_BINDING_TYPE,
    pub Anonymous: WINML_BINDING_DESC_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl WINML_BINDING_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::default::Default for WINML_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::cmp::PartialEq for WINML_BINDING_DESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::cmp::Eq for WINML_BINDING_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
unsafe impl ::windows::core::Abi for WINML_BINDING_DESC {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::clone::Clone for WINML_BINDING_DESC_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub union WINML_BINDING_DESC_0 {
    pub Tensor: WINML_TENSOR_BINDING_DESC,
    pub Sequence: WINML_SEQUENCE_BINDING_DESC,
    pub Map: WINML_MAP_BINDING_DESC,
    pub Image: WINML_IMAGE_BINDING_DESC,
    pub Resource: ::core::mem::ManuallyDrop<WINML_RESOURCE_BINDING_DESC>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl WINML_BINDING_DESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::default::Default for WINML_BINDING_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::cmp::PartialEq for WINML_BINDING_DESC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::cmp::Eq for WINML_BINDING_DESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
unsafe impl ::windows::core::Abi for WINML_BINDING_DESC_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINML_BINDING_TYPE(pub i32);
pub const WINML_BINDING_UNDEFINED: WINML_BINDING_TYPE = WINML_BINDING_TYPE(0i32);
pub const WINML_BINDING_TENSOR: WINML_BINDING_TYPE = WINML_BINDING_TYPE(1i32);
pub const WINML_BINDING_SEQUENCE: WINML_BINDING_TYPE = WINML_BINDING_TYPE(2i32);
pub const WINML_BINDING_MAP: WINML_BINDING_TYPE = WINML_BINDING_TYPE(3i32);
pub const WINML_BINDING_IMAGE: WINML_BINDING_TYPE = WINML_BINDING_TYPE(4i32);
pub const WINML_BINDING_RESOURCE: WINML_BINDING_TYPE = WINML_BINDING_TYPE(5i32);
impl ::core::convert::From<i32> for WINML_BINDING_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WINML_BINDING_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINML_FEATURE_TYPE(pub i32);
pub const WINML_FEATURE_UNDEFINED: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(0i32);
pub const WINML_FEATURE_TENSOR: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(1i32);
pub const WINML_FEATURE_SEQUENCE: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(2i32);
pub const WINML_FEATURE_MAP: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(3i32);
pub const WINML_FEATURE_IMAGE: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(4i32);
impl ::core::convert::From<i32> for WINML_FEATURE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WINML_FEATURE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
pub struct WINML_IMAGE_BINDING_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub DataSize: u32,
    pub pData: *mut ::core::ffi::c_void,
}
impl WINML_IMAGE_BINDING_DESC {}
impl ::core::default::Default for WINML_IMAGE_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WINML_IMAGE_BINDING_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WINML_IMAGE_BINDING_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).field("DataSize", &self.DataSize).field("pData", &self.pData).finish()
    }
}
impl ::core::cmp::PartialEq for WINML_IMAGE_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape && self.DataSize == other.DataSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for WINML_IMAGE_BINDING_DESC {}
unsafe impl ::windows::core::Abi for WINML_IMAGE_BINDING_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
pub struct WINML_IMAGE_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
}
impl WINML_IMAGE_VARIABLE_DESC {}
impl ::core::default::Default for WINML_IMAGE_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WINML_IMAGE_VARIABLE_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WINML_IMAGE_VARIABLE_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).finish()
    }
}
impl ::core::cmp::PartialEq for WINML_IMAGE_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape
    }
}
impl ::core::cmp::Eq for WINML_IMAGE_VARIABLE_DESC {}
unsafe impl ::windows::core::Abi for WINML_IMAGE_VARIABLE_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
pub struct WINML_MAP_BINDING_DESC {
    pub ElementCount: u32,
    pub KeyType: WINML_TENSOR_DATA_TYPE,
    pub Anonymous1: WINML_MAP_BINDING_DESC_0,
    pub Fields: WINML_TENSOR_DATA_TYPE,
    pub Anonymous2: WINML_MAP_BINDING_DESC_1,
}
#[cfg(feature = "Win32_Foundation")]
impl WINML_MAP_BINDING_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_MAP_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINML_MAP_BINDING_DESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINML_MAP_BINDING_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINML_MAP_BINDING_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WINML_MAP_BINDING_DESC_0 {
    pub pStringKeys: *mut super::super::super::Foundation::PWSTR,
    pub pIntKeys: *mut i64,
}
#[cfg(feature = "Win32_Foundation")]
impl WINML_MAP_BINDING_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_MAP_BINDING_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINML_MAP_BINDING_DESC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINML_MAP_BINDING_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINML_MAP_BINDING_DESC_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WINML_MAP_BINDING_DESC_1 {
    pub pStringFields: *mut super::super::super::Foundation::PWSTR,
    pub pIntFields: *mut i64,
    pub pFloatFields: *mut f32,
    pub pDoubleFields: *mut f64,
}
#[cfg(feature = "Win32_Foundation")]
impl WINML_MAP_BINDING_DESC_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_MAP_BINDING_DESC_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINML_MAP_BINDING_DESC_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINML_MAP_BINDING_DESC_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINML_MAP_BINDING_DESC_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
pub struct WINML_MAP_VARIABLE_DESC {
    pub KeyType: WINML_TENSOR_DATA_TYPE,
    pub Fields: WINML_TENSOR_DATA_TYPE,
}
impl WINML_MAP_VARIABLE_DESC {}
impl ::core::default::Default for WINML_MAP_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WINML_MAP_VARIABLE_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WINML_MAP_VARIABLE_DESC").field("KeyType", &self.KeyType).field("Fields", &self.Fields).finish()
    }
}
impl ::core::cmp::PartialEq for WINML_MAP_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.KeyType == other.KeyType && self.Fields == other.Fields
    }
}
impl ::core::cmp::Eq for WINML_MAP_VARIABLE_DESC {}
unsafe impl ::windows::core::Abi for WINML_MAP_VARIABLE_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
pub struct WINML_MODEL_DESC {
    pub Author: super::super::super::Foundation::PWSTR,
    pub Name: super::super::super::Foundation::PWSTR,
    pub Domain: super::super::super::Foundation::PWSTR,
    pub Description: super::super::super::Foundation::PWSTR,
    pub Version: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl WINML_MODEL_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_MODEL_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINML_MODEL_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WINML_MODEL_DESC").field("Author", &self.Author).field("Name", &self.Name).field("Domain", &self.Domain).field("Description", &self.Description).field("Version", &self.Version).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINML_MODEL_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Author == other.Author && self.Name == other.Name && self.Domain == other.Domain && self.Description == other.Description && self.Version == other.Version
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINML_MODEL_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINML_MODEL_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Graphics_Direct3D12`*"]
pub struct WINML_RESOURCE_BINDING_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub pResource: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl WINML_RESOURCE_BINDING_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for WINML_RESOURCE_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for WINML_RESOURCE_BINDING_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WINML_RESOURCE_BINDING_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).field("pResource", &self.pResource).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for WINML_RESOURCE_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape && self.pResource == other.pResource
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for WINML_RESOURCE_BINDING_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
unsafe impl ::windows::core::Abi for WINML_RESOURCE_BINDING_DESC {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINML_RUNTIME_TYPE(pub i32);
pub const WINML_RUNTIME_CNTK: WINML_RUNTIME_TYPE = WINML_RUNTIME_TYPE(0i32);
impl ::core::convert::From<i32> for WINML_RUNTIME_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WINML_RUNTIME_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
pub struct WINML_SEQUENCE_BINDING_DESC {
    pub ElementCount: u32,
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub Anonymous: WINML_SEQUENCE_BINDING_DESC_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WINML_SEQUENCE_BINDING_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_SEQUENCE_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINML_SEQUENCE_BINDING_DESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINML_SEQUENCE_BINDING_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINML_SEQUENCE_BINDING_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WINML_SEQUENCE_BINDING_DESC_0 {
    pub pStrings: *mut super::super::super::Foundation::PWSTR,
    pub pInts: *mut i64,
    pub pFloats: *mut f32,
    pub pDoubles: *mut f64,
}
#[cfg(feature = "Win32_Foundation")]
impl WINML_SEQUENCE_BINDING_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_SEQUENCE_BINDING_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINML_SEQUENCE_BINDING_DESC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINML_SEQUENCE_BINDING_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINML_SEQUENCE_BINDING_DESC_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
pub struct WINML_SEQUENCE_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
}
impl WINML_SEQUENCE_VARIABLE_DESC {}
impl ::core::default::Default for WINML_SEQUENCE_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WINML_SEQUENCE_VARIABLE_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WINML_SEQUENCE_VARIABLE_DESC").field("ElementType", &self.ElementType).finish()
    }
}
impl ::core::cmp::PartialEq for WINML_SEQUENCE_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType
    }
}
impl ::core::cmp::Eq for WINML_SEQUENCE_VARIABLE_DESC {}
unsafe impl ::windows::core::Abi for WINML_SEQUENCE_VARIABLE_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
pub struct WINML_TENSOR_BINDING_DESC {
    pub DataType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub DataSize: u32,
    pub pData: *mut ::core::ffi::c_void,
}
impl WINML_TENSOR_BINDING_DESC {}
impl ::core::default::Default for WINML_TENSOR_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WINML_TENSOR_BINDING_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WINML_TENSOR_BINDING_DESC").field("DataType", &self.DataType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).field("DataSize", &self.DataSize).field("pData", &self.pData).finish()
    }
}
impl ::core::cmp::PartialEq for WINML_TENSOR_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.DataType == other.DataType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape && self.DataSize == other.DataSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for WINML_TENSOR_BINDING_DESC {}
unsafe impl ::windows::core::Abi for WINML_TENSOR_BINDING_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINML_TENSOR_DATA_TYPE(pub i32);
pub const WINML_TENSOR_UNDEFINED: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(0i32);
pub const WINML_TENSOR_FLOAT: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(1i32);
pub const WINML_TENSOR_UINT8: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(2i32);
pub const WINML_TENSOR_INT8: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(3i32);
pub const WINML_TENSOR_UINT16: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(4i32);
pub const WINML_TENSOR_INT16: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(5i32);
pub const WINML_TENSOR_INT32: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(6i32);
pub const WINML_TENSOR_INT64: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(7i32);
pub const WINML_TENSOR_STRING: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(8i32);
pub const WINML_TENSOR_BOOLEAN: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(9i32);
pub const WINML_TENSOR_FLOAT16: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(10i32);
pub const WINML_TENSOR_DOUBLE: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(11i32);
pub const WINML_TENSOR_UINT32: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(12i32);
pub const WINML_TENSOR_UINT64: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(13i32);
pub const WINML_TENSOR_COMPLEX64: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(14i32);
pub const WINML_TENSOR_COMPLEX128: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(15i32);
impl ::core::convert::From<i32> for WINML_TENSOR_DATA_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WINML_TENSOR_DATA_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
pub const WINML_TENSOR_DIMENSION_COUNT_MAX: u32 = 4u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
pub struct WINML_TENSOR_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
}
impl WINML_TENSOR_VARIABLE_DESC {}
impl ::core::default::Default for WINML_TENSOR_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WINML_TENSOR_VARIABLE_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WINML_TENSOR_VARIABLE_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).finish()
    }
}
impl ::core::cmp::PartialEq for WINML_TENSOR_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape
    }
}
impl ::core::cmp::Eq for WINML_TENSOR_VARIABLE_DESC {}
unsafe impl ::windows::core::Abi for WINML_TENSOR_VARIABLE_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`, `Win32_Foundation`*"]
pub struct WINML_VARIABLE_DESC {
    pub Name: super::super::super::Foundation::PWSTR,
    pub Description: super::super::super::Foundation::PWSTR,
    pub FeatureType: WINML_FEATURE_TYPE,
    pub Required: super::super::super::Foundation::BOOL,
    pub Anonymous: WINML_VARIABLE_DESC_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WINML_VARIABLE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINML_VARIABLE_DESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINML_VARIABLE_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINML_VARIABLE_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WINML_VARIABLE_DESC_0 {
    pub Tensor: WINML_TENSOR_VARIABLE_DESC,
    pub Sequence: WINML_SEQUENCE_VARIABLE_DESC,
    pub Map: WINML_MAP_VARIABLE_DESC,
    pub Image: WINML_IMAGE_VARIABLE_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl WINML_VARIABLE_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_VARIABLE_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINML_VARIABLE_DESC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINML_VARIABLE_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINML_VARIABLE_DESC_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
#[inline]
pub unsafe fn WinMLCreateRuntime() -> ::windows::core::Result<IWinMLRuntime> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinMLCreateRuntime(runtime: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IWinMLRuntime as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WinMLCreateRuntime(&mut result__).from_abi::<IWinMLRuntime>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
