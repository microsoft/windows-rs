#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_UI_Input_Touch', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseGestureInfoHandle<'a, Param0: ::windows::core::IntoParam<'a, HGESTUREINFO>>(hgestureinfo: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseGestureInfoHandle(hgestureinfo: HGESTUREINFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CloseGestureInfoHandle(hgestureinfo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Touch', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseTouchInputHandle<'a, Param0: ::windows::core::IntoParam<'a, HTOUCHINPUT>>(htouchinput: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseTouchInputHandle(htouchinput: HTOUCHINPUT) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CloseTouchInputHandle(htouchinput.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub struct GESTURECONFIG {
    pub dwID: GESTURECONFIG_ID,
    pub dwWant: u32,
    pub dwBlock: u32,
}
impl ::core::marker::Copy for GESTURECONFIG {}
impl ::core::clone::Clone for GESTURECONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GESTURECONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GESTURECONFIG").field("dwID", &self.dwID).field("dwWant", &self.dwWant).field("dwBlock", &self.dwBlock).finish()
    }
}
unsafe impl ::windows::core::Abi for GESTURECONFIG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GESTURECONFIG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GESTURECONFIG>()) == 0 }
    }
}
impl ::core::cmp::Eq for GESTURECONFIG {}
impl ::core::default::Default for GESTURECONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub type GESTURECONFIG_ID = u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const GID_BEGIN: GESTURECONFIG_ID = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const GID_END: GESTURECONFIG_ID = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const GID_ZOOM: GESTURECONFIG_ID = 3u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const GID_PAN: GESTURECONFIG_ID = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const GID_ROTATE: GESTURECONFIG_ID = 5u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const GID_TWOFINGERTAP: GESTURECONFIG_ID = 6u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const GID_PRESSANDTAP: GESTURECONFIG_ID = 7u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const GID_ROLLOVER: GESTURECONFIG_ID = 7u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Touch', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GESTUREINFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwID: u32,
    pub hwndTarget: super::super::super::Foundation::HWND,
    pub ptsLocation: super::super::super::Foundation::POINTS,
    pub dwInstanceID: u32,
    pub dwSequenceID: u32,
    pub ullArguments: u64,
    pub cbExtraArgs: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GESTUREINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GESTUREINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GESTUREINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GESTUREINFO").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("dwID", &self.dwID).field("hwndTarget", &self.hwndTarget).field("ptsLocation", &self.ptsLocation).field("dwInstanceID", &self.dwInstanceID).field("dwSequenceID", &self.dwSequenceID).field("ullArguments", &self.ullArguments).field("cbExtraArgs", &self.cbExtraArgs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GESTUREINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GESTUREINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GESTUREINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GESTUREINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GESTUREINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Touch', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GESTURENOTIFYSTRUCT {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub hwndTarget: super::super::super::Foundation::HWND,
    pub ptsLocation: super::super::super::Foundation::POINTS,
    pub dwInstanceID: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GESTURENOTIFYSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GESTURENOTIFYSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GESTURENOTIFYSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GESTURENOTIFYSTRUCT").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("hwndTarget", &self.hwndTarget).field("ptsLocation", &self.ptsLocation).field("dwInstanceID", &self.dwInstanceID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GESTURENOTIFYSTRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GESTURENOTIFYSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GESTURENOTIFYSTRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GESTURENOTIFYSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GESTURENOTIFYSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Touch', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGestureConfig<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(hwnd: Param0, dwreserved: u32, dwflags: u32, pcids: *const u32, pgestureconfig: *mut GESTURECONFIG, cbsize: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGestureConfig(hwnd: super::super::super::Foundation::HWND, dwreserved: u32, dwflags: u32, pcids: *const u32, pgestureconfig: *mut GESTURECONFIG, cbsize: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetGestureConfig(hwnd.into_param().abi(), ::core::mem::transmute(dwreserved), ::core::mem::transmute(dwflags), ::core::mem::transmute(pcids), ::core::mem::transmute(pgestureconfig), ::core::mem::transmute(cbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Touch', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGestureExtraArgs<'a, Param0: ::windows::core::IntoParam<'a, HGESTUREINFO>>(hgestureinfo: Param0, cbextraargs: u32, pextraargs: *mut u8) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGestureExtraArgs(hgestureinfo: HGESTUREINFO, cbextraargs: u32, pextraargs: *mut u8) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetGestureExtraArgs(hgestureinfo.into_param().abi(), ::core::mem::transmute(cbextraargs), ::core::mem::transmute(pextraargs)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Touch', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGestureInfo<'a, Param0: ::windows::core::IntoParam<'a, HGESTUREINFO>>(hgestureinfo: Param0, pgestureinfo: *mut GESTUREINFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGestureInfo(hgestureinfo: HGESTUREINFO, pgestureinfo: *mut GESTUREINFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetGestureInfo(hgestureinfo.into_param().abi(), ::core::mem::transmute(pgestureinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Touch', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTouchInputInfo<'a, Param0: ::windows::core::IntoParam<'a, HTOUCHINPUT>>(htouchinput: Param0, cinputs: u32, pinputs: *mut TOUCHINPUT, cbsize: i32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTouchInputInfo(htouchinput: HTOUCHINPUT, cinputs: u32, pinputs: *mut TOUCHINPUT, cbsize: i32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetTouchInputInfo(htouchinput.into_param().abi(), ::core::mem::transmute(cinputs), ::core::mem::transmute(pinputs), ::core::mem::transmute(cbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HGESTUREINFO(pub isize);
