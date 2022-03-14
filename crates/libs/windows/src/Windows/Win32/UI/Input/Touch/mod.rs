#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GESTURECONFIG_ID(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_BEGIN: GESTURECONFIG_ID = GESTURECONFIG_ID(1u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_END: GESTURECONFIG_ID = GESTURECONFIG_ID(2u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_ZOOM: GESTURECONFIG_ID = GESTURECONFIG_ID(3u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_PAN: GESTURECONFIG_ID = GESTURECONFIG_ID(4u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_ROTATE: GESTURECONFIG_ID = GESTURECONFIG_ID(5u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_TWOFINGERTAP: GESTURECONFIG_ID = GESTURECONFIG_ID(6u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_PRESSANDTAP: GESTURECONFIG_ID = GESTURECONFIG_ID(7u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_ROLLOVER: GESTURECONFIG_ID = GESTURECONFIG_ID(7u32);
impl ::core::marker::Copy for GESTURECONFIG_ID {}
impl ::core::clone::Clone for GESTURECONFIG_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GESTURECONFIG_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GESTURECONFIG_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for GESTURECONFIG_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GESTURECONFIG_ID").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GESTURECONFIG_ID {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GESTURECONFIG_ID {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GESTURECONFIG_ID {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GESTURECONFIG_ID {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GESTURECONFIG_ID {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGestureConfig<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(hwnd: Param0, dwreserved: u32, dwflags: u32, pgestureconfig: &mut [GESTURECONFIG], cbsize: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGestureConfig(hwnd: super::super::super::Foundation::HWND, dwreserved: u32, dwflags: u32, pcids: *const u32, pgestureconfig: *mut GESTURECONFIG, cbsize: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetGestureConfig(hwnd.into_param().abi(), ::core::mem::transmute(dwreserved), ::core::mem::transmute(dwflags), pgestureconfig.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pgestureconfig)), ::core::mem::transmute(cbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTouchInputInfo<'a, Param0: ::windows::core::IntoParam<'a, HTOUCHINPUT>>(htouchinput: Param0, pinputs: &mut [TOUCHINPUT], cbsize: i32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTouchInputInfo(htouchinput: HTOUCHINPUT, cinputs: u32, pinputs: *mut TOUCHINPUT, cbsize: i32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetTouchInputInfo(htouchinput.into_param().abi(), pinputs.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pinputs)), ::core::mem::transmute(cbsize)))
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
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
pub struct IInertiaProcessor(::windows::core::IUnknown);
impl IInertiaProcessor {
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn InitialOriginX(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InitialOriginX)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetInitialOriginX(&self, x: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialOriginX)(::core::mem::transmute_copy(self), ::core::mem::transmute(x)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn InitialOriginY(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InitialOriginY)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetInitialOriginY(&self, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialOriginY)(::core::mem::transmute_copy(self), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn InitialVelocityX(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InitialVelocityX)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetInitialVelocityX(&self, x: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialVelocityX)(::core::mem::transmute_copy(self), ::core::mem::transmute(x)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn InitialVelocityY(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InitialVelocityY)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetInitialVelocityY(&self, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialVelocityY)(::core::mem::transmute_copy(self), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn InitialAngularVelocity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InitialAngularVelocity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetInitialAngularVelocity(&self, velocity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialAngularVelocity)(::core::mem::transmute_copy(self), ::core::mem::transmute(velocity)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn InitialExpansionVelocity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InitialExpansionVelocity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetInitialExpansionVelocity(&self, velocity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialExpansionVelocity)(::core::mem::transmute_copy(self), ::core::mem::transmute(velocity)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn InitialRadius(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InitialRadius)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetInitialRadius(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialRadius)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn BoundaryLeft(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BoundaryLeft)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetBoundaryLeft(&self, left: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBoundaryLeft)(::core::mem::transmute_copy(self), ::core::mem::transmute(left)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn BoundaryTop(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BoundaryTop)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetBoundaryTop(&self, top: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBoundaryTop)(::core::mem::transmute_copy(self), ::core::mem::transmute(top)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn BoundaryRight(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BoundaryRight)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetBoundaryRight(&self, right: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBoundaryRight)(::core::mem::transmute_copy(self), ::core::mem::transmute(right)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn BoundaryBottom(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BoundaryBottom)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetBoundaryBottom(&self, bottom: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBoundaryBottom)(::core::mem::transmute_copy(self), ::core::mem::transmute(bottom)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn ElasticMarginLeft(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ElasticMarginLeft)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetElasticMarginLeft(&self, left: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetElasticMarginLeft)(::core::mem::transmute_copy(self), ::core::mem::transmute(left)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn ElasticMarginTop(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ElasticMarginTop)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetElasticMarginTop(&self, top: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetElasticMarginTop)(::core::mem::transmute_copy(self), ::core::mem::transmute(top)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn ElasticMarginRight(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ElasticMarginRight)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetElasticMarginRight(&self, right: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetElasticMarginRight)(::core::mem::transmute_copy(self), ::core::mem::transmute(right)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn ElasticMarginBottom(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ElasticMarginBottom)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetElasticMarginBottom(&self, bottom: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetElasticMarginBottom)(::core::mem::transmute_copy(self), ::core::mem::transmute(bottom)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn DesiredDisplacement(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DesiredDisplacement)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetDesiredDisplacement(&self, displacement: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDesiredDisplacement)(::core::mem::transmute_copy(self), ::core::mem::transmute(displacement)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn DesiredRotation(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DesiredRotation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetDesiredRotation(&self, rotation: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDesiredRotation)(::core::mem::transmute_copy(self), ::core::mem::transmute(rotation)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn DesiredExpansion(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DesiredExpansion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetDesiredExpansion(&self, expansion: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDesiredExpansion)(::core::mem::transmute_copy(self), ::core::mem::transmute(expansion)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn DesiredDeceleration(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DesiredDeceleration)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetDesiredDeceleration(&self, deceleration: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDesiredDeceleration)(::core::mem::transmute_copy(self), ::core::mem::transmute(deceleration)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn DesiredAngularDeceleration(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DesiredAngularDeceleration)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetDesiredAngularDeceleration(&self, deceleration: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDesiredAngularDeceleration)(::core::mem::transmute_copy(self), ::core::mem::transmute(deceleration)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn DesiredExpansionDeceleration(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DesiredExpansionDeceleration)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetDesiredExpansionDeceleration(&self, deceleration: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDesiredExpansionDeceleration)(::core::mem::transmute_copy(self), ::core::mem::transmute(deceleration)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn InitialTimestamp(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InitialTimestamp)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetInitialTimestamp(&self, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialTimestamp)(::core::mem::transmute_copy(self), ::core::mem::transmute(timestamp)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Process(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: super::super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Process)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProcessTime(&self, timestamp: u32) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: super::super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ProcessTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(timestamp), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn Complete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Complete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn CompleteTime(&self, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CompleteTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(timestamp)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInertiaProcessor {
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
    type Vtable = IInertiaProcessor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18b00c6d_c5ee_41b1_90a9_9d4a929095ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaProcessor_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub InitialOriginX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: *mut f32) -> ::windows::core::HRESULT,
    pub SetInitialOriginX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32) -> ::windows::core::HRESULT,
    pub InitialOriginY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, y: *mut f32) -> ::windows::core::HRESULT,
    pub SetInitialOriginY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, y: f32) -> ::windows::core::HRESULT,
    pub InitialVelocityX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: *mut f32) -> ::windows::core::HRESULT,
    pub SetInitialVelocityX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32) -> ::windows::core::HRESULT,
    pub InitialVelocityY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, y: *mut f32) -> ::windows::core::HRESULT,
    pub SetInitialVelocityY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, y: f32) -> ::windows::core::HRESULT,
    pub InitialAngularVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: *mut f32) -> ::windows::core::HRESULT,
    pub SetInitialAngularVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: f32) -> ::windows::core::HRESULT,
    pub InitialExpansionVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: *mut f32) -> ::windows::core::HRESULT,
    pub SetInitialExpansionVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: f32) -> ::windows::core::HRESULT,
    pub InitialRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: *mut f32) -> ::windows::core::HRESULT,
    pub SetInitialRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
    pub BoundaryLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: *mut f32) -> ::windows::core::HRESULT,
    pub SetBoundaryLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT,
    pub BoundaryTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: *mut f32) -> ::windows::core::HRESULT,
    pub SetBoundaryTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT,
    pub BoundaryRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: *mut f32) -> ::windows::core::HRESULT,
    pub SetBoundaryRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT,
    pub BoundaryBottom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: *mut f32) -> ::windows::core::HRESULT,
    pub SetBoundaryBottom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT,
    pub ElasticMarginLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: *mut f32) -> ::windows::core::HRESULT,
    pub SetElasticMarginLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT,
    pub ElasticMarginTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: *mut f32) -> ::windows::core::HRESULT,
    pub SetElasticMarginTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT,
    pub ElasticMarginRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: *mut f32) -> ::windows::core::HRESULT,
    pub SetElasticMarginRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT,
    pub ElasticMarginBottom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: *mut f32) -> ::windows::core::HRESULT,
    pub SetElasticMarginBottom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT,
    pub DesiredDisplacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displacement: *mut f32) -> ::windows::core::HRESULT,
    pub SetDesiredDisplacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displacement: f32) -> ::windows::core::HRESULT,
    pub DesiredRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotation: *mut f32) -> ::windows::core::HRESULT,
    pub SetDesiredRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotation: f32) -> ::windows::core::HRESULT,
    pub DesiredExpansion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expansion: *mut f32) -> ::windows::core::HRESULT,
    pub SetDesiredExpansion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expansion: f32) -> ::windows::core::HRESULT,
    pub DesiredDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT,
    pub DesiredAngularDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT,
    pub SetDesiredAngularDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT,
    pub DesiredExpansionDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT,
    pub SetDesiredExpansionDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT,
    pub InitialTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: *mut u32) -> ::windows::core::HRESULT,
    pub SetInitialTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Process: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Process: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProcessTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u32, completed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProcessTime: usize,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CompleteTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
