#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceServicingDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceServicingDetails {
    type Vtable = IDeviceServicingDetails_Vtbl;
}
impl ::core::clone::Clone for IDeviceServicingDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceServicingDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4aabee29_2344_4ac4_8527_4a8ef6905645);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceServicingDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpectedDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpectedDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceUseDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceUseDetails {
    type Vtable = IDeviceUseDetails_Vtbl;
}
impl ::core::clone::Clone for IDeviceUseDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceUseDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d565141_557e_4154_b994_e4f7a11fb323);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceUseDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Background\"`*"]
#[repr(transparent)]
pub struct DeviceServicingDetails(::windows_core::IUnknown);
impl DeviceServicingDetails {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpectedDuration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExpectedDuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DeviceServicingDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceServicingDetails {}
impl ::core::fmt::Debug for DeviceServicingDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceServicingDetails").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceServicingDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Background.DeviceServicingDetails;{4aabee29-2344-4ac4-8527-4a8ef6905645})");
}
impl ::core::clone::Clone for DeviceServicingDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DeviceServicingDetails {
    type Vtable = IDeviceServicingDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceServicingDetails {
    const IID: ::windows_core::GUID = <IDeviceServicingDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceServicingDetails {
    const NAME: &'static str = "Windows.Devices.Background.DeviceServicingDetails";
}
::windows_core::imp::interface_hierarchy!(DeviceServicingDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DeviceServicingDetails {}
unsafe impl ::core::marker::Sync for DeviceServicingDetails {}
#[doc = "*Required features: `\"Devices_Background\"`*"]
#[repr(transparent)]
pub struct DeviceUseDetails(::windows_core::IUnknown);
impl DeviceUseDetails {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DeviceUseDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceUseDetails {}
impl ::core::fmt::Debug for DeviceUseDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceUseDetails").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceUseDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Background.DeviceUseDetails;{7d565141-557e-4154-b994-e4f7a11fb323})");
}
impl ::core::clone::Clone for DeviceUseDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DeviceUseDetails {
    type Vtable = IDeviceUseDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceUseDetails {
    const IID: ::windows_core::GUID = <IDeviceUseDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceUseDetails {
    const NAME: &'static str = "Windows.Devices.Background.DeviceUseDetails";
}
::windows_core::imp::interface_hierarchy!(DeviceUseDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DeviceUseDetails {}
unsafe impl ::core::marker::Sync for DeviceUseDetails {}
