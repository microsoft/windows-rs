#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const CLSID_MILBitmapEffectBevel: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd361dbe_6c9b_4de0_8290_f6400c2737ed);
pub const CLSID_MILBitmapEffectBlur: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa924df87_225d_4373_8f5b_b90ec85ae3de);
pub const CLSID_MILBitmapEffectDropShadow: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x459a3fbe_d8ac_4692_874b_7a265715aa16);
pub const CLSID_MILBitmapEffectEmboss: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd299846_824f_47ec_a007_12aa767f2816);
pub const CLSID_MILBitmapEffectGroup: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac9c1a9a_7e18_4f64_ac7e_47cf7f051e95);
pub const CLSID_MILBitmapEffectOuterGlow: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2161bdd_7eb6_4725_9c0b_8a2a1b4f0667);
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffect(::windows::core::IUnknown);
impl IMILBitmapEffect {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutput<'a, Param1: ::windows::core::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, uiindex: u32, pcontext: Param1) -> ::windows::core::Result<super::super::Graphics::Imaging::IWICBitmapSource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), pcontext.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Graphics::Imaging::IWICBitmapSource>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetParentEffect(&self) -> ::windows::core::Result<IMILBitmapEffectGroup> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetParentEffect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffectGroup>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn SetInputSource<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::IWICBitmapSource>>(&self, uiindex: u32, pbitmapsource: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInputSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), pbitmapsource.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMILBitmapEffect> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffect> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffect {}
impl ::core::fmt::Debug for IMILBitmapEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffect {
    type Vtable = IMILBitmapEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a6ff321_c944_4a1b_9944_9954af301258);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffect_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: ::windows::core::RawPtr, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetOutput: usize,
    pub GetParentEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparenteffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub SetInputSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pbitmapsource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    SetInputSource: usize,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectConnections(::windows::core::IUnknown);
impl IMILBitmapEffectConnections {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetInputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectInputConnector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInputConnector)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffectInputConnector>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetOutputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputConnector)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffectOutputConnector>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectConnections> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectConnections) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectConnections> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectConnections) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectConnections {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectConnections {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffectConnections {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffectConnections {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffectConnections {}
impl ::core::fmt::Debug for IMILBitmapEffectConnections {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffectConnections").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectConnections {
    type Vtable = IMILBitmapEffectConnections_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2b5d861_9b1a_4374_89b0_dec4874d6a81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnections_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetInputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetOutputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectConnectionsInfo(::windows::core::IUnknown);
impl IMILBitmapEffectConnectionsInfo {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetNumberInputs(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNumberInputs)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetNumberOutputs(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNumberOutputs)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetInputConnectorInfo(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectConnectorInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInputConnectorInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffectConnectorInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetOutputConnectorInfo(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectConnectorInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputConnectorInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffectConnectorInfo>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectConnectionsInfo> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectConnectionsInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectConnectionsInfo> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectConnectionsInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectConnectionsInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectConnectionsInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffectConnectionsInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffectConnectionsInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffectConnectionsInfo {}
impl ::core::fmt::Debug for IMILBitmapEffectConnectionsInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffectConnectionsInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectConnectionsInfo {
    type Vtable = IMILBitmapEffectConnectionsInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x476b538a_c765_4237_ba4a_d6a880ff0cfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnectionsInfo_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetNumberInputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puinuminputs: *mut u32) -> ::windows::core::HRESULT,
    pub GetNumberOutputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puinumoutputs: *mut u32) -> ::windows::core::HRESULT,
    pub GetInputConnectorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetOutputConnectorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectConnector(::windows::core::IUnknown);
impl IMILBitmapEffectConnector {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetOptimalFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOptimalFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetNumberFormats(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetNumberFormats)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulindex), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsConnected)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetBitmapEffect(&self) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBitmapEffect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffect>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectConnector> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectConnector> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IMILBitmapEffectConnectorInfo> for &'a IMILBitmapEffectConnector {
    fn into_param(self) -> ::windows::core::Param<'a, IMILBitmapEffectConnectorInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffectConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffectConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffectConnector {}
impl ::core::fmt::Debug for IMILBitmapEffectConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffectConnector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectConnector {
    type Vtable = IMILBitmapEffectConnector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf59567b3_76c1_4d47_ba1e_79f955e350ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnector_Vtbl {
    pub base: IMILBitmapEffectConnectorInfo_Vtbl,
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfconnected: *mut i16) -> ::windows::core::HRESULT,
    pub GetBitmapEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectConnectorInfo(::windows::core::IUnknown);
impl IMILBitmapEffectConnectorInfo {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetOptimalFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOptimalFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetNumberFormats(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNumberFormats)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulindex), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectConnectorInfo> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectConnectorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectConnectorInfo> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectConnectorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectConnectorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectConnectorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffectConnectorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffectConnectorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffectConnectorInfo {}
impl ::core::fmt::Debug for IMILBitmapEffectConnectorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffectConnectorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectConnectorInfo {
    type Vtable = IMILBitmapEffectConnectorInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf66d2e4b_b46b_42fc_859e_3da0ecdb3c43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnectorInfo_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiindex: *mut u32) -> ::windows::core::HRESULT,
    pub GetOptimalFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetNumberFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulnumberformats: *mut u32) -> ::windows::core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectEvents(::windows::core::IUnknown);
impl IMILBitmapEffectEvents {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PropertyChange<'a, Param0: ::windows::core::IntoParam<'a, IMILBitmapEffect>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, peffect: Param0, bstrpropertyname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PropertyChange)(::core::mem::transmute_copy(self), peffect.into_param().abi(), bstrpropertyname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn DirtyRegion<'a, Param0: ::windows::core::IntoParam<'a, IMILBitmapEffect>>(&self, peffect: Param0, prect: *const MilRectD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DirtyRegion)(::core::mem::transmute_copy(self), peffect.into_param().abi(), ::core::mem::transmute(prect)).ok()
    }
}
impl ::core::convert::From<IMILBitmapEffectEvents> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectEvents> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffectEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffectEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffectEvents {}
impl ::core::fmt::Debug for IMILBitmapEffectEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffectEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectEvents {
    type Vtable = IMILBitmapEffectEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e880dd8_f8ce_457b_8199_d60bb3d7ef98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub PropertyChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peffect: ::windows::core::RawPtr, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PropertyChange: usize,
    pub DirtyRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peffect: ::windows::core::RawPtr, prect: *const MilRectD) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectFactory(::windows::core::IUnknown);
impl IMILBitmapEffectFactory {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn CreateEffect(&self, pguideffect: *const ::windows::core::GUID) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateEffect)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguideffect), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffect>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn CreateContext(&self) -> ::windows::core::Result<IMILBitmapEffectRenderContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateContext)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffectRenderContext>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn CreateEffectOuter(&self) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateEffectOuter)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffect>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectFactory> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectFactory> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffectFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffectFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffectFactory {}
impl ::core::fmt::Debug for IMILBitmapEffectFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffectFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectFactory {
    type Vtable = IMILBitmapEffectFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33a9df34_a403_4ec7_b07e_bc0682370845);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectFactory_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguideffect: *const ::windows::core::GUID, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateEffectOuter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectGroup(::windows::core::IUnknown);
impl IMILBitmapEffectGroup {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetInteriorInputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInteriorInputConnector)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffectOutputConnector>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetInteriorOutputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectInputConnector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInteriorOutputConnector)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffectInputConnector>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, IMILBitmapEffect>>(&self, peffect: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), peffect.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMILBitmapEffectGroup> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectGroup> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffectGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffectGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffectGroup {}
impl ::core::fmt::Debug for IMILBitmapEffectGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffectGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectGroup {
    type Vtable = IMILBitmapEffectGroup_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f952360_698a_4ac6_81a1_bcfdf08eb8e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectGroup_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetInteriorInputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetInteriorOutputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peffect: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectGroupImpl(::windows::core::IUnknown);
impl IMILBitmapEffectGroupImpl {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn Preprocess<'a, Param0: ::windows::core::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, pcontext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Preprocess)(::core::mem::transmute_copy(self), pcontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetNumberChildren(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNumberChildren)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetChildren(&self) -> ::windows::core::Result<IMILBitmapEffects> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetChildren)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffects>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectGroupImpl> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectGroupImpl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectGroupImpl> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectGroupImpl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectGroupImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectGroupImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffectGroupImpl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffectGroupImpl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffectGroupImpl {}
impl ::core::fmt::Debug for IMILBitmapEffectGroupImpl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffectGroupImpl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectGroupImpl {
    type Vtable = IMILBitmapEffectGroupImpl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78fed518_1cfc_4807_8b85_6b6e51398f62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectGroupImpl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Preprocess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNumberChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puinumberchildren: *mut u32) -> ::windows::core::HRESULT,
    pub GetChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchildren: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectImpl(::windows::core::IUnknown);
impl IMILBitmapEffectImpl {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn IsInPlaceModificationAllowed<'a, Param0: ::windows::core::IntoParam<'a, IMILBitmapEffectOutputConnector>>(&self, poutputconnector: Param0) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsInPlaceModificationAllowed)(::core::mem::transmute_copy(self), poutputconnector.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn SetParentEffect<'a, Param0: ::windows::core::IntoParam<'a, IMILBitmapEffectGroup>>(&self, pparenteffect: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetParentEffect)(::core::mem::transmute_copy(self), pparenteffect.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetInputSource(&self, uiindex: u32) -> ::windows::core::Result<super::super::Graphics::Imaging::IWICBitmapSource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInputSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Graphics::Imaging::IWICBitmapSource>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetInputSourceBounds(&self, uiindex: u32) -> ::windows::core::Result<MilRectD> {
        let mut result__: MilRectD = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInputSourceBounds)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), ::core::mem::transmute(&mut result__)).from_abi::<MilRectD>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetInputBitmapSource<'a, Param1: ::windows::core::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, uiindex: u32, prendercontext: Param1, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInputBitmapSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), prendercontext.into_param().abi(), ::core::mem::transmute(pfmodifyinplace), ::core::mem::transmute(ppbitmapsource)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutputBitmapSource<'a, Param1: ::windows::core::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, uiindex: u32, prendercontext: Param1, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOutputBitmapSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), prendercontext.into_param().abi(), ::core::mem::transmute(pfmodifyinplace), ::core::mem::transmute(ppbitmapsource)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pinner: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), pinner.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMILBitmapEffectImpl> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectImpl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectImpl> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectImpl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffectImpl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffectImpl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffectImpl {}
impl ::core::fmt::Debug for IMILBitmapEffectImpl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffectImpl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectImpl {
    type Vtable = IMILBitmapEffectImpl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc2468f2_9936_47be_b4af_06b5df5dbcbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectImpl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub IsInPlaceModificationAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutputconnector: ::windows::core::RawPtr, pfmodifyinplace: *mut i16) -> ::windows::core::HRESULT,
    pub SetParentEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparenteffect: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetInputSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetInputSource: usize,
    pub GetInputSourceBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, prect: *mut MilRectD) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetInputBitmapSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: ::windows::core::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetInputBitmapSource: usize,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetOutputBitmapSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: ::windows::core::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetOutputBitmapSource: usize,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinner: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectInputConnector(::windows::core::IUnknown);
