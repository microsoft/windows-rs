#[doc(hidden)]
#[repr(transparent)]
pub struct IOemSupportInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IOemSupportInfo {
    type Vtable = IOemSupportInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IOemSupportInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d2eae55_87ef_4266_86d0_c4afbeb29bb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOemSupportInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SupportLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SupportLink: usize,
    #[cfg(feature = "Foundation")]
    pub SupportAppLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SupportAppLink: usize,
    pub SupportProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmbiosInformationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISmbiosInformationStatics {
    type Vtable = ISmbiosInformationStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISmbiosInformationStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x080cca7c_637c_48c4_b728_f9273812db8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmbiosInformationStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemSupportDeviceInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISystemSupportDeviceInfo {
    type Vtable = ISystemSupportDeviceInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ISystemSupportDeviceInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05880b99_8247_441b_a996_a1784bab79a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemSupportDeviceInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub OperatingSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SystemManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SystemProductName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SystemSku: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SystemHardwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SystemFirmwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemSupportInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISystemSupportInfoStatics {
    type Vtable = ISystemSupportInfoStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISystemSupportInfoStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef750974_c422_45d7_a44d_5c1c0043a2b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemSupportInfoStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LocalSystemEdition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OemSupportInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemSupportInfoStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISystemSupportInfoStatics2 {
    type Vtable = ISystemSupportInfoStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for ISystemSupportInfoStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33f349a4_3fa1_4986_aa4b_057420455e6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemSupportInfoStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LocalDeviceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"System_Profile_SystemManufacturers\"`*"]
#[repr(transparent)]
pub struct OemSupportInfo(::windows::core::IUnknown);
impl OemSupportInfo {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SupportLink(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportLink)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SupportAppLink(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportAppLink)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SupportProvider(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportProvider)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for OemSupportInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OemSupportInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OemSupportInfo {}
impl ::core::fmt::Debug for OemSupportInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OemSupportInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for OemSupportInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Profile.SystemManufacturers.OemSupportInfo;{8d2eae55-87ef-4266-86d0-c4afbeb29bb9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for OemSupportInfo {
    type Vtable = IOemSupportInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for OemSupportInfo {
    const IID: ::windows::core::GUID = <IOemSupportInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for OemSupportInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemManufacturers.OemSupportInfo";
}
::windows::core::interface_hierarchy!(OemSupportInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for OemSupportInfo {}
unsafe impl ::core::marker::Sync for OemSupportInfo {}
#[doc = "*Required features: `\"System_Profile_SystemManufacturers\"`*"]
pub struct SmbiosInformation;
impl SmbiosInformation {
    pub fn SerialNumber() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISmbiosInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SerialNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmbiosInformationStatics<R, F: FnOnce(&ISmbiosInformationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SmbiosInformation, ISmbiosInformationStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for SmbiosInformation {
    const NAME: &'static str = "Windows.System.Profile.SystemManufacturers.SmbiosInformation";
}
#[doc = "*Required features: `\"System_Profile_SystemManufacturers\"`*"]
#[repr(transparent)]
pub struct SystemSupportDeviceInfo(::windows::core::IUnknown);
impl SystemSupportDeviceInfo {
    pub fn OperatingSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OperatingSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FriendlyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SystemManufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemManufacturer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SystemProductName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemProductName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SystemSku(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemSku)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SystemHardwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemHardwareVersion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SystemFirmwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemFirmwareVersion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for SystemSupportDeviceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemSupportDeviceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemSupportDeviceInfo {}
impl ::core::fmt::Debug for SystemSupportDeviceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemSupportDeviceInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemSupportDeviceInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Profile.SystemManufacturers.SystemSupportDeviceInfo;{05880b99-8247-441b-a996-a1784bab79a8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SystemSupportDeviceInfo {
    type Vtable = ISystemSupportDeviceInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for SystemSupportDeviceInfo {
    const IID: ::windows::core::GUID = <ISystemSupportDeviceInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemSupportDeviceInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemManufacturers.SystemSupportDeviceInfo";
}
::windows::core::interface_hierarchy!(SystemSupportDeviceInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SystemSupportDeviceInfo {}
unsafe impl ::core::marker::Sync for SystemSupportDeviceInfo {}
#[doc = "*Required features: `\"System_Profile_SystemManufacturers\"`*"]
pub struct SystemSupportInfo;
impl SystemSupportInfo {
    pub fn LocalSystemEdition() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISystemSupportInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalSystemEdition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn OemSupportInfo() -> ::windows::core::Result<OemSupportInfo> {
        Self::ISystemSupportInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OemSupportInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn LocalDeviceInfo() -> ::windows::core::Result<SystemSupportDeviceInfo> {
        Self::ISystemSupportInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalDeviceInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISystemSupportInfoStatics<R, F: FnOnce(&ISystemSupportInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SystemSupportInfo, ISystemSupportInfoStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISystemSupportInfoStatics2<R, F: FnOnce(&ISystemSupportInfoStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SystemSupportInfo, ISystemSupportInfoStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for SystemSupportInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemManufacturers.SystemSupportInfo";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