pub struct IManipulationProcessor(::windows::core::IUnknown);
impl IManipulationProcessor {
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SupportedManipulations(&self) -> ::windows::core::Result<MANIPULATION_PROCESSOR_MANIPULATIONS> {
        let mut result__: MANIPULATION_PROCESSOR_MANIPULATIONS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SupportedManipulations)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MANIPULATION_PROCESSOR_MANIPULATIONS>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetSupportedManipulations(&self, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSupportedManipulations)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulations)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn PivotPointX(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PivotPointX)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetPivotPointX(&self, pivotpointx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPivotPointX)(::core::mem::transmute_copy(self), ::core::mem::transmute(pivotpointx)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn PivotPointY(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PivotPointY)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetPivotPointY(&self, pivotpointy: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPivotPointY)(::core::mem::transmute_copy(self), ::core::mem::transmute(pivotpointy)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn PivotRadius(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PivotRadius)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetPivotRadius(&self, pivotradius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPivotRadius)(::core::mem::transmute_copy(self), ::core::mem::transmute(pivotradius)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn CompleteManipulation(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CompleteManipulation)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn ProcessDown(&self, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessDown)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulatorid), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn ProcessMove(&self, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessMove)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulatorid), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn ProcessUp(&self, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessUp)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulatorid), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn ProcessDownWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessDownWithTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulatorid), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(timestamp)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn ProcessMoveWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessMoveWithTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulatorid), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(timestamp)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn ProcessUpWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessUpWithTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulatorid), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(timestamp)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn GetVelocityX(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVelocityX)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn GetVelocityY(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVelocityY)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn GetExpansionVelocity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetExpansionVelocity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn GetAngularVelocity(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAngularVelocity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn MinimumScaleRotateRadius(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MinimumScaleRotateRadius)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn SetMinimumScaleRotateRadius(&self, minradius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMinimumScaleRotateRadius)(::core::mem::transmute_copy(self), ::core::mem::transmute(minradius)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IManipulationProcessor {
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
    type Vtable = IManipulationProcessor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa22ac519_8300_48a0_bef4_f1be8737dba4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationProcessor_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SupportedManipulations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulations: *mut MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::HRESULT,
    pub SetSupportedManipulations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::HRESULT,
    pub PivotPointX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivotpointx: *mut f32) -> ::windows::core::HRESULT,
    pub SetPivotPointX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivotpointx: f32) -> ::windows::core::HRESULT,
    pub PivotPointY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivotpointy: *mut f32) -> ::windows::core::HRESULT,
    pub SetPivotPointY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivotpointy: f32) -> ::windows::core::HRESULT,
    pub PivotRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivotradius: *mut f32) -> ::windows::core::HRESULT,
    pub SetPivotRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivotradius: f32) -> ::windows::core::HRESULT,
    pub CompleteManipulation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProcessDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT,
    pub ProcessMove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT,
    pub ProcessUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT,
    pub ProcessDownWithTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT,
    pub ProcessMoveWithTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT,
    pub ProcessUpWithTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT,
    pub GetVelocityX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocityx: *mut f32) -> ::windows::core::HRESULT,
    pub GetVelocityY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocityy: *mut f32) -> ::windows::core::HRESULT,
    pub GetExpansionVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expansionvelocity: *mut f32) -> ::windows::core::HRESULT,
    pub GetAngularVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, angularvelocity: *mut f32) -> ::windows::core::HRESULT,
    pub MinimumScaleRotateRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minradius: *mut f32) -> ::windows::core::HRESULT,
    pub SetMinimumScaleRotateRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minradius: f32) -> ::windows::core::HRESULT,
}
pub const InertiaProcessor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabb27087_4ce0_4e58_a0cb_e24df96814be);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MANIPULATION_PROCESSOR_MANIPULATIONS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const MANIPULATION_NONE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(0i32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const MANIPULATION_TRANSLATE_X: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(1i32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const MANIPULATION_TRANSLATE_Y: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(2i32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const MANIPULATION_SCALE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(4i32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const MANIPULATION_ROTATE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(8i32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const MANIPULATION_ALL: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(15i32);
impl ::core::marker::Copy for MANIPULATION_PROCESSOR_MANIPULATIONS {}
impl ::core::clone::Clone for MANIPULATION_PROCESSOR_MANIPULATIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MANIPULATION_PROCESSOR_MANIPULATIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MANIPULATION_PROCESSOR_MANIPULATIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MANIPULATION_PROCESSOR_MANIPULATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MANIPULATION_PROCESSOR_MANIPULATIONS").field(&self.0).finish()
    }
}
pub const ManipulationProcessor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x597d4fb0_47fd_4aff_89b9_c6cfae8cf08e);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REGISTER_TOUCH_WINDOW_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TWF_FINETOUCH: REGISTER_TOUCH_WINDOW_FLAGS = REGISTER_TOUCH_WINDOW_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TWF_WANTPALM: REGISTER_TOUCH_WINDOW_FLAGS = REGISTER_TOUCH_WINDOW_FLAGS(2u32);
impl ::core::marker::Copy for REGISTER_TOUCH_WINDOW_FLAGS {}
impl ::core::clone::Clone for REGISTER_TOUCH_WINDOW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REGISTER_TOUCH_WINDOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REGISTER_TOUCH_WINDOW_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for REGISTER_TOUCH_WINDOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REGISTER_TOUCH_WINDOW_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetGestureConfig<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(hwnd: Param0, dwreserved: u32, pgestureconfig: &[GESTURECONFIG], cbsize: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetGestureConfig(hwnd: super::super::super::Foundation::HWND, dwreserved: u32, cids: u32, pgestureconfig: *const GESTURECONFIG, cbsize: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetGestureConfig(hwnd.into_param().abi(), ::core::mem::transmute(dwreserved), pgestureconfig.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pgestureconfig)), ::core::mem::transmute(cbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TOUCHEVENTF_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_MOVE: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_DOWN: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_UP: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_INRANGE: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_PRIMARY: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_NOCOALESCE: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_PEN: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_PALM: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(128u32);
impl ::core::marker::Copy for TOUCHEVENTF_FLAGS {}
impl ::core::clone::Clone for TOUCHEVENTF_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TOUCHEVENTF_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TOUCHEVENTF_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TOUCHEVENTF_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOUCHEVENTF_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TOUCHEVENTF_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TOUCHEVENTF_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TOUCHEVENTF_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TOUCHEVENTF_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TOUCHEVENTF_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TOUCHINPUTMASKF_MASK(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHINPUTMASKF_TIMEFROMSYSTEM: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(1u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHINPUTMASKF_EXTRAINFO: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(2u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHINPUTMASKF_CONTACTAREA: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(4u32);
impl ::core::marker::Copy for TOUCHINPUTMASKF_MASK {}
impl ::core::clone::Clone for TOUCHINPUTMASKF_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TOUCHINPUTMASKF_MASK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TOUCHINPUTMASKF_MASK {
    type Abi = Self;
}
impl ::core::fmt::Debug for TOUCHINPUTMASKF_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOUCHINPUTMASKF_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TOUCHINPUTMASKF_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TOUCHINPUTMASKF_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TOUCHINPUTMASKF_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TOUCHINPUTMASKF_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TOUCHINPUTMASKF_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
pub struct _IManipulationEvents(::windows::core::IUnknown);
impl _IManipulationEvents {
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn ManipulationStarted(&self, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ManipulationStarted)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn ManipulationDelta(&self, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ManipulationDelta)(
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
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
    pub unsafe fn ManipulationCompleted(&self, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ManipulationCompleted)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(cumulativetranslationx), ::core::mem::transmute(cumulativetranslationy), ::core::mem::transmute(cumulativescale), ::core::mem::transmute(cumulativeexpansion), ::core::mem::transmute(cumulativerotation)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a _IManipulationEvents {
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
    type Vtable = _IManipulationEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f62c8da_9c53_4b22_93df_927a862bbb03);
}
#[repr(C)]
#[doc(hidden)]
pub struct _IManipulationEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ManipulationStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32) -> ::windows::core::HRESULT,
    pub ManipulationDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::HRESULT,
    pub ManipulationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
