#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMLOperatorAttributes(::windows::runtime::IUnknown);
impl IMLOperatorAttributes {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeElementCount<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(r#type), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttribute<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(elementcount), ::std::mem::transmute(elementbytesize), ::std::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringAttributeElementLength<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(elementindex), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringAttributeElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(elementindex), ::std::mem::transmute(attributeelementbytesize), ::std::mem::transmute(attributeelement)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMLOperatorAttributes {
    type Vtable = IMLOperatorAttributes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1260066649, 60480, 18028, [170, 180, 190, 181, 52, 127, 210, 76]);
}
impl ::std::convert::From<IMLOperatorAttributes> for ::windows::runtime::IUnknown {
    fn from(value: IMLOperatorAttributes) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMLOperatorAttributes> for ::windows::runtime::IUnknown {
    fn from(value: &IMLOperatorAttributes) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMLOperatorAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMLOperatorAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorAttributes_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMLOperatorKernel(::windows::runtime::IUnknown);
impl IMLOperatorKernel {
    pub unsafe fn Compute<'a, Param0: ::windows::runtime::IntoParam<'a, IMLOperatorKernelContext>>(&self, context: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), context.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMLOperatorKernel {
    type Vtable = IMLOperatorKernel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(298103968, 46183, 20138, [161, 166, 185, 97, 216, 208, 237, 121]);
}
impl ::std::convert::From<IMLOperatorKernel> for ::windows::runtime::IUnknown {
    fn from(value: IMLOperatorKernel) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMLOperatorKernel> for ::windows::runtime::IUnknown {
    fn from(value: &IMLOperatorKernel) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMLOperatorKernel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMLOperatorKernel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMLOperatorKernelContext(::windows::runtime::IUnknown);
impl IMLOperatorKernelContext {
    pub unsafe fn GetInputTensor(&self, inputindex: u32) -> ::windows::runtime::Result<IMLOperatorTensor> {
        let mut result__: <IMLOperatorTensor as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(inputindex), &mut result__).from_abi::<IMLOperatorTensor>(result__)
    }
    pub unsafe fn GetOutputTensor(&self, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32) -> ::windows::runtime::Result<IMLOperatorTensor> {
        let mut result__: <IMLOperatorTensor as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(outputindex), ::std::mem::transmute(dimensioncount), ::std::mem::transmute(dimensionsizes), &mut result__).from_abi::<IMLOperatorTensor>(result__)
    }
    pub unsafe fn GetOutputTensor2(&self, outputindex: u32) -> ::windows::runtime::Result<IMLOperatorTensor> {
        let mut result__: <IMLOperatorTensor as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(outputindex), &mut result__).from_abi::<IMLOperatorTensor>(result__)
    }
    pub unsafe fn AllocateTemporaryData(&self, size: usize) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(size), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    pub unsafe fn GetExecutionInterface(&self, executionobject: *mut ::std::option::Option<::windows::runtime::IUnknown>) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(executionobject)))
    }
}
unsafe impl ::windows::runtime::Interface for IMLOperatorKernelContext {
    type Vtable = IMLOperatorKernelContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2186504744, 61474, 18281, [157, 63, 139, 39, 143, 132, 192, 195]);
}
impl ::std::convert::From<IMLOperatorKernelContext> for ::windows::runtime::IUnknown {
    fn from(value: IMLOperatorKernelContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMLOperatorKernelContext> for ::windows::runtime::IUnknown {
    fn from(value: &IMLOperatorKernelContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMLOperatorKernelContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMLOperatorKernelContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernelContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputindex: u32, tensor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32, tensor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputindex: u32, tensor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, size: usize, data: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, executionobject: *mut ::windows::runtime::RawPtr),
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMLOperatorKernelCreationContext(::windows::runtime::IUnknown);
impl IMLOperatorKernelCreationContext {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeElementCount<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(r#type), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttribute<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(elementcount), ::std::mem::transmute(elementbytesize), ::std::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringAttributeElementLength<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(elementindex), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringAttributeElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(elementindex), ::std::mem::transmute(attributeelementbytesize), ::std::mem::transmute(attributeelement)).ok()
    }
    pub unsafe fn GetInputCount(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)))
    }
    pub unsafe fn GetOutputCount(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)))
    }
    pub unsafe fn IsInputValid(&self, inputindex: u32) -> bool {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(inputindex)))
    }
    pub unsafe fn IsOutputValid(&self, outputindex: u32) -> bool {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(outputindex)))
    }
    pub unsafe fn GetInputEdgeDescription(&self, inputindex: u32) -> ::windows::runtime::Result<MLOperatorEdgeDescription> {
        let mut result__: <MLOperatorEdgeDescription as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(inputindex), &mut result__).from_abi::<MLOperatorEdgeDescription>(result__)
    }
    pub unsafe fn GetOutputEdgeDescription(&self, outputindex: u32) -> ::windows::runtime::Result<MLOperatorEdgeDescription> {
        let mut result__: <MLOperatorEdgeDescription as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(outputindex), &mut result__).from_abi::<MLOperatorEdgeDescription>(result__)
    }
    pub unsafe fn HasTensorShapeDescription(&self) -> bool {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)))
    }
    pub unsafe fn GetTensorShapeDescription(&self) -> ::windows::runtime::Result<IMLOperatorTensorShapeDescription> {
        let mut result__: <IMLOperatorTensorShapeDescription as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMLOperatorTensorShapeDescription>(result__)
    }
    pub unsafe fn GetExecutionInterface(&self, executionobject: *mut ::std::option::Option<::windows::runtime::IUnknown>) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(executionobject)))
    }
}
unsafe impl ::windows::runtime::Interface for IMLOperatorKernelCreationContext {
    type Vtable = IMLOperatorKernelCreationContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1415165245, 41212, 18021, [173, 221, 112, 23, 30, 247, 230, 49]);
}
impl ::std::convert::From<IMLOperatorKernelCreationContext> for ::windows::runtime::IUnknown {
    fn from(value: IMLOperatorKernelCreationContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMLOperatorKernelCreationContext> for ::windows::runtime::IUnknown {
    fn from(value: &IMLOperatorKernelCreationContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMLOperatorKernelCreationContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMLOperatorKernelCreationContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMLOperatorKernelCreationContext> for IMLOperatorAttributes {
    fn from(value: IMLOperatorKernelCreationContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMLOperatorKernelCreationContext> for IMLOperatorAttributes {
    fn from(value: &IMLOperatorKernelCreationContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMLOperatorAttributes> for IMLOperatorKernelCreationContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMLOperatorAttributes> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMLOperatorAttributes>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMLOperatorAttributes> for &IMLOperatorKernelCreationContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMLOperatorAttributes> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMLOperatorAttributes>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernelCreationContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputindex: u32) -> bool,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputindex: u32) -> bool,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> bool,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shapedescription: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, executionobject: *mut ::windows::runtime::RawPtr),
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMLOperatorKernelFactory(::windows::runtime::IUnknown);
impl IMLOperatorKernelFactory {
    pub unsafe fn CreateKernel<'a, Param0: ::windows::runtime::IntoParam<'a, IMLOperatorKernelCreationContext>>(&self, context: Param0) -> ::windows::runtime::Result<IMLOperatorKernel> {
        let mut result__: <IMLOperatorKernel as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), context.into_param().abi(), &mut result__).from_abi::<IMLOperatorKernel>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMLOperatorKernelFactory {
    type Vtable = IMLOperatorKernelFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4011175279, 3529, 18696, [171, 53, 165, 117, 163, 13, 251, 248]);
}
impl ::std::convert::From<IMLOperatorKernelFactory> for ::windows::runtime::IUnknown {
    fn from(value: IMLOperatorKernelFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMLOperatorKernelFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IMLOperatorKernelFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMLOperatorKernelFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMLOperatorKernelFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernelFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: ::windows::runtime::RawPtr, kernel: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMLOperatorRegistry(::windows::runtime::IUnknown);
impl IMLOperatorRegistry {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOperatorSetSchema<'a, Param4: ::windows::runtime::IntoParam<'a, IMLOperatorTypeInferrer>, Param5: ::windows::runtime::IntoParam<'a, IMLOperatorShapeInferrer>>(&self, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: Param4, shapeinferrer: Param5) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(operatorsetid), ::std::mem::transmute(baselineversion), ::std::mem::transmute(schema), ::std::mem::transmute(schemacount), typeinferrer.into_param().abi(), shapeinferrer.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOperatorKernel<'a, Param1: ::windows::runtime::IntoParam<'a, IMLOperatorKernelFactory>, Param2: ::windows::runtime::IntoParam<'a, IMLOperatorShapeInferrer>>(&self, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: Param1, shapeinferrer: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(operatorkernel), operatorkernelfactory.into_param().abi(), shapeinferrer.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMLOperatorRegistry {
    type Vtable = IMLOperatorRegistry_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(721018157, 46358, 18034, [154, 181, 83, 12, 32, 132, 147, 173]);
}
impl ::std::convert::From<IMLOperatorRegistry> for ::windows::runtime::IUnknown {
    fn from(value: IMLOperatorRegistry) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMLOperatorRegistry> for ::windows::runtime::IUnknown {
    fn from(value: &IMLOperatorRegistry) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMLOperatorRegistry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMLOperatorRegistry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorRegistry_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: ::windows::runtime::RawPtr, shapeinferrer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: ::windows::runtime::RawPtr, shapeinferrer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMLOperatorShapeInferenceContext(::windows::runtime::IUnknown);
impl IMLOperatorShapeInferenceContext {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeElementCount<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(r#type), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttribute<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(elementcount), ::std::mem::transmute(elementbytesize), ::std::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringAttributeElementLength<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(elementindex), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringAttributeElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(elementindex), ::std::mem::transmute(attributeelementbytesize), ::std::mem::transmute(attributeelement)).ok()
    }
    pub unsafe fn GetInputCount(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)))
    }
    pub unsafe fn GetOutputCount(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)))
    }
    pub unsafe fn IsInputValid(&self, inputindex: u32) -> bool {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(inputindex)))
    }
    pub unsafe fn IsOutputValid(&self, outputindex: u32) -> bool {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(outputindex)))
    }
    pub unsafe fn GetInputEdgeDescription(&self, inputindex: u32) -> ::windows::runtime::Result<MLOperatorEdgeDescription> {
        let mut result__: <MLOperatorEdgeDescription as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(inputindex), &mut result__).from_abi::<MLOperatorEdgeDescription>(result__)
    }
    pub unsafe fn GetInputTensorDimensionCount(&self, inputindex: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(inputindex), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetInputTensorShape(&self, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(inputindex), ::std::mem::transmute(dimensioncount), ::std::mem::transmute(dimensions)).ok()
    }
    pub unsafe fn SetOutputTensorShape(&self, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(outputindex), ::std::mem::transmute(dimensioncount), ::std::mem::transmute(dimensions)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMLOperatorShapeInferenceContext {
    type Vtable = IMLOperatorShapeInferenceContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(274426665, 21512, 19048, [153, 89, 9, 181, 149, 90, 52, 146]);
}
impl ::std::convert::From<IMLOperatorShapeInferenceContext> for ::windows::runtime::IUnknown {
    fn from(value: IMLOperatorShapeInferenceContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMLOperatorShapeInferenceContext> for ::windows::runtime::IUnknown {
    fn from(value: &IMLOperatorShapeInferenceContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMLOperatorShapeInferenceContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMLOperatorShapeInferenceContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMLOperatorShapeInferenceContext> for IMLOperatorAttributes {
    fn from(value: IMLOperatorShapeInferenceContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMLOperatorShapeInferenceContext> for IMLOperatorAttributes {
    fn from(value: &IMLOperatorShapeInferenceContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMLOperatorAttributes> for IMLOperatorShapeInferenceContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMLOperatorAttributes> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMLOperatorAttributes>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMLOperatorAttributes> for &IMLOperatorShapeInferenceContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMLOperatorAttributes> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMLOperatorAttributes>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorShapeInferenceContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputindex: u32) -> bool,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputindex: u32) -> bool,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputindex: u32, dimensioncount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMLOperatorShapeInferrer(::windows::runtime::IUnknown);
impl IMLOperatorShapeInferrer {
    pub unsafe fn InferOutputShapes<'a, Param0: ::windows::runtime::IntoParam<'a, IMLOperatorShapeInferenceContext>>(&self, context: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), context.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMLOperatorShapeInferrer {
    type Vtable = IMLOperatorShapeInferrer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1410065854, 42697, 16622, [131, 246, 210, 184, 180, 10, 119, 152]);
}
impl ::std::convert::From<IMLOperatorShapeInferrer> for ::windows::runtime::IUnknown {
    fn from(value: IMLOperatorShapeInferrer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMLOperatorShapeInferrer> for ::windows::runtime::IUnknown {
    fn from(value: &IMLOperatorShapeInferrer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMLOperatorShapeInferrer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMLOperatorShapeInferrer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorShapeInferrer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMLOperatorTensor(::windows::runtime::IUnknown);
impl IMLOperatorTensor {
    pub unsafe fn GetDimensionCount(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)))
    }
    pub unsafe fn GetShape(&self, dimensioncount: u32, dimensions: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dimensioncount), ::std::mem::transmute(dimensions)).ok()
    }
    pub unsafe fn GetTensorDataType(&self) -> MLOperatorTensorDataType {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)))
    }
    pub unsafe fn IsCpuData(&self) -> bool {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)))
    }
    pub unsafe fn IsDataInterface(&self) -> bool {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)))
    }
    pub unsafe fn GetData(&self) -> *mut ::std::ffi::c_void {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)))
    }
    pub unsafe fn GetDataInterface(&self, datainterface: *mut ::std::option::Option<::windows::runtime::IUnknown>) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(datainterface)))
    }
}
unsafe impl ::windows::runtime::Interface for IMLOperatorTensor {
    type Vtable = IMLOperatorTensor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2145656641, 62512, 17422, [174, 206, 84, 65, 109, 200, 185, 219]);
}
impl ::std::convert::From<IMLOperatorTensor> for ::windows::runtime::IUnknown {
    fn from(value: IMLOperatorTensor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMLOperatorTensor> for ::windows::runtime::IUnknown {
    fn from(value: &IMLOperatorTensor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMLOperatorTensor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMLOperatorTensor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTensor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dimensioncount: u32, dimensions: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> MLOperatorTensorDataType,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> bool,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> bool,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> *mut ::std::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datainterface: *mut ::windows::runtime::RawPtr),
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMLOperatorTensorShapeDescription(::windows::runtime::IUnknown);
impl IMLOperatorTensorShapeDescription {
    pub unsafe fn GetInputTensorDimensionCount(&self, inputindex: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(inputindex), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetInputTensorShape(&self, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(inputindex), ::std::mem::transmute(dimensioncount), ::std::mem::transmute(dimensions)).ok()
    }
    pub unsafe fn HasOutputShapeDescription(&self) -> bool {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)))
    }
    pub unsafe fn GetOutputTensorDimensionCount(&self, outputindex: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(outputindex), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOutputTensorShape(&self, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(outputindex), ::std::mem::transmute(dimensioncount), ::std::mem::transmute(dimensions)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMLOperatorTensorShapeDescription {
    type Vtable = IMLOperatorTensorShapeDescription_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4061039806, 15144, 16968, [190, 149, 249, 111, 188, 110, 70, 67]);
}
impl ::std::convert::From<IMLOperatorTensorShapeDescription> for ::windows::runtime::IUnknown {
    fn from(value: IMLOperatorTensorShapeDescription) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMLOperatorTensorShapeDescription> for ::windows::runtime::IUnknown {
    fn from(value: &IMLOperatorTensorShapeDescription) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMLOperatorTensorShapeDescription {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMLOperatorTensorShapeDescription {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTensorShapeDescription_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputindex: u32, dimensioncount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> bool,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputindex: u32, dimensioncount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMLOperatorTypeInferenceContext(::windows::runtime::IUnknown);
impl IMLOperatorTypeInferenceContext {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeElementCount<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(r#type), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttribute<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(elementcount), ::std::mem::transmute(elementbytesize), ::std::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringAttributeElementLength<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(elementindex), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringAttributeElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, name: Param0, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(elementindex), ::std::mem::transmute(attributeelementbytesize), ::std::mem::transmute(attributeelement)).ok()
    }
    pub unsafe fn GetInputCount(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)))
    }
    pub unsafe fn GetOutputCount(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)))
    }
    pub unsafe fn IsInputValid(&self, inputindex: u32) -> bool {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(inputindex)))
    }
    pub unsafe fn IsOutputValid(&self, outputindex: u32) -> bool {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(outputindex)))
    }
    pub unsafe fn GetInputEdgeDescription(&self, inputindex: u32) -> ::windows::runtime::Result<MLOperatorEdgeDescription> {
        let mut result__: <MLOperatorEdgeDescription as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(inputindex), &mut result__).from_abi::<MLOperatorEdgeDescription>(result__)
    }
    pub unsafe fn SetOutputEdgeDescription(&self, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(outputindex), ::std::mem::transmute(edgedescription)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMLOperatorTypeInferenceContext {
    type Vtable = IMLOperatorTypeInferenceContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3968416689, 63800, 17019, [132, 136, 200, 220, 247, 117, 241, 56]);
}
impl ::std::convert::From<IMLOperatorTypeInferenceContext> for ::windows::runtime::IUnknown {
    fn from(value: IMLOperatorTypeInferenceContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMLOperatorTypeInferenceContext> for ::windows::runtime::IUnknown {
    fn from(value: &IMLOperatorTypeInferenceContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMLOperatorTypeInferenceContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMLOperatorTypeInferenceContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMLOperatorTypeInferenceContext> for IMLOperatorAttributes {
    fn from(value: IMLOperatorTypeInferenceContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMLOperatorTypeInferenceContext> for IMLOperatorAttributes {
    fn from(value: &IMLOperatorTypeInferenceContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMLOperatorAttributes> for IMLOperatorTypeInferenceContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMLOperatorAttributes> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMLOperatorAttributes>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMLOperatorAttributes> for &IMLOperatorTypeInferenceContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMLOperatorAttributes> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMLOperatorAttributes>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTypeInferenceContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: super::super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputindex: u32) -> bool,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputindex: u32) -> bool,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMLOperatorTypeInferrer(::windows::runtime::IUnknown);
impl IMLOperatorTypeInferrer {
    pub unsafe fn InferOutputTypes<'a, Param0: ::windows::runtime::IntoParam<'a, IMLOperatorTypeInferenceContext>>(&self, context: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), context.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMLOperatorTypeInferrer {
    type Vtable = IMLOperatorTypeInferrer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2015030088, 39883, 18327, [191, 119, 139, 244, 85, 33, 123, 235]);
}
impl ::std::convert::From<IMLOperatorTypeInferrer> for ::windows::runtime::IUnknown {
    fn from(value: IMLOperatorTypeInferrer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMLOperatorTypeInferrer> for ::windows::runtime::IUnknown {
    fn from(value: &IMLOperatorTypeInferrer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMLOperatorTypeInferrer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMLOperatorTypeInferrer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTypeInferrer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWinMLEvaluationContext(::windows::runtime::IUnknown);
impl IWinMLEvaluationContext {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn BindValue(&self, pdescriptor: *const WINML_BINDING_DESC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdescriptor)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn GetValueByName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<*mut WINML_BINDING_DESC> {
        let mut result__: <*mut WINML_BINDING_DESC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<*mut WINML_BINDING_DESC>(result__)
    }
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWinMLEvaluationContext {
    type Vtable = IWinMLEvaluationContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2508492702, 22589, 16468, [175, 18, 145, 99, 135, 205, 132, 38]);
}
impl ::std::convert::From<IWinMLEvaluationContext> for ::windows::runtime::IUnknown {
    fn from(value: IWinMLEvaluationContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWinMLEvaluationContext> for ::windows::runtime::IUnknown {
    fn from(value: &IWinMLEvaluationContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWinMLEvaluationContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWinMLEvaluationContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLEvaluationContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdescriptor: *const ::std::mem::ManuallyDrop<WINML_BINDING_DESC>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR, pdescriptor: *mut *mut WINML_BINDING_DESC) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWinMLModel(::windows::runtime::IUnknown);
impl IWinMLModel {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<*mut WINML_MODEL_DESC> {
        let mut result__: <*mut WINML_MODEL_DESC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut WINML_MODEL_DESC>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateMetadata(&self, index: u32, pkey: *mut super::super::super::Foundation::PWSTR, pvalue: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), ::std::mem::transmute(pkey), ::std::mem::transmute(pvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateModelInputs(&self, index: u32) -> ::windows::runtime::Result<*mut WINML_VARIABLE_DESC> {
        let mut result__: <*mut WINML_VARIABLE_DESC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), &mut result__).from_abi::<*mut WINML_VARIABLE_DESC>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateModelOutputs(&self, index: u32) -> ::windows::runtime::Result<*mut WINML_VARIABLE_DESC> {
        let mut result__: <*mut WINML_VARIABLE_DESC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), &mut result__).from_abi::<*mut WINML_VARIABLE_DESC>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWinMLModel {
    type Vtable = IWinMLModel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3807295145, 62239, 16469, [165, 33, 227, 11, 91, 51, 102, 74]);
}
impl ::std::convert::From<IWinMLModel> for ::windows::runtime::IUnknown {
    fn from(value: IWinMLModel) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWinMLModel> for ::windows::runtime::IUnknown {
    fn from(value: &IWinMLModel) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWinMLModel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWinMLModel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLModel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdescription: *mut *mut WINML_MODEL_DESC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, pkey: *mut super::super::super::Foundation::PWSTR, pvalue: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, ppinputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, ppoutputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWinMLRuntime(::windows::runtime::IUnknown);
impl IWinMLRuntime {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadModel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, path: Param0) -> ::windows::runtime::Result<IWinMLModel> {
        let mut result__: <IWinMLModel as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<IWinMLModel>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateEvaluationContext<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>>(&self, device: Param0) -> ::windows::runtime::Result<IWinMLEvaluationContext> {
        let mut result__: <IWinMLEvaluationContext as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), device.into_param().abi(), &mut result__).from_abi::<IWinMLEvaluationContext>(result__)
    }
    pub unsafe fn EvaluateModel<'a, Param0: ::windows::runtime::IntoParam<'a, IWinMLEvaluationContext>>(&self, pcontext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pcontext.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWinMLRuntime {
    type Vtable = IWinMLRuntime_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2688701225, 16558, 18649, [188, 227, 130, 158, 247, 184, 164, 26]);
}
impl ::std::convert::From<IWinMLRuntime> for ::windows::runtime::IUnknown {
    fn from(value: IWinMLRuntime) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWinMLRuntime> for ::windows::runtime::IUnknown {
    fn from(value: &IWinMLRuntime) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWinMLRuntime {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWinMLRuntime {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLRuntime_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: super::super::super::Foundation::PWSTR, ppmodel: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr, ppcontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontext: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWinMLRuntimeFactory(::windows::runtime::IUnknown);
impl IWinMLRuntimeFactory {
    pub unsafe fn CreateRuntime(&self, runtimetype: WINML_RUNTIME_TYPE) -> ::windows::runtime::Result<IWinMLRuntime> {
        let mut result__: <IWinMLRuntime as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(runtimetype), &mut result__).from_abi::<IWinMLRuntime>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWinMLRuntimeFactory {
    type Vtable = IWinMLRuntimeFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2819078221, 19173, 19392, [167, 106, 148, 26, 162, 70, 189, 65]);
}
impl ::std::convert::From<IWinMLRuntimeFactory> for ::windows::runtime::IUnknown {
    fn from(value: IWinMLRuntimeFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWinMLRuntimeFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IWinMLRuntimeFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWinMLRuntimeFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWinMLRuntimeFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLRuntimeFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, runtimetype: WINML_RUNTIME_TYPE, ppruntime: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[inline]
pub unsafe fn MLCreateOperatorRegistry() -> ::windows::runtime::Result<IMLOperatorRegistry> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MLCreateOperatorRegistry(registry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IMLOperatorRegistry as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        MLCreateOperatorRegistry(&mut result__).from_abi::<IMLOperatorRegistry>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorAttribute {
    pub name: super::super::super::Foundation::PSTR,
    pub r#type: MLOperatorAttributeType,
    pub required: bool,
}
#[cfg(feature = "Win32_Foundation")]
impl MLOperatorAttribute {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MLOperatorAttribute {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MLOperatorAttribute {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MLOperatorAttribute").field("name", &self.name).field("r#type", &self.r#type).field("required", &self.required).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MLOperatorAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.r#type == other.r#type && self.required == other.required
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MLOperatorAttribute {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MLOperatorAttribute {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorAttributeNameValue {
    pub name: super::super::super::Foundation::PSTR,
    pub r#type: MLOperatorAttributeType,
    pub valueCount: u32,
    pub Anonymous: MLOperatorAttributeNameValue_0,
}
#[cfg(feature = "Win32_Foundation")]
impl MLOperatorAttributeNameValue {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MLOperatorAttributeNameValue {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MLOperatorAttributeNameValue {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MLOperatorAttributeNameValue {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MLOperatorAttributeNameValue {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union MLOperatorAttributeNameValue_0 {
    pub reserved: *mut ::std::ffi::c_void,
    pub ints: *mut i64,
    pub strings: *mut *mut i8,
    pub floats: *mut f32,
}
impl MLOperatorAttributeNameValue_0 {}
impl ::std::default::Default for MLOperatorAttributeNameValue_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MLOperatorAttributeNameValue_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MLOperatorAttributeNameValue_0 {}
unsafe impl ::windows::runtime::Abi for MLOperatorAttributeNameValue_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<u32> for MLOperatorAttributeType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MLOperatorAttributeType {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MLOperatorAttributeType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MLOperatorAttributeType {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MLOperatorAttributeType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MLOperatorAttributeType {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MLOperatorAttributeType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MLOperatorEdgeDescription {
    pub edgeType: MLOperatorEdgeType,
    pub Anonymous: MLOperatorEdgeDescription_0,
}
impl MLOperatorEdgeDescription {}
impl ::std::default::Default for MLOperatorEdgeDescription {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MLOperatorEdgeDescription {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MLOperatorEdgeDescription {}
unsafe impl ::windows::runtime::Abi for MLOperatorEdgeDescription {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union MLOperatorEdgeDescription_0 {
    pub reserved: u64,
    pub tensorDataType: MLOperatorTensorDataType,
}
impl MLOperatorEdgeDescription_0 {}
impl ::std::default::Default for MLOperatorEdgeDescription_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MLOperatorEdgeDescription_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MLOperatorEdgeDescription_0 {}
unsafe impl ::windows::runtime::Abi for MLOperatorEdgeDescription_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MLOperatorEdgeType(pub u32);
impl MLOperatorEdgeType {
    pub const Undefined: MLOperatorEdgeType = MLOperatorEdgeType(0u32);
    pub const Tensor: MLOperatorEdgeType = MLOperatorEdgeType(1u32);
}
impl ::std::convert::From<u32> for MLOperatorEdgeType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MLOperatorEdgeType {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MLOperatorEdgeType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MLOperatorEdgeType {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MLOperatorEdgeType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MLOperatorEdgeType {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MLOperatorEdgeType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorEdgeTypeConstraint {
    pub typeLabel: super::super::super::Foundation::PSTR,
    pub allowedTypes: *mut MLOperatorEdgeDescription,
    pub allowedTypeCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MLOperatorEdgeTypeConstraint {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MLOperatorEdgeTypeConstraint {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MLOperatorEdgeTypeConstraint {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MLOperatorEdgeTypeConstraint").field("typeLabel", &self.typeLabel).field("allowedTypes", &self.allowedTypes).field("allowedTypeCount", &self.allowedTypeCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MLOperatorEdgeTypeConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.typeLabel == other.typeLabel && self.allowedTypes == other.allowedTypes && self.allowedTypeCount == other.allowedTypeCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MLOperatorEdgeTypeConstraint {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MLOperatorEdgeTypeConstraint {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MLOperatorExecutionType(pub u32);
impl MLOperatorExecutionType {
    pub const Undefined: MLOperatorExecutionType = MLOperatorExecutionType(0u32);
    pub const Cpu: MLOperatorExecutionType = MLOperatorExecutionType(1u32);
    pub const D3D12: MLOperatorExecutionType = MLOperatorExecutionType(2u32);
}
impl ::std::convert::From<u32> for MLOperatorExecutionType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MLOperatorExecutionType {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MLOperatorExecutionType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MLOperatorExecutionType {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MLOperatorExecutionType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MLOperatorExecutionType {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MLOperatorExecutionType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl MLOperatorKernelDescription {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MLOperatorKernelDescription {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MLOperatorKernelDescription {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for MLOperatorKernelDescription {
    fn eq(&self, other: &Self) -> bool {
        self.domain == other.domain && self.name == other.name && self.minimumOperatorSetVersion == other.minimumOperatorSetVersion && self.executionType == other.executionType && self.typeConstraints == other.typeConstraints && self.typeConstraintCount == other.typeConstraintCount && self.defaultAttributes == other.defaultAttributes && self.defaultAttributeCount == other.defaultAttributeCount && self.options == other.options && self.executionOptions == other.executionOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MLOperatorKernelDescription {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MLOperatorKernelDescription {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MLOperatorKernelOptions(pub u32);
impl MLOperatorKernelOptions {
    pub const None: MLOperatorKernelOptions = MLOperatorKernelOptions(0u32);
    pub const AllowDynamicInputShapes: MLOperatorKernelOptions = MLOperatorKernelOptions(1u32);
}
impl ::std::convert::From<u32> for MLOperatorKernelOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MLOperatorKernelOptions {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MLOperatorKernelOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MLOperatorKernelOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MLOperatorKernelOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MLOperatorKernelOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MLOperatorKernelOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MLOperatorParameterOptions(pub u32);
impl MLOperatorParameterOptions {
    pub const Single: MLOperatorParameterOptions = MLOperatorParameterOptions(0u32);
    pub const Optional: MLOperatorParameterOptions = MLOperatorParameterOptions(1u32);
    pub const Variadic: MLOperatorParameterOptions = MLOperatorParameterOptions(2u32);
}
impl ::std::convert::From<u32> for MLOperatorParameterOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MLOperatorParameterOptions {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MLOperatorParameterOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MLOperatorParameterOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MLOperatorParameterOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MLOperatorParameterOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MLOperatorParameterOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl MLOperatorSchemaDescription {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MLOperatorSchemaDescription {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MLOperatorSchemaDescription {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for MLOperatorSchemaDescription {
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
impl ::std::cmp::Eq for MLOperatorSchemaDescription {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MLOperatorSchemaDescription {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorSchemaEdgeDescription {
    pub options: MLOperatorParameterOptions,
    pub typeFormat: MLOperatorSchemaEdgeTypeFormat,
    pub Anonymous: MLOperatorSchemaEdgeDescription_0,
}
#[cfg(feature = "Win32_Foundation")]
impl MLOperatorSchemaEdgeDescription {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MLOperatorSchemaEdgeDescription {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MLOperatorSchemaEdgeDescription {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MLOperatorSchemaEdgeDescription {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MLOperatorSchemaEdgeDescription {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union MLOperatorSchemaEdgeDescription_0 {
    pub reserved: *mut ::std::ffi::c_void,
    pub typeLabel: super::super::super::Foundation::PSTR,
    pub edgeDescription: MLOperatorEdgeDescription,
}
#[cfg(feature = "Win32_Foundation")]
impl MLOperatorSchemaEdgeDescription_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MLOperatorSchemaEdgeDescription_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MLOperatorSchemaEdgeDescription_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MLOperatorSchemaEdgeDescription_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MLOperatorSchemaEdgeDescription_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MLOperatorSchemaEdgeTypeFormat(pub i32);
impl MLOperatorSchemaEdgeTypeFormat {
    pub const EdgeDescription: MLOperatorSchemaEdgeTypeFormat = MLOperatorSchemaEdgeTypeFormat(0i32);
    pub const Label: MLOperatorSchemaEdgeTypeFormat = MLOperatorSchemaEdgeTypeFormat(1i32);
}
impl ::std::convert::From<i32> for MLOperatorSchemaEdgeTypeFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MLOperatorSchemaEdgeTypeFormat {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorSetId {
    pub domain: super::super::super::Foundation::PSTR,
    pub version: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl MLOperatorSetId {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MLOperatorSetId {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MLOperatorSetId {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MLOperatorSetId").field("domain", &self.domain).field("version", &self.version).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MLOperatorSetId {
    fn eq(&self, other: &Self) -> bool {
        self.domain == other.domain && self.version == other.version
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MLOperatorSetId {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MLOperatorSetId {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<u32> for MLOperatorTensorDataType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MLOperatorTensorDataType {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MLOperatorTensorDataType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MLOperatorTensorDataType {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MLOperatorTensorDataType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MLOperatorTensorDataType {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MLOperatorTensorDataType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::std::clone::Clone for WINML_BINDING_DESC {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub struct WINML_BINDING_DESC {
    pub Name: super::super::super::Foundation::PWSTR,
    pub BindType: WINML_BINDING_TYPE,
    pub Anonymous: WINML_BINDING_DESC_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl WINML_BINDING_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::std::default::Default for WINML_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::std::cmp::PartialEq for WINML_BINDING_DESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::std::cmp::Eq for WINML_BINDING_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
unsafe impl ::windows::runtime::Abi for WINML_BINDING_DESC {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::std::clone::Clone for WINML_BINDING_DESC_0 {
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
    pub Resource: ::std::mem::ManuallyDrop<WINML_RESOURCE_BINDING_DESC>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl WINML_BINDING_DESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::std::default::Default for WINML_BINDING_DESC_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::std::cmp::PartialEq for WINML_BINDING_DESC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::std::cmp::Eq for WINML_BINDING_DESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
unsafe impl ::windows::runtime::Abi for WINML_BINDING_DESC_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINML_BINDING_TYPE(pub i32);
pub const WINML_BINDING_UNDEFINED: WINML_BINDING_TYPE = WINML_BINDING_TYPE(0i32);
pub const WINML_BINDING_TENSOR: WINML_BINDING_TYPE = WINML_BINDING_TYPE(1i32);
pub const WINML_BINDING_SEQUENCE: WINML_BINDING_TYPE = WINML_BINDING_TYPE(2i32);
pub const WINML_BINDING_MAP: WINML_BINDING_TYPE = WINML_BINDING_TYPE(3i32);
pub const WINML_BINDING_IMAGE: WINML_BINDING_TYPE = WINML_BINDING_TYPE(4i32);
pub const WINML_BINDING_RESOURCE: WINML_BINDING_TYPE = WINML_BINDING_TYPE(5i32);
impl ::std::convert::From<i32> for WINML_BINDING_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINML_BINDING_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINML_FEATURE_TYPE(pub i32);
pub const WINML_FEATURE_UNDEFINED: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(0i32);
pub const WINML_FEATURE_TENSOR: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(1i32);
pub const WINML_FEATURE_SEQUENCE: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(2i32);
pub const WINML_FEATURE_MAP: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(3i32);
pub const WINML_FEATURE_IMAGE: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(4i32);
impl ::std::convert::From<i32> for WINML_FEATURE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINML_FEATURE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WINML_IMAGE_BINDING_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub DataSize: u32,
    pub pData: *mut ::std::ffi::c_void,
}
impl WINML_IMAGE_BINDING_DESC {}
impl ::std::default::Default for WINML_IMAGE_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINML_IMAGE_BINDING_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINML_IMAGE_BINDING_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).field("DataSize", &self.DataSize).field("pData", &self.pData).finish()
    }
}
impl ::std::cmp::PartialEq for WINML_IMAGE_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape && self.DataSize == other.DataSize && self.pData == other.pData
    }
}
impl ::std::cmp::Eq for WINML_IMAGE_BINDING_DESC {}
unsafe impl ::windows::runtime::Abi for WINML_IMAGE_BINDING_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WINML_IMAGE_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
}
impl WINML_IMAGE_VARIABLE_DESC {}
impl ::std::default::Default for WINML_IMAGE_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINML_IMAGE_VARIABLE_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINML_IMAGE_VARIABLE_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).finish()
    }
}
impl ::std::cmp::PartialEq for WINML_IMAGE_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape
    }
}
impl ::std::cmp::Eq for WINML_IMAGE_VARIABLE_DESC {}
unsafe impl ::windows::runtime::Abi for WINML_IMAGE_VARIABLE_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::std::default::Default for WINML_MAP_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINML_MAP_BINDING_DESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINML_MAP_BINDING_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINML_MAP_BINDING_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WINML_MAP_BINDING_DESC_0 {
    pub pStringKeys: *mut super::super::super::Foundation::PWSTR,
    pub pIntKeys: *mut i64,
}
#[cfg(feature = "Win32_Foundation")]
impl WINML_MAP_BINDING_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINML_MAP_BINDING_DESC_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINML_MAP_BINDING_DESC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINML_MAP_BINDING_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINML_MAP_BINDING_DESC_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for WINML_MAP_BINDING_DESC_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINML_MAP_BINDING_DESC_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINML_MAP_BINDING_DESC_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINML_MAP_BINDING_DESC_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WINML_MAP_VARIABLE_DESC {
    pub KeyType: WINML_TENSOR_DATA_TYPE,
    pub Fields: WINML_TENSOR_DATA_TYPE,
}
impl WINML_MAP_VARIABLE_DESC {}
impl ::std::default::Default for WINML_MAP_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINML_MAP_VARIABLE_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINML_MAP_VARIABLE_DESC").field("KeyType", &self.KeyType).field("Fields", &self.Fields).finish()
    }
}
impl ::std::cmp::PartialEq for WINML_MAP_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.KeyType == other.KeyType && self.Fields == other.Fields
    }
}
impl ::std::cmp::Eq for WINML_MAP_VARIABLE_DESC {}
unsafe impl ::windows::runtime::Abi for WINML_MAP_VARIABLE_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::std::default::Default for WINML_MODEL_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WINML_MODEL_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINML_MODEL_DESC").field("Author", &self.Author).field("Name", &self.Name).field("Domain", &self.Domain).field("Description", &self.Description).field("Version", &self.Version).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINML_MODEL_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Author == other.Author && self.Name == other.Name && self.Domain == other.Domain && self.Description == other.Description && self.Version == other.Version
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINML_MODEL_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINML_MODEL_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct WINML_RESOURCE_BINDING_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub pResource: ::std::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl WINML_RESOURCE_BINDING_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::std::default::Default for WINML_RESOURCE_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::std::fmt::Debug for WINML_RESOURCE_BINDING_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINML_RESOURCE_BINDING_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).field("pResource", &self.pResource).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::std::cmp::PartialEq for WINML_RESOURCE_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape && self.pResource == other.pResource
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::std::cmp::Eq for WINML_RESOURCE_BINDING_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
unsafe impl ::windows::runtime::Abi for WINML_RESOURCE_BINDING_DESC {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINML_RUNTIME_TYPE(pub i32);
pub const WINML_RUNTIME_CNTK: WINML_RUNTIME_TYPE = WINML_RUNTIME_TYPE(0i32);
impl ::std::convert::From<i32> for WINML_RUNTIME_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINML_RUNTIME_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WINML_SEQUENCE_BINDING_DESC {
    pub ElementCount: u32,
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub Anonymous: WINML_SEQUENCE_BINDING_DESC_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WINML_SEQUENCE_BINDING_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINML_SEQUENCE_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINML_SEQUENCE_BINDING_DESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINML_SEQUENCE_BINDING_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINML_SEQUENCE_BINDING_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for WINML_SEQUENCE_BINDING_DESC_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINML_SEQUENCE_BINDING_DESC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINML_SEQUENCE_BINDING_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINML_SEQUENCE_BINDING_DESC_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WINML_SEQUENCE_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
}
impl WINML_SEQUENCE_VARIABLE_DESC {}
impl ::std::default::Default for WINML_SEQUENCE_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINML_SEQUENCE_VARIABLE_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINML_SEQUENCE_VARIABLE_DESC").field("ElementType", &self.ElementType).finish()
    }
}
impl ::std::cmp::PartialEq for WINML_SEQUENCE_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType
    }
}
impl ::std::cmp::Eq for WINML_SEQUENCE_VARIABLE_DESC {}
unsafe impl ::windows::runtime::Abi for WINML_SEQUENCE_VARIABLE_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WINML_TENSOR_BINDING_DESC {
    pub DataType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub DataSize: u32,
    pub pData: *mut ::std::ffi::c_void,
}
impl WINML_TENSOR_BINDING_DESC {}
impl ::std::default::Default for WINML_TENSOR_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINML_TENSOR_BINDING_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINML_TENSOR_BINDING_DESC").field("DataType", &self.DataType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).field("DataSize", &self.DataSize).field("pData", &self.pData).finish()
    }
}
impl ::std::cmp::PartialEq for WINML_TENSOR_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.DataType == other.DataType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape && self.DataSize == other.DataSize && self.pData == other.pData
    }
}
impl ::std::cmp::Eq for WINML_TENSOR_BINDING_DESC {}
unsafe impl ::windows::runtime::Abi for WINML_TENSOR_BINDING_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for WINML_TENSOR_DATA_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINML_TENSOR_DATA_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WINML_TENSOR_DIMENSION_COUNT_MAX: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WINML_TENSOR_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
}
impl WINML_TENSOR_VARIABLE_DESC {}
impl ::std::default::Default for WINML_TENSOR_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINML_TENSOR_VARIABLE_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINML_TENSOR_VARIABLE_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).finish()
    }
}
impl ::std::cmp::PartialEq for WINML_TENSOR_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape
    }
}
impl ::std::cmp::Eq for WINML_TENSOR_VARIABLE_DESC {}
unsafe impl ::windows::runtime::Abi for WINML_TENSOR_VARIABLE_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::std::default::Default for WINML_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINML_VARIABLE_DESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINML_VARIABLE_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINML_VARIABLE_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WINML_VARIABLE_DESC_0 {
    pub Tensor: WINML_TENSOR_VARIABLE_DESC,
    pub Sequence: WINML_SEQUENCE_VARIABLE_DESC,
    pub Map: WINML_MAP_VARIABLE_DESC,
    pub Image: WINML_IMAGE_VARIABLE_DESC,
}
impl WINML_VARIABLE_DESC_0 {}
impl ::std::default::Default for WINML_VARIABLE_DESC_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINML_VARIABLE_DESC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINML_VARIABLE_DESC_0 {}
unsafe impl ::windows::runtime::Abi for WINML_VARIABLE_DESC_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[inline]
pub unsafe fn WinMLCreateRuntime() -> ::windows::runtime::Result<IWinMLRuntime> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinMLCreateRuntime(runtime: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWinMLRuntime as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WinMLCreateRuntime(&mut result__).from_abi::<IWinMLRuntime>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
