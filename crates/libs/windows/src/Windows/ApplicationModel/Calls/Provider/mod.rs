#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneCallOrigin(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPhoneCallOrigin {
    type Vtable = IPhoneCallOrigin_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPhoneCallOrigin {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20613479_0ef9_4454_871c_afb66a14b6a5);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallOrigin_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Category: usize,
    #[cfg(feature = "deprecated")]
    pub SetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetCategory: usize,
    #[cfg(feature = "deprecated")]
    pub CategoryDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CategoryDescription: usize,
    #[cfg(feature = "deprecated")]
    pub SetCategoryDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetCategoryDescription: usize,
    #[cfg(feature = "deprecated")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Location: usize,
    #[cfg(feature = "deprecated")]
    pub SetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetLocation: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneCallOrigin2(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPhoneCallOrigin2 {
    type Vtable = IPhoneCallOrigin2_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPhoneCallOrigin2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04c7e980_9ac2_4768_b536_b68da4957d02);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallOrigin2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayName: usize,
    #[cfg(feature = "deprecated")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDisplayName: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneCallOrigin3(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPhoneCallOrigin3 {
    type Vtable = IPhoneCallOrigin3_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPhoneCallOrigin3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49330fb4_d1a7_43a2_aeee_c07b6dbaf068);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallOrigin3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub DisplayPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    DisplayPicture: usize,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub SetDisplayPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    SetDisplayPicture: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneCallOriginManagerStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPhoneCallOriginManagerStatics {
    type Vtable = IPhoneCallOriginManagerStatics_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPhoneCallOriginManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xccfc5a0a_9af7_6149_39d0_e076fcce1395);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallOriginManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub IsCurrentAppActiveCallOriginApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsCurrentAppActiveCallOriginApp: usize,
    #[cfg(feature = "deprecated")]
    pub ShowPhoneCallOriginSettingsUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowPhoneCallOriginSettingsUI: usize,
    #[cfg(feature = "deprecated")]
    pub SetCallOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: ::windows_core::GUID, callorigin: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetCallOrigin: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneCallOriginManagerStatics2(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPhoneCallOriginManagerStatics2 {
    type Vtable = IPhoneCallOriginManagerStatics2_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPhoneCallOriginManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8bf3ee3f_40f4_4380_8c7c_aea2c9b8dd7a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallOriginManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RequestSetAsActiveCallOriginAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RequestSetAsActiveCallOriginAppAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneCallOriginManagerStatics3(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPhoneCallOriginManagerStatics3 {
    type Vtable = IPhoneCallOriginManagerStatics3_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPhoneCallOriginManagerStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ed69764_a6e3_50f0_b76a_d67cb39bdfde);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallOriginManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsSupported: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PhoneCallOrigin(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PhoneCallOrigin {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneCallOrigin, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Category(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Category)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn SetCategory(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCategory)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn CategoryDescription(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CategoryDescription)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn SetCategoryDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCategoryDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Location(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Location)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn SetLocation(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLocation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPhoneCallOrigin2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPhoneCallOrigin2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Storage\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn DisplayPicture(&self) -> ::windows_core::Result<super::super::super::Storage::StorageFile> {
        let this = &::windows_core::ComInterface::cast::<IPhoneCallOrigin3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayPicture)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn SetDisplayPicture<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Storage::StorageFile>,
    {
        let this = &::windows_core::ComInterface::cast::<IPhoneCallOrigin3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayPicture)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for PhoneCallOrigin {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Provider.PhoneCallOrigin;{20613479-0ef9-4454-871c-afb66a14b6a5})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PhoneCallOrigin {
    type Vtable = IPhoneCallOrigin_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for PhoneCallOrigin {
    const IID: ::windows_core::GUID = <IPhoneCallOrigin as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PhoneCallOrigin {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Provider.PhoneCallOrigin";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(PhoneCallOrigin, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PhoneCallOrigin {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PhoneCallOrigin {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
pub struct PhoneCallOriginManager;
#[cfg(feature = "deprecated")]
impl PhoneCallOriginManager {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn IsCurrentAppActiveCallOriginApp() -> ::windows_core::Result<bool> {
        Self::IPhoneCallOriginManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCurrentAppActiveCallOriginApp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ShowPhoneCallOriginSettingsUI() -> ::windows_core::Result<()> {
        Self::IPhoneCallOriginManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ShowPhoneCallOriginSettingsUI)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn SetCallOrigin<P0>(requestid: ::windows_core::GUID, callorigin: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PhoneCallOrigin>,
    {
        Self::IPhoneCallOriginManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetCallOrigin)(::windows_core::Interface::as_raw(this), requestid, callorigin.into_param().abi()).ok() })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RequestSetAsActiveCallOriginAppAsync() -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        Self::IPhoneCallOriginManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestSetAsActiveCallOriginAppAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IPhoneCallOriginManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IPhoneCallOriginManagerStatics<R, F: FnOnce(&IPhoneCallOriginManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneCallOriginManager, IPhoneCallOriginManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IPhoneCallOriginManagerStatics2<R, F: FnOnce(&IPhoneCallOriginManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneCallOriginManager, IPhoneCallOriginManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IPhoneCallOriginManagerStatics3<R, F: FnOnce(&IPhoneCallOriginManagerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneCallOriginManager, IPhoneCallOriginManagerStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PhoneCallOriginManager {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Provider.PhoneCallOriginManager";
}
