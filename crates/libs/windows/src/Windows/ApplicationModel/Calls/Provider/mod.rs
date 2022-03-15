#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallOrigin(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallOrigin {
    type Vtable = IPhoneCallOrigin_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20613479_0ef9_4454_871c_afb66a14b6a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallOrigin_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CategoryDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCategoryDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallOrigin2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallOrigin2 {
    type Vtable = IPhoneCallOrigin2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04c7e980_9ac2_4768_b536_b68da4957d02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallOrigin2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallOrigin3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallOrigin3 {
    type Vtable = IPhoneCallOrigin3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49330fb4_d1a7_43a2_aeee_c07b6dbaf068);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallOrigin3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub DisplayPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    DisplayPicture: usize,
    #[cfg(feature = "Storage")]
    pub SetDisplayPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetDisplayPicture: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallOriginManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallOriginManagerStatics {
    type Vtable = IPhoneCallOriginManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccfc5a0a_9af7_6149_39d0_e076fcce1395);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallOriginManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsCurrentAppActiveCallOriginApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ShowPhoneCallOriginSettingsUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCallOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, callorigin: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallOriginManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallOriginManagerStatics2 {
    type Vtable = IPhoneCallOriginManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bf3ee3f_40f4_4380_8c7c_aea2c9b8dd7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallOriginManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestSetAsActiveCallOriginAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetAsActiveCallOriginAppAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallOriginManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallOriginManagerStatics3 {
    type Vtable = IPhoneCallOriginManagerStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ed69764_a6e3_50f0_b76a_d67cb39bdfde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallOriginManagerStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Calls_Provider\"`*"]
#[repr(transparent)]
pub struct PhoneCallOrigin(::windows::core::IUnknown);
impl PhoneCallOrigin {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallOrigin, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Provider\"`*"]
    pub fn Category(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Category)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Provider\"`*"]
    pub fn SetCategory<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCategory)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Provider\"`*"]
    pub fn CategoryDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CategoryDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Provider\"`*"]
    pub fn SetCategoryDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCategoryDescription)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Provider\"`*"]
    pub fn Location(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Provider\"`*"]
    pub fn SetLocation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLocation)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Provider\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPhoneCallOrigin2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Provider\"`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPhoneCallOrigin2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Provider\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn DisplayPicture(&self) -> ::windows::core::Result<super::super::super::Storage::StorageFile> {
        let this = &::windows::core::Interface::cast::<IPhoneCallOrigin3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayPicture)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::StorageFile>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Provider\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn SetDisplayPicture<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::StorageFile>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPhoneCallOrigin3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayPicture)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for PhoneCallOrigin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallOrigin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallOrigin {}
impl ::core::fmt::Debug for PhoneCallOrigin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallOrigin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallOrigin {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Provider.PhoneCallOrigin;{20613479-0ef9-4454-871c-afb66a14b6a5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneCallOrigin {
    type Vtable = IPhoneCallOrigin_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneCallOrigin as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneCallOrigin {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Provider.PhoneCallOrigin";
}
impl ::core::convert::From<PhoneCallOrigin> for ::windows::core::IUnknown {
    fn from(value: PhoneCallOrigin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallOrigin> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallOrigin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallOrigin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallOrigin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallOrigin> for ::windows::core::IInspectable {
    fn from(value: PhoneCallOrigin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallOrigin> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallOrigin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallOrigin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallOrigin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneCallOrigin {}
unsafe impl ::core::marker::Sync for PhoneCallOrigin {}
#[doc = "*Required features: `\"ApplicationModel_Calls_Provider\"`*"]
pub struct PhoneCallOriginManager {}
impl PhoneCallOriginManager {
    #[doc = "*Required features: `\"ApplicationModel_Calls_Provider\"`*"]
    pub fn IsCurrentAppActiveCallOriginApp() -> ::windows::core::Result<bool> {
        Self::IPhoneCallOriginManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCurrentAppActiveCallOriginApp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Provider\"`*"]
    pub fn ShowPhoneCallOriginSettingsUI() -> ::windows::core::Result<()> {
        Self::IPhoneCallOriginManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ShowPhoneCallOriginSettingsUI)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Provider\"`*"]
    pub fn SetCallOrigin<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, PhoneCallOrigin>>(requestid: Param0, callorigin: Param1) -> ::windows::core::Result<()> {
        Self::IPhoneCallOriginManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetCallOrigin)(::core::mem::transmute_copy(this), requestid.into_param().abi(), callorigin.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSetAsActiveCallOriginAppAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        Self::IPhoneCallOriginManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestSetAsActiveCallOriginAppAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Provider\"`*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IPhoneCallOriginManagerStatics3(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallOriginManagerStatics<R, F: FnOnce(&IPhoneCallOriginManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallOriginManager, IPhoneCallOriginManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IPhoneCallOriginManagerStatics2<R, F: FnOnce(&IPhoneCallOriginManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallOriginManager, IPhoneCallOriginManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IPhoneCallOriginManagerStatics3<R, F: FnOnce(&IPhoneCallOriginManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallOriginManager, IPhoneCallOriginManagerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PhoneCallOriginManager {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Provider.PhoneCallOriginManager";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
