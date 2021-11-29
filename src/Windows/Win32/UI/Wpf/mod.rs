#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_MILBitmapEffectBevel: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd361dbe_6c9b_4de0_8290_f6400c2737ed);
pub const CLSID_MILBitmapEffectBlur: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa924df87_225d_4373_8f5b_b90ec85ae3de);
pub const CLSID_MILBitmapEffectDropShadow: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x459a3fbe_d8ac_4692_874b_7a265715aa16);
pub const CLSID_MILBitmapEffectEmboss: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd299846_824f_47ec_a007_12aa767f2816);
pub const CLSID_MILBitmapEffectGroup: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac9c1a9a_7e18_4f64_ac7e_47cf7f051e95);
pub const CLSID_MILBitmapEffectOuterGlow: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2161bdd_7eb6_4725_9c0b_8a2a1b4f0667);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffect(pub ::windows::core::IUnknown);
impl IMILBitmapEffect {
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutput<'a, Param1: ::windows::core::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, uiindex: u32, pcontext: Param1) -> ::windows::core::Result<super::super::Graphics::Imaging::IWICBitmapSource> {
        let mut result__: <super::super::Graphics::Imaging::IWICBitmapSource as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), pcontext.into_param().abi(), &mut result__).from_abi::<super::super::Graphics::Imaging::IWICBitmapSource>(result__)
    }
    pub unsafe fn GetParentEffect(&self) -> ::windows::core::Result<IMILBitmapEffectGroup> {
        let mut result__: <IMILBitmapEffectGroup as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffectGroup>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn SetInputSource<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::IWICBitmapSource>>(&self, uiindex: u32, pbitmapsource: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), pbitmapsource.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffect {
    type Vtable = IMILBitmapEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a6ff321_c944_4a1b_9944_9954af301258);
}
impl ::core::convert::From<IMILBitmapEffect> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffect> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, pcontext: ::windows::core::RawPtr, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppparenteffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, pbitmapsource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffectConnections(pub ::windows::core::IUnknown);
impl IMILBitmapEffectConnections {
    pub unsafe fn GetInputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectInputConnector> {
        let mut result__: <IMILBitmapEffectInputConnector as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), &mut result__).from_abi::<IMILBitmapEffectInputConnector>(result__)
    }
    pub unsafe fn GetOutputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__: <IMILBitmapEffectOutputConnector as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), &mut result__).from_abi::<IMILBitmapEffectOutputConnector>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectConnections {
    type Vtable = IMILBitmapEffectConnections_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2b5d861_9b1a_4374_89b0_dec4874d6a81);
}
impl ::core::convert::From<IMILBitmapEffectConnections> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectConnections) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffectConnections> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectConnections) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectConnections {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectConnections {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnections_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffectConnectionsInfo(pub ::windows::core::IUnknown);
impl IMILBitmapEffectConnectionsInfo {
    pub unsafe fn GetNumberInputs(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetNumberOutputs(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetInputConnectorInfo(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectConnectorInfo> {
        let mut result__: <IMILBitmapEffectConnectorInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), &mut result__).from_abi::<IMILBitmapEffectConnectorInfo>(result__)
    }
    pub unsafe fn GetOutputConnectorInfo(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectConnectorInfo> {
        let mut result__: <IMILBitmapEffectConnectorInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), &mut result__).from_abi::<IMILBitmapEffectConnectorInfo>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectConnectionsInfo {
    type Vtable = IMILBitmapEffectConnectionsInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x476b538a_c765_4237_ba4a_d6a880ff0cfc);
}
impl ::core::convert::From<IMILBitmapEffectConnectionsInfo> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectConnectionsInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffectConnectionsInfo> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectConnectionsInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectConnectionsInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectConnectionsInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnectionsInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puinuminputs: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puinumoutputs: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, ppconnectorinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, ppconnectorinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffectConnector(pub ::windows::core::IUnknown);
impl IMILBitmapEffectConnector {
    pub unsafe fn GetIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulindex), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn GetBitmapEffect(&self) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__: <IMILBitmapEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffect>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectConnector {
    type Vtable = IMILBitmapEffectConnector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf59567b3_76c1_4d47_ba1e_79f955e350ef);
}
impl ::core::convert::From<IMILBitmapEffectConnector> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectConnector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffectConnector> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectConnector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMILBitmapEffectConnector> for IMILBitmapEffectConnectorInfo {
    fn from(value: IMILBitmapEffectConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectConnector> for IMILBitmapEffectConnectorInfo {
    fn from(value: &IMILBitmapEffectConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMILBitmapEffectConnectorInfo> for IMILBitmapEffectConnector {
    fn into_param(self) -> ::windows::core::Param<'a, IMILBitmapEffectConnectorInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMILBitmapEffectConnectorInfo> for &IMILBitmapEffectConnector {
    fn into_param(self) -> ::windows::core::Param<'a, IMILBitmapEffectConnectorInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puiindex: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pulnumberformats: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulindex: u32, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfconnected: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffectConnectorInfo(pub ::windows::core::IUnknown);
impl IMILBitmapEffectConnectorInfo {
    pub unsafe fn GetIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulindex), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectConnectorInfo {
    type Vtable = IMILBitmapEffectConnectorInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf66d2e4b_b46b_42fc_859e_3da0ecdb3c43);
}
impl ::core::convert::From<IMILBitmapEffectConnectorInfo> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectConnectorInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffectConnectorInfo> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectConnectorInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectConnectorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectConnectorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnectorInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puiindex: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pulnumberformats: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulindex: u32, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffectEvents(pub ::windows::core::IUnknown);
impl IMILBitmapEffectEvents {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PropertyChange<'a, Param0: ::windows::core::IntoParam<'a, IMILBitmapEffect>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, peffect: Param0, bstrpropertyname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), peffect.into_param().abi(), bstrpropertyname.into_param().abi()).ok()
    }
    pub unsafe fn DirtyRegion<'a, Param0: ::windows::core::IntoParam<'a, IMILBitmapEffect>>(&self, peffect: Param0, prect: *const MilRectD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), peffect.into_param().abi(), ::core::mem::transmute(prect)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectEvents {
    type Vtable = IMILBitmapEffectEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e880dd8_f8ce_457b_8199_d60bb3d7ef98);
}
impl ::core::convert::From<IMILBitmapEffectEvents> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffectEvents> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, peffect: ::windows::core::RawPtr, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, peffect: ::windows::core::RawPtr, prect: *const MilRectD) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffectFactory(pub ::windows::core::IUnknown);
impl IMILBitmapEffectFactory {
    pub unsafe fn CreateEffect(&self, pguideffect: *const ::windows::core::GUID) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__: <IMILBitmapEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguideffect), &mut result__).from_abi::<IMILBitmapEffect>(result__)
    }
    pub unsafe fn CreateContext(&self) -> ::windows::core::Result<IMILBitmapEffectRenderContext> {
        let mut result__: <IMILBitmapEffectRenderContext as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffectRenderContext>(result__)
    }
    pub unsafe fn CreateEffectOuter(&self) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__: <IMILBitmapEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffect>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectFactory {
    type Vtable = IMILBitmapEffectFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33a9df34_a403_4ec7_b07e_bc0682370845);
}
impl ::core::convert::From<IMILBitmapEffectFactory> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffectFactory> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pguideffect: *const ::windows::core::GUID, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffectGroup(pub ::windows::core::IUnknown);
impl IMILBitmapEffectGroup {
    pub unsafe fn GetInteriorInputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__: <IMILBitmapEffectOutputConnector as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), &mut result__).from_abi::<IMILBitmapEffectOutputConnector>(result__)
    }
    pub unsafe fn GetInteriorOutputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectInputConnector> {
        let mut result__: <IMILBitmapEffectInputConnector as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), &mut result__).from_abi::<IMILBitmapEffectInputConnector>(result__)
    }
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, IMILBitmapEffect>>(&self, peffect: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), peffect.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectGroup {
    type Vtable = IMILBitmapEffectGroup_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f952360_698a_4ac6_81a1_bcfdf08eb8e8);
}
impl ::core::convert::From<IMILBitmapEffectGroup> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectGroup) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffectGroup> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, peffect: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffectGroupImpl(pub ::windows::core::IUnknown);
impl IMILBitmapEffectGroupImpl {
    pub unsafe fn Preprocess<'a, Param0: ::windows::core::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, pcontext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcontext.into_param().abi()).ok()
    }
    pub unsafe fn GetNumberChildren(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetChildren(&self) -> ::windows::core::Result<IMILBitmapEffects> {
        let mut result__: <IMILBitmapEffects as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffects>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectGroupImpl {
    type Vtable = IMILBitmapEffectGroupImpl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78fed518_1cfc_4807_8b85_6b6e51398f62);
}
impl ::core::convert::From<IMILBitmapEffectGroupImpl> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectGroupImpl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffectGroupImpl> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectGroupImpl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectGroupImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectGroupImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectGroupImpl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puinumberchildren: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pchildren: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffectImpl(pub ::windows::core::IUnknown);
impl IMILBitmapEffectImpl {
    pub unsafe fn IsInPlaceModificationAllowed<'a, Param0: ::windows::core::IntoParam<'a, IMILBitmapEffectOutputConnector>>(&self, poutputconnector: Param0) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), poutputconnector.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetParentEffect<'a, Param0: ::windows::core::IntoParam<'a, IMILBitmapEffectGroup>>(&self, pparenteffect: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pparenteffect.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetInputSource(&self, uiindex: u32) -> ::windows::core::Result<super::super::Graphics::Imaging::IWICBitmapSource> {
        let mut result__: <super::super::Graphics::Imaging::IWICBitmapSource as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), &mut result__).from_abi::<super::super::Graphics::Imaging::IWICBitmapSource>(result__)
    }
    pub unsafe fn GetInputSourceBounds(&self, uiindex: u32) -> ::windows::core::Result<MilRectD> {
        let mut result__: <MilRectD as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), &mut result__).from_abi::<MilRectD>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetInputBitmapSource<'a, Param1: ::windows::core::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, uiindex: u32, prendercontext: Param1, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), prendercontext.into_param().abi(), ::core::mem::transmute(pfmodifyinplace), ::core::mem::transmute(ppbitmapsource)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutputBitmapSource<'a, Param1: ::windows::core::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, uiindex: u32, prendercontext: Param1, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), prendercontext.into_param().abi(), ::core::mem::transmute(pfmodifyinplace), ::core::mem::transmute(ppbitmapsource)).ok()
    }
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pinner: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pinner.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectImpl {
    type Vtable = IMILBitmapEffectImpl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc2468f2_9936_47be_b4af_06b5df5dbcbb);
}
impl ::core::convert::From<IMILBitmapEffectImpl> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectImpl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffectImpl> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectImpl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectImpl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poutputconnector: ::windows::core::RawPtr, pfmodifyinplace: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pparenteffect: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, prect: *mut MilRectD) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, prendercontext: ::windows::core::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))] usize,
    #[cfg(feature = "Win32_Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, prendercontext: ::windows::core::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinner: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffectInputConnector(pub ::windows::core::IUnknown);
