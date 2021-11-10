#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseGestureInfoHandle<'a, Param0: ::windows::runtime::IntoParam<'a, HGESTUREINFO>>(hgestureinfo: Param0) -> super::super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseTouchInputHandle<'a, Param0: ::windows::runtime::IntoParam<'a, HTOUCHINPUT>>(htouchinput: Param0) -> super::super::super::Foundation::BOOL {
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Input_Touch`*"]
pub struct GESTURECONFIG {
    pub dwID: GESTURECONFIG_ID,
    pub dwWant: u32,
    pub dwBlock: u32,
}
impl GESTURECONFIG {}
impl ::core::default::Default for GESTURECONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GESTURECONFIG {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GESTURECONFIG").field("dwID", &self.dwID).field("dwWant", &self.dwWant).field("dwBlock", &self.dwBlock).finish()
    }
}
impl ::core::cmp::PartialEq for GESTURECONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.dwID == other.dwID && self.dwWant == other.dwWant && self.dwBlock == other.dwBlock
    }
}
impl ::core::cmp::Eq for GESTURECONFIG {}
unsafe impl ::windows::runtime::Abi for GESTURECONFIG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Input_Touch`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GESTURECONFIG_ID(pub u32);
pub const GID_BEGIN: GESTURECONFIG_ID = GESTURECONFIG_ID(1u32);
pub const GID_END: GESTURECONFIG_ID = GESTURECONFIG_ID(2u32);
pub const GID_ZOOM: GESTURECONFIG_ID = GESTURECONFIG_ID(3u32);
pub const GID_PAN: GESTURECONFIG_ID = GESTURECONFIG_ID(4u32);
pub const GID_ROTATE: GESTURECONFIG_ID = GESTURECONFIG_ID(5u32);
pub const GID_TWOFINGERTAP: GESTURECONFIG_ID = GESTURECONFIG_ID(6u32);
pub const GID_PRESSANDTAP: GESTURECONFIG_ID = GESTURECONFIG_ID(7u32);
pub const GID_ROLLOVER: GESTURECONFIG_ID = GESTURECONFIG_ID(7u32);
impl ::core::convert::From<u32> for GESTURECONFIG_ID {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GESTURECONFIG_ID {
    type Abi = Self;
}
impl ::core::ops::BitOr for GESTURECONFIG_ID {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for GESTURECONFIG_ID {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for GESTURECONFIG_ID {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for GESTURECONFIG_ID {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for GESTURECONFIG_ID {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
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
impl GESTUREINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GESTUREINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GESTUREINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GESTUREINFO")
            .field("cbSize", &self.cbSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwID", &self.dwID)
            .field("hwndTarget", &self.hwndTarget)
            .field("ptsLocation", &self.ptsLocation)
            .field("dwInstanceID", &self.dwInstanceID)
            .field("dwSequenceID", &self.dwSequenceID)
            .field("ullArguments", &self.ullArguments)
            .field("cbExtraArgs", &self.cbExtraArgs)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GESTUREINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.dwID == other.dwID && self.hwndTarget == other.hwndTarget && self.ptsLocation == other.ptsLocation && self.dwInstanceID == other.dwInstanceID && self.dwSequenceID == other.dwSequenceID && self.ullArguments == other.ullArguments && self.cbExtraArgs == other.cbExtraArgs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GESTUREINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GESTUREINFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
pub struct GESTURENOTIFYSTRUCT {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub hwndTarget: super::super::super::Foundation::HWND,
    pub ptsLocation: super::super::super::Foundation::POINTS,
    pub dwInstanceID: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl GESTURENOTIFYSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GESTURENOTIFYSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GESTURENOTIFYSTRUCT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GESTURENOTIFYSTRUCT").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("hwndTarget", &self.hwndTarget).field("ptsLocation", &self.ptsLocation).field("dwInstanceID", &self.dwInstanceID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GESTURENOTIFYSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.hwndTarget == other.hwndTarget && self.ptsLocation == other.ptsLocation && self.dwInstanceID == other.dwInstanceID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GESTURENOTIFYSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GESTURENOTIFYSTRUCT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGestureConfig<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(hwnd: Param0, dwreserved: u32, dwflags: u32, pcids: *const u32, pgestureconfig: *mut GESTURECONFIG, cbsize: u32) -> super::super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGestureExtraArgs<'a, Param0: ::windows::runtime::IntoParam<'a, HGESTUREINFO>>(hgestureinfo: Param0, cbextraargs: u32, pextraargs: *mut u8) -> super::super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGestureInfo<'a, Param0: ::windows::runtime::IntoParam<'a, HGESTUREINFO>>(hgestureinfo: Param0, pgestureinfo: *mut GESTUREINFO) -> super::super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTouchInputInfo<'a, Param0: ::windows::runtime::IntoParam<'a, HTOUCHINPUT>>(htouchinput: Param0, cinputs: u32, pinputs: *mut TOUCHINPUT, cbsize: i32) -> super::super::super::Foundation::BOOL {
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HGESTUREINFO(pub isize);
impl ::core::default::Default for HGESTUREINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HGESTUREINFO {}
unsafe impl ::windows::runtime::Abi for HGESTUREINFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HTOUCHINPUT(pub isize);
impl ::core::default::Default for HTOUCHINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HTOUCHINPUT {}
unsafe impl ::windows::runtime::Abi for HTOUCHINPUT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Input_Touch`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInertiaProcessor(pub ::windows::runtime::IUnknown);
impl IInertiaProcessor {
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn InitialOriginX(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetInitialOriginX(&self, x: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(x)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn InitialOriginY(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetInitialOriginY(&self, y: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn InitialVelocityX(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetInitialVelocityX(&self, x: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(x)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn InitialVelocityY(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetInitialVelocityY(&self, y: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn InitialAngularVelocity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetInitialAngularVelocity(&self, velocity: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(velocity)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn InitialExpansionVelocity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetInitialExpansionVelocity(&self, velocity: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(velocity)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn InitialRadius(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetInitialRadius(&self, radius: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn BoundaryLeft(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetBoundaryLeft(&self, left: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(left)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn BoundaryTop(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetBoundaryTop(&self, top: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(top)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn BoundaryRight(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetBoundaryRight(&self, right: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(right)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn BoundaryBottom(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetBoundaryBottom(&self, bottom: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(bottom)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn ElasticMarginLeft(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetElasticMarginLeft(&self, left: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(left)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn ElasticMarginTop(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetElasticMarginTop(&self, top: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(top)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn ElasticMarginRight(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetElasticMarginRight(&self, right: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(right)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn ElasticMarginBottom(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetElasticMarginBottom(&self, bottom: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(bottom)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn DesiredDisplacement(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetDesiredDisplacement(&self, displacement: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(displacement)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn DesiredRotation(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetDesiredRotation(&self, rotation: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(rotation)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn DesiredExpansion(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetDesiredExpansion(&self, expansion: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(expansion)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn DesiredDeceleration(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetDesiredDeceleration(&self, deceleration: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(deceleration)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn DesiredAngularDeceleration(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetDesiredAngularDeceleration(&self, deceleration: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(deceleration)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn DesiredExpansionDeceleration(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetDesiredExpansionDeceleration(&self, deceleration: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(deceleration)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn InitialTimestamp(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetInitialTimestamp(&self, timestamp: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(timestamp)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
    pub unsafe fn Process(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
    pub unsafe fn ProcessTime(&self, timestamp: u32) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(timestamp), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn Complete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn CompleteTime(&self, timestamp: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(timestamp)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInertiaProcessor {
    type Vtable = IInertiaProcessor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x18b00c6d_c5ee_41b1_90a9_9d4a929095ad);
}
impl ::core::convert::From<IInertiaProcessor> for ::windows::runtime::IUnknown {
    fn from(value: IInertiaProcessor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInertiaProcessor> for ::windows::runtime::IUnknown {
    fn from(value: &IInertiaProcessor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInertiaProcessor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInertiaProcessor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaProcessor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, y: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, y: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, y: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, y: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, velocity: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, velocity: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, velocity: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, velocity: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, radius: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, radius: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, left: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, left: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, top: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, top: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, right: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, right: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bottom: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bottom: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, left: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, left: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, top: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, top: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, right: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, right: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bottom: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bottom: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displacement: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displacement: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rotation: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rotation: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, expansion: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, expansion: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deceleration: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deceleration: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deceleration: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deceleration: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deceleration: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deceleration: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timestamp: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timestamp: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, completed: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timestamp: u32, completed: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timestamp: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Input_Touch`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IManipulationProcessor(pub ::windows::runtime::IUnknown);
impl IManipulationProcessor {
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SupportedManipulations(&self) -> ::windows::runtime::Result<MANIPULATION_PROCESSOR_MANIPULATIONS> {
        let mut result__: <MANIPULATION_PROCESSOR_MANIPULATIONS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MANIPULATION_PROCESSOR_MANIPULATIONS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetSupportedManipulations(&self, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulations)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn PivotPointX(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetPivotPointX(&self, pivotpointx: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pivotpointx)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn PivotPointY(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetPivotPointY(&self, pivotpointy: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pivotpointy)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn PivotRadius(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetPivotRadius(&self, pivotradius: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pivotradius)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn CompleteManipulation(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn ProcessDown(&self, manipulatorid: u32, x: f32, y: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulatorid), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn ProcessMove(&self, manipulatorid: u32, x: f32, y: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulatorid), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn ProcessUp(&self, manipulatorid: u32, x: f32, y: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulatorid), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn ProcessDownWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulatorid), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(timestamp)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn ProcessMoveWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulatorid), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(timestamp)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn ProcessUpWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(manipulatorid), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(timestamp)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn GetVelocityX(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn GetVelocityY(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn GetExpansionVelocity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn GetAngularVelocity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn MinimumScaleRotateRadius(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn SetMinimumScaleRotateRadius(&self, minradius: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(minradius)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IManipulationProcessor {
    type Vtable = IManipulationProcessor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa22ac519_8300_48a0_bef4_f1be8737dba4);
}
impl ::core::convert::From<IManipulationProcessor> for ::windows::runtime::IUnknown {
    fn from(value: IManipulationProcessor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IManipulationProcessor> for ::windows::runtime::IUnknown {
    fn from(value: &IManipulationProcessor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IManipulationProcessor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IManipulationProcessor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationProcessor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manipulations: *mut MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pivotpointx: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pivotpointx: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pivotpointy: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pivotpointy: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pivotradius: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pivotradius: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manipulatorid: u32, x: f32, y: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manipulatorid: u32, x: f32, y: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manipulatorid: u32, x: f32, y: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, velocityx: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, velocityy: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, expansionvelocity: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, angularvelocity: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minradius: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minradius: f32) -> ::windows::runtime::HRESULT,
);
pub const InertiaProcessor: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xabb27087_4ce0_4e58_a0cb_e24df96814be);
#[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsTouchWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(hwnd: Param0, pulflags: *mut u32) -> super::super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_UI_Input_Touch`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MANIPULATION_PROCESSOR_MANIPULATIONS(pub i32);
pub const MANIPULATION_NONE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(0i32);
pub const MANIPULATION_TRANSLATE_X: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(1i32);
pub const MANIPULATION_TRANSLATE_Y: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(2i32);
pub const MANIPULATION_SCALE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(4i32);
pub const MANIPULATION_ROTATE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(8i32);
pub const MANIPULATION_ALL: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(15i32);
impl ::core::convert::From<i32> for MANIPULATION_PROCESSOR_MANIPULATIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MANIPULATION_PROCESSOR_MANIPULATIONS {
    type Abi = Self;
}
pub const ManipulationProcessor: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x597d4fb0_47fd_4aff_89b9_c6cfae8cf08e);
#[doc = "*Required features: `Win32_UI_Input_Touch`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct REGISTER_TOUCH_WINDOW_FLAGS(pub u32);
pub const TWF_FINETOUCH: REGISTER_TOUCH_WINDOW_FLAGS = REGISTER_TOUCH_WINDOW_FLAGS(1u32);
pub const TWF_WANTPALM: REGISTER_TOUCH_WINDOW_FLAGS = REGISTER_TOUCH_WINDOW_FLAGS(2u32);
impl ::core::convert::From<u32> for REGISTER_TOUCH_WINDOW_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REGISTER_TOUCH_WINDOW_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for REGISTER_TOUCH_WINDOW_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for REGISTER_TOUCH_WINDOW_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for REGISTER_TOUCH_WINDOW_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for REGISTER_TOUCH_WINDOW_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for REGISTER_TOUCH_WINDOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterTouchWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(hwnd: Param0, ulflags: REGISTER_TOUCH_WINDOW_FLAGS) -> super::super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetGestureConfig<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(hwnd: Param0, dwreserved: u32, cids: u32, pgestureconfig: *const GESTURECONFIG, cbsize: u32) -> super::super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_UI_Input_Touch`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TOUCHEVENTF_FLAGS(pub u32);
pub const TOUCHEVENTF_MOVE: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(1u32);
pub const TOUCHEVENTF_DOWN: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(2u32);
pub const TOUCHEVENTF_UP: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(4u32);
pub const TOUCHEVENTF_INRANGE: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(8u32);
pub const TOUCHEVENTF_PRIMARY: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(16u32);
pub const TOUCHEVENTF_NOCOALESCE: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(32u32);
pub const TOUCHEVENTF_PEN: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(64u32);
pub const TOUCHEVENTF_PALM: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(128u32);
impl ::core::convert::From<u32> for TOUCHEVENTF_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TOUCHEVENTF_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for TOUCHEVENTF_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for TOUCHEVENTF_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for TOUCHEVENTF_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for TOUCHEVENTF_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for TOUCHEVENTF_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
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
impl TOUCHINPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOUCHINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOUCHINPUT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TOUCHINPUT")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("hSource", &self.hSource)
            .field("dwID", &self.dwID)
            .field("dwFlags", &self.dwFlags)
            .field("dwMask", &self.dwMask)
            .field("dwTime", &self.dwTime)
            .field("dwExtraInfo", &self.dwExtraInfo)
            .field("cxContact", &self.cxContact)
            .field("cyContact", &self.cyContact)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOUCHINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.hSource == other.hSource && self.dwID == other.dwID && self.dwFlags == other.dwFlags && self.dwMask == other.dwMask && self.dwTime == other.dwTime && self.dwExtraInfo == other.dwExtraInfo && self.cxContact == other.cxContact && self.cyContact == other.cyContact
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOUCHINPUT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TOUCHINPUT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Input_Touch`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TOUCHINPUTMASKF_MASK(pub u32);
pub const TOUCHINPUTMASKF_TIMEFROMSYSTEM: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(1u32);
pub const TOUCHINPUTMASKF_EXTRAINFO: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(2u32);
pub const TOUCHINPUTMASKF_CONTACTAREA: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(4u32);
impl ::core::convert::From<u32> for TOUCHINPUTMASKF_MASK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TOUCHINPUTMASKF_MASK {
    type Abi = Self;
}
impl ::core::ops::BitOr for TOUCHINPUTMASKF_MASK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for TOUCHINPUTMASKF_MASK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for TOUCHINPUTMASKF_MASK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for TOUCHINPUTMASKF_MASK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for TOUCHINPUTMASKF_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterTouchWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(hwnd: Param0) -> super::super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_UI_Input_Touch`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct _IManipulationEvents(pub ::windows::runtime::IUnknown);
impl _IManipulationEvents {
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn ManipulationStarted(&self, x: f32, y: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn ManipulationDelta(&self, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
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
    #[doc = "*Required features: `Win32_UI_Input_Touch`*"]
    pub unsafe fn ManipulationCompleted(&self, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(x),
            ::core::mem::transmute(y),
            ::core::mem::transmute(cumulativetranslationx),
            ::core::mem::transmute(cumulativetranslationy),
            ::core::mem::transmute(cumulativescale),
            ::core::mem::transmute(cumulativeexpansion),
            ::core::mem::transmute(cumulativerotation),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for _IManipulationEvents {
    type Vtable = _IManipulationEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4f62c8da_9c53_4b22_93df_927a862bbb03);
}
impl ::core::convert::From<_IManipulationEvents> for ::windows::runtime::IUnknown {
    fn from(value: _IManipulationEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&_IManipulationEvents> for ::windows::runtime::IUnknown {
    fn from(value: &_IManipulationEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for _IManipulationEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a _IManipulationEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _IManipulationEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: f32, y: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::runtime::HRESULT,
);
