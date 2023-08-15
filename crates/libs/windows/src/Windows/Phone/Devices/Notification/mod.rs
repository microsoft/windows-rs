#[doc(hidden)]
#[repr(transparent)]
pub struct IVibrationDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVibrationDevice {
    type Vtable = IVibrationDevice_Vtbl;
}
impl ::core::clone::Clone for IVibrationDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVibrationDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b4a6595_cfcd_4e08_92fb_c1906d04498c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVibrationDevice_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Vibrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Vibrate: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVibrationDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVibrationDeviceStatics {
    type Vtable = IVibrationDeviceStatics_Vtbl;
}
impl ::core::clone::Clone for IVibrationDeviceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVibrationDeviceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x332fd2f1_1c69_4c91_949e_4bb67a85bdc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVibrationDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Phone_Devices_Notification\"`*"]
#[repr(transparent)]
pub struct VibrationDevice(::windows_core::IUnknown);
impl VibrationDevice {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Vibrate(&self, duration: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Vibrate)(::windows_core::Interface::as_raw(this), duration).ok() }
    }
    pub fn Cancel(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetDefault() -> ::windows_core::Result<VibrationDevice> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVibrationDeviceStatics<R, F: FnOnce(&IVibrationDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VibrationDevice, IVibrationDeviceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for VibrationDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VibrationDevice {}
impl ::core::fmt::Debug for VibrationDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VibrationDevice").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VibrationDevice {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Devices.Notification.VibrationDevice;{1b4a6595-cfcd-4e08-92fb-c1906d04498c})");
}
impl ::core::clone::Clone for VibrationDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VibrationDevice {
    type Vtable = IVibrationDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VibrationDevice {
    const IID: ::windows_core::GUID = <IVibrationDevice as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VibrationDevice {
    const NAME: &'static str = "Windows.Phone.Devices.Notification.VibrationDevice";
}
::windows_core::imp::interface_hierarchy!(VibrationDevice, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VibrationDevice {}
unsafe impl ::core::marker::Sync for VibrationDevice {}
