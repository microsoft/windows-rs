#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Phone_UI_Input\"`*"]
#[repr(transparent)]
pub struct BackPressedEventArgs(::windows::core::IUnknown);
impl BackPressedEventArgs {
    #[doc = "*Required features: `\"Phone_UI_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_UI_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BackPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BackPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CameraEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CameraEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CameraEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CameraEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CameraEventArgs {}
unsafe impl ::core::marker::Sync for CameraEventArgs {}
#[doc = "*Required features: `\"Phone_UI_Input\"`*"]
pub struct HardwareButtons {}
impl HardwareButtons {
    #[doc = "*Required features: `\"Phone_UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BackPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<BackPressedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IHardwareButtonsStatics(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BackPressed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBackPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IHardwareButtonsStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveBackPressed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CameraHalfPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<CameraEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IHardwareButtonsStatics2(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CameraHalfPressed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraHalfPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IHardwareButtonsStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveCameraHalfPressed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CameraPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<CameraEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IHardwareButtonsStatics2(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CameraPressed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IHardwareButtonsStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveCameraPressed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CameraReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<CameraEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IHardwareButtonsStatics2(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CameraReleased)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IHardwareButtonsStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveCameraReleased)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IHardwareButtonsStatics<R, F: FnOnce(&IHardwareButtonsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HardwareButtons, IHardwareButtonsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHardwareButtonsStatics2<R, F: FnOnce(&IHardwareButtonsStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HardwareButtons, IHardwareButtonsStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
    pub base: ::windows::core::IInspectableVtbl,
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
    pub base: ::windows::core::IInspectableVtbl,
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
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub BackPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CameraHalfPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraHalfPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraHalfPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraHalfPressed: usize,
    #[cfg(feature = "Foundation")]
    pub CameraPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraPressed: usize,
    #[cfg(feature = "Foundation")]
    pub CameraReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraReleased: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
