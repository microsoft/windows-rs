#[doc(hidden)]
#[repr(transparent)]
pub struct IBackPressedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackPressedEventArgs {
    type Vtable = IBackPressedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IBackPressedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBackPressedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6f555ff_64ec_42a2_b93b_2fbc0c36a121);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackPressedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICameraEventArgs {
    type Vtable = ICameraEventArgs_Vtbl;
}
impl ::core::clone::Clone for ICameraEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICameraEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb4063bda_201f_473d_bc69_e9e4ac57c9d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHardwareButtonsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHardwareButtonsStatics {
    type Vtable = IHardwareButtonsStatics_Vtbl;
}
impl ::core::clone::Clone for IHardwareButtonsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IHardwareButtonsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x594b8780_da66_4fd8_a776_7506bd0cbfa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHardwareButtonsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub BackPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BackPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBackPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBackPressed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHardwareButtonsStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHardwareButtonsStatics2 {
    type Vtable = IHardwareButtonsStatics2_Vtbl;
}
impl ::core::clone::Clone for IHardwareButtonsStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IHardwareButtonsStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39c6c274_993f_40dd_854c_831a8934b92e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHardwareButtonsStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CameraHalfPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraHalfPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraHalfPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraHalfPressed: usize,
    #[cfg(feature = "Foundation")]
    pub CameraPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraPressed: usize,
    #[cfg(feature = "Foundation")]
    pub CameraReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraReleased: usize,
}
#[doc = "*Required features: `\"Phone_UI_Input\"`*"]
#[repr(transparent)]
pub struct BackPressedEventArgs(::windows_core::IUnknown);
impl BackPressedEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for BackPressedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackPressedEventArgs {}
impl ::core::fmt::Debug for BackPressedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackPressedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BackPressedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.UI.Input.BackPressedEventArgs;{f6f555ff-64ec-42a2-b93b-2fbc0c36a121})");
}
impl ::core::clone::Clone for BackPressedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for BackPressedEventArgs {
    type Vtable = IBackPressedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BackPressedEventArgs {
    const IID: ::windows_core::GUID = <IBackPressedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BackPressedEventArgs {
    const NAME: &'static str = "Windows.Phone.UI.Input.BackPressedEventArgs";
}
::windows_core::imp::interface_hierarchy!(BackPressedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BackPressedEventArgs {}
unsafe impl ::core::marker::Sync for BackPressedEventArgs {}
#[doc = "*Required features: `\"Phone_UI_Input\"`*"]
#[repr(transparent)]
pub struct CameraEventArgs(::windows_core::IUnknown);
impl CameraEventArgs {}
impl ::core::cmp::PartialEq for CameraEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraEventArgs {}
impl ::core::fmt::Debug for CameraEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CameraEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.UI.Input.CameraEventArgs;{b4063bda-201f-473d-bc69-e9e4ac57c9d0})");
}
impl ::core::clone::Clone for CameraEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CameraEventArgs {
    type Vtable = ICameraEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CameraEventArgs {
    const IID: ::windows_core::GUID = <ICameraEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CameraEventArgs {
    const NAME: &'static str = "Windows.Phone.UI.Input.CameraEventArgs";
}
::windows_core::imp::interface_hierarchy!(CameraEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CameraEventArgs {}
unsafe impl ::core::marker::Sync for CameraEventArgs {}
#[doc = "*Required features: `\"Phone_UI_Input\"`*"]
pub struct HardwareButtons;
impl HardwareButtons {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BackPressed<P0>(handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::EventHandler<BackPressedEventArgs>>,
    {
        Self::IHardwareButtonsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BackPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBackPressed(token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IHardwareButtonsStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveBackPressed)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CameraHalfPressed<P0>(handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::EventHandler<CameraEventArgs>>,
    {
        Self::IHardwareButtonsStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CameraHalfPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraHalfPressed(token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IHardwareButtonsStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveCameraHalfPressed)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CameraPressed<P0>(handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::EventHandler<CameraEventArgs>>,
    {
        Self::IHardwareButtonsStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CameraPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraPressed(token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IHardwareButtonsStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveCameraPressed)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CameraReleased<P0>(handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::EventHandler<CameraEventArgs>>,
    {
        Self::IHardwareButtonsStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CameraReleased)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraReleased(token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IHardwareButtonsStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveCameraReleased)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IHardwareButtonsStatics<R, F: FnOnce(&IHardwareButtonsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<HardwareButtons, IHardwareButtonsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHardwareButtonsStatics2<R, F: FnOnce(&IHardwareButtonsStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<HardwareButtons, IHardwareButtonsStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for HardwareButtons {
    const NAME: &'static str = "Windows.Phone.UI.Input.HardwareButtons";
}
