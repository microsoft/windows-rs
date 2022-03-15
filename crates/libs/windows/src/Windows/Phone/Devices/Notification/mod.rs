#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IVibrationDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVibrationDevice {
    type Vtable = IVibrationDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b4a6595_cfcd_4e08_92fb_c1906d04498c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVibrationDevice_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Vibrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Vibrate: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVibrationDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVibrationDeviceStatics {
    type Vtable = IVibrationDeviceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x332fd2f1_1c69_4c91_949e_4bb67a85bdc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVibrationDeviceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Phone_Devices_Notification\"`*"]
#[repr(transparent)]
pub struct VibrationDevice(::windows::core::IUnknown);
impl VibrationDevice {
    #[doc = "*Required features: `\"Phone_Devices_Notification\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Vibrate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, duration: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Vibrate)(::core::mem::transmute_copy(this), duration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Phone_Devices_Notification\"`*"]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Cancel)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Phone_Devices_Notification\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<VibrationDevice> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VibrationDevice>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVibrationDeviceStatics<R, F: FnOnce(&IVibrationDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VibrationDevice, IVibrationDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for VibrationDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for VibrationDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Devices.Notification.VibrationDevice;{1b4a6595-cfcd-4e08-92fb-c1906d04498c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for VibrationDevice {
    type Vtable = IVibrationDevice_Vtbl;
    const IID: ::windows::core::GUID = <IVibrationDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VibrationDevice {
    const NAME: &'static str = "Windows.Phone.Devices.Notification.VibrationDevice";
}
impl ::core::convert::From<VibrationDevice> for ::windows::core::IUnknown {
    fn from(value: VibrationDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VibrationDevice> for ::windows::core::IUnknown {
    fn from(value: &VibrationDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VibrationDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VibrationDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VibrationDevice> for ::windows::core::IInspectable {
    fn from(value: VibrationDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VibrationDevice> for ::windows::core::IInspectable {
    fn from(value: &VibrationDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VibrationDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VibrationDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VibrationDevice {}
unsafe impl ::core::marker::Sync for VibrationDevice {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
