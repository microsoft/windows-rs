#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct IMLOperatorAttributes(::windows::core::IUnknown);
impl IMLOperatorAttributes {
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeElementCount<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttribute<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(elementcount), ::core::mem::transmute(elementbytesize), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringAttributeElementLength<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(elementindex), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringAttributeElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(elementindex), ::core::mem::transmute(attributeelementbytesize), ::core::mem::transmute(attributeelement)).ok()
    }
}
impl ::core::convert::From<IMLOperatorAttributes> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMLOperatorAttributes> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMLOperatorAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMLOperatorAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMLOperatorAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorAttributes {}
impl ::core::fmt::Debug for IMLOperatorAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorAttributes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorAttributes {
    type Vtable = IMLOperatorAttributesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b1b1759_ec40_466c_aab4_beb5347fd24c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorAttributesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct IMLOperatorKernel(::windows::core::IUnknown);
impl IMLOperatorKernel {
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn Compute<'a, Param0: ::windows::core::IntoParam<'a, IMLOperatorKernelContext>>(&self, context: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), context.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMLOperatorKernel> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorKernel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMLOperatorKernel> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorKernel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorKernel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMLOperatorKernel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMLOperatorKernel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMLOperatorKernel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorKernel {}
impl ::core::fmt::Debug for IMLOperatorKernel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorKernel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorKernel {
    type Vtable = IMLOperatorKernelVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11c4b4a0_b467_4eaa_a1a6_b961d8d0ed79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernelVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct IMLOperatorKernelContext(::windows::core::IUnknown);
impl IMLOperatorKernelContext {
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetInputTensor(&self, inputindex: u32) -> ::windows::core::Result<IMLOperatorTensor> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex), ::core::mem::transmute(&mut result__)).from_abi::<IMLOperatorTensor>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetOutputTensor(&self, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32) -> ::windows::core::Result<IMLOperatorTensor> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex), ::core::mem::transmute(dimensioncount), ::core::mem::transmute(dimensionsizes), ::core::mem::transmute(&mut result__)).from_abi::<IMLOperatorTensor>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetOutputTensor2(&self, outputindex: u32) -> ::windows::core::Result<IMLOperatorTensor> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex), ::core::mem::transmute(&mut result__)).from_abi::<IMLOperatorTensor>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn AllocateTemporaryData(&self, size: usize) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(size), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetExecutionInterface(&self, executionobject: *mut ::core::option::Option<::windows::core::IUnknown>) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(executionobject))
    }
}
impl ::core::convert::From<IMLOperatorKernelContext> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorKernelContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMLOperatorKernelContext> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorKernelContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorKernelContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMLOperatorKernelContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMLOperatorKernelContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMLOperatorKernelContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorKernelContext {}
impl ::core::fmt::Debug for IMLOperatorKernelContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorKernelContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorKernelContext {
    type Vtable = IMLOperatorKernelContextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82536a28_f022_4769_9d3f_8b278f84c0c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernelContextVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32, tensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32, tensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32, tensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: usize, data: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executionobject: *mut *mut ::core::ffi::c_void),
);
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct IMLOperatorKernelCreationContext(::windows::core::IUnknown);
impl IMLOperatorKernelCreationContext {
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeElementCount<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttribute<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(elementcount), ::core::mem::transmute(elementbytesize), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringAttributeElementLength<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(elementindex), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringAttributeElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(elementindex), ::core::mem::transmute(attributeelementbytesize), ::core::mem::transmute(attributeelement)).ok()
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetInputCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetOutputCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn IsInputValid(&self, inputindex: u32) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn IsOutputValid(&self, outputindex: u32) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetInputEdgeDescription(&self, inputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription> {
        let mut result__: MLOperatorEdgeDescription = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex), ::core::mem::transmute(&mut result__)).from_abi::<MLOperatorEdgeDescription>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetOutputEdgeDescription(&self, outputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription> {
        let mut result__: MLOperatorEdgeDescription = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex), ::core::mem::transmute(&mut result__)).from_abi::<MLOperatorEdgeDescription>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn HasTensorShapeDescription(&self) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetTensorShapeDescription(&self) -> ::windows::core::Result<IMLOperatorTensorShapeDescription> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMLOperatorTensorShapeDescription>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetExecutionInterface(&self, executionobject: *mut ::core::option::Option<::windows::core::IUnknown>) {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(executionobject))
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
impl ::core::convert::From<IMLOperatorKernelCreationContext> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorKernelCreationContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMLOperatorKernelCreationContext> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorKernelCreationContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorKernelCreationContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMLOperatorKernelCreationContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMLOperatorKernelCreationContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMLOperatorKernelCreationContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorKernelCreationContext {}
impl ::core::fmt::Debug for IMLOperatorKernelCreationContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorKernelCreationContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorKernelCreationContext {
    type Vtable = IMLOperatorKernelCreationContextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5459b53d_a0fc_4665_addd_70171ef7e631);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernelCreationContextVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> bool,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shapedescription: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executionobject: *mut *mut ::core::ffi::c_void),
);
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct IMLOperatorKernelFactory(::windows::core::IUnknown);
impl IMLOperatorKernelFactory {
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn CreateKernel<'a, Param0: ::windows::core::IntoParam<'a, IMLOperatorKernelCreationContext>>(&self, context: Param0) -> ::windows::core::Result<IMLOperatorKernel> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), context.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMLOperatorKernel>(result__)
    }
}
impl ::core::convert::From<IMLOperatorKernelFactory> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorKernelFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMLOperatorKernelFactory> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorKernelFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorKernelFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMLOperatorKernelFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMLOperatorKernelFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMLOperatorKernelFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorKernelFactory {}
impl ::core::fmt::Debug for IMLOperatorKernelFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorKernelFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorKernelFactory {
    type Vtable = IMLOperatorKernelFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef15ad6f_0dc9_4908_ab35_a575a30dfbf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernelFactoryVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, kernel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct IMLOperatorRegistry(::windows::core::IUnknown);
impl IMLOperatorRegistry {
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOperatorSetSchema<'a, Param4: ::windows::core::IntoParam<'a, IMLOperatorTypeInferrer>, Param5: ::windows::core::IntoParam<'a, IMLOperatorShapeInferrer>>(&self, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: Param4, shapeinferrer: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(operatorsetid), ::core::mem::transmute(baselineversion), ::core::mem::transmute(schema), ::core::mem::transmute(schemacount), typeinferrer.into_param().abi(), shapeinferrer.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOperatorKernel<'a, Param1: ::windows::core::IntoParam<'a, IMLOperatorKernelFactory>, Param2: ::windows::core::IntoParam<'a, IMLOperatorShapeInferrer>>(&self, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: Param1, shapeinferrer: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(operatorkernel), operatorkernelfactory.into_param().abi(), shapeinferrer.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMLOperatorRegistry> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorRegistry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMLOperatorRegistry> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorRegistry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorRegistry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMLOperatorRegistry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMLOperatorRegistry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMLOperatorRegistry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorRegistry {}
impl ::core::fmt::Debug for IMLOperatorRegistry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorRegistry").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorRegistry {
    type Vtable = IMLOperatorRegistryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2af9dd2d_b516_4672_9ab5_530c208493ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorRegistryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: ::windows::core::RawPtr, shapeinferrer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: ::windows::core::RawPtr, shapeinferrer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct IMLOperatorShapeInferenceContext(::windows::core::IUnknown);
impl IMLOperatorShapeInferenceContext {
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeElementCount<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttribute<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(elementcount), ::core::mem::transmute(elementbytesize), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringAttributeElementLength<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(elementindex), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringAttributeElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(elementindex), ::core::mem::transmute(attributeelementbytesize), ::core::mem::transmute(attributeelement)).ok()
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetInputCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetOutputCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn IsInputValid(&self, inputindex: u32) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn IsOutputValid(&self, outputindex: u32) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetInputEdgeDescription(&self, inputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription> {
        let mut result__: MLOperatorEdgeDescription = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex), ::core::mem::transmute(&mut result__)).from_abi::<MLOperatorEdgeDescription>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetInputTensorDimensionCount(&self, inputindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetInputTensorShape(&self, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex), ::core::mem::transmute(dimensioncount), ::core::mem::transmute(dimensions)).ok()
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn SetOutputTensorShape(&self, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex), ::core::mem::transmute(dimensioncount), ::core::mem::transmute(dimensions)).ok()
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
impl ::core::convert::From<IMLOperatorShapeInferenceContext> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorShapeInferenceContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMLOperatorShapeInferenceContext> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorShapeInferenceContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorShapeInferenceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMLOperatorShapeInferenceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMLOperatorShapeInferenceContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMLOperatorShapeInferenceContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorShapeInferenceContext {}
impl ::core::fmt::Debug for IMLOperatorShapeInferenceContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorShapeInferenceContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorShapeInferenceContext {
    type Vtable = IMLOperatorShapeInferenceContextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x105b6b29_5408_4a68_9959_09b5955a3492);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorShapeInferenceContextVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct IMLOperatorShapeInferrer(::windows::core::IUnknown);
impl IMLOperatorShapeInferrer {
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn InferOutputShapes<'a, Param0: ::windows::core::IntoParam<'a, IMLOperatorShapeInferenceContext>>(&self, context: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), context.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMLOperatorShapeInferrer> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorShapeInferrer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMLOperatorShapeInferrer> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorShapeInferrer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorShapeInferrer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMLOperatorShapeInferrer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMLOperatorShapeInferrer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMLOperatorShapeInferrer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorShapeInferrer {}
impl ::core::fmt::Debug for IMLOperatorShapeInferrer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorShapeInferrer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorShapeInferrer {
    type Vtable = IMLOperatorShapeInferrerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x540be5be_a6c9_40ee_83f6_d2b8b40a7798);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorShapeInferrerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct IMLOperatorTensor(::windows::core::IUnknown);
impl IMLOperatorTensor {
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetDimensionCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetShape(&self, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dimensioncount), ::core::mem::transmute(dimensions)).ok()
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetTensorDataType(&self) -> MLOperatorTensorDataType {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn IsCpuData(&self) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn IsDataInterface(&self) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetData(&self) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetDataInterface(&self, datainterface: *mut ::core::option::Option<::windows::core::IUnknown>) {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(datainterface))
    }
}
impl ::core::convert::From<IMLOperatorTensor> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorTensor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMLOperatorTensor> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorTensor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorTensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMLOperatorTensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMLOperatorTensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMLOperatorTensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorTensor {}
impl ::core::fmt::Debug for IMLOperatorTensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorTensor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorTensor {
    type Vtable = IMLOperatorTensorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fe41f41_f430_440e_aece_54416dc8b9db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTensorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> MLOperatorTensorDataType,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> bool,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> bool,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datainterface: *mut *mut ::core::ffi::c_void),
);
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct IMLOperatorTensorShapeDescription(::windows::core::IUnknown);
impl IMLOperatorTensorShapeDescription {
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetInputTensorDimensionCount(&self, inputindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetInputTensorShape(&self, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex), ::core::mem::transmute(dimensioncount), ::core::mem::transmute(dimensions)).ok()
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn HasOutputShapeDescription(&self) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetOutputTensorDimensionCount(&self, outputindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetOutputTensorShape(&self, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex), ::core::mem::transmute(dimensioncount), ::core::mem::transmute(dimensions)).ok()
    }
}
impl ::core::convert::From<IMLOperatorTensorShapeDescription> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorTensorShapeDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMLOperatorTensorShapeDescription> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorTensorShapeDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorTensorShapeDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMLOperatorTensorShapeDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMLOperatorTensorShapeDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMLOperatorTensorShapeDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorTensorShapeDescription {}
impl ::core::fmt::Debug for IMLOperatorTensorShapeDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorTensorShapeDescription").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorTensorShapeDescription {
    type Vtable = IMLOperatorTensorShapeDescriptionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf20e8cbe_3b28_4248_be95_f96fbc6e4643);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTensorShapeDescriptionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> bool,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct IMLOperatorTypeInferenceContext(::windows::core::IUnknown);
