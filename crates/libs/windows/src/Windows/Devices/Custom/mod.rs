#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
pub struct CustomDevice(::windows::core::IUnknown);
impl CustomDevice {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InputStream)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OutputStream)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendIOControlAsync<'a, P0, E0, P1, E1, P2, E2>(&self, iocontrolcode: P0, inputbuffer: P1, outputbuffer: P2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IIOControlCode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IBuffer>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SendIOControlAsync)(::windows::core::Interface::as_raw(this), iocontrolcode.try_into().map_err(|e| e.into())?.abi(), inputbuffer.try_into().map_err(|e| e.into())?.abi(), outputbuffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn TrySendIOControlAsync<'a, P0, E0, P1, E1, P2, E2>(&self, iocontrolcode: P0, inputbuffer: P1, outputbuffer: P2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IIOControlCode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IBuffer>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TrySendIOControlAsync)(::windows::core::Interface::as_raw(this), iocontrolcode.try_into().map_err(|e| e.into())?.abi(), inputbuffer.try_into().map_err(|e| e.into())?.abi(), outputbuffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn GetDeviceSelector(classguid: ::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICustomDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), classguid, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING, desiredaccess: DeviceAccessMode, sharingmode: DeviceSharingMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CustomDevice>> {
        Self::ICustomDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), desiredaccess, sharingmode, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<CustomDevice>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICustomDeviceStatics<R, F: FnOnce(&ICustomDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CustomDevice, ICustomDeviceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CustomDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CustomDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CustomDevice {}
impl ::core::fmt::Debug for CustomDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CustomDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CustomDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Custom.CustomDevice;{dd30251f-c48b-43bd-bcb1-dec88f15143e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CustomDevice {
    type Vtable = ICustomDevice_Vtbl;
    const IID: ::windows::core::GUID = <ICustomDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CustomDevice {
    const NAME: &'static str = "Windows.Devices.Custom.CustomDevice";
}
impl ::core::convert::From<CustomDevice> for ::windows::core::IUnknown {
    fn from(value: CustomDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CustomDevice> for ::windows::core::IUnknown {
    fn from(value: &CustomDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CustomDevice> for &::windows::core::IUnknown {
    fn from(value: &CustomDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CustomDevice> for ::windows::core::IInspectable {
    fn from(value: CustomDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CustomDevice> for ::windows::core::IInspectable {
    fn from(value: &CustomDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CustomDevice> for &::windows::core::IInspectable {
    fn from(value: &CustomDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CustomDevice {}
unsafe impl ::core::marker::Sync for CustomDevice {}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeviceAccessMode(pub i32);
impl DeviceAccessMode {
    pub const Read: Self = Self(0i32);
    pub const Write: Self = Self(1i32);
    pub const ReadWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for DeviceAccessMode {}
impl ::core::clone::Clone for DeviceAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceAccessMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DeviceAccessMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccessMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceAccessMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.DeviceAccessMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeviceSharingMode(pub i32);
impl DeviceSharingMode {
    pub const Shared: Self = Self(0i32);
    pub const Exclusive: Self = Self(1i32);
}
impl ::core::marker::Copy for DeviceSharingMode {}
impl ::core::clone::Clone for DeviceSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceSharingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DeviceSharingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceSharingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceSharingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.DeviceSharingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICustomDevice {
    type Vtable = ICustomDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd30251f_c48b_43bd_bcb1_dec88f15143e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomDevice_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendIOControlAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iocontrolcode: *mut ::core::ffi::c_void, inputbuffer: *mut ::core::ffi::c_void, outputbuffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendIOControlAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TrySendIOControlAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iocontrolcode: *mut ::core::ffi::c_void, inputbuffer: *mut ::core::ffi::c_void, outputbuffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TrySendIOControlAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICustomDeviceStatics {
    type Vtable = ICustomDeviceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8220312_ef4c_46b1_a58e_eeb308dc8917);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classguid: ::windows::core::GUID, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, desiredaccess: DeviceAccessMode, sharingmode: DeviceSharingMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
pub struct IIOControlCode(::windows::core::IUnknown);
impl IIOControlCode {
    pub fn AccessMode(&self) -> ::windows::core::Result<IOControlAccessMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AccessMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IOControlAccessMode>(result__)
        }
    }
    pub fn BufferingMethod(&self) -> ::windows::core::Result<IOControlBufferingMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BufferingMethod)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IOControlBufferingMethod>(result__)
        }
    }
    pub fn Function(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Function)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn DeviceType(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn ControlCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ControlCode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::convert::From<IIOControlCode> for ::windows::core::IUnknown {
    fn from(value: IIOControlCode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IIOControlCode> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IIOControlCode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IIOControlCode> for ::windows::core::IUnknown {
    fn from(value: &IIOControlCode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IIOControlCode> for ::windows::core::IInspectable {
    fn from(value: IIOControlCode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IIOControlCode> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IIOControlCode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IIOControlCode> for ::windows::core::IInspectable {
    fn from(value: &IIOControlCode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IIOControlCode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IIOControlCode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIOControlCode {}
impl ::core::fmt::Debug for IIOControlCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIOControlCode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IIOControlCode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{0e9559e7-60c8-4375-a761-7f8808066c60}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IIOControlCode {
    type Vtable = IIOControlCode_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e9559e7_60c8_4375_a761_7f8808066c60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIOControlCode_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AccessMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IOControlAccessMode) -> ::windows::core::HRESULT,
    pub BufferingMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IOControlBufferingMethod) -> ::windows::core::HRESULT,
    pub Function: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub DeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub ControlCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIOControlCodeFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIOControlCodeFactory {
    type Vtable = IIOControlCodeFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x856a7cf0_4c11_44ae_afc6_b8d4a212788f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIOControlCodeFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateIOControlCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicetype: u16, function: u16, accessmode: IOControlAccessMode, bufferingmethod: IOControlBufferingMethod, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownDeviceTypesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownDeviceTypesStatics {
    type Vtable = IKnownDeviceTypesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee5479c2_5448_45da_ad1b_24948c239094);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownDeviceTypesStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Unknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IOControlAccessMode(pub i32);
impl IOControlAccessMode {
    pub const Any: Self = Self(0i32);
    pub const Read: Self = Self(1i32);
    pub const Write: Self = Self(2i32);
    pub const ReadWrite: Self = Self(3i32);
}
impl ::core::marker::Copy for IOControlAccessMode {}
impl ::core::clone::Clone for IOControlAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IOControlAccessMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IOControlAccessMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for IOControlAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOControlAccessMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IOControlAccessMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.IOControlAccessMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IOControlBufferingMethod(pub i32);
impl IOControlBufferingMethod {
    pub const Buffered: Self = Self(0i32);
    pub const DirectInput: Self = Self(1i32);
    pub const DirectOutput: Self = Self(2i32);
    pub const Neither: Self = Self(3i32);
}
impl ::core::marker::Copy for IOControlBufferingMethod {}
impl ::core::clone::Clone for IOControlBufferingMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IOControlBufferingMethod {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IOControlBufferingMethod {
    type Abi = Self;
}
impl ::core::fmt::Debug for IOControlBufferingMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOControlBufferingMethod").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IOControlBufferingMethod {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.IOControlBufferingMethod;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
pub struct IOControlCode(::windows::core::IUnknown);
impl IOControlCode {
    pub fn AccessMode(&self) -> ::windows::core::Result<IOControlAccessMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AccessMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IOControlAccessMode>(result__)
        }
    }
    pub fn BufferingMethod(&self) -> ::windows::core::Result<IOControlBufferingMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BufferingMethod)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IOControlBufferingMethod>(result__)
        }
    }
    pub fn Function(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Function)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn DeviceType(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn ControlCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ControlCode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn CreateIOControlCode(devicetype: u16, function: u16, accessmode: IOControlAccessMode, bufferingmethod: IOControlBufferingMethod) -> ::windows::core::Result<IOControlCode> {
        Self::IIOControlCodeFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateIOControlCode)(::windows::core::Interface::as_raw(this), devicetype, function, accessmode, bufferingmethod, result__.as_mut_ptr()).from_abi::<IOControlCode>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IIOControlCodeFactory<R, F: FnOnce(&IIOControlCodeFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<IOControlCode, IIOControlCodeFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for IOControlCode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOControlCode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOControlCode {}
impl ::core::fmt::Debug for IOControlCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOControlCode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IOControlCode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Custom.IOControlCode;{0e9559e7-60c8-4375-a761-7f8808066c60})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IOControlCode {
    type Vtable = IIOControlCode_Vtbl;
    const IID: ::windows::core::GUID = <IIOControlCode as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IOControlCode {
    const NAME: &'static str = "Windows.Devices.Custom.IOControlCode";
}
impl ::core::convert::From<IOControlCode> for ::windows::core::IUnknown {
    fn from(value: IOControlCode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IOControlCode> for ::windows::core::IUnknown {
    fn from(value: &IOControlCode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&IOControlCode> for &::windows::core::IUnknown {
    fn from(value: &IOControlCode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<IOControlCode> for ::windows::core::IInspectable {
    fn from(value: IOControlCode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IOControlCode> for ::windows::core::IInspectable {
    fn from(value: &IOControlCode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&IOControlCode> for &::windows::core::IInspectable {
    fn from(value: &IOControlCode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<IOControlCode> for IIOControlCode {
    type Error = ::windows::core::Error;
    fn try_from(value: IOControlCode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IOControlCode> for IIOControlCode {
    type Error = ::windows::core::Error;
    fn try_from(value: &IOControlCode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IOControlCode> for ::windows::core::InParam<'a, IIOControlCode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IOControlCode) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for IOControlCode {}
unsafe impl ::core::marker::Sync for IOControlCode {}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
pub struct KnownDeviceTypes;
impl KnownDeviceTypes {
    pub fn Unknown() -> ::windows::core::Result<u16> {
        Self::IKnownDeviceTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Unknown)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownDeviceTypesStatics<R, F: FnOnce(&IKnownDeviceTypesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownDeviceTypes, IKnownDeviceTypesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for KnownDeviceTypes {
    const NAME: &'static str = "Windows.Devices.Custom.KnownDeviceTypes";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
