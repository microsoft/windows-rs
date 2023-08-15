#[doc(hidden)]
#[repr(transparent)]
pub struct IOemSupportInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOemSupportInfo {
    type Vtable = IOemSupportInfo_Vtbl;
}
impl ::core::clone::Clone for IOemSupportInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IOemSupportInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d2eae55_87ef_4266_86d0_c4afbeb29bb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOemSupportInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SupportLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SupportLink: usize,
    #[cfg(feature = "Foundation")]
    pub SupportAppLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SupportAppLink: usize,
    pub SupportProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmbiosInformationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmbiosInformationStatics {
    type Vtable = ISmbiosInformationStatics_Vtbl;
}
impl ::core::clone::Clone for ISmbiosInformationStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmbiosInformationStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x080cca7c_637c_48c4_b728_f9273812db8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmbiosInformationStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemSupportDeviceInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemSupportDeviceInfo {
    type Vtable = ISystemSupportDeviceInfo_Vtbl;
}
impl ::core::clone::Clone for ISystemSupportDeviceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISystemSupportDeviceInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05880b99_8247_441b_a996_a1784bab79a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemSupportDeviceInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OperatingSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SystemManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SystemProductName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SystemSku: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SystemHardwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SystemFirmwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemSupportInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemSupportInfoStatics {
    type Vtable = ISystemSupportInfoStatics_Vtbl;
}
impl ::core::clone::Clone for ISystemSupportInfoStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISystemSupportInfoStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef750974_c422_45d7_a44d_5c1c0043a2b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemSupportInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LocalSystemEdition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OemSupportInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemSupportInfoStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemSupportInfoStatics2 {
    type Vtable = ISystemSupportInfoStatics2_Vtbl;
}
impl ::core::clone::Clone for ISystemSupportInfoStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISystemSupportInfoStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33f349a4_3fa1_4986_aa4b_057420455e6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemSupportInfoStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LocalDeviceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"System_Profile_SystemManufacturers\"`*"]
#[repr(transparent)]
pub struct OemSupportInfo(::windows_core::IUnknown);
impl OemSupportInfo {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SupportLink(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportLink)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SupportAppLink(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportAppLink)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SupportProvider(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportProvider)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for OemSupportInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.System.Profile.SystemManufacturers.OemSupportInfo;{8d2eae55-87ef-4266-86d0-c4afbeb29bb9})");
}
impl ::core::clone::Clone for OemSupportInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for OemSupportInfo {
    type Vtable = IOemSupportInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for OemSupportInfo {
    const IID: ::windows_core::GUID = <IOemSupportInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for OemSupportInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemManufacturers.OemSupportInfo";
}
::windows_core::imp::interface_hierarchy!(OemSupportInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for OemSupportInfo {}
unsafe impl ::core::marker::Sync for OemSupportInfo {}
#[doc = "*Required features: `\"System_Profile_SystemManufacturers\"`*"]
pub struct SmbiosInformation;
impl SmbiosInformation {
    pub fn SerialNumber() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISmbiosInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SerialNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmbiosInformationStatics<R, F: FnOnce(&ISmbiosInformationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmbiosInformation, ISmbiosInformationStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for SmbiosInformation {
    const NAME: &'static str = "Windows.System.Profile.SystemManufacturers.SmbiosInformation";
}
#[doc = "*Required features: `\"System_Profile_SystemManufacturers\"`*"]
#[repr(transparent)]
pub struct SystemSupportDeviceInfo(::windows_core::IUnknown);
impl SystemSupportDeviceInfo {
    pub fn OperatingSystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OperatingSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FriendlyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SystemManufacturer(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemManufacturer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SystemProductName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemProductName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SystemSku(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemSku)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SystemHardwareVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemHardwareVersion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SystemFirmwareVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemFirmwareVersion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for SystemSupportDeviceInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.System.Profile.SystemManufacturers.SystemSupportDeviceInfo;{05880b99-8247-441b-a996-a1784bab79a8})");
}
impl ::core::clone::Clone for SystemSupportDeviceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SystemSupportDeviceInfo {
    type Vtable = ISystemSupportDeviceInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SystemSupportDeviceInfo {
    const IID: ::windows_core::GUID = <ISystemSupportDeviceInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SystemSupportDeviceInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemManufacturers.SystemSupportDeviceInfo";
}
::windows_core::imp::interface_hierarchy!(SystemSupportDeviceInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SystemSupportDeviceInfo {}
unsafe impl ::core::marker::Sync for SystemSupportDeviceInfo {}
#[doc = "*Required features: `\"System_Profile_SystemManufacturers\"`*"]
pub struct SystemSupportInfo;
impl SystemSupportInfo {
    pub fn LocalSystemEdition() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISystemSupportInfoStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalSystemEdition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn OemSupportInfo() -> ::windows_core::Result<OemSupportInfo> {
        Self::ISystemSupportInfoStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OemSupportInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn LocalDeviceInfo() -> ::windows_core::Result<SystemSupportDeviceInfo> {
        Self::ISystemSupportInfoStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalDeviceInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISystemSupportInfoStatics<R, F: FnOnce(&ISystemSupportInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SystemSupportInfo, ISystemSupportInfoStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISystemSupportInfoStatics2<R, F: FnOnce(&ISystemSupportInfoStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SystemSupportInfo, ISystemSupportInfoStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for SystemSupportInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemManufacturers.SystemSupportInfo";
}