impl IMLOperatorTypeInferenceContext {
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeElementCount<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttribute<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(elementcount), ::core::mem::transmute(elementbytesize), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringAttributeElementLength<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(elementindex), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringAttributeElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(elementindex), ::core::mem::transmute(attributeelementbytesize), ::core::mem::transmute(attributeelement)).ok()
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetInputCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetOutputCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn IsInputValid(&self, inputindex: u32) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn IsOutputValid(&self, outputindex: u32) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex)))
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn GetInputEdgeDescription(&self, inputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription> {
        let mut result__: MLOperatorEdgeDescription = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(inputindex), ::core::mem::transmute(&mut result__)).from_abi::<MLOperatorEdgeDescription>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn SetOutputEdgeDescription(&self, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(outputindex), ::core::mem::transmute(edgedescription)).ok()
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
impl ::core::convert::From<IMLOperatorTypeInferenceContext> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorTypeInferenceContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMLOperatorTypeInferenceContext> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorTypeInferenceContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorTypeInferenceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMLOperatorTypeInferenceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMLOperatorTypeInferenceContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMLOperatorTypeInferenceContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorTypeInferenceContext {}
impl ::core::fmt::Debug for IMLOperatorTypeInferenceContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorTypeInferenceContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorTypeInferenceContext {
    type Vtable = IMLOperatorTypeInferenceContextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec893bb1_f938_427b_8488_c8dcf775f138);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTypeInferenceContextVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct IMLOperatorTypeInferrer(::windows::core::IUnknown);
impl IMLOperatorTypeInferrer {
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn InferOutputTypes<'a, Param0: ::windows::core::IntoParam<'a, IMLOperatorTypeInferenceContext>>(&self, context: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), context.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMLOperatorTypeInferrer> for ::windows::core::IUnknown {
    fn from(value: IMLOperatorTypeInferrer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMLOperatorTypeInferrer> for ::windows::core::IUnknown {
    fn from(value: &IMLOperatorTypeInferrer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMLOperatorTypeInferrer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMLOperatorTypeInferrer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMLOperatorTypeInferrer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMLOperatorTypeInferrer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorTypeInferrer {}
impl ::core::fmt::Debug for IMLOperatorTypeInferrer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorTypeInferrer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMLOperatorTypeInferrer {
    type Vtable = IMLOperatorTypeInferrerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x781aeb48_9bcb_4797_bf77_8bf455217beb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTypeInferrerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct IWinMLEvaluationContext(::windows::core::IUnknown);
impl IWinMLEvaluationContext {
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation', 'Win32_Graphics_Direct3D12'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn BindValue(&self, pdescriptor: *const WINML_BINDING_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdescriptor)).ok()
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation', 'Win32_Graphics_Direct3D12'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn GetValueByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<*mut WINML_BINDING_DESC> {
        let mut result__: *mut WINML_BINDING_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<*mut WINML_BINDING_DESC>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWinMLEvaluationContext> for ::windows::core::IUnknown {
    fn from(value: IWinMLEvaluationContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWinMLEvaluationContext> for ::windows::core::IUnknown {
    fn from(value: &IWinMLEvaluationContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWinMLEvaluationContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWinMLEvaluationContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWinMLEvaluationContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWinMLEvaluationContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinMLEvaluationContext {}
impl ::core::fmt::Debug for IWinMLEvaluationContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinMLEvaluationContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWinMLEvaluationContext {
    type Vtable = IWinMLEvaluationContextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95848f9e_583d_4054_af12_916387cd8426);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLEvaluationContextVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdescriptor: *const WINML_BINDING_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR, pdescriptor: *mut *mut WINML_BINDING_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct IWinMLModel(::windows::core::IUnknown);
impl IWinMLModel {
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<*mut WINML_MODEL_DESC> {
        let mut result__: *mut WINML_MODEL_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut WINML_MODEL_DESC>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateMetadata(&self, index: u32, pkey: *mut super::super::super::Foundation::PWSTR, pvalue: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(pkey), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateModelInputs(&self, index: u32) -> ::windows::core::Result<*mut WINML_VARIABLE_DESC> {
        let mut result__: *mut WINML_VARIABLE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<*mut WINML_VARIABLE_DESC>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateModelOutputs(&self, index: u32) -> ::windows::core::Result<*mut WINML_VARIABLE_DESC> {
        let mut result__: *mut WINML_VARIABLE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<*mut WINML_VARIABLE_DESC>(result__)
    }
}
impl ::core::convert::From<IWinMLModel> for ::windows::core::IUnknown {
    fn from(value: IWinMLModel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWinMLModel> for ::windows::core::IUnknown {
    fn from(value: &IWinMLModel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWinMLModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWinMLModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWinMLModel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWinMLModel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinMLModel {}
impl ::core::fmt::Debug for IWinMLModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinMLModel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWinMLModel {
    type Vtable = IWinMLModelVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2eeb6a9_f31f_4055_a521_e30b5b33664a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLModelVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdescription: *mut *mut WINML_MODEL_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pkey: *mut super::super::super::Foundation::PWSTR, pvalue: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppinputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppoutputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct IWinMLRuntime(::windows::core::IUnknown);
impl IWinMLRuntime {
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadModel<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, path: Param0) -> ::windows::core::Result<IWinMLModel> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWinMLModel>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Graphics_Direct3D12'*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateEvaluationContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>>(&self, device: Param0) -> ::windows::core::Result<IWinMLEvaluationContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), device.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWinMLEvaluationContext>(result__)
    }
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn EvaluateModel<'a, Param0: ::windows::core::IntoParam<'a, IWinMLEvaluationContext>>(&self, pcontext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pcontext.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWinMLRuntime> for ::windows::core::IUnknown {
    fn from(value: IWinMLRuntime) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWinMLRuntime> for ::windows::core::IUnknown {
    fn from(value: &IWinMLRuntime) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWinMLRuntime {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWinMLRuntime {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWinMLRuntime {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWinMLRuntime {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinMLRuntime {}
impl ::core::fmt::Debug for IWinMLRuntime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinMLRuntime").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWinMLRuntime {
    type Vtable = IWinMLRuntimeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0425329_40ae_48d9_bce3_829ef7b8a41a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLRuntimeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: super::super::super::Foundation::PWSTR, ppmodel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct IWinMLRuntimeFactory(::windows::core::IUnknown);
impl IWinMLRuntimeFactory {
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub unsafe fn CreateRuntime(&self, runtimetype: WINML_RUNTIME_TYPE) -> ::windows::core::Result<IWinMLRuntime> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(runtimetype), ::core::mem::transmute(&mut result__)).from_abi::<IWinMLRuntime>(result__)
    }
}
impl ::core::convert::From<IWinMLRuntimeFactory> for ::windows::core::IUnknown {
    fn from(value: IWinMLRuntimeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWinMLRuntimeFactory> for ::windows::core::IUnknown {
    fn from(value: &IWinMLRuntimeFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWinMLRuntimeFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWinMLRuntimeFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWinMLRuntimeFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWinMLRuntimeFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinMLRuntimeFactory {}
impl ::core::fmt::Debug for IWinMLRuntimeFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinMLRuntimeFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWinMLRuntimeFactory {
    type Vtable = IWinMLRuntimeFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa807b84d_4ae5_4bc0_a76a_941aa246bd41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLRuntimeFactoryVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runtimetype: WINML_RUNTIME_TYPE, ppruntime: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[inline]
pub unsafe fn MLCreateOperatorRegistry() -> ::windows::core::Result<IMLOperatorRegistry> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MLCreateOperatorRegistry(registry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        MLCreateOperatorRegistry(::core::mem::transmute(&mut result__)).from_abi::<IMLOperatorRegistry>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorAttribute {
    pub name: super::super::super::Foundation::PSTR,
    pub r#type: MLOperatorAttributeType,
    pub required: bool,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MLOperatorAttribute {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MLOperatorAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MLOperatorAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MLOperatorAttribute").field("name", &self.name).field("type", &self.r#type).field("required", &self.required).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MLOperatorAttribute {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MLOperatorAttribute {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLOperatorAttribute>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MLOperatorAttribute {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MLOperatorAttribute {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorAttributeNameValue {
    pub name: super::super::super::Foundation::PSTR,
    pub r#type: MLOperatorAttributeType,
    pub valueCount: u32,
    pub Anonymous: MLOperatorAttributeNameValue_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MLOperatorAttributeNameValue {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MLOperatorAttributeNameValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MLOperatorAttributeNameValue {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MLOperatorAttributeNameValue {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLOperatorAttributeNameValue>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MLOperatorAttributeNameValue {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MLOperatorAttributeNameValue {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union MLOperatorAttributeNameValue_0 {
    pub reserved: *mut ::core::ffi::c_void,
    pub ints: *mut i64,
    pub strings: *mut *mut i8,
    pub floats: *mut f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MLOperatorAttributeNameValue_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MLOperatorAttributeNameValue_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MLOperatorAttributeNameValue_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MLOperatorAttributeNameValue_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLOperatorAttributeNameValue_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MLOperatorAttributeNameValue_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MLOperatorAttributeNameValue_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct MLOperatorAttributeType(pub u32);
impl MLOperatorAttributeType {
    pub const Undefined: Self = Self(0u32);
    pub const Float: Self = Self(2u32);
    pub const Int: Self = Self(3u32);
    pub const String: Self = Self(4u32);
    pub const FloatArray: Self = Self(7u32);
    pub const IntArray: Self = Self(8u32);
    pub const StringArray: Self = Self(9u32);
}
impl ::core::marker::Copy for MLOperatorAttributeType {}
impl ::core::clone::Clone for MLOperatorAttributeType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MLOperatorAttributeType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MLOperatorAttributeType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MLOperatorAttributeType {}
impl ::core::fmt::Debug for MLOperatorAttributeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorAttributeType").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub struct MLOperatorEdgeDescription {
    pub edgeType: MLOperatorEdgeType,
    pub Anonymous: MLOperatorEdgeDescription_0,
}
impl ::core::marker::Copy for MLOperatorEdgeDescription {}
impl ::core::clone::Clone for MLOperatorEdgeDescription {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MLOperatorEdgeDescription {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MLOperatorEdgeDescription {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLOperatorEdgeDescription>()) == 0 }
    }
}
impl ::core::cmp::Eq for MLOperatorEdgeDescription {}
impl ::core::default::Default for MLOperatorEdgeDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub union MLOperatorEdgeDescription_0 {
    pub reserved: u64,
    pub tensorDataType: MLOperatorTensorDataType,
}
impl ::core::marker::Copy for MLOperatorEdgeDescription_0 {}
impl ::core::clone::Clone for MLOperatorEdgeDescription_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MLOperatorEdgeDescription_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MLOperatorEdgeDescription_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLOperatorEdgeDescription_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MLOperatorEdgeDescription_0 {}
impl ::core::default::Default for MLOperatorEdgeDescription_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct MLOperatorEdgeType(pub u32);
impl MLOperatorEdgeType {
    pub const Undefined: Self = Self(0u32);
    pub const Tensor: Self = Self(1u32);
}
impl ::core::marker::Copy for MLOperatorEdgeType {}
impl ::core::clone::Clone for MLOperatorEdgeType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MLOperatorEdgeType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MLOperatorEdgeType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MLOperatorEdgeType {}
impl ::core::fmt::Debug for MLOperatorEdgeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorEdgeType").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorEdgeTypeConstraint {
    pub typeLabel: super::super::super::Foundation::PSTR,
    pub allowedTypes: *mut MLOperatorEdgeDescription,
    pub allowedTypeCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MLOperatorEdgeTypeConstraint {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MLOperatorEdgeTypeConstraint {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MLOperatorEdgeTypeConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MLOperatorEdgeTypeConstraint").field("typeLabel", &self.typeLabel).field("allowedTypes", &self.allowedTypes).field("allowedTypeCount", &self.allowedTypeCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MLOperatorEdgeTypeConstraint {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MLOperatorEdgeTypeConstraint {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLOperatorEdgeTypeConstraint>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MLOperatorEdgeTypeConstraint {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MLOperatorEdgeTypeConstraint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct MLOperatorExecutionType(pub u32);
impl MLOperatorExecutionType {
    pub const Undefined: Self = Self(0u32);
    pub const Cpu: Self = Self(1u32);
    pub const D3D12: Self = Self(2u32);
}
impl ::core::marker::Copy for MLOperatorExecutionType {}
impl ::core::clone::Clone for MLOperatorExecutionType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MLOperatorExecutionType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MLOperatorExecutionType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MLOperatorExecutionType {}
impl ::core::fmt::Debug for MLOperatorExecutionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorExecutionType").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for MLOperatorKernelDescription {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MLOperatorKernelDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MLOperatorKernelDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MLOperatorKernelDescription")
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
unsafe impl ::windows::core::Abi for MLOperatorKernelDescription {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MLOperatorKernelDescription {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLOperatorKernelDescription>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MLOperatorKernelDescription {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MLOperatorKernelDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct MLOperatorKernelOptions(pub u32);
impl MLOperatorKernelOptions {
    pub const None: Self = Self(0u32);
    pub const AllowDynamicInputShapes: Self = Self(1u32);
}
impl ::core::marker::Copy for MLOperatorKernelOptions {}
impl ::core::clone::Clone for MLOperatorKernelOptions {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MLOperatorKernelOptions {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MLOperatorKernelOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MLOperatorKernelOptions {}
impl ::core::fmt::Debug for MLOperatorKernelOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorKernelOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct MLOperatorParameterOptions(pub u32);
impl MLOperatorParameterOptions {
    pub const Single: Self = Self(0u32);
    pub const Optional: Self = Self(1u32);
    pub const Variadic: Self = Self(2u32);
}
impl ::core::marker::Copy for MLOperatorParameterOptions {}
impl ::core::clone::Clone for MLOperatorParameterOptions {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MLOperatorParameterOptions {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MLOperatorParameterOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MLOperatorParameterOptions {}
impl ::core::fmt::Debug for MLOperatorParameterOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorParameterOptions").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for MLOperatorSchemaDescription {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MLOperatorSchemaDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MLOperatorSchemaDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MLOperatorSchemaDescription")
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
unsafe impl ::windows::core::Abi for MLOperatorSchemaDescription {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MLOperatorSchemaDescription {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLOperatorSchemaDescription>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MLOperatorSchemaDescription {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MLOperatorSchemaDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorSchemaEdgeDescription {
    pub options: MLOperatorParameterOptions,
    pub typeFormat: MLOperatorSchemaEdgeTypeFormat,
    pub Anonymous: MLOperatorSchemaEdgeDescription_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MLOperatorSchemaEdgeDescription {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MLOperatorSchemaEdgeDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MLOperatorSchemaEdgeDescription {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MLOperatorSchemaEdgeDescription {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLOperatorSchemaEdgeDescription>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MLOperatorSchemaEdgeDescription {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MLOperatorSchemaEdgeDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union MLOperatorSchemaEdgeDescription_0 {
    pub reserved: *mut ::core::ffi::c_void,
    pub typeLabel: super::super::super::Foundation::PSTR,
    pub edgeDescription: MLOperatorEdgeDescription,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MLOperatorSchemaEdgeDescription_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MLOperatorSchemaEdgeDescription_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MLOperatorSchemaEdgeDescription_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MLOperatorSchemaEdgeDescription_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLOperatorSchemaEdgeDescription_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MLOperatorSchemaEdgeDescription_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MLOperatorSchemaEdgeDescription_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct MLOperatorSchemaEdgeTypeFormat(pub i32);
impl MLOperatorSchemaEdgeTypeFormat {
    pub const EdgeDescription: Self = Self(0i32);
    pub const Label: Self = Self(1i32);
}
impl ::core::marker::Copy for MLOperatorSchemaEdgeTypeFormat {}
impl ::core::clone::Clone for MLOperatorSchemaEdgeTypeFormat {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MLOperatorSchemaEdgeTypeFormat {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MLOperatorSchemaEdgeTypeFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MLOperatorSchemaEdgeTypeFormat {}
impl ::core::fmt::Debug for MLOperatorSchemaEdgeTypeFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorSchemaEdgeTypeFormat").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorSetId {
    pub domain: super::super::super::Foundation::PSTR,
    pub version: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MLOperatorSetId {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MLOperatorSetId {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MLOperatorSetId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MLOperatorSetId").field("domain", &self.domain).field("version", &self.version).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MLOperatorSetId {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MLOperatorSetId {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLOperatorSetId>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MLOperatorSetId {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MLOperatorSetId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct MLOperatorTensorDataType(pub u32);
impl MLOperatorTensorDataType {
    pub const Undefined: Self = Self(0u32);
    pub const Float: Self = Self(1u32);
    pub const UInt8: Self = Self(2u32);
    pub const Int8: Self = Self(3u32);
    pub const UInt16: Self = Self(4u32);
    pub const Int16: Self = Self(5u32);
    pub const Int32: Self = Self(6u32);
    pub const Int64: Self = Self(7u32);
    pub const String: Self = Self(8u32);
    pub const Bool: Self = Self(9u32);
    pub const Float16: Self = Self(10u32);
    pub const Double: Self = Self(11u32);
    pub const UInt32: Self = Self(12u32);
    pub const UInt64: Self = Self(13u32);
    pub const Complex64: Self = Self(14u32);
    pub const Complex128: Self = Self(15u32);
}
impl ::core::marker::Copy for MLOperatorTensorDataType {}
impl ::core::clone::Clone for MLOperatorTensorDataType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MLOperatorTensorDataType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MLOperatorTensorDataType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MLOperatorTensorDataType {}
impl ::core::fmt::Debug for MLOperatorTensorDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorTensorDataType").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation', 'Win32_Graphics_Direct3D12'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub struct WINML_BINDING_DESC {
    pub Name: super::super::super::Foundation::PWSTR,
    pub BindType: WINML_BINDING_TYPE,
    pub Anonymous: WINML_BINDING_DESC_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::clone::Clone for WINML_BINDING_DESC {
    fn clone(&self) -> Self {
        Self { Name: self.Name, BindType: self.BindType, Anonymous: self.Anonymous.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
unsafe impl ::windows::core::Abi for WINML_BINDING_DESC {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::cmp::PartialEq for WINML_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.BindType == other.BindType && self.Anonymous == other.Anonymous
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::cmp::Eq for WINML_BINDING_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::default::Default for WINML_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation', 'Win32_Graphics_Direct3D12'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub union WINML_BINDING_DESC_0 {
    pub Tensor: WINML_TENSOR_BINDING_DESC,
    pub Sequence: WINML_SEQUENCE_BINDING_DESC,
    pub Map: WINML_MAP_BINDING_DESC,
    pub Image: WINML_IMAGE_BINDING_DESC,
    pub Resource: ::core::mem::ManuallyDrop<WINML_RESOURCE_BINDING_DESC>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::clone::Clone for WINML_BINDING_DESC_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
unsafe impl ::windows::core::Abi for WINML_BINDING_DESC_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::cmp::PartialEq for WINML_BINDING_DESC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINML_BINDING_DESC_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::cmp::Eq for WINML_BINDING_DESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::default::Default for WINML_BINDING_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub type WINML_BINDING_TYPE = i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_BINDING_UNDEFINED: WINML_BINDING_TYPE = 0i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_BINDING_TENSOR: WINML_BINDING_TYPE = 1i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_BINDING_SEQUENCE: WINML_BINDING_TYPE = 2i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_BINDING_MAP: WINML_BINDING_TYPE = 3i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_BINDING_IMAGE: WINML_BINDING_TYPE = 4i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_BINDING_RESOURCE: WINML_BINDING_TYPE = 5i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub type WINML_FEATURE_TYPE = i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_FEATURE_UNDEFINED: WINML_FEATURE_TYPE = 0i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_FEATURE_TENSOR: WINML_FEATURE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_FEATURE_SEQUENCE: WINML_FEATURE_TYPE = 2i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_FEATURE_MAP: WINML_FEATURE_TYPE = 3i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_FEATURE_IMAGE: WINML_FEATURE_TYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub struct WINML_IMAGE_BINDING_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub DataSize: u32,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WINML_IMAGE_BINDING_DESC {}
impl ::core::clone::Clone for WINML_IMAGE_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINML_IMAGE_BINDING_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_IMAGE_BINDING_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).field("DataSize", &self.DataSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows::core::Abi for WINML_IMAGE_BINDING_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINML_IMAGE_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINML_IMAGE_BINDING_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINML_IMAGE_BINDING_DESC {}
impl ::core::default::Default for WINML_IMAGE_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub struct WINML_IMAGE_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
}
impl ::core::marker::Copy for WINML_IMAGE_VARIABLE_DESC {}
impl ::core::clone::Clone for WINML_IMAGE_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINML_IMAGE_VARIABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_IMAGE_VARIABLE_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).finish()
    }
}
unsafe impl ::windows::core::Abi for WINML_IMAGE_VARIABLE_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINML_IMAGE_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINML_IMAGE_VARIABLE_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINML_IMAGE_VARIABLE_DESC {}
impl ::core::default::Default for WINML_IMAGE_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINML_MAP_BINDING_DESC {
    pub ElementCount: u32,
    pub KeyType: WINML_TENSOR_DATA_TYPE,
    pub Anonymous1: WINML_MAP_BINDING_DESC_0,
    pub Fields: WINML_TENSOR_DATA_TYPE,
    pub Anonymous2: WINML_MAP_BINDING_DESC_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_MAP_BINDING_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_MAP_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINML_MAP_BINDING_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINML_MAP_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINML_MAP_BINDING_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINML_MAP_BINDING_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_MAP_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union WINML_MAP_BINDING_DESC_0 {
    pub pStringKeys: *mut super::super::super::Foundation::PWSTR,
    pub pIntKeys: *mut i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_MAP_BINDING_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_MAP_BINDING_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINML_MAP_BINDING_DESC_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINML_MAP_BINDING_DESC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINML_MAP_BINDING_DESC_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINML_MAP_BINDING_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_MAP_BINDING_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union WINML_MAP_BINDING_DESC_1 {
    pub pStringFields: *mut super::super::super::Foundation::PWSTR,
    pub pIntFields: *mut i64,
    pub pFloatFields: *mut f32,
    pub pDoubleFields: *mut f64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_MAP_BINDING_DESC_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_MAP_BINDING_DESC_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINML_MAP_BINDING_DESC_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINML_MAP_BINDING_DESC_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINML_MAP_BINDING_DESC_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINML_MAP_BINDING_DESC_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_MAP_BINDING_DESC_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub struct WINML_MAP_VARIABLE_DESC {
    pub KeyType: WINML_TENSOR_DATA_TYPE,
    pub Fields: WINML_TENSOR_DATA_TYPE,
}
impl ::core::marker::Copy for WINML_MAP_VARIABLE_DESC {}
impl ::core::clone::Clone for WINML_MAP_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINML_MAP_VARIABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_MAP_VARIABLE_DESC").field("KeyType", &self.KeyType).field("Fields", &self.Fields).finish()
    }
}
unsafe impl ::windows::core::Abi for WINML_MAP_VARIABLE_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINML_MAP_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINML_MAP_VARIABLE_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINML_MAP_VARIABLE_DESC {}
impl ::core::default::Default for WINML_MAP_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINML_MODEL_DESC {
    pub Author: super::super::super::Foundation::PWSTR,
    pub Name: super::super::super::Foundation::PWSTR,
    pub Domain: super::super::super::Foundation::PWSTR,
    pub Description: super::super::super::Foundation::PWSTR,
    pub Version: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_MODEL_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_MODEL_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINML_MODEL_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_MODEL_DESC").field("Author", &self.Author).field("Name", &self.Name).field("Domain", &self.Domain).field("Description", &self.Description).field("Version", &self.Version).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINML_MODEL_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINML_MODEL_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINML_MODEL_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINML_MODEL_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_MODEL_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Graphics_Direct3D12'*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct WINML_RESOURCE_BINDING_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub pResource: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for WINML_RESOURCE_BINDING_DESC {
    fn clone(&self) -> Self {
        Self { ElementType: self.ElementType, NumDimensions: self.NumDimensions, pShape: self.pShape, pResource: self.pResource.clone() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for WINML_RESOURCE_BINDING_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_RESOURCE_BINDING_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).field("pResource", &self.pResource).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
unsafe impl ::windows::core::Abi for WINML_RESOURCE_BINDING_DESC {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for WINML_RESOURCE_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub type WINML_RUNTIME_TYPE = i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_RUNTIME_CNTK: WINML_RUNTIME_TYPE = 0i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINML_SEQUENCE_BINDING_DESC {
    pub ElementCount: u32,
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub Anonymous: WINML_SEQUENCE_BINDING_DESC_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_SEQUENCE_BINDING_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_SEQUENCE_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINML_SEQUENCE_BINDING_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINML_SEQUENCE_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINML_SEQUENCE_BINDING_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINML_SEQUENCE_BINDING_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_SEQUENCE_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union WINML_SEQUENCE_BINDING_DESC_0 {
    pub pStrings: *mut super::super::super::Foundation::PWSTR,
    pub pInts: *mut i64,
    pub pFloats: *mut f32,
    pub pDoubles: *mut f64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_SEQUENCE_BINDING_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_SEQUENCE_BINDING_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINML_SEQUENCE_BINDING_DESC_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINML_SEQUENCE_BINDING_DESC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINML_SEQUENCE_BINDING_DESC_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINML_SEQUENCE_BINDING_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_SEQUENCE_BINDING_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub struct WINML_SEQUENCE_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
}
impl ::core::marker::Copy for WINML_SEQUENCE_VARIABLE_DESC {}
impl ::core::clone::Clone for WINML_SEQUENCE_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINML_SEQUENCE_VARIABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_SEQUENCE_VARIABLE_DESC").field("ElementType", &self.ElementType).finish()
    }
}
unsafe impl ::windows::core::Abi for WINML_SEQUENCE_VARIABLE_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINML_SEQUENCE_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINML_SEQUENCE_VARIABLE_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINML_SEQUENCE_VARIABLE_DESC {}
impl ::core::default::Default for WINML_SEQUENCE_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub struct WINML_TENSOR_BINDING_DESC {
    pub DataType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub DataSize: u32,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WINML_TENSOR_BINDING_DESC {}
impl ::core::clone::Clone for WINML_TENSOR_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINML_TENSOR_BINDING_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_TENSOR_BINDING_DESC").field("DataType", &self.DataType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).field("DataSize", &self.DataSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows::core::Abi for WINML_TENSOR_BINDING_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINML_TENSOR_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINML_TENSOR_BINDING_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINML_TENSOR_BINDING_DESC {}
impl ::core::default::Default for WINML_TENSOR_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub type WINML_TENSOR_DATA_TYPE = i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_UNDEFINED: WINML_TENSOR_DATA_TYPE = 0i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_FLOAT: WINML_TENSOR_DATA_TYPE = 1i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_UINT8: WINML_TENSOR_DATA_TYPE = 2i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_INT8: WINML_TENSOR_DATA_TYPE = 3i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_UINT16: WINML_TENSOR_DATA_TYPE = 4i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_INT16: WINML_TENSOR_DATA_TYPE = 5i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_INT32: WINML_TENSOR_DATA_TYPE = 6i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_INT64: WINML_TENSOR_DATA_TYPE = 7i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_STRING: WINML_TENSOR_DATA_TYPE = 8i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_BOOLEAN: WINML_TENSOR_DATA_TYPE = 9i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_FLOAT16: WINML_TENSOR_DATA_TYPE = 10i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_DOUBLE: WINML_TENSOR_DATA_TYPE = 11i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_UINT32: WINML_TENSOR_DATA_TYPE = 12i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_UINT64: WINML_TENSOR_DATA_TYPE = 13i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_COMPLEX64: WINML_TENSOR_DATA_TYPE = 14i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_COMPLEX128: WINML_TENSOR_DATA_TYPE = 15i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_DIMENSION_COUNT_MAX: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub struct WINML_TENSOR_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
}
impl ::core::marker::Copy for WINML_TENSOR_VARIABLE_DESC {}
impl ::core::clone::Clone for WINML_TENSOR_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINML_TENSOR_VARIABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_TENSOR_VARIABLE_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).finish()
    }
}
unsafe impl ::windows::core::Abi for WINML_TENSOR_VARIABLE_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINML_TENSOR_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINML_TENSOR_VARIABLE_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINML_TENSOR_VARIABLE_DESC {}
impl ::core::default::Default for WINML_TENSOR_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINML_VARIABLE_DESC {
    pub Name: super::super::super::Foundation::PWSTR,
    pub Description: super::super::super::Foundation::PWSTR,
    pub FeatureType: WINML_FEATURE_TYPE,
    pub Required: super::super::super::Foundation::BOOL,
    pub Anonymous: WINML_VARIABLE_DESC_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_VARIABLE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINML_VARIABLE_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINML_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINML_VARIABLE_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINML_VARIABLE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union WINML_VARIABLE_DESC_0 {
    pub Tensor: WINML_TENSOR_VARIABLE_DESC,
    pub Sequence: WINML_SEQUENCE_VARIABLE_DESC,
    pub Map: WINML_MAP_VARIABLE_DESC,
    pub Image: WINML_IMAGE_VARIABLE_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_VARIABLE_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_VARIABLE_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINML_VARIABLE_DESC_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINML_VARIABLE_DESC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINML_VARIABLE_DESC_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINML_VARIABLE_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_VARIABLE_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[inline]
pub unsafe fn WinMLCreateRuntime() -> ::windows::core::Result<IWinMLRuntime> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinMLCreateRuntime(runtime: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WinMLCreateRuntime(::core::mem::transmute(&mut result__)).from_abi::<IWinMLRuntime>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