impl HGESTUREINFO {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for HGESTUREINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HGESTUREINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HGESTUREINFO {}
impl ::core::fmt::Debug for HGESTUREINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HGESTUREINFO").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HGESTUREINFO {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTOUCHINPUT(pub isize);
impl HTOUCHINPUT {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for HTOUCHINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HTOUCHINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HTOUCHINPUT {}
impl ::core::fmt::Debug for HTOUCHINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTOUCHINPUT").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HTOUCHINPUT {
    type Abi = Self;
}
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
#[repr(transparent)]
pub struct IInertiaProcessor(::windows::core::IUnknown);
impl IInertiaProcessor {
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn InitialOriginX(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetInitialOriginX(&self, x: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(x)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn InitialOriginY(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetInitialOriginY(&self, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn InitialVelocityX(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetInitialVelocityX(&self, x: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(x)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn InitialVelocityY(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetInitialVelocityY(&self, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn InitialAngularVelocity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetInitialAngularVelocity(&self, velocity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(velocity)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn InitialExpansionVelocity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetInitialExpansionVelocity(&self, velocity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(velocity)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn InitialRadius(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetInitialRadius(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn BoundaryLeft(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetBoundaryLeft(&self, left: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(left)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn BoundaryTop(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetBoundaryTop(&self, top: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(top)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn BoundaryRight(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetBoundaryRight(&self, right: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(right)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn BoundaryBottom(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetBoundaryBottom(&self, bottom: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(bottom)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn ElasticMarginLeft(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetElasticMarginLeft(&self, left: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(left)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn ElasticMarginTop(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetElasticMarginTop(&self, top: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(top)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn ElasticMarginRight(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetElasticMarginRight(&self, right: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(right)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn ElasticMarginBottom(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetElasticMarginBottom(&self, bottom: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(bottom)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn DesiredDisplacement(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetDesiredDisplacement(&self, displacement: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(displacement)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn DesiredRotation(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetDesiredRotation(&self, rotation: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(rotation)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn DesiredExpansion(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetDesiredExpansion(&self, expansion: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(expansion)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn DesiredDeceleration(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetDesiredDeceleration(&self, deceleration: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(deceleration)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn DesiredAngularDeceleration(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetDesiredAngularDeceleration(&self, deceleration: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(deceleration)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn DesiredExpansionDeceleration(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetDesiredExpansionDeceleration(&self, deceleration: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(deceleration)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn InitialTimestamp(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetInitialTimestamp(&self, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(timestamp)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Process(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: super::super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProcessTime(&self, timestamp: u32) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: super::super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(timestamp), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn Complete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn CompleteTime(&self, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(timestamp)).ok()
    }
}
impl ::core::convert::From<IInertiaProcessor> for ::windows::core::IUnknown {
    fn from(value: IInertiaProcessor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInertiaProcessor> for ::windows::core::IUnknown {
    fn from(value: &IInertiaProcessor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInertiaProcessor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IInertiaProcessor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInertiaProcessor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInertiaProcessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInertiaProcessor {}
impl ::core::fmt::Debug for IInertiaProcessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInertiaProcessor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInertiaProcessor {
    type Vtable = IInertiaProcessorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18b00c6d_c5ee_41b1_90a9_9d4a929095ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaProcessorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, y: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, y: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, y: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, y: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displacement: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displacement: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotation: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotation: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expansion: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expansion: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u32, completed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
#[repr(transparent)]
pub struct IManipulationProcessor(::windows::core::IUnknown);
impl IManipulationProcessor {
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SupportedManipulations(&self) -> ::windows::core::Result<MANIPULATION_PROCESSOR_MANIPULATIONS> {
        let mut result__: MANIPULATION_PROCESSOR_MANIPULATIONS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MANIPULATION_PROCESSOR_MANIPULATIONS>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetSupportedManipulations(&self, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulations)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn PivotPointX(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetPivotPointX(&self, pivotpointx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pivotpointx)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn PivotPointY(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetPivotPointY(&self, pivotpointy: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pivotpointy)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn PivotRadius(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetPivotRadius(&self, pivotradius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pivotradius)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn CompleteManipulation(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn ProcessDown(&self, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulatorid), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn ProcessMove(&self, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulatorid), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn ProcessUp(&self, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulatorid), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn ProcessDownWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulatorid), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(timestamp)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn ProcessMoveWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulatorid), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(timestamp)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn ProcessUpWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulatorid), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(timestamp)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn GetVelocityX(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn GetVelocityY(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn GetExpansionVelocity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn GetAngularVelocity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn MinimumScaleRotateRadius(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn SetMinimumScaleRotateRadius(&self, minradius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(minradius)).ok()
    }
}
impl ::core::convert::From<IManipulationProcessor> for ::windows::core::IUnknown {
    fn from(value: IManipulationProcessor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IManipulationProcessor> for ::windows::core::IUnknown {
    fn from(value: &IManipulationProcessor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IManipulationProcessor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IManipulationProcessor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IManipulationProcessor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IManipulationProcessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IManipulationProcessor {}
impl ::core::fmt::Debug for IManipulationProcessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IManipulationProcessor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IManipulationProcessor {
    type Vtable = IManipulationProcessorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa22ac519_8300_48a0_bef4_f1be8737dba4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationProcessorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulations: *mut MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivotpointx: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivotpointx: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivotpointy: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivotpointy: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivotradius: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivotradius: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocityx: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocityy: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expansionvelocity: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, angularvelocity: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minradius: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minradius: f32) -> ::windows::core::HRESULT,
);
pub const InertiaProcessor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabb27087_4ce0_4e58_a0cb_e24df96814be);
#[doc = "*Required features: 'Win32_UI_Input_Touch', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsTouchWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(hwnd: Param0, pulflags: *mut u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsTouchWindow(hwnd: super::super::super::Foundation::HWND, pulflags: *mut u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsTouchWindow(hwnd.into_param().abi(), ::core::mem::transmute(pulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub type MANIPULATION_PROCESSOR_MANIPULATIONS = i32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const MANIPULATION_NONE: MANIPULATION_PROCESSOR_MANIPULATIONS = 0i32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const MANIPULATION_TRANSLATE_X: MANIPULATION_PROCESSOR_MANIPULATIONS = 1i32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const MANIPULATION_TRANSLATE_Y: MANIPULATION_PROCESSOR_MANIPULATIONS = 2i32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const MANIPULATION_SCALE: MANIPULATION_PROCESSOR_MANIPULATIONS = 4i32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const MANIPULATION_ROTATE: MANIPULATION_PROCESSOR_MANIPULATIONS = 8i32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const MANIPULATION_ALL: MANIPULATION_PROCESSOR_MANIPULATIONS = 15i32;
pub const ManipulationProcessor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x597d4fb0_47fd_4aff_89b9_c6cfae8cf08e);
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub type REGISTER_TOUCH_WINDOW_FLAGS = u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const TWF_FINETOUCH: REGISTER_TOUCH_WINDOW_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const TWF_WANTPALM: REGISTER_TOUCH_WINDOW_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterTouchWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(hwnd: Param0, ulflags: REGISTER_TOUCH_WINDOW_FLAGS) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterTouchWindow(hwnd: super::super::super::Foundation::HWND, ulflags: REGISTER_TOUCH_WINDOW_FLAGS) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(RegisterTouchWindow(hwnd.into_param().abi(), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Touch', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetGestureConfig<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(hwnd: Param0, dwreserved: u32, cids: u32, pgestureconfig: *const GESTURECONFIG, cbsize: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetGestureConfig(hwnd: super::super::super::Foundation::HWND, dwreserved: u32, cids: u32, pgestureconfig: *const GESTURECONFIG, cbsize: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetGestureConfig(hwnd.into_param().abi(), ::core::mem::transmute(dwreserved), ::core::mem::transmute(cids), ::core::mem::transmute(pgestureconfig), ::core::mem::transmute(cbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub type TOUCHEVENTF_FLAGS = u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const TOUCHEVENTF_MOVE: TOUCHEVENTF_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const TOUCHEVENTF_DOWN: TOUCHEVENTF_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const TOUCHEVENTF_UP: TOUCHEVENTF_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const TOUCHEVENTF_INRANGE: TOUCHEVENTF_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const TOUCHEVENTF_PRIMARY: TOUCHEVENTF_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const TOUCHEVENTF_NOCOALESCE: TOUCHEVENTF_FLAGS = 32u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const TOUCHEVENTF_PEN: TOUCHEVENTF_FLAGS = 64u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const TOUCHEVENTF_PALM: TOUCHEVENTF_FLAGS = 128u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Touch', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOUCHINPUT {
    pub x: i32,
    pub y: i32,
    pub hSource: super::super::super::Foundation::HANDLE,
    pub dwID: u32,
    pub dwFlags: TOUCHEVENTF_FLAGS,
    pub dwMask: TOUCHINPUTMASKF_MASK,
    pub dwTime: u32,
    pub dwExtraInfo: usize,
    pub cxContact: u32,
    pub cyContact: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOUCHINPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOUCHINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOUCHINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOUCHINPUT").field("x", &self.x).field("y", &self.y).field("hSource", &self.hSource).field("dwID", &self.dwID).field("dwFlags", &self.dwFlags).field("dwMask", &self.dwMask).field("dwTime", &self.dwTime).field("dwExtraInfo", &self.dwExtraInfo).field("cxContact", &self.cxContact).field("cyContact", &self.cyContact).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TOUCHINPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOUCHINPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TOUCHINPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOUCHINPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOUCHINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub type TOUCHINPUTMASKF_MASK = u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const TOUCHINPUTMASKF_TIMEFROMSYSTEM: TOUCHINPUTMASKF_MASK = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const TOUCHINPUTMASKF_EXTRAINFO: TOUCHINPUTMASKF_MASK = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
pub const TOUCHINPUTMASKF_CONTACTAREA: TOUCHINPUTMASKF_MASK = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Touch', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterTouchWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(hwnd: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterTouchWindow(hwnd: super::super::super::Foundation::HWND) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UnregisterTouchWindow(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
#[repr(transparent)]
pub struct _IManipulationEvents(::windows::core::IUnknown);
impl _IManipulationEvents {
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn ManipulationStarted(&self, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn ManipulationDelta(&self, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(x),
            ::core::mem::transmute(y),
            ::core::mem::transmute(translationdeltax),
            ::core::mem::transmute(translationdeltay),
            ::core::mem::transmute(scaledelta),
            ::core::mem::transmute(expansiondelta),
            ::core::mem::transmute(rotationdelta),
            ::core::mem::transmute(cumulativetranslationx),
            ::core::mem::transmute(cumulativetranslationy),
            ::core::mem::transmute(cumulativescale),
            ::core::mem::transmute(cumulativeexpansion),
            ::core::mem::transmute(cumulativerotation),
        )
        .ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Touch'*"]
    pub unsafe fn ManipulationCompleted(&self, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(cumulativetranslationx), ::core::mem::transmute(cumulativetranslationy), ::core::mem::transmute(cumulativescale), ::core::mem::transmute(cumulativeexpansion), ::core::mem::transmute(cumulativerotation)).ok()
    }
}
impl ::core::convert::From<_IManipulationEvents> for ::windows::core::IUnknown {
    fn from(value: _IManipulationEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&_IManipulationEvents> for ::windows::core::IUnknown {
    fn from(value: &_IManipulationEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for _IManipulationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &_IManipulationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for _IManipulationEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for _IManipulationEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for _IManipulationEvents {}
impl ::core::fmt::Debug for _IManipulationEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IManipulationEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for _IManipulationEvents {
    type Vtable = _IManipulationEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f62c8da_9c53_4b22_93df_927a862bbb03);
}
#[repr(C)]
#[doc(hidden)]
pub struct _IManipulationEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::HRESULT,
);
