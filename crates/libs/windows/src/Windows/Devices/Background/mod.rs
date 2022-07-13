#[doc = "*Required features: `\"Devices_Background\"`*"]
#[repr(transparent)]
pub struct DeviceServicingDetails(::windows::core::IUnknown);
impl DeviceServicingDetails {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpectedDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ExpectedDuration)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for DeviceServicingDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for DeviceServicingDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Background.DeviceServicingDetails;{4aabee29-2344-4ac4-8527-4a8ef6905645})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DeviceServicingDetails {
    type Vtable = IDeviceServicingDetails_Vtbl;
    const IID: ::windows::core::GUID = <IDeviceServicingDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DeviceServicingDetails {
    const NAME: &'static str = "Windows.Devices.Background.DeviceServicingDetails";
}
impl ::core::convert::From<DeviceServicingDetails> for ::windows::core::IUnknown {
    fn from(value: DeviceServicingDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceServicingDetails> for ::windows::core::IUnknown {
    fn from(value: &DeviceServicingDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DeviceServicingDetails> for &::windows::core::IUnknown {
    fn from(value: &DeviceServicingDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DeviceServicingDetails> for ::windows::core::IInspectable {
    fn from(value: DeviceServicingDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceServicingDetails> for ::windows::core::IInspectable {
    fn from(value: &DeviceServicingDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DeviceServicingDetails> for &::windows::core::IInspectable {
    fn from(value: &DeviceServicingDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for DeviceServicingDetails {}
unsafe impl ::core::marker::Sync for DeviceServicingDetails {}
#[doc = "*Required features: `\"Devices_Background\"`*"]
#[repr(transparent)]
pub struct DeviceUseDetails(::windows::core::IUnknown);
impl DeviceUseDetails {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for DeviceUseDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for DeviceUseDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Background.DeviceUseDetails;{7d565141-557e-4154-b994-e4f7a11fb323})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DeviceUseDetails {
    type Vtable = IDeviceUseDetails_Vtbl;
    const IID: ::windows::core::GUID = <IDeviceUseDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DeviceUseDetails {
    const NAME: &'static str = "Windows.Devices.Background.DeviceUseDetails";
}
impl ::core::convert::From<DeviceUseDetails> for ::windows::core::IUnknown {
    fn from(value: DeviceUseDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceUseDetails> for ::windows::core::IUnknown {
    fn from(value: &DeviceUseDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DeviceUseDetails> for &::windows::core::IUnknown {
    fn from(value: &DeviceUseDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DeviceUseDetails> for ::windows::core::IInspectable {
    fn from(value: DeviceUseDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceUseDetails> for ::windows::core::IInspectable {
    fn from(value: &DeviceUseDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DeviceUseDetails> for &::windows::core::IInspectable {
    fn from(value: &DeviceUseDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for DeviceUseDetails {}
unsafe impl ::core::marker::Sync for DeviceUseDetails {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceServicingDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeviceServicingDetails {
    type Vtable = IDeviceServicingDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4aabee29_2344_4ac4_8527_4a8ef6905645);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceServicingDetails_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpectedDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpectedDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceUseDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeviceUseDetails {
    type Vtable = IDeviceUseDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d565141_557e_4154_b994_e4f7a11fb323);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceUseDetails_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
