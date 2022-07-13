#[doc = "*Required features: `\"Phone_UI_Input\"`*"]
#[repr(transparent)]
pub struct BackPressedEventArgs(::windows::core::IUnknown);
impl BackPressedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for BackPressedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for BackPressedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.UI.Input.BackPressedEventArgs;{f6f555ff-64ec-42a2-b93b-2fbc0c36a121})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BackPressedEventArgs {
    type Vtable = IBackPressedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IBackPressedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BackPressedEventArgs {
    const NAME: &'static str = "Windows.Phone.UI.Input.BackPressedEventArgs";
}
impl ::core::convert::From<BackPressedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BackPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackPressedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BackPressedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&BackPressedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &BackPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<BackPressedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BackPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackPressedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BackPressedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&BackPressedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &BackPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for BackPressedEventArgs {}
unsafe impl ::core::marker::Sync for BackPressedEventArgs {}
#[doc = "*Required features: `\"Phone_UI_Input\"`*"]
#[repr(transparent)]
pub struct CameraEventArgs(::windows::core::IUnknown);
impl CameraEventArgs {}
impl ::core::clone::Clone for CameraEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for CameraEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.UI.Input.CameraEventArgs;{b4063bda-201f-473d-bc69-e9e4ac57c9d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CameraEventArgs {
    type Vtable = ICameraEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICameraEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CameraEventArgs {
    const NAME: &'static str = "Windows.Phone.UI.Input.CameraEventArgs";
}
impl ::core::convert::From<CameraEventArgs> for ::windows::core::IUnknown {
    fn from(value: CameraEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CameraEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CameraEventArgs> for &::windows::core::IUnknown {
    fn from(value: &CameraEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CameraEventArgs> for ::windows::core::IInspectable {
    fn from(value: CameraEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CameraEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CameraEventArgs> for &::windows::core::IInspectable {
    fn from(value: &CameraEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CameraEventArgs {}
unsafe impl ::core::marker::Sync for CameraEventArgs {}
#[doc = "*Required features: `\"Phone_UI_Input\"`*"]
pub struct HardwareButtons;
impl HardwareButtons {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BackPressed<'a, P0>(handler: P0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::EventHandler<BackPressedEventArgs>>>,
    {
        Self::IHardwareButtonsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BackPressed)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBackPressed(token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IHardwareButtonsStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveBackPressed)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CameraHalfPressed<'a, P0>(handler: P0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::EventHandler<CameraEventArgs>>>,
    {
        Self::IHardwareButtonsStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CameraHalfPressed)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraHalfPressed(token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IHardwareButtonsStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveCameraHalfPressed)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CameraPressed<'a, P0>(handler: P0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::EventHandler<CameraEventArgs>>>,
    {
        Self::IHardwareButtonsStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CameraPressed)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraPressed(token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IHardwareButtonsStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveCameraPressed)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CameraReleased<'a, P0>(handler: P0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::EventHandler<CameraEventArgs>>>,
    {
        Self::IHardwareButtonsStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CameraReleased)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraReleased(token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IHardwareButtonsStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveCameraReleased)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IHardwareButtonsStatics<R, F: FnOnce(&IHardwareButtonsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HardwareButtons, IHardwareButtonsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHardwareButtonsStatics2<R, F: FnOnce(&IHardwareButtonsStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HardwareButtons, IHardwareButtonsStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for HardwareButtons {
    const NAME: &'static str = "Windows.Phone.UI.Input.HardwareButtons";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackPressedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackPressedEventArgs {
    type Vtable = IBackPressedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6f555ff_64ec_42a2_b93b_2fbc0c36a121);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackPressedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICameraEventArgs {
    type Vtable = ICameraEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4063bda_201f_473d_bc69_e9e4ac57c9d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHardwareButtonsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHardwareButtonsStatics {
    type Vtable = IHardwareButtonsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x594b8780_da66_4fd8_a776_7506bd0cbfa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHardwareButtonsStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub BackPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BackPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBackPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBackPressed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHardwareButtonsStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHardwareButtonsStatics2 {
    type Vtable = IHardwareButtonsStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39c6c274_993f_40dd_854c_831a8934b92e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHardwareButtonsStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CameraHalfPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraHalfPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraHalfPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraHalfPressed: usize,
    #[cfg(feature = "Foundation")]
    pub CameraPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraPressed: usize,
    #[cfg(feature = "Foundation")]
    pub CameraReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraReleased: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
