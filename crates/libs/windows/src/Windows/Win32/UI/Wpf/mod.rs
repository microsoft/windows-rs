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
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutput<'a, P0>(&self, uiindex: u32, pcontext: P0) -> ::windows::core::Result<super::super::Graphics::Imaging::IWICBitmapSource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMILBitmapEffectRenderContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOutput)(::windows::core::Interface::as_raw(self), uiindex, pcontext.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Graphics::Imaging::IWICBitmapSource>(result__)
    }
    pub unsafe fn GetParentEffect(&self) -> ::windows::core::Result<IMILBitmapEffectGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetParentEffect)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffectGroup>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn SetInputSource<'a, P0>(&self, uiindex: u32, pbitmapsource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Graphics::Imaging::IWICBitmapSource>>,
    {
        (::windows::core::Interface::vtable(self).SetInputSource)(::windows::core::Interface::as_raw(self), uiindex, pbitmapsource.into().abi()).ok()
    }
}
impl ::core::convert::From<IMILBitmapEffect> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffect> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffect> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: *mut ::core::ffi::c_void, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetOutput: usize,
    pub GetParentEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparenteffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub SetInputSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pbitmapsource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    SetInputSource: usize,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectConnections(::windows::core::IUnknown);
impl IMILBitmapEffectConnections {
    pub unsafe fn GetInputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectInputConnector> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInputConnector)(::windows::core::Interface::as_raw(self), uiindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffectInputConnector>(result__)
    }
    pub unsafe fn GetOutputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputConnector)(::windows::core::Interface::as_raw(self), uiindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffectOutputConnector>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectConnections> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectConnections) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectConnections> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffectConnections) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectConnections> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectConnections) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetInputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOutputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectConnectionsInfo(::windows::core::IUnknown);
impl IMILBitmapEffectConnectionsInfo {
    pub unsafe fn GetNumberInputs(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetNumberInputs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetNumberOutputs(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetNumberOutputs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetInputConnectorInfo(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectConnectorInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInputConnectorInfo)(::windows::core::Interface::as_raw(self), uiindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffectConnectorInfo>(result__)
    }
    pub unsafe fn GetOutputConnectorInfo(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectConnectorInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputConnectorInfo)(::windows::core::Interface::as_raw(self), uiindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffectConnectorInfo>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectConnectionsInfo> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectConnectionsInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectConnectionsInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffectConnectionsInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectConnectionsInfo> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectConnectionsInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetNumberInputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puinuminputs: *mut u32) -> ::windows::core::HRESULT,
    pub GetNumberOutputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puinumoutputs: *mut u32) -> ::windows::core::HRESULT,
    pub GetInputConnectorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOutputConnectorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectConnector(::windows::core::IUnknown);
impl IMILBitmapEffectConnector {
    pub unsafe fn GetIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetIndex)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetOptimalFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetNumberFormats)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetFormat)(::windows::core::Interface::as_raw(self), ulindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsConnected)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn GetBitmapEffect(&self) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetBitmapEffect)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffect>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectConnector> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectConnector> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffectConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectConnector> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IMILBitmapEffectConnector> for IMILBitmapEffectConnectorInfo {
    fn from(value: IMILBitmapEffectConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectConnector> for &'a IMILBitmapEffectConnectorInfo {
    fn from(value: &'a IMILBitmapEffectConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectConnector> for IMILBitmapEffectConnectorInfo {
    fn from(value: &IMILBitmapEffectConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: IMILBitmapEffectConnectorInfo_Vtbl,
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfconnected: *mut i16) -> ::windows::core::HRESULT,
    pub GetBitmapEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectConnectorInfo(::windows::core::IUnknown);
impl IMILBitmapEffectConnectorInfo {
    pub unsafe fn GetIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetIndex)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOptimalFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetNumberFormats)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFormat)(::windows::core::Interface::as_raw(self), ulindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectConnectorInfo> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectConnectorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectConnectorInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffectConnectorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectConnectorInfo> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectConnectorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiindex: *mut u32) -> ::windows::core::HRESULT,
    pub GetOptimalFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetNumberFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulnumberformats: *mut u32) -> ::windows::core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectEvents(::windows::core::IUnknown);