impl IMILBitmapEffectInputConnector {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetOptimalFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetOptimalFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetNumberFormats(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetNumberFormats)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulindex), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsConnected)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetBitmapEffect(&self) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetBitmapEffect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffect>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn ConnectTo<'a, Param0: ::windows::core::IntoParam<'a, IMILBitmapEffectOutputConnector>>(&self, pconnector: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConnectTo)(::core::mem::transmute_copy(self), pconnector.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetConnection(&self) -> ::windows::core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffectOutputConnector>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectInputConnector> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectInputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectInputConnector> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectInputConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectInputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectInputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, IMILBitmapEffectConnectorInfo> for &'a IMILBitmapEffectInputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, IMILBitmapEffectConnectorInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IMILBitmapEffectConnector> for &'a IMILBitmapEffectInputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, IMILBitmapEffectConnector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffectInputConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffectInputConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffectInputConnector {}
impl ::core::fmt::Debug for IMILBitmapEffectInputConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffectInputConnector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectInputConnector {
    type Vtable = IMILBitmapEffectInputConnector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9b4ecaa_7a3c_45e7_8573_f4b81b60dd6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectInputConnector_Vtbl {
    pub base: IMILBitmapEffectConnector_Vtbl,
    pub ConnectTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnector: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectInteriorInputConnector(::windows::core::IUnknown);
impl IMILBitmapEffectInteriorInputConnector {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetInputConnector(&self) -> ::windows::core::Result<IMILBitmapEffectInputConnector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInputConnector)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffectInputConnector>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectInteriorInputConnector> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectInteriorInputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectInteriorInputConnector> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectInteriorInputConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectInteriorInputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectInteriorInputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffectInteriorInputConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffectInteriorInputConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffectInteriorInputConnector {}
impl ::core::fmt::Debug for IMILBitmapEffectInteriorInputConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffectInteriorInputConnector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectInteriorInputConnector {
    type Vtable = IMILBitmapEffectInteriorInputConnector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20287e9e_86a2_4e15_953d_eb1438a5b842);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectInteriorInputConnector_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetInputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinputconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectInteriorOutputConnector(::windows::core::IUnknown);
impl IMILBitmapEffectInteriorOutputConnector {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetOutputConnector(&self) -> ::windows::core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputConnector)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffectOutputConnector>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectInteriorOutputConnector> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectInteriorOutputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectInteriorOutputConnector> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectInteriorOutputConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectInteriorOutputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectInteriorOutputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffectInteriorOutputConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffectInteriorOutputConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffectInteriorOutputConnector {}
impl ::core::fmt::Debug for IMILBitmapEffectInteriorOutputConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffectInteriorOutputConnector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectInteriorOutputConnector {
    type Vtable = IMILBitmapEffectInteriorOutputConnector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00bbb6dc_acc9_4bfc_b344_8bee383dfefa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectInteriorOutputConnector_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetOutputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutputconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectOutputConnector(::windows::core::IUnknown);
impl IMILBitmapEffectOutputConnector {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetOptimalFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetOptimalFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetNumberFormats(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetNumberFormats)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulindex), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsConnected)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetBitmapEffect(&self) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetBitmapEffect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffect>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetNumberConnections(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNumberConnections)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetConnection(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectInputConnector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnection)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffectInputConnector>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectOutputConnector> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectOutputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectOutputConnector> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectOutputConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectOutputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectOutputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, IMILBitmapEffectConnectorInfo> for &'a IMILBitmapEffectOutputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, IMILBitmapEffectConnectorInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IMILBitmapEffectConnector> for &'a IMILBitmapEffectOutputConnector {
    fn into_param(self) -> ::windows::core::Param<'a, IMILBitmapEffectConnector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffectOutputConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffectOutputConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffectOutputConnector {}
impl ::core::fmt::Debug for IMILBitmapEffectOutputConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffectOutputConnector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectOutputConnector {
    type Vtable = IMILBitmapEffectOutputConnector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92957aad_841b_4866_82ec_8752468b07fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectOutputConnector_Vtbl {
    pub base: IMILBitmapEffectConnector_Vtbl,
    pub GetNumberConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puinumberconnections: *mut u32) -> ::windows::core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectOutputConnectorImpl(::windows::core::IUnknown);
impl IMILBitmapEffectOutputConnectorImpl {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn AddBackLink<'a, Param0: ::windows::core::IntoParam<'a, IMILBitmapEffectInputConnector>>(&self, pconnection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddBackLink)(::core::mem::transmute_copy(self), pconnection.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn RemoveBackLink<'a, Param0: ::windows::core::IntoParam<'a, IMILBitmapEffectInputConnector>>(&self, pconnection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveBackLink)(::core::mem::transmute_copy(self), pconnection.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMILBitmapEffectOutputConnectorImpl> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectOutputConnectorImpl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectOutputConnectorImpl> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectOutputConnectorImpl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectOutputConnectorImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectOutputConnectorImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffectOutputConnectorImpl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffectOutputConnectorImpl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffectOutputConnectorImpl {}
impl ::core::fmt::Debug for IMILBitmapEffectOutputConnectorImpl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffectOutputConnectorImpl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectOutputConnectorImpl {
    type Vtable = IMILBitmapEffectOutputConnectorImpl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21fae777_8b39_4bfa_9f2d_f3941ed36913);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectOutputConnectorImpl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AddBackLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveBackLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectPrimitive(::windows::core::IUnknown);
impl IMILBitmapEffectPrimitive {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutput<'a, Param1: ::windows::core::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, uiindex: u32, pcontext: Param1, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), pcontext.into_param().abi(), ::core::mem::transmute(pfmodifyinplace), ::core::mem::transmute(ppbitmapsource)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn TransformPoint<'a, Param3: ::windows::core::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: i16, pcontext: Param3, pfpointtransformed: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TransformPoint)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), ::core::mem::transmute(p), ::core::mem::transmute(fforwardtransform), pcontext.into_param().abi(), ::core::mem::transmute(pfpointtransformed)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn TransformRect<'a, Param3: ::windows::core::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, uiindex: u32, p: *mut MilRectD, fforwardtransform: i16, pcontext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TransformRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), ::core::mem::transmute(p), ::core::mem::transmute(fforwardtransform), pcontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn HasAffineTransform(&self, uiindex: u32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).HasAffineTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn HasInverseTransform(&self, uiindex: u32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).HasInverseTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"Win32_Graphics_Dwm\"`*"]
    #[cfg(feature = "Win32_Graphics_Dwm")]
    pub unsafe fn GetAffineMatrix(&self, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAffineMatrix)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiindex), ::core::mem::transmute(pmatrix)).ok()
    }
}
impl ::core::convert::From<IMILBitmapEffectPrimitive> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectPrimitive) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectPrimitive> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectPrimitive) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectPrimitive {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectPrimitive {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffectPrimitive {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffectPrimitive {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffectPrimitive {}
impl ::core::fmt::Debug for IMILBitmapEffectPrimitive {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffectPrimitive").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectPrimitive {
    type Vtable = IMILBitmapEffectPrimitive_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67e31025_3091_4dfc_98d6_dd494551461d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectPrimitive_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: ::windows::core::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetOutput: usize,
    pub TransformPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: i16, pcontext: ::windows::core::RawPtr, pfpointtransformed: *mut i16) -> ::windows::core::HRESULT,
    pub TransformRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilRectD, fforwardtransform: i16, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub HasAffineTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pfaffine: *mut i16) -> ::windows::core::HRESULT,
    pub HasInverseTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pfhasinverse: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dwm")]
    pub GetAffineMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dwm"))]
    GetAffineMatrix: usize,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectPrimitiveImpl(::windows::core::IUnknown);
impl IMILBitmapEffectPrimitiveImpl {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn IsDirty(&self, uioutputindex: u32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsDirty)(::core::mem::transmute_copy(self), ::core::mem::transmute(uioutputindex), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn IsVolatile(&self, uioutputindex: u32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsVolatile)(::core::mem::transmute_copy(self), ::core::mem::transmute(uioutputindex), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectPrimitiveImpl> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectPrimitiveImpl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectPrimitiveImpl> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectPrimitiveImpl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectPrimitiveImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectPrimitiveImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffectPrimitiveImpl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffectPrimitiveImpl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffectPrimitiveImpl {}
impl ::core::fmt::Debug for IMILBitmapEffectPrimitiveImpl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffectPrimitiveImpl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectPrimitiveImpl {
    type Vtable = IMILBitmapEffectPrimitiveImpl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce41e00b_efa6_44e7_b007_dd042e3ae126);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectPrimitiveImpl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfdirty: *mut i16) -> ::windows::core::HRESULT,
    pub IsVolatile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfvolatile: *mut i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectRenderContext(::windows::core::IUnknown);
impl IMILBitmapEffectRenderContext {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn SetOutputPixelFormat(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutputPixelFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetOutputPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputPixelFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn SetUseSoftwareRenderer(&self, fsoftware: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUseSoftwareRenderer)(::core::mem::transmute_copy(self), ::core::mem::transmute(fsoftware)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn SetInitialTransform(&self, pmatrix: *const MILMatrixF) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmatrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetFinalTransform(&self) -> ::windows::core::Result<MILMatrixF> {
        let mut result__: MILMatrixF = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFinalTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MILMatrixF>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn SetOutputDPI(&self, dbldpix: f64, dbldpiy: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutputDPI)(::core::mem::transmute_copy(self), ::core::mem::transmute(dbldpix), ::core::mem::transmute(dbldpiy)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetOutputDPI(&self, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOutputDPI)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdbldpix), ::core::mem::transmute(pdbldpiy)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn SetRegionOfInterest(&self, prect: *const MilRectD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRegionOfInterest)(::core::mem::transmute_copy(self), ::core::mem::transmute(prect)).ok()
    }
}
impl ::core::convert::From<IMILBitmapEffectRenderContext> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectRenderContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectRenderContext> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectRenderContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectRenderContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectRenderContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffectRenderContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffectRenderContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffectRenderContext {}
impl ::core::fmt::Debug for IMILBitmapEffectRenderContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffectRenderContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectRenderContext {
    type Vtable = IMILBitmapEffectRenderContext_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12a2ec7e_2d33_44b2_b334_1abb7846e390);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectRenderContext_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetOutputPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetOutputPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetUseSoftwareRenderer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsoftware: i16) -> ::windows::core::HRESULT,
    pub SetInitialTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmatrix: *const MILMatrixF) -> ::windows::core::HRESULT,
    pub GetFinalTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmatrix: *mut MILMatrixF) -> ::windows::core::HRESULT,
    pub SetOutputDPI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dbldpix: f64, dbldpiy: f64) -> ::windows::core::HRESULT,
    pub GetOutputDPI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows::core::HRESULT,
    pub SetRegionOfInterest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectRenderContextImpl(::windows::core::IUnknown);
impl IMILBitmapEffectRenderContextImpl {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetUseSoftwareRenderer(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUseSoftwareRenderer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetTransform(&self, pmatrix: *mut MILMatrixF) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmatrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn UpdateTransform(&self, pmatrix: *const MILMatrixF) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmatrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn GetOutputBounds(&self, prect: *mut MilRectD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOutputBounds)(::core::mem::transmute_copy(self), ::core::mem::transmute(prect)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn UpdateOutputBounds(&self, prect: *const MilRectD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateOutputBounds)(::core::mem::transmute_copy(self), ::core::mem::transmute(prect)).ok()
    }
}
impl ::core::convert::From<IMILBitmapEffectRenderContextImpl> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectRenderContextImpl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectRenderContextImpl> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectRenderContextImpl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffectRenderContextImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffectRenderContextImpl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffectRenderContextImpl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffectRenderContextImpl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffectRenderContextImpl {}
impl ::core::fmt::Debug for IMILBitmapEffectRenderContextImpl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffectRenderContextImpl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffectRenderContextImpl {
    type Vtable = IMILBitmapEffectRenderContextImpl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d25accb_797d_4fd2_b128_dffeff84fcc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectRenderContextImpl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetUseSoftwareRenderer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsoftware: *mut i16) -> ::windows::core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmatrix: *mut MILMatrixF) -> ::windows::core::HRESULT,
    pub UpdateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmatrix: *const MILMatrixF) -> ::windows::core::HRESULT,
    pub GetOutputBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *mut MilRectD) -> ::windows::core::HRESULT,
    pub UpdateOutputBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffects(::windows::core::IUnknown);
impl IMILBitmapEffects {
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<IMILBitmapEffectGroup> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Parent)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffectGroup>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn Item(&self, uindex: u32) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), ::core::mem::transmute(&mut result__)).from_abi::<IMILBitmapEffect>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffects> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffects) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffects> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffects) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMILBitmapEffects {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMILBitmapEffects {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMILBitmapEffects {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMILBitmapEffects {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMILBitmapEffects {}
impl ::core::fmt::Debug for IMILBitmapEffects {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMILBitmapEffects").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMILBitmapEffects {
    type Vtable = IMILBitmapEffects_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51ac3dce_67c5_448b_9180_ad3eabddd5dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffects_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiureturn: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puicount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub const MILBITMAPEFFECT_SDK_VERSION: u32 = 16777216u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
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
impl ::core::marker::Copy for MILMatrixF {}
impl ::core::clone::Clone for MILMatrixF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MILMatrixF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MILMatrixF").field("_11", &self._11).field("_12", &self._12).field("_13", &self._13).field("_14", &self._14).field("_21", &self._21).field("_22", &self._22).field("_23", &self._23).field("_24", &self._24).field("_31", &self._31).field("_32", &self._32).field("_33", &self._33).field("_34", &self._34).field("_41", &self._41).field("_42", &self._42).field("_43", &self._43).field("_44", &self._44).finish()
    }
}
unsafe impl ::windows::core::Abi for MILMatrixF {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MILMatrixF {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MILMatrixF>()) == 0 }
    }
}
impl ::core::cmp::Eq for MILMatrixF {}
impl ::core::default::Default for MILMatrixF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub struct MilPoint2D {
    pub X: f64,
    pub Y: f64,
}
impl ::core::marker::Copy for MilPoint2D {}
impl ::core::clone::Clone for MilPoint2D {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MilPoint2D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MilPoint2D").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
unsafe impl ::windows::core::Abi for MilPoint2D {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MilPoint2D {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MilPoint2D>()) == 0 }
    }
}
impl ::core::cmp::Eq for MilPoint2D {}
impl ::core::default::Default for MilPoint2D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub struct MilRectD {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
impl ::core::marker::Copy for MilRectD {}
impl ::core::clone::Clone for MilRectD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MilRectD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MilRectD").field("left", &self.left).field("top", &self.top).field("right", &self.right).field("bottom", &self.bottom).finish()
    }
}
unsafe impl ::windows::core::Abi for MilRectD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MilRectD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MilRectD>()) == 0 }
    }
}
impl ::core::cmp::Eq for MilRectD {}
impl ::core::default::Default for MilRectD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
