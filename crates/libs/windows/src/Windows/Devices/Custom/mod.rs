#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICustomDevice {
    type Vtable = ICustomDevice_Vtbl;
}
impl ::core::clone::Clone for ICustomDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICustomDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd30251f_c48b_43bd_bcb1_dec88f15143e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomDevice_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendIOControlAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iocontrolcode: *mut ::core::ffi::c_void, inputbuffer: *mut ::core::ffi::c_void, outputbuffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendIOControlAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TrySendIOControlAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iocontrolcode: *mut ::core::ffi::c_void, inputbuffer: *mut ::core::ffi::c_void, outputbuffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TrySendIOControlAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICustomDeviceStatics {
    type Vtable = ICustomDeviceStatics_Vtbl;
}
impl ::core::clone::Clone for ICustomDeviceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICustomDeviceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8220312_ef4c_46b1_a58e_eeb308dc8917);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classguid: ::windows_core::GUID, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, desiredaccess: DeviceAccessMode, sharingmode: DeviceSharingMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
pub struct IIOControlCode(::windows_core::IUnknown);
impl IIOControlCode {
    pub fn AccessMode(&self) -> ::windows_core::Result<IOControlAccessMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BufferingMethod(&self) -> ::windows_core::Result<IOControlBufferingMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BufferingMethod)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Function(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Function)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceType(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ControlCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ControlCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IIOControlCode, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for IIOControlCode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{0e9559e7-60c8-4375-a761-7f8808066c60}");
}
unsafe impl ::windows_core::Interface for IIOControlCode {
    type Vtable = IIOControlCode_Vtbl;
}
impl ::core::clone::Clone for IIOControlCode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIOControlCode {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e9559e7_60c8_4375_a761_7f8808066c60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIOControlCode_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AccessMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IOControlAccessMode) -> ::windows_core::HRESULT,
    pub BufferingMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IOControlBufferingMethod) -> ::windows_core::HRESULT,
    pub Function: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub DeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub ControlCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIOControlCodeFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIOControlCodeFactory {
    type Vtable = IIOControlCodeFactory_Vtbl;
}
impl ::core::clone::Clone for IIOControlCodeFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIOControlCodeFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x856a7cf0_4c11_44ae_afc6_b8d4a212788f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIOControlCodeFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateIOControlCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicetype: u16, function: u16, accessmode: IOControlAccessMode, bufferingmethod: IOControlBufferingMethod, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownDeviceTypesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKnownDeviceTypesStatics {
    type Vtable = IKnownDeviceTypesStatics_Vtbl;
}
impl ::core::clone::Clone for IKnownDeviceTypesStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IKnownDeviceTypesStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee5479c2_5448_45da_ad1b_24948c239094);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownDeviceTypesStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Unknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
pub struct CustomDevice(::windows_core::IUnknown);
impl CustomDevice {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> ::windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InputStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows_core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OutputStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendIOControlAsync<P0, P1, P2>(&self, iocontrolcode: P0, inputbuffer: P1, outputbuffer: P2) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>
    where
        P0: ::windows_core::TryIntoParam<IIOControlCode>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
        P2: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendIOControlAsync)(::windows_core::Interface::as_raw(this), iocontrolcode.try_into_param()?.abi(), inputbuffer.try_into_param()?.abi(), outputbuffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn TrySendIOControlAsync<P0, P1, P2>(&self, iocontrolcode: P0, inputbuffer: P1, outputbuffer: P2) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::TryIntoParam<IIOControlCode>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
        P2: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySendIOControlAsync)(::windows_core::Interface::as_raw(this), iocontrolcode.try_into_param()?.abi(), inputbuffer.try_into_param()?.abi(), outputbuffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeviceSelector(classguid: ::windows_core::GUID) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICustomDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), classguid, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows_core::HSTRING, desiredaccess: DeviceAccessMode, sharingmode: DeviceSharingMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<CustomDevice>> {
        Self::ICustomDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), desiredaccess, sharingmode, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICustomDeviceStatics<R, F: FnOnce(&ICustomDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CustomDevice, ICustomDeviceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for CustomDevice {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Custom.CustomDevice;{dd30251f-c48b-43bd-bcb1-dec88f15143e})");
}
impl ::core::clone::Clone for CustomDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CustomDevice {
    type Vtable = ICustomDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CustomDevice {
    const IID: ::windows_core::GUID = <ICustomDevice as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CustomDevice {
    const NAME: &'static str = "Windows.Devices.Custom.CustomDevice";
}
::windows_core::imp::interface_hierarchy!(CustomDevice, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CustomDevice {}
unsafe impl ::core::marker::Sync for CustomDevice {}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
pub struct IOControlCode(::windows_core::IUnknown);
impl IOControlCode {
    pub fn AccessMode(&self) -> ::windows_core::Result<IOControlAccessMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BufferingMethod(&self) -> ::windows_core::Result<IOControlBufferingMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BufferingMethod)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Function(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Function)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceType(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ControlCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ControlCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateIOControlCode(devicetype: u16, function: u16, accessmode: IOControlAccessMode, bufferingmethod: IOControlBufferingMethod) -> ::windows_core::Result<IOControlCode> {
        Self::IIOControlCodeFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateIOControlCode)(::windows_core::Interface::as_raw(this), devicetype, function, accessmode, bufferingmethod, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IIOControlCodeFactory<R, F: FnOnce(&IIOControlCodeFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<IOControlCode, IIOControlCodeFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for IOControlCode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Custom.IOControlCode;{0e9559e7-60c8-4375-a761-7f8808066c60})");
}
impl ::core::clone::Clone for IOControlCode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for IOControlCode {
    type Vtable = IIOControlCode_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOControlCode {
    const IID: ::windows_core::GUID = <IIOControlCode as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for IOControlCode {
    const NAME: &'static str = "Windows.Devices.Custom.IOControlCode";
}
::windows_core::imp::interface_hierarchy!(IOControlCode, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IIOControlCode> for IOControlCode {}
unsafe impl ::core::marker::Send for IOControlCode {}
unsafe impl ::core::marker::Sync for IOControlCode {}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
pub struct KnownDeviceTypes;
impl KnownDeviceTypes {
    pub fn Unknown() -> ::windows_core::Result<u16> {
        Self::IKnownDeviceTypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Unknown)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownDeviceTypesStatics<R, F: FnOnce(&IKnownDeviceTypesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<KnownDeviceTypes, IKnownDeviceTypesStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for KnownDeviceTypes {
    const NAME: &'static str = "Windows.Devices.Custom.KnownDeviceTypes";
}
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
impl ::windows_core::TypeKind for DeviceAccessMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DeviceAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccessMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceAccessMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.DeviceAccessMode;i4)");
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
impl ::windows_core::TypeKind for DeviceSharingMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DeviceSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceSharingMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceSharingMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.DeviceSharingMode;i4)");
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
impl ::windows_core::TypeKind for IOControlAccessMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IOControlAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOControlAccessMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IOControlAccessMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.IOControlAccessMode;i4)");
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
impl ::windows_core::TypeKind for IOControlBufferingMethod {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IOControlBufferingMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOControlBufferingMethod").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IOControlBufferingMethod {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.IOControlBufferingMethod;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