impl IMILBitmapEffectEvents {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PropertyChange<'a, P0, P1>(&self, peffect: P0, bstrpropertyname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMILBitmapEffect>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).PropertyChange)(::windows::core::Interface::as_raw(self), peffect.into().abi(), bstrpropertyname.into().abi()).ok()
    }
    pub unsafe fn DirtyRegion<'a, P0>(&self, peffect: P0, prect: *const MilRectD) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMILBitmapEffect>>,
    {
        (::windows::core::Interface::vtable(self).DirtyRegion)(::windows::core::Interface::as_raw(self), peffect.into().abi(), ::core::mem::transmute(prect)).ok()
    }
}
impl ::core::convert::From<IMILBitmapEffectEvents> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffectEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectEvents> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub PropertyChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PropertyChange: usize,
    pub DirtyRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectFactory(::windows::core::IUnknown);
impl IMILBitmapEffectFactory {
    pub unsafe fn CreateEffect(&self, pguideffect: *const ::windows::core::GUID) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateEffect)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguideffect), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffect>(result__)
    }
    pub unsafe fn CreateContext(&self) -> ::windows::core::Result<IMILBitmapEffectRenderContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateContext)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffectRenderContext>(result__)
    }
    pub unsafe fn CreateEffectOuter(&self) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateEffectOuter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffect>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectFactory> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectFactory> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffectFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectFactory> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    pub CreateEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguideffect: *const ::windows::core::GUID, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateEffectOuter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectGroup(::windows::core::IUnknown);
impl IMILBitmapEffectGroup {
    pub unsafe fn GetInteriorInputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInteriorInputConnector)(::windows::core::Interface::as_raw(self), uiindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffectOutputConnector>(result__)
    }
    pub unsafe fn GetInteriorOutputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectInputConnector> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInteriorOutputConnector)(::windows::core::Interface::as_raw(self), uiindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffectInputConnector>(result__)
    }
    pub unsafe fn Add<'a, P0>(&self, peffect: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMILBitmapEffect>>,
    {
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), peffect.into().abi()).ok()
    }
}
impl ::core::convert::From<IMILBitmapEffectGroup> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectGroup> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffectGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectGroup> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetInteriorInputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetInteriorOutputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectGroupImpl(::windows::core::IUnknown);
impl IMILBitmapEffectGroupImpl {
    pub unsafe fn Preprocess<'a, P0>(&self, pcontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMILBitmapEffectRenderContext>>,
    {
        (::windows::core::Interface::vtable(self).Preprocess)(::windows::core::Interface::as_raw(self), pcontext.into().abi()).ok()
    }
    pub unsafe fn GetNumberChildren(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetNumberChildren)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetChildren(&self) -> ::windows::core::Result<IMILBitmapEffects> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetChildren)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffects>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectGroupImpl> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectGroupImpl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectGroupImpl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffectGroupImpl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectGroupImpl> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectGroupImpl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    pub Preprocess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNumberChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puinumberchildren: *mut u32) -> ::windows::core::HRESULT,
    pub GetChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchildren: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectImpl(::windows::core::IUnknown);
