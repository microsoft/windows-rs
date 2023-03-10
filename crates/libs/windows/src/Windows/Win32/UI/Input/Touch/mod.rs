#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseGestureInfoHandle<P0>(hgestureinfo: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<HGESTUREINFO>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn CloseGestureInfoHandle ( hgestureinfo : HGESTUREINFO ) -> super::super::super::Foundation:: BOOL );
    CloseGestureInfoHandle(hgestureinfo.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseTouchInputHandle<P0>(htouchinput: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<HTOUCHINPUT>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn CloseTouchInputHandle ( htouchinput : HTOUCHINPUT ) -> super::super::super::Foundation:: BOOL );
    CloseTouchInputHandle(htouchinput.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGestureConfig<P0>(hwnd: P0, dwreserved: u32, dwflags: u32, pcids: *const u32, pgestureconfig: *mut GESTURECONFIG, cbsize: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HWND>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn GetGestureConfig ( hwnd : super::super::super::Foundation:: HWND , dwreserved : u32 , dwflags : u32 , pcids : *const u32 , pgestureconfig : *mut GESTURECONFIG , cbsize : u32 ) -> super::super::super::Foundation:: BOOL );
    GetGestureConfig(hwnd.into_param().abi(), dwreserved, dwflags, pcids, pgestureconfig, cbsize)
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGestureExtraArgs<P0>(hgestureinfo: P0, pextraargs: &mut [u8]) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<HGESTUREINFO>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn GetGestureExtraArgs ( hgestureinfo : HGESTUREINFO , cbextraargs : u32 , pextraargs : *mut u8 ) -> super::super::super::Foundation:: BOOL );
    GetGestureExtraArgs(hgestureinfo.into_param().abi(), pextraargs.len() as _, ::core::mem::transmute(pextraargs.as_ptr()))
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGestureInfo<P0>(hgestureinfo: P0, pgestureinfo: *mut GESTUREINFO) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<HGESTUREINFO>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn GetGestureInfo ( hgestureinfo : HGESTUREINFO , pgestureinfo : *mut GESTUREINFO ) -> super::super::super::Foundation:: BOOL );
    GetGestureInfo(hgestureinfo.into_param().abi(), pgestureinfo)
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTouchInputInfo<P0>(htouchinput: P0, pinputs: &mut [TOUCHINPUT], cbsize: i32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<HTOUCHINPUT>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn GetTouchInputInfo ( htouchinput : HTOUCHINPUT , cinputs : u32 , pinputs : *mut TOUCHINPUT , cbsize : i32 ) -> super::super::super::Foundation:: BOOL );
    GetTouchInputInfo(htouchinput.into_param().abi(), pinputs.len() as _, ::core::mem::transmute(pinputs.as_ptr()), cbsize)
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsTouchWindow<P0>(hwnd: P0, pulflags: ::core::option::Option<*mut u32>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HWND>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn IsTouchWindow ( hwnd : super::super::super::Foundation:: HWND , pulflags : *mut u32 ) -> super::super::super::Foundation:: BOOL );
    IsTouchWindow(hwnd.into_param().abi(), ::core::mem::transmute(pulflags.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterTouchWindow<P0>(hwnd: P0, ulflags: REGISTER_TOUCH_WINDOW_FLAGS) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HWND>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn RegisterTouchWindow ( hwnd : super::super::super::Foundation:: HWND , ulflags : REGISTER_TOUCH_WINDOW_FLAGS ) -> super::super::super::Foundation:: BOOL );
    RegisterTouchWindow(hwnd.into_param().abi(), ulflags)
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetGestureConfig<P0>(hwnd: P0, dwreserved: u32, pgestureconfig: &[GESTURECONFIG], cbsize: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HWND>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn SetGestureConfig ( hwnd : super::super::super::Foundation:: HWND , dwreserved : u32 , cids : u32 , pgestureconfig : *const GESTURECONFIG , cbsize : u32 ) -> super::super::super::Foundation:: BOOL );
    SetGestureConfig(hwnd.into_param().abi(), dwreserved, pgestureconfig.len() as _, ::core::mem::transmute(pgestureconfig.as_ptr()), cbsize)
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterTouchWindow<P0>(hwnd: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HWND>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn UnregisterTouchWindow ( hwnd : super::super::super::Foundation:: HWND ) -> super::super::super::Foundation:: BOOL );
    UnregisterTouchWindow(hwnd.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
pub struct IInertiaProcessor(::windows::core::IUnknown);
impl IInertiaProcessor {
    pub unsafe fn InitialOriginX(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).InitialOriginX)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInitialOriginX(&self, x: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialOriginX)(::windows::core::Interface::as_raw(self), x).ok()
    }
    pub unsafe fn InitialOriginY(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).InitialOriginY)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInitialOriginY(&self, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialOriginY)(::windows::core::Interface::as_raw(self), y).ok()
    }
    pub unsafe fn InitialVelocityX(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).InitialVelocityX)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInitialVelocityX(&self, x: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialVelocityX)(::windows::core::Interface::as_raw(self), x).ok()
    }
    pub unsafe fn InitialVelocityY(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).InitialVelocityY)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInitialVelocityY(&self, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialVelocityY)(::windows::core::Interface::as_raw(self), y).ok()
    }
    pub unsafe fn InitialAngularVelocity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).InitialAngularVelocity)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInitialAngularVelocity(&self, velocity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialAngularVelocity)(::windows::core::Interface::as_raw(self), velocity).ok()
    }
    pub unsafe fn InitialExpansionVelocity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).InitialExpansionVelocity)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInitialExpansionVelocity(&self, velocity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialExpansionVelocity)(::windows::core::Interface::as_raw(self), velocity).ok()
    }
    pub unsafe fn InitialRadius(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).InitialRadius)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInitialRadius(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialRadius)(::windows::core::Interface::as_raw(self), radius).ok()
    }
    pub unsafe fn BoundaryLeft(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).BoundaryLeft)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBoundaryLeft(&self, left: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBoundaryLeft)(::windows::core::Interface::as_raw(self), left).ok()
    }
    pub unsafe fn BoundaryTop(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).BoundaryTop)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBoundaryTop(&self, top: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBoundaryTop)(::windows::core::Interface::as_raw(self), top).ok()
    }
    pub unsafe fn BoundaryRight(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).BoundaryRight)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBoundaryRight(&self, right: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBoundaryRight)(::windows::core::Interface::as_raw(self), right).ok()
    }
    pub unsafe fn BoundaryBottom(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).BoundaryBottom)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBoundaryBottom(&self, bottom: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBoundaryBottom)(::windows::core::Interface::as_raw(self), bottom).ok()
    }
    pub unsafe fn ElasticMarginLeft(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).ElasticMarginLeft)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetElasticMarginLeft(&self, left: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetElasticMarginLeft)(::windows::core::Interface::as_raw(self), left).ok()
    }
    pub unsafe fn ElasticMarginTop(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).ElasticMarginTop)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetElasticMarginTop(&self, top: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetElasticMarginTop)(::windows::core::Interface::as_raw(self), top).ok()
    }
    pub unsafe fn ElasticMarginRight(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).ElasticMarginRight)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetElasticMarginRight(&self, right: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetElasticMarginRight)(::windows::core::Interface::as_raw(self), right).ok()
    }
    pub unsafe fn ElasticMarginBottom(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).ElasticMarginBottom)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetElasticMarginBottom(&self, bottom: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetElasticMarginBottom)(::windows::core::Interface::as_raw(self), bottom).ok()
    }
    pub unsafe fn DesiredDisplacement(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).DesiredDisplacement)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDesiredDisplacement(&self, displacement: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDesiredDisplacement)(::windows::core::Interface::as_raw(self), displacement).ok()
    }
    pub unsafe fn DesiredRotation(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).DesiredRotation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDesiredRotation(&self, rotation: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDesiredRotation)(::windows::core::Interface::as_raw(self), rotation).ok()
    }
    pub unsafe fn DesiredExpansion(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).DesiredExpansion)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDesiredExpansion(&self, expansion: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDesiredExpansion)(::windows::core::Interface::as_raw(self), expansion).ok()
    }
    pub unsafe fn DesiredDeceleration(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).DesiredDeceleration)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDesiredDeceleration(&self, deceleration: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDesiredDeceleration)(::windows::core::Interface::as_raw(self), deceleration).ok()
    }
    pub unsafe fn DesiredAngularDeceleration(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).DesiredAngularDeceleration)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDesiredAngularDeceleration(&self, deceleration: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDesiredAngularDeceleration)(::windows::core::Interface::as_raw(self), deceleration).ok()
    }
    pub unsafe fn DesiredExpansionDeceleration(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).DesiredExpansionDeceleration)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDesiredExpansionDeceleration(&self, deceleration: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDesiredExpansionDeceleration)(::windows::core::Interface::as_raw(self), deceleration).ok()
    }
    pub unsafe fn InitialTimestamp(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).InitialTimestamp)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInitialTimestamp(&self, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialTimestamp)(::windows::core::Interface::as_raw(self), timestamp).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Process(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).Process)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProcessTime(&self, timestamp: u32) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).ProcessTime)(::windows::core::Interface::as_raw(self), timestamp, &mut result__).from_abi(result__)
    }
    pub unsafe fn Complete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Complete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CompleteTime(&self, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CompleteTime)(::windows::core::Interface::as_raw(self), timestamp).ok()
    }
}
::windows::imp::interface_hierarchy!(IInertiaProcessor, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IInertiaProcessor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInertiaProcessor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18b00c6d_c5ee_41b1_90a9_9d4a929095ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaProcessor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
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
    pub unsafe fn SupportedManipulations(&self) -> ::windows::core::Result<MANIPULATION_PROCESSOR_MANIPULATIONS> {
        let mut result__ = ::windows::core::zeroed::<MANIPULATION_PROCESSOR_MANIPULATIONS>();
        (::windows::core::Interface::vtable(self).SupportedManipulations)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSupportedManipulations(&self, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSupportedManipulations)(::windows::core::Interface::as_raw(self), manipulations).ok()
    }
    pub unsafe fn PivotPointX(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).PivotPointX)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPivotPointX(&self, pivotpointx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPivotPointX)(::windows::core::Interface::as_raw(self), pivotpointx).ok()
    }
    pub unsafe fn PivotPointY(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).PivotPointY)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPivotPointY(&self, pivotpointy: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPivotPointY)(::windows::core::Interface::as_raw(self), pivotpointy).ok()
    }
    pub unsafe fn PivotRadius(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).PivotRadius)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPivotRadius(&self, pivotradius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPivotRadius)(::windows::core::Interface::as_raw(self), pivotradius).ok()
    }
    pub unsafe fn CompleteManipulation(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CompleteManipulation)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ProcessDown(&self, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessDown)(::windows::core::Interface::as_raw(self), manipulatorid, x, y).ok()
    }
    pub unsafe fn ProcessMove(&self, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessMove)(::windows::core::Interface::as_raw(self), manipulatorid, x, y).ok()
    }
    pub unsafe fn ProcessUp(&self, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessUp)(::windows::core::Interface::as_raw(self), manipulatorid, x, y).ok()
    }
    pub unsafe fn ProcessDownWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessDownWithTime)(::windows::core::Interface::as_raw(self), manipulatorid, x, y, timestamp).ok()
    }
    pub unsafe fn ProcessMoveWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessMoveWithTime)(::windows::core::Interface::as_raw(self), manipulatorid, x, y, timestamp).ok()
    }
    pub unsafe fn ProcessUpWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessUpWithTime)(::windows::core::Interface::as_raw(self), manipulatorid, x, y, timestamp).ok()
    }
    pub unsafe fn GetVelocityX(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).GetVelocityX)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetVelocityY(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).GetVelocityY)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetExpansionVelocity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).GetExpansionVelocity)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAngularVelocity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).GetAngularVelocity)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MinimumScaleRotateRadius(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).MinimumScaleRotateRadius)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMinimumScaleRotateRadius(&self, minradius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMinimumScaleRotateRadius)(::windows::core::Interface::as_raw(self), minradius).ok()
    }
}
::windows::imp::interface_hierarchy!(IManipulationProcessor, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IManipulationProcessor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IManipulationProcessor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa22ac519_8300_48a0_bef4_f1be8737dba4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationProcessor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
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
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
pub struct _IManipulationEvents(::windows::core::IUnknown);
impl _IManipulationEvents {
    pub unsafe fn ManipulationStarted(&self, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ManipulationStarted)(::windows::core::Interface::as_raw(self), x, y).ok()
    }
    pub unsafe fn ManipulationDelta(&self, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ManipulationDelta)(::windows::core::Interface::as_raw(self), x, y, translationdeltax, translationdeltay, scaledelta, expansiondelta, rotationdelta, cumulativetranslationx, cumulativetranslationy, cumulativescale, cumulativeexpansion, cumulativerotation).ok()
    }
    pub unsafe fn ManipulationCompleted(&self, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ManipulationCompleted)(::windows::core::Interface::as_raw(self), x, y, cumulativetranslationx, cumulativetranslationy, cumulativescale, cumulativeexpansion, cumulativerotation).ok()
    }
}
::windows::imp::interface_hierarchy!(_IManipulationEvents, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for _IManipulationEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for _IManipulationEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f62c8da_9c53_4b22_93df_927a862bbb03);
}
#[repr(C)]
#[doc(hidden)]
pub struct _IManipulationEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ManipulationStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32) -> ::windows::core::HRESULT,
    pub ManipulationDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::HRESULT,
    pub ManipulationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const InertiaProcessor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabb27087_4ce0_4e58_a0cb_e24df96814be);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const ManipulationProcessor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x597d4fb0_47fd_4aff_89b9_c6cfae8cf08e);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for GESTURECONFIG_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GESTURECONFIG_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GESTURECONFIG_ID").field(&self.0).finish()
    }
}
impl GESTURECONFIG_ID {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for MANIPULATION_PROCESSOR_MANIPULATIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MANIPULATION_PROCESSOR_MANIPULATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MANIPULATION_PROCESSOR_MANIPULATIONS").field(&self.0).finish()
    }
}
impl MANIPULATION_PROCESSOR_MANIPULATIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for MANIPULATION_PROCESSOR_MANIPULATIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MANIPULATION_PROCESSOR_MANIPULATIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MANIPULATION_PROCESSOR_MANIPULATIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MANIPULATION_PROCESSOR_MANIPULATIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MANIPULATION_PROCESSOR_MANIPULATIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for REGISTER_TOUCH_WINDOW_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for REGISTER_TOUCH_WINDOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REGISTER_TOUCH_WINDOW_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for TOUCHEVENTF_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TOUCHEVENTF_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOUCHEVENTF_FLAGS").field(&self.0).finish()
    }
}
impl TOUCHEVENTF_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for TOUCHINPUTMASKF_MASK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TOUCHINPUTMASKF_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOUCHINPUTMASKF_MASK").field(&self.0).finish()
    }
}
impl TOUCHINPUTMASKF_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
impl ::windows::core::TypeKind for GESTURECONFIG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GESTURECONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.dwID == other.dwID && self.dwWant == other.dwWant && self.dwBlock == other.dwBlock
    }
}
impl ::core::cmp::Eq for GESTURECONFIG {}
impl ::core::default::Default for GESTURECONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for GESTUREINFO {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for GESTURENOTIFYSTRUCT {
    type TypeKind = ::windows::core::CopyType;
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
impl ::core::default::Default for GESTURENOTIFYSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HGESTUREINFO(pub isize);
impl HGESTUREINFO {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
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
impl ::windows::core::TypeKind for HGESTUREINFO {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HTOUCHINPUT(pub isize);
impl HTOUCHINPUT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
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
impl ::windows::core::TypeKind for HTOUCHINPUT {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for TOUCHINPUT {
    type TypeKind = ::windows::core::CopyType;
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
impl ::core::default::Default for TOUCHINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