impl IMILBitmapEffectInputConnector {
    pub unsafe fn GetIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulindex), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn GetBitmapEffect(&self) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__: <IMILBitmapEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffect>(result__)
    }
    pub unsafe fn ConnectTo<'a, Param0: ::windows::core::IntoParam<'a, IMILBitmapEffectOutputConnector>>(&self, pconnector: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pconnector.into_param().abi()).ok()
    }
    pub unsafe fn GetConnection(&self) -> ::windows::core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__: <IMILBitmapEffectOutputConnector as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffectOutputConnector>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectInputConnector {
    type Vtable = IMILBitmapEffectInputConnector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9b4ecaa_7a3c_45e7_8573_f4b81b60dd6c);
}
impl ::core::convert::From<IMILBitmapEffectInputConnector> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectInputConnector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffectInputConnector> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectInputConnector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectInputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectInputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMILBitmapEffectInputConnector> for IMILBitmapEffectConnector {
    fn from(value: IMILBitmapEffectInputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectInputConnector> for IMILBitmapEffectConnector {
    fn from(value: &IMILBitmapEffectInputConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMILBitmapEffectConnector> for IMILBitmapEffectInputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, IMILBitmapEffectConnector> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMILBitmapEffectConnector> for &IMILBitmapEffectInputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, IMILBitmapEffectConnector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMILBitmapEffectInputConnector> for IMILBitmapEffectConnectorInfo {
    fn from(value: IMILBitmapEffectInputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectInputConnector> for IMILBitmapEffectConnectorInfo {
    fn from(value: &IMILBitmapEffectInputConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMILBitmapEffectConnectorInfo> for IMILBitmapEffectInputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, IMILBitmapEffectConnectorInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMILBitmapEffectConnectorInfo> for &IMILBitmapEffectInputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, IMILBitmapEffectConnectorInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectInputConnector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puiindex: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pulnumberformats: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulindex: u32, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfconnected: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pconnector: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffectInteriorInputConnector(pub ::windows::core::IUnknown);
impl IMILBitmapEffectInteriorInputConnector {
    pub unsafe fn GetInputConnector(&self) -> ::windows::core::Result<IMILBitmapEffectInputConnector> {
        let mut result__: <IMILBitmapEffectInputConnector as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffectInputConnector>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectInteriorInputConnector {
    type Vtable = IMILBitmapEffectInteriorInputConnector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20287e9e_86a2_4e15_953d_eb1438a5b842);
}
impl ::core::convert::From<IMILBitmapEffectInteriorInputConnector> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectInteriorInputConnector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffectInteriorInputConnector> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectInteriorInputConnector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectInteriorInputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectInteriorInputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectInteriorInputConnector_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinputconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffectInteriorOutputConnector(pub ::windows::core::IUnknown);
impl IMILBitmapEffectInteriorOutputConnector {
    pub unsafe fn GetOutputConnector(&self) -> ::windows::core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__: <IMILBitmapEffectOutputConnector as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffectOutputConnector>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectInteriorOutputConnector {
    type Vtable = IMILBitmapEffectInteriorOutputConnector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00bbb6dc_acc9_4bfc_b344_8bee383dfefa);
}
impl ::core::convert::From<IMILBitmapEffectInteriorOutputConnector> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectInteriorOutputConnector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffectInteriorOutputConnector> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectInteriorOutputConnector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectInteriorOutputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectInteriorOutputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectInteriorOutputConnector_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poutputconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffectOutputConnector(pub ::windows::core::IUnknown);
impl IMILBitmapEffectOutputConnector {
    pub unsafe fn GetIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulindex), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn GetBitmapEffect(&self) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__: <IMILBitmapEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffect>(result__)
    }
    pub unsafe fn GetNumberConnections(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetConnection(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectInputConnector> {
        let mut result__: <IMILBitmapEffectInputConnector as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), &mut result__).from_abi::<IMILBitmapEffectInputConnector>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectOutputConnector {
    type Vtable = IMILBitmapEffectOutputConnector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92957aad_841b_4866_82ec_8752468b07fd);
}
impl ::core::convert::From<IMILBitmapEffectOutputConnector> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectOutputConnector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffectOutputConnector> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectOutputConnector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectOutputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectOutputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMILBitmapEffectOutputConnector> for IMILBitmapEffectConnector {
    fn from(value: IMILBitmapEffectOutputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectOutputConnector> for IMILBitmapEffectConnector {
    fn from(value: &IMILBitmapEffectOutputConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMILBitmapEffectConnector> for IMILBitmapEffectOutputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, IMILBitmapEffectConnector> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMILBitmapEffectConnector> for &IMILBitmapEffectOutputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, IMILBitmapEffectConnector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMILBitmapEffectOutputConnector> for IMILBitmapEffectConnectorInfo {
    fn from(value: IMILBitmapEffectOutputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectOutputConnector> for IMILBitmapEffectConnectorInfo {
    fn from(value: &IMILBitmapEffectOutputConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMILBitmapEffectConnectorInfo> for IMILBitmapEffectOutputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, IMILBitmapEffectConnectorInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMILBitmapEffectConnectorInfo> for &IMILBitmapEffectOutputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, IMILBitmapEffectConnectorInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectOutputConnector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puiindex: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pulnumberformats: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulindex: u32, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfconnected: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puinumberconnections: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, ppconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffectOutputConnectorImpl(pub ::windows::core::IUnknown);
impl IMILBitmapEffectOutputConnectorImpl {
    pub unsafe fn AddBackLink<'a, Param0: ::windows::core::IntoParam<'a, IMILBitmapEffectInputConnector>>(&self, pconnection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pconnection.into_param().abi()).ok()
    }
    pub unsafe fn RemoveBackLink<'a, Param0: ::windows::core::IntoParam<'a, IMILBitmapEffectInputConnector>>(&self, pconnection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pconnection.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectOutputConnectorImpl {
    type Vtable = IMILBitmapEffectOutputConnectorImpl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21fae777_8b39_4bfa_9f2d_f3941ed36913);
}
impl ::core::convert::From<IMILBitmapEffectOutputConnectorImpl> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectOutputConnectorImpl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffectOutputConnectorImpl> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectOutputConnectorImpl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectOutputConnectorImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectOutputConnectorImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectOutputConnectorImpl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffectPrimitive(pub ::windows::core::IUnknown);
impl IMILBitmapEffectPrimitive {
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutput<'a, Param1: ::windows::core::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, uiindex: u32, pcontext: Param1, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), pcontext.into_param().abi(), ::core::mem::transmute(pfmodifyinplace), ::core::mem::transmute(ppbitmapsource)).ok()
    }
    pub unsafe fn TransformPoint<'a, Param3: ::windows::core::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: i16, pcontext: Param3, pfpointtransformed: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), ::core::mem::transmute(p), ::core::mem::transmute(fforwardtransform), pcontext.into_param().abi(), ::core::mem::transmute(pfpointtransformed)).ok()
    }
    pub unsafe fn TransformRect<'a, Param3: ::windows::core::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, uiindex: u32, p: *mut MilRectD, fforwardtransform: i16, pcontext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), ::core::mem::transmute(p), ::core::mem::transmute(fforwardtransform), pcontext.into_param().abi()).ok()
    }
    pub unsafe fn HasAffineTransform(&self, uiindex: u32) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn HasInverseTransform(&self, uiindex: u32) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dwm")]
    pub unsafe fn GetAffineMatrix(&self, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), ::core::mem::transmute(pmatrix)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectPrimitive {
    type Vtable = IMILBitmapEffectPrimitive_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67e31025_3091_4dfc_98d6_dd494551461d);
}
impl ::core::convert::From<IMILBitmapEffectPrimitive> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectPrimitive) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffectPrimitive> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectPrimitive) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectPrimitive {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectPrimitive {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectPrimitive_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, pcontext: ::windows::core::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: i16, pcontext: ::windows::core::RawPtr, pfpointtransformed: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, p: *mut MilRectD, fforwardtransform: i16, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, pfaffine: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, pfhasinverse: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dwm")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dwm"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffectPrimitiveImpl(pub ::windows::core::IUnknown);
impl IMILBitmapEffectPrimitiveImpl {
    pub unsafe fn IsDirty(&self, uioutputindex: u32) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uioutputindex), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn IsVolatile(&self, uioutputindex: u32) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uioutputindex), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectPrimitiveImpl {
    type Vtable = IMILBitmapEffectPrimitiveImpl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce41e00b_efa6_44e7_b007_dd042e3ae126);
}
impl ::core::convert::From<IMILBitmapEffectPrimitiveImpl> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectPrimitiveImpl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffectPrimitiveImpl> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectPrimitiveImpl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectPrimitiveImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectPrimitiveImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectPrimitiveImpl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uioutputindex: u32, pfdirty: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uioutputindex: u32, pfvolatile: *mut i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffectRenderContext(pub ::windows::core::IUnknown);
impl IMILBitmapEffectRenderContext {
    pub unsafe fn SetOutputPixelFormat(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    pub unsafe fn GetOutputPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn SetUseSoftwareRenderer(&self, fsoftware: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(fsoftware)).ok()
    }
    pub unsafe fn SetInitialTransform(&self, pmatrix: *const MILMatrixF) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmatrix)).ok()
    }
    pub unsafe fn GetFinalTransform(&self) -> ::windows::core::Result<MILMatrixF> {
        let mut result__: <MILMatrixF as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MILMatrixF>(result__)
    }
    pub unsafe fn SetOutputDPI(&self, dbldpix: f64, dbldpiy: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dbldpix), ::core::mem::transmute(dbldpiy)).ok()
    }
    pub unsafe fn GetOutputDPI(&self, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdbldpix), ::core::mem::transmute(pdbldpiy)).ok()
    }
    pub unsafe fn SetRegionOfInterest(&self, prect: *const MilRectD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(prect)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectRenderContext {
    type Vtable = IMILBitmapEffectRenderContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12a2ec7e_2d33_44b2_b334_1abb7846e390);
}
impl ::core::convert::From<IMILBitmapEffectRenderContext> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectRenderContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffectRenderContext> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectRenderContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectRenderContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectRenderContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectRenderContext_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fsoftware: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmatrix: *const MILMatrixF) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmatrix: *mut MILMatrixF) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dbldpix: f64, dbldpiy: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prect: *const MilRectD) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffectRenderContextImpl(pub ::windows::core::IUnknown);
impl IMILBitmapEffectRenderContextImpl {
    pub unsafe fn GetUseSoftwareRenderer(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn GetTransform(&self, pmatrix: *mut MILMatrixF) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmatrix)).ok()
    }
    pub unsafe fn UpdateTransform(&self, pmatrix: *const MILMatrixF) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmatrix)).ok()
    }
    pub unsafe fn GetOutputBounds(&self, prect: *mut MilRectD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(prect)).ok()
    }
    pub unsafe fn UpdateOutputBounds(&self, prect: *const MilRectD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(prect)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectRenderContextImpl {
    type Vtable = IMILBitmapEffectRenderContextImpl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d25accb_797d_4fd2_b128_dffeff84fcc3);
}
impl ::core::convert::From<IMILBitmapEffectRenderContextImpl> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectRenderContextImpl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffectRenderContextImpl> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectRenderContextImpl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectRenderContextImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectRenderContextImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectRenderContextImpl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfsoftware: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmatrix: *mut MILMatrixF) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmatrix: *const MILMatrixF) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prect: *mut MilRectD) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prect: *const MilRectD) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMILBitmapEffects(pub ::windows::core::IUnknown);
impl IMILBitmapEffects {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<IMILBitmapEffectGroup> {
        let mut result__: <IMILBitmapEffectGroup as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffectGroup>(result__)
    }
    pub unsafe fn Item(&self, uindex: u32) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__: <IMILBitmapEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), &mut result__).from_abi::<IMILBitmapEffect>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffects {
    type Vtable = IMILBitmapEffects_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51ac3dce_67c5_448b_9180_ad3eabddd5dd);
}
impl ::core::convert::From<IMILBitmapEffects> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffects) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMILBitmapEffects> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffects) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffects {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffects {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffects_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppiureturn: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uindex: u32, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puicount: *mut u32) -> ::windows::core::HRESULT,
);
pub const MILBITMAPEFFECT_SDK_VERSION: u32 = 16777216u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MILMatrixF {
    pub _11: f64,
    pub _12: f64,
    pub _13: f64,
    pub _14: f64,
    pub _21: f64,
    pub _22: f64,
    pub _23: f64,
    pub _24: f64,
    pub _31: f64,
    pub _32: f64,
    pub _33: f64,
    pub _34: f64,
    pub _41: f64,
    pub _42: f64,
    pub _43: f64,
    pub _44: f64,
}
impl MILMatrixF {}
impl ::core::default::Default for MILMatrixF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MILMatrixF {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MILMatrixF").field("_11", &self._11).field("_12", &self._12).field("_13", &self._13).field("_14", &self._14).field("_21", &self._21).field("_22", &self._22).field("_23", &self._23).field("_24", &self._24).field("_31", &self._31).field("_32", &self._32).field("_33", &self._33).field("_34", &self._34).field("_41", &self._41).field("_42", &self._42).field("_43", &self._43).field("_44", &self._44).finish()
    }
}
impl ::core::cmp::PartialEq for MILMatrixF {
    fn eq(&self, other: &Self) -> bool {
        self._11 == other._11 && self._12 == other._12 && self._13 == other._13 && self._14 == other._14 && self._21 == other._21 && self._22 == other._22 && self._23 == other._23 && self._24 == other._24 && self._31 == other._31 && self._32 == other._32 && self._33 == other._33 && self._34 == other._34 && self._41 == other._41 && self._42 == other._42 && self._43 == other._43 && self._44 == other._44
    }
}
impl ::core::cmp::Eq for MILMatrixF {}
unsafe impl ::windows::core::Abi for MILMatrixF {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MilPoint2D {
    pub X: f64,
    pub Y: f64,
}
impl MilPoint2D {}
impl ::core::default::Default for MilPoint2D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MilPoint2D {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MilPoint2D").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
impl ::core::cmp::PartialEq for MilPoint2D {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for MilPoint2D {}
unsafe impl ::windows::core::Abi for MilPoint2D {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MilRectD {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
impl MilRectD {}
impl ::core::default::Default for MilRectD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MilRectD {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MilRectD").field("left", &self.left).field("top", &self.top).field("right", &self.right).field("bottom", &self.bottom).finish()
    }
}
impl ::core::cmp::PartialEq for MilRectD {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.right == other.right && self.bottom == other.bottom
    }
}
impl ::core::cmp::Eq for MilRectD {}
unsafe impl ::windows::core::Abi for MilRectD {
    type Abi = Self;
}