impl IMILBitmapEffectImpl {
    pub unsafe fn IsInPlaceModificationAllowed<'a, P0>(&self, poutputconnector: P0) -> ::windows::core::Result<i16>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMILBitmapEffectOutputConnector>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsInPlaceModificationAllowed)(::windows::core::Interface::as_raw(self), poutputconnector.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetParentEffect<'a, P0>(&self, pparenteffect: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMILBitmapEffectGroup>>,
    {
        (::windows::core::Interface::vtable(self).SetParentEffect)(::windows::core::Interface::as_raw(self), pparenteffect.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetInputSource(&self, uiindex: u32) -> ::windows::core::Result<super::super::Graphics::Imaging::IWICBitmapSource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInputSource)(::windows::core::Interface::as_raw(self), uiindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Graphics::Imaging::IWICBitmapSource>(result__)
    }
    pub unsafe fn GetInputSourceBounds(&self, uiindex: u32) -> ::windows::core::Result<MilRectD> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInputSourceBounds)(::windows::core::Interface::as_raw(self), uiindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<MilRectD>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetInputBitmapSource<'a, P0>(&self, uiindex: u32, prendercontext: P0, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMILBitmapEffectRenderContext>>,
    {
        (::windows::core::Interface::vtable(self).GetInputBitmapSource)(::windows::core::Interface::as_raw(self), uiindex, prendercontext.into().abi(), ::core::mem::transmute(pfmodifyinplace), ::core::mem::transmute(ppbitmapsource)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutputBitmapSource<'a, P0>(&self, uiindex: u32, prendercontext: P0, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMILBitmapEffectRenderContext>>,
    {
        (::windows::core::Interface::vtable(self).GetOutputBitmapSource)(::windows::core::Interface::as_raw(self), uiindex, prendercontext.into().abi(), ::core::mem::transmute(pfmodifyinplace), ::core::mem::transmute(ppbitmapsource)).ok()
    }
    pub unsafe fn Initialize<'a, P0>(&self, pinner: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pinner.into().abi()).ok()
    }
}
impl ::core::convert::From<IMILBitmapEffectImpl> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectImpl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectImpl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffectImpl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectImpl> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectImpl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    pub IsInPlaceModificationAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutputconnector: *mut ::core::ffi::c_void, pfmodifyinplace: *mut i16) -> ::windows::core::HRESULT,
    pub SetParentEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparenteffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetInputSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetInputSource: usize,
    pub GetInputSourceBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, prect: *mut MilRectD) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetInputBitmapSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: *mut ::core::ffi::c_void, pfmodifyinplace: *mut i16, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetInputBitmapSource: usize,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetOutputBitmapSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: *mut ::core::ffi::c_void, pfmodifyinplace: *mut i16, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetOutputBitmapSource: usize,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinner: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectInputConnector(::windows::core::IUnknown);
impl IMILBitmapEffectInputConnector {
    pub unsafe fn GetIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetIndex)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetOptimalFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetNumberFormats)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetFormat)(::windows::core::Interface::as_raw(self), ulindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsConnected)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn GetBitmapEffect(&self) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetBitmapEffect)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffect>(result__)
    }
    pub unsafe fn ConnectTo<'a, P0>(&self, pconnector: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMILBitmapEffectOutputConnector>>,
    {
        (::windows::core::Interface::vtable(self).ConnectTo)(::windows::core::Interface::as_raw(self), pconnector.into().abi()).ok()
    }
    pub unsafe fn GetConnection(&self) -> ::windows::core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetConnection)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffectOutputConnector>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectInputConnector> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectInputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectInputConnector> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffectInputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectInputConnector> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectInputConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IMILBitmapEffectInputConnector> for IMILBitmapEffectConnectorInfo {
    fn from(value: IMILBitmapEffectInputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectInputConnector> for &'a IMILBitmapEffectConnectorInfo {
    fn from(value: &'a IMILBitmapEffectInputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectInputConnector> for IMILBitmapEffectConnectorInfo {
    fn from(value: &IMILBitmapEffectInputConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IMILBitmapEffectInputConnector> for IMILBitmapEffectConnector {
    fn from(value: IMILBitmapEffectInputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectInputConnector> for &'a IMILBitmapEffectConnector {
    fn from(value: &'a IMILBitmapEffectInputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectInputConnector> for IMILBitmapEffectConnector {
    fn from(value: &IMILBitmapEffectInputConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: IMILBitmapEffectConnector_Vtbl,
    pub ConnectTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnector: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectInteriorInputConnector(::windows::core::IUnknown);
impl IMILBitmapEffectInteriorInputConnector {
    pub unsafe fn GetInputConnector(&self) -> ::windows::core::Result<IMILBitmapEffectInputConnector> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInputConnector)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffectInputConnector>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectInteriorInputConnector> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectInteriorInputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectInteriorInputConnector> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffectInteriorInputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectInteriorInputConnector> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectInteriorInputConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetInputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinputconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectInteriorOutputConnector(::windows::core::IUnknown);
impl IMILBitmapEffectInteriorOutputConnector {
    pub unsafe fn GetOutputConnector(&self) -> ::windows::core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputConnector)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffectOutputConnector>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectInteriorOutputConnector> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectInteriorOutputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectInteriorOutputConnector> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffectInteriorOutputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectInteriorOutputConnector> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectInteriorOutputConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetOutputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutputconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectOutputConnector(::windows::core::IUnknown);
impl IMILBitmapEffectOutputConnector {
    pub unsafe fn GetIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetIndex)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetOptimalFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetNumberFormats)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetFormat)(::windows::core::Interface::as_raw(self), ulindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsConnected)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn GetBitmapEffect(&self) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetBitmapEffect)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffect>(result__)
    }
    pub unsafe fn GetNumberConnections(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetNumberConnections)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetConnection(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectInputConnector> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetConnection)(::windows::core::Interface::as_raw(self), uiindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffectInputConnector>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectOutputConnector> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectOutputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectOutputConnector> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffectOutputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectOutputConnector> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectOutputConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IMILBitmapEffectOutputConnector> for IMILBitmapEffectConnectorInfo {
    fn from(value: IMILBitmapEffectOutputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectOutputConnector> for &'a IMILBitmapEffectConnectorInfo {
    fn from(value: &'a IMILBitmapEffectOutputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectOutputConnector> for IMILBitmapEffectConnectorInfo {
    fn from(value: &IMILBitmapEffectOutputConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IMILBitmapEffectOutputConnector> for IMILBitmapEffectConnector {
    fn from(value: IMILBitmapEffectOutputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectOutputConnector> for &'a IMILBitmapEffectConnector {
    fn from(value: &'a IMILBitmapEffectOutputConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectOutputConnector> for IMILBitmapEffectConnector {
    fn from(value: &IMILBitmapEffectOutputConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: IMILBitmapEffectConnector_Vtbl,
    pub GetNumberConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puinumberconnections: *mut u32) -> ::windows::core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectOutputConnectorImpl(::windows::core::IUnknown);
impl IMILBitmapEffectOutputConnectorImpl {
    pub unsafe fn AddBackLink<'a, P0>(&self, pconnection: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMILBitmapEffectInputConnector>>,
    {
        (::windows::core::Interface::vtable(self).AddBackLink)(::windows::core::Interface::as_raw(self), pconnection.into().abi()).ok()
    }
    pub unsafe fn RemoveBackLink<'a, P0>(&self, pconnection: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMILBitmapEffectInputConnector>>,
    {
        (::windows::core::Interface::vtable(self).RemoveBackLink)(::windows::core::Interface::as_raw(self), pconnection.into().abi()).ok()
    }
}
impl ::core::convert::From<IMILBitmapEffectOutputConnectorImpl> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectOutputConnectorImpl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectOutputConnectorImpl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffectOutputConnectorImpl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectOutputConnectorImpl> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectOutputConnectorImpl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    pub AddBackLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveBackLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectPrimitive(::windows::core::IUnknown);
impl IMILBitmapEffectPrimitive {
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutput<'a, P0>(&self, uiindex: u32, pcontext: P0, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMILBitmapEffectRenderContext>>,
    {
        (::windows::core::Interface::vtable(self).GetOutput)(::windows::core::Interface::as_raw(self), uiindex, pcontext.into().abi(), ::core::mem::transmute(pfmodifyinplace), ::core::mem::transmute(ppbitmapsource)).ok()
    }
    pub unsafe fn TransformPoint<'a, P0>(&self, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: i16, pcontext: P0, pfpointtransformed: *mut i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMILBitmapEffectRenderContext>>,
    {
        (::windows::core::Interface::vtable(self).TransformPoint)(::windows::core::Interface::as_raw(self), uiindex, ::core::mem::transmute(p), fforwardtransform, pcontext.into().abi(), ::core::mem::transmute(pfpointtransformed)).ok()
    }
    pub unsafe fn TransformRect<'a, P0>(&self, uiindex: u32, p: *mut MilRectD, fforwardtransform: i16, pcontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMILBitmapEffectRenderContext>>,
    {
        (::windows::core::Interface::vtable(self).TransformRect)(::windows::core::Interface::as_raw(self), uiindex, ::core::mem::transmute(p), fforwardtransform, pcontext.into().abi()).ok()
    }
    pub unsafe fn HasAffineTransform(&self, uiindex: u32) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).HasAffineTransform)(::windows::core::Interface::as_raw(self), uiindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn HasInverseTransform(&self, uiindex: u32) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).HasInverseTransform)(::windows::core::Interface::as_raw(self), uiindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
    #[cfg(feature = "Win32_Graphics_Dwm")]
    pub unsafe fn GetAffineMatrix(&self, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAffineMatrix)(::windows::core::Interface::as_raw(self), uiindex, ::core::mem::transmute(pmatrix)).ok()
    }
}
impl ::core::convert::From<IMILBitmapEffectPrimitive> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectPrimitive) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectPrimitive> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffectPrimitive) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectPrimitive> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectPrimitive) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: *mut ::core::ffi::c_void, pfmodifyinplace: *mut i16, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetOutput: usize,
    pub TransformPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: i16, pcontext: *mut ::core::ffi::c_void, pfpointtransformed: *mut i16) -> ::windows::core::HRESULT,
    pub TransformRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilRectD, fforwardtransform: i16, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
    pub unsafe fn IsDirty(&self, uioutputindex: u32, pfdirty: *mut i16) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).IsDirty)(::windows::core::Interface::as_raw(self), uioutputindex, ::core::mem::transmute(pfdirty))
    }
    pub unsafe fn IsVolatile(&self, uioutputindex: u32) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsVolatile)(::windows::core::Interface::as_raw(self), uioutputindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffectPrimitiveImpl> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectPrimitiveImpl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectPrimitiveImpl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffectPrimitiveImpl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectPrimitiveImpl> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectPrimitiveImpl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfdirty: *mut i16) -> ::windows::core::HRESULT,
    pub IsVolatile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfvolatile: *mut i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
#[repr(transparent)]
pub struct IMILBitmapEffectRenderContext(::windows::core::IUnknown);
impl IMILBitmapEffectRenderContext {
    pub unsafe fn SetOutputPixelFormat(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutputPixelFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(format)).ok()
    }
    pub unsafe fn GetOutputPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputPixelFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn SetUseSoftwareRenderer(&self, fsoftware: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUseSoftwareRenderer)(::windows::core::Interface::as_raw(self), fsoftware).ok()
    }
    pub unsafe fn SetInitialTransform(&self, pmatrix: *const MILMatrixF) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialTransform)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pmatrix)).ok()
    }
    pub unsafe fn GetFinalTransform(&self) -> ::windows::core::Result<MILMatrixF> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFinalTransform)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<MILMatrixF>(result__)
    }
    pub unsafe fn SetOutputDPI(&self, dbldpix: f64, dbldpiy: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutputDPI)(::windows::core::Interface::as_raw(self), dbldpix, dbldpiy).ok()
    }
    pub unsafe fn GetOutputDPI(&self, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOutputDPI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdbldpix), ::core::mem::transmute(pdbldpiy)).ok()
    }
    pub unsafe fn SetRegionOfInterest(&self, prect: *const MilRectD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRegionOfInterest)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prect)).ok()
    }
}
impl ::core::convert::From<IMILBitmapEffectRenderContext> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectRenderContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectRenderContext> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffectRenderContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectRenderContext> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectRenderContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
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
    pub unsafe fn GetUseSoftwareRenderer(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetUseSoftwareRenderer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn GetTransform(&self, pmatrix: *mut MILMatrixF) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTransform)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pmatrix)).ok()
    }
    pub unsafe fn UpdateTransform(&self, pmatrix: *const MILMatrixF) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateTransform)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pmatrix)).ok()
    }
    pub unsafe fn GetOutputBounds(&self, prect: *mut MilRectD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOutputBounds)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prect)).ok()
    }
    pub unsafe fn UpdateOutputBounds(&self, prect: *const MilRectD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateOutputBounds)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prect)).ok()
    }
}
impl ::core::convert::From<IMILBitmapEffectRenderContextImpl> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffectRenderContextImpl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffectRenderContextImpl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffectRenderContextImpl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffectRenderContextImpl> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffectRenderContextImpl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
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
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<IMILBitmapEffectGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Parent)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffectGroup>(result__)
    }
    pub unsafe fn Item(&self, uindex: u32) -> ::windows::core::Result<IMILBitmapEffect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), uindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMILBitmapEffect>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IMILBitmapEffects> for ::windows::core::IUnknown {
    fn from(value: IMILBitmapEffects) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMILBitmapEffects> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMILBitmapEffects) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMILBitmapEffects> for ::windows::core::IUnknown {
    fn from(value: &IMILBitmapEffects) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiureturn: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
