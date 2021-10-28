#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CloseGestureInfoHandle<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HGESTUREINFO>,
>(
    hgestureinfo: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseGestureInfoHandle(hgestureinfo: HGESTUREINFO)
                -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CloseGestureInfoHandle(hgestureinfo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CloseTouchInputHandle<'a, Param0: ::windows::runtime::IntoParam<'a, HTOUCHINPUT>>(
    htouchinput: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseTouchInputHandle(htouchinput: HTOUCHINPUT) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CloseTouchInputHandle(htouchinput.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GESTURECONFIG {
    pub dwID: GESTURECONFIG_ID,
    pub dwWant: u32,
    pub dwBlock: u32,
}
impl GESTURECONFIG {}
impl ::std::default::Default for GESTURECONFIG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GESTURECONFIG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GESTURECONFIG")
            .field("dwID", &self.dwID)
            .field("dwWant", &self.dwWant)
            .field("dwBlock", &self.dwBlock)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GESTURECONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.dwID == other.dwID && self.dwWant == other.dwWant && self.dwBlock == other.dwBlock
    }
}
impl ::std::cmp::Eq for GESTURECONFIG {}
unsafe impl ::windows::runtime::Abi for GESTURECONFIG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
impl ::std::convert::From<u32> for GESTURECONFIG_ID {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GESTURECONFIG_ID {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for GESTURECONFIG_ID {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for GESTURECONFIG_ID {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for GESTURECONFIG_ID {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for GESTURECONFIG_ID {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for GESTURECONFIG_ID {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GESTUREINFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwID: u32,
    pub hwndTarget: super::super::Foundation::HWND,
    pub ptsLocation: super::super::Foundation::POINTS,
    pub dwInstanceID: u32,
    pub dwSequenceID: u32,
    pub ullArguments: u64,
    pub cbExtraArgs: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl GESTUREINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GESTUREINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GESTUREINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for GESTUREINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwFlags == other.dwFlags
            && self.dwID == other.dwID
            && self.hwndTarget == other.hwndTarget
            && self.ptsLocation == other.ptsLocation
            && self.dwInstanceID == other.dwInstanceID
            && self.dwSequenceID == other.dwSequenceID
            && self.ullArguments == other.ullArguments
            && self.cbExtraArgs == other.cbExtraArgs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GESTUREINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GESTUREINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GESTURENOTIFYSTRUCT {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub hwndTarget: super::super::Foundation::HWND,
    pub ptsLocation: super::super::Foundation::POINTS,
    pub dwInstanceID: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl GESTURENOTIFYSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GESTURENOTIFYSTRUCT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GESTURENOTIFYSTRUCT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GESTURENOTIFYSTRUCT")
            .field("cbSize", &self.cbSize)
            .field("dwFlags", &self.dwFlags)
            .field("hwndTarget", &self.hwndTarget)
            .field("ptsLocation", &self.ptsLocation)
            .field("dwInstanceID", &self.dwInstanceID)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GESTURENOTIFYSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwFlags == other.dwFlags
            && self.hwndTarget == other.hwndTarget
            && self.ptsLocation == other.ptsLocation
            && self.dwInstanceID == other.dwInstanceID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GESTURENOTIFYSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GESTURENOTIFYSTRUCT {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetGestureConfig<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    dwreserved: u32,
    dwflags: u32,
    pcids: *const u32,
    pgestureconfig: *mut GESTURECONFIG,
    cbsize: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGestureConfig(
                hwnd: super::super::Foundation::HWND,
                dwreserved: u32,
                dwflags: u32,
                pcids: *const u32,
                pgestureconfig: *mut GESTURECONFIG,
                cbsize: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetGestureConfig(
            hwnd.into_param().abi(),
            ::std::mem::transmute(dwreserved),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pcids),
            ::std::mem::transmute(pgestureconfig),
            ::std::mem::transmute(cbsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetGestureExtraArgs<'a, Param0: ::windows::runtime::IntoParam<'a, HGESTUREINFO>>(
    hgestureinfo: Param0,
    cbextraargs: u32,
    pextraargs: *mut u8,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGestureExtraArgs(
                hgestureinfo: HGESTUREINFO,
                cbextraargs: u32,
                pextraargs: *mut u8,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetGestureExtraArgs(
            hgestureinfo.into_param().abi(),
            ::std::mem::transmute(cbextraargs),
            ::std::mem::transmute(pextraargs),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetGestureInfo<'a, Param0: ::windows::runtime::IntoParam<'a, HGESTUREINFO>>(
    hgestureinfo: Param0,
    pgestureinfo: *mut GESTUREINFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGestureInfo(
                hgestureinfo: HGESTUREINFO,
                pgestureinfo: *mut GESTUREINFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetGestureInfo(
            hgestureinfo.into_param().abi(),
            ::std::mem::transmute(pgestureinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetTouchInputInfo<'a, Param0: ::windows::runtime::IntoParam<'a, HTOUCHINPUT>>(
    htouchinput: Param0,
    cinputs: u32,
    pinputs: *mut TOUCHINPUT,
    cbsize: i32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTouchInputInfo(
                htouchinput: HTOUCHINPUT,
                cinputs: u32,
                pinputs: *mut TOUCHINPUT,
                cbsize: i32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetTouchInputInfo(
            htouchinput.into_param().abi(),
            ::std::mem::transmute(cinputs),
            ::std::mem::transmute(pinputs),
            ::std::mem::transmute(cbsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HGESTUREINFO(pub isize);
impl ::std::default::Default for HGESTUREINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HGESTUREINFO {}
unsafe impl ::windows::runtime::Abi for HGESTUREINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HTOUCHINPUT(pub isize);
impl ::std::default::Default for HTOUCHINPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HTOUCHINPUT {}
unsafe impl ::windows::runtime::Abi for HTOUCHINPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IInertiaProcessor(::windows::runtime::IUnknown);
impl IInertiaProcessor {
    pub unsafe fn InitialOriginX(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetInitialOriginX(&self, x: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(x),
        )
        .ok()
    }
    pub unsafe fn InitialOriginY(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetInitialOriginY(&self, y: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(y),
        )
        .ok()
    }
    pub unsafe fn InitialVelocityX(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetInitialVelocityX(&self, x: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(x),
        )
        .ok()
    }
    pub unsafe fn InitialVelocityY(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetInitialVelocityY(&self, y: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(y),
        )
        .ok()
    }
    pub unsafe fn InitialAngularVelocity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetInitialAngularVelocity(
        &self,
        velocity: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(velocity),
        )
        .ok()
    }
    pub unsafe fn InitialExpansionVelocity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetInitialExpansionVelocity(
        &self,
        velocity: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(velocity),
        )
        .ok()
    }
    pub unsafe fn InitialRadius(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetInitialRadius(&self, radius: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(radius),
        )
        .ok()
    }
    pub unsafe fn BoundaryLeft(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetBoundaryLeft(&self, left: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(left),
        )
        .ok()
    }
    pub unsafe fn BoundaryTop(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetBoundaryTop(&self, top: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(top),
        )
        .ok()
    }
    pub unsafe fn BoundaryRight(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetBoundaryRight(&self, right: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(right),
        )
        .ok()
    }
    pub unsafe fn BoundaryBottom(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetBoundaryBottom(&self, bottom: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bottom),
        )
        .ok()
    }
    pub unsafe fn ElasticMarginLeft(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetElasticMarginLeft(&self, left: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(left),
        )
        .ok()
    }
    pub unsafe fn ElasticMarginTop(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetElasticMarginTop(&self, top: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(top),
        )
        .ok()
    }
    pub unsafe fn ElasticMarginRight(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetElasticMarginRight(&self, right: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(right),
        )
        .ok()
    }
    pub unsafe fn ElasticMarginBottom(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetElasticMarginBottom(&self, bottom: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bottom),
        )
        .ok()
    }
    pub unsafe fn DesiredDisplacement(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetDesiredDisplacement(
        &self,
        displacement: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(displacement),
        )
        .ok()
    }
    pub unsafe fn DesiredRotation(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetDesiredRotation(&self, rotation: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rotation),
        )
        .ok()
    }
    pub unsafe fn DesiredExpansion(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetDesiredExpansion(&self, expansion: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(expansion),
        )
        .ok()
    }
    pub unsafe fn DesiredDeceleration(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetDesiredDeceleration(
        &self,
        deceleration: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(deceleration),
        )
        .ok()
    }
    pub unsafe fn DesiredAngularDeceleration(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetDesiredAngularDeceleration(
        &self,
        deceleration: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(deceleration),
        )
        .ok()
    }
    pub unsafe fn DesiredExpansionDeceleration(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetDesiredExpansionDeceleration(
        &self,
        deceleration: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(deceleration),
        )
        .ok()
    }
    pub unsafe fn InitialTimestamp(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn SetInitialTimestamp(&self, timestamp: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(timestamp),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Process(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProcessTime(
        &self,
        timestamp: u32,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(timestamp),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn Complete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CompleteTime(&self, timestamp: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).51)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(timestamp),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInertiaProcessor {
    type Vtable = IInertiaProcessor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        414190701,
        50670,
        16817,
        [144, 169, 157, 74, 146, 144, 149, 173],
    );
}
impl ::std::convert::From<IInertiaProcessor> for ::windows::runtime::IUnknown {
    fn from(value: IInertiaProcessor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInertiaProcessor> for ::windows::runtime::IUnknown {
    fn from(value: &IInertiaProcessor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInertiaProcessor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IInertiaProcessor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaProcessor_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        x: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        x: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        y: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        y: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        x: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        x: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        y: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        y: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        velocity: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        velocity: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        velocity: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        velocity: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        radius: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        radius: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        left: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        left: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        top: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        top: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        right: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        right: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bottom: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bottom: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        left: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        left: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        top: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        top: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        right: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        right: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bottom: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bottom: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        displacement: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        displacement: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rotation: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rotation: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        expansion: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        expansion: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deceleration: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deceleration: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deceleration: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deceleration: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deceleration: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deceleration: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        timestamp: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        timestamp: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        completed: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        timestamp: u32,
        completed: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        timestamp: u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IManipulationProcessor(::windows::runtime::IUnknown);
impl IManipulationProcessor {
    pub unsafe fn SupportedManipulations(
        &self,
    ) -> ::windows::runtime::Result<MANIPULATION_PROCESSOR_MANIPULATIONS> {
        let mut result__: <MANIPULATION_PROCESSOR_MANIPULATIONS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<MANIPULATION_PROCESSOR_MANIPULATIONS>(result__)
    }
    pub unsafe fn SetSupportedManipulations(
        &self,
        manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(manipulations),
        )
        .ok()
    }
    pub unsafe fn PivotPointX(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetPivotPointX(&self, pivotpointx: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pivotpointx),
        )
        .ok()
    }
    pub unsafe fn PivotPointY(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetPivotPointY(&self, pivotpointy: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pivotpointy),
        )
        .ok()
    }
    pub unsafe fn PivotRadius(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetPivotRadius(&self, pivotradius: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pivotradius),
        )
        .ok()
    }
    pub unsafe fn CompleteManipulation(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ProcessDown(
        &self,
        manipulatorid: u32,
        x: f32,
        y: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(manipulatorid),
            ::std::mem::transmute(x),
            ::std::mem::transmute(y),
        )
        .ok()
    }
    pub unsafe fn ProcessMove(
        &self,
        manipulatorid: u32,
        x: f32,
        y: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(manipulatorid),
            ::std::mem::transmute(x),
            ::std::mem::transmute(y),
        )
        .ok()
    }
    pub unsafe fn ProcessUp(
        &self,
        manipulatorid: u32,
        x: f32,
        y: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(manipulatorid),
            ::std::mem::transmute(x),
            ::std::mem::transmute(y),
        )
        .ok()
    }
    pub unsafe fn ProcessDownWithTime(
        &self,
        manipulatorid: u32,
        x: f32,
        y: f32,
        timestamp: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(manipulatorid),
            ::std::mem::transmute(x),
            ::std::mem::transmute(y),
            ::std::mem::transmute(timestamp),
        )
        .ok()
    }
    pub unsafe fn ProcessMoveWithTime(
        &self,
        manipulatorid: u32,
        x: f32,
        y: f32,
        timestamp: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(manipulatorid),
            ::std::mem::transmute(x),
            ::std::mem::transmute(y),
            ::std::mem::transmute(timestamp),
        )
        .ok()
    }
    pub unsafe fn ProcessUpWithTime(
        &self,
        manipulatorid: u32,
        x: f32,
        y: f32,
        timestamp: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(manipulatorid),
            ::std::mem::transmute(x),
            ::std::mem::transmute(y),
            ::std::mem::transmute(timestamp),
        )
        .ok()
    }
    pub unsafe fn GetVelocityX(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn GetVelocityY(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn GetExpansionVelocity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn GetAngularVelocity(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn MinimumScaleRotateRadius(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetMinimumScaleRotateRadius(
        &self,
        minradius: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(minradius),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IManipulationProcessor {
    type Vtable = IManipulationProcessor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2720711961,
        33536,
        18592,
        [190, 244, 241, 190, 135, 55, 219, 164],
    );
}
impl ::std::convert::From<IManipulationProcessor> for ::windows::runtime::IUnknown {
    fn from(value: IManipulationProcessor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IManipulationProcessor> for ::windows::runtime::IUnknown {
    fn from(value: &IManipulationProcessor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IManipulationProcessor
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IManipulationProcessor
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationProcessor_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        manipulations: *mut MANIPULATION_PROCESSOR_MANIPULATIONS,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pivotpointx: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pivotpointx: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pivotpointy: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pivotpointy: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pivotradius: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pivotradius: f32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        manipulatorid: u32,
        x: f32,
        y: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        manipulatorid: u32,
        x: f32,
        y: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        manipulatorid: u32,
        x: f32,
        y: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        manipulatorid: u32,
        x: f32,
        y: f32,
        timestamp: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        manipulatorid: u32,
        x: f32,
        y: f32,
        timestamp: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        manipulatorid: u32,
        x: f32,
        y: f32,
        timestamp: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        velocityx: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        velocityy: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        expansionvelocity: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        angularvelocity: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        minradius: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        minradius: f32,
    ) -> ::windows::runtime::HRESULT,
);
pub const InertiaProcessor: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2880598151,
    19680,
    20056,
    [160, 203, 226, 77, 249, 104, 20, 190],
);
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn IsTouchWindow<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    pulflags: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsTouchWindow(
                hwnd: super::super::Foundation::HWND,
                pulflags: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsTouchWindow(
            hwnd.into_param().abi(),
            ::std::mem::transmute(pulflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MANIPULATION_PROCESSOR_MANIPULATIONS(pub i32);
pub const MANIPULATION_NONE: MANIPULATION_PROCESSOR_MANIPULATIONS =
    MANIPULATION_PROCESSOR_MANIPULATIONS(0i32);
pub const MANIPULATION_TRANSLATE_X: MANIPULATION_PROCESSOR_MANIPULATIONS =
    MANIPULATION_PROCESSOR_MANIPULATIONS(1i32);
pub const MANIPULATION_TRANSLATE_Y: MANIPULATION_PROCESSOR_MANIPULATIONS =
    MANIPULATION_PROCESSOR_MANIPULATIONS(2i32);
pub const MANIPULATION_SCALE: MANIPULATION_PROCESSOR_MANIPULATIONS =
    MANIPULATION_PROCESSOR_MANIPULATIONS(4i32);
pub const MANIPULATION_ROTATE: MANIPULATION_PROCESSOR_MANIPULATIONS =
    MANIPULATION_PROCESSOR_MANIPULATIONS(8i32);
pub const MANIPULATION_ALL: MANIPULATION_PROCESSOR_MANIPULATIONS =
    MANIPULATION_PROCESSOR_MANIPULATIONS(15i32);
impl ::std::convert::From<i32> for MANIPULATION_PROCESSOR_MANIPULATIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MANIPULATION_PROCESSOR_MANIPULATIONS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ManipulationProcessor: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1501384624,
    18429,
    19199,
    [137, 185, 198, 207, 174, 140, 240, 142],
);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct REGISTER_TOUCH_WINDOW_FLAGS(pub u32);
pub const TWF_FINETOUCH: REGISTER_TOUCH_WINDOW_FLAGS = REGISTER_TOUCH_WINDOW_FLAGS(1u32);
pub const TWF_WANTPALM: REGISTER_TOUCH_WINDOW_FLAGS = REGISTER_TOUCH_WINDOW_FLAGS(2u32);
impl ::std::convert::From<u32> for REGISTER_TOUCH_WINDOW_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REGISTER_TOUCH_WINDOW_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for REGISTER_TOUCH_WINDOW_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for REGISTER_TOUCH_WINDOW_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for REGISTER_TOUCH_WINDOW_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for REGISTER_TOUCH_WINDOW_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for REGISTER_TOUCH_WINDOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RegisterTouchWindow<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    ulflags: REGISTER_TOUCH_WINDOW_FLAGS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterTouchWindow(
                hwnd: super::super::Foundation::HWND,
                ulflags: REGISTER_TOUCH_WINDOW_FLAGS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RegisterTouchWindow(
            hwnd.into_param().abi(),
            ::std::mem::transmute(ulflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetGestureConfig<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    dwreserved: u32,
    cids: u32,
    pgestureconfig: *const GESTURECONFIG,
    cbsize: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetGestureConfig(
                hwnd: super::super::Foundation::HWND,
                dwreserved: u32,
                cids: u32,
                pgestureconfig: *const GESTURECONFIG,
                cbsize: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetGestureConfig(
            hwnd.into_param().abi(),
            ::std::mem::transmute(dwreserved),
            ::std::mem::transmute(cids),
            ::std::mem::transmute(pgestureconfig),
            ::std::mem::transmute(cbsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
impl ::std::convert::From<u32> for TOUCHEVENTF_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TOUCHEVENTF_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for TOUCHEVENTF_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TOUCHEVENTF_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TOUCHEVENTF_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TOUCHEVENTF_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TOUCHEVENTF_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TOUCHINPUT {
    pub x: i32,
    pub y: i32,
    pub hSource: super::super::Foundation::HANDLE,
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
impl ::std::default::Default for TOUCHINPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TOUCHINPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for TOUCHINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.hSource == other.hSource
            && self.dwID == other.dwID
            && self.dwFlags == other.dwFlags
            && self.dwMask == other.dwMask
            && self.dwTime == other.dwTime
            && self.dwExtraInfo == other.dwExtraInfo
            && self.cxContact == other.cxContact
            && self.cyContact == other.cyContact
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TOUCHINPUT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TOUCHINPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TOUCHINPUTMASKF_MASK(pub u32);
pub const TOUCHINPUTMASKF_TIMEFROMSYSTEM: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(1u32);
pub const TOUCHINPUTMASKF_EXTRAINFO: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(2u32);
pub const TOUCHINPUTMASKF_CONTACTAREA: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(4u32);
impl ::std::convert::From<u32> for TOUCHINPUTMASKF_MASK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TOUCHINPUTMASKF_MASK {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for TOUCHINPUTMASKF_MASK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TOUCHINPUTMASKF_MASK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TOUCHINPUTMASKF_MASK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TOUCHINPUTMASKF_MASK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TOUCHINPUTMASKF_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn UnregisterTouchWindow<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterTouchWindow(
                hwnd: super::super::Foundation::HWND,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UnregisterTouchWindow(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct _IManipulationEvents(::windows::runtime::IUnknown);
impl _IManipulationEvents {
    pub unsafe fn ManipulationStarted(&self, x: f32, y: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(x),
            ::std::mem::transmute(y),
        )
        .ok()
    }
    pub unsafe fn ManipulationDelta(
        &self,
        x: f32,
        y: f32,
        translationdeltax: f32,
        translationdeltay: f32,
        scaledelta: f32,
        expansiondelta: f32,
        rotationdelta: f32,
        cumulativetranslationx: f32,
        cumulativetranslationy: f32,
        cumulativescale: f32,
        cumulativeexpansion: f32,
        cumulativerotation: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(x),
            ::std::mem::transmute(y),
            ::std::mem::transmute(translationdeltax),
            ::std::mem::transmute(translationdeltay),
            ::std::mem::transmute(scaledelta),
            ::std::mem::transmute(expansiondelta),
            ::std::mem::transmute(rotationdelta),
            ::std::mem::transmute(cumulativetranslationx),
            ::std::mem::transmute(cumulativetranslationy),
            ::std::mem::transmute(cumulativescale),
            ::std::mem::transmute(cumulativeexpansion),
            ::std::mem::transmute(cumulativerotation),
        )
        .ok()
    }
    pub unsafe fn ManipulationCompleted(
        &self,
        x: f32,
        y: f32,
        cumulativetranslationx: f32,
        cumulativetranslationy: f32,
        cumulativescale: f32,
        cumulativeexpansion: f32,
        cumulativerotation: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(x),
            ::std::mem::transmute(y),
            ::std::mem::transmute(cumulativetranslationx),
            ::std::mem::transmute(cumulativetranslationy),
            ::std::mem::transmute(cumulativescale),
            ::std::mem::transmute(cumulativeexpansion),
            ::std::mem::transmute(cumulativerotation),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for _IManipulationEvents {
    type Vtable = _IManipulationEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1331874010,
        40019,
        19234,
        [147, 223, 146, 122, 134, 43, 187, 3],
    );
}
impl ::std::convert::From<_IManipulationEvents> for ::windows::runtime::IUnknown {
    fn from(value: _IManipulationEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&_IManipulationEvents> for ::windows::runtime::IUnknown {
    fn from(value: &_IManipulationEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for _IManipulationEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &_IManipulationEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _IManipulationEvents_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        x: f32,
        y: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        x: f32,
        y: f32,
        translationdeltax: f32,
        translationdeltay: f32,
        scaledelta: f32,
        expansiondelta: f32,
        rotationdelta: f32,
        cumulativetranslationx: f32,
        cumulativetranslationy: f32,
        cumulativescale: f32,
        cumulativeexpansion: f32,
        cumulativerotation: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        x: f32,
        y: f32,
        cumulativetranslationx: f32,
        cumulativetranslationy: f32,
        cumulativescale: f32,
        cumulativeexpansion: f32,
        cumulativerotation: f32,
    ) -> ::windows::runtime::HRESULT,
);
