#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_DeviceIoControl: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(315876210, 34635, 17789, [159, 223, 115, 151, 119, 120, 104, 108]);
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_DeviceAccess`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CreateDeviceAccessInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(deviceinterfacepath: Param0, desiredaccess: u32) -> ::windows::runtime::Result<ICreateDeviceAccessAsync> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDeviceAccessInstance(deviceinterfacepath: super::super::Foundation::PWSTR, desiredaccess: u32, createasync: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <ICreateDeviceAccessAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateDeviceAccessInstance(deviceinterfacepath.into_param().abi(), ::std::mem::transmute(desiredaccess), &mut result__).from_abi::<ICreateDeviceAccessAsync>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const DEV_PORT_1394: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const DEV_PORT_ARTI: u32 = 7u32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const DEV_PORT_COM1: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const DEV_PORT_COM2: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const DEV_PORT_COM3: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const DEV_PORT_COM4: u32 = 5u32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const DEV_PORT_DIAQ: u32 = 6u32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const DEV_PORT_MAX: u32 = 9u32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const DEV_PORT_MIN: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const DEV_PORT_SIM: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const DEV_PORT_USB: u32 = 9u32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_1: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_10: i32 = 512i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_11: i32 = 1024i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_12: i32 = 2048i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_13: i32 = 4096i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_14: i32 = 8192i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_15: i32 = 16384i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_16: i32 = 32768i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_17: i32 = 65536i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_18: i32 = 131072i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_19: i32 = 262144i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_2: i32 = 2i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_20: i32 = 524288i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_21: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_22: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_23: i32 = 4194304i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_24: i32 = 8388608i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_3: i32 = 4i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_4: i32 = 8i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_5: i32 = 16i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_6: i32 = 32i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_7: i32 = 64i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_8: i32 = 128i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_9: i32 = 256i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_AUDIO_ALL: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_BASE: i32 = 4096i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_BOTTOM: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_CENTER: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_LEFT: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_MIDDLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_RIGHT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_TOP: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
pub const ED_VIDEO: i32 = 33554432i32;
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ICreateDeviceAccessAsync(::windows::runtime::IUnknown);
impl ICreateDeviceAccessAsync {
    #[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
    pub unsafe fn Wait(&self, timeout: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(timeout)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
    pub unsafe fn GetResult<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICreateDeviceAccessAsync {
    type Vtable = ICreateDeviceAccessAsync_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(880042639, 26685, 17106, [171, 203, 219, 1, 140, 101, 3, 188]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateDeviceAccessAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timeout: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, deviceaccess: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDeviceIoControl(::windows::runtime::IUnknown);
impl IDeviceIoControl {
    #[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
    pub unsafe fn DeviceIoControlSync(&self, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, bytesreturned: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(iocontrolcode), ::std::mem::transmute(inputbuffer), ::std::mem::transmute(inputbuffersize), ::std::mem::transmute(outputbuffer), ::std::mem::transmute(outputbuffersize), ::std::mem::transmute(bytesreturned)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
    pub unsafe fn DeviceIoControlAsync<'a, Param5: ::windows::runtime::IntoParam<'a, IDeviceRequestCompletionCallback>>(&self, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, requestcompletioncallback: Param5, cancelcontext: *mut usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(iocontrolcode),
            ::std::mem::transmute(inputbuffer),
            ::std::mem::transmute(inputbuffersize),
            ::std::mem::transmute(outputbuffer),
            ::std::mem::transmute(outputbuffersize),
            requestcompletioncallback.into_param().abi(),
            ::std::mem::transmute(cancelcontext),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
    pub unsafe fn CancelOperation(&self, cancelcontext: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cancelcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDeviceIoControl {
    type Vtable = IDeviceIoControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2666520929, 9131, 20248, [155, 73, 153, 27, 88, 106, 233, 112]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceIoControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, bytesreturned: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, requestcompletioncallback: ::windows::runtime::RawPtr, cancelcontext: *mut usize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cancelcontext: usize) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDeviceRequestCompletionCallback(::windows::runtime::IUnknown);
impl IDeviceRequestCompletionCallback {
    #[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
    pub unsafe fn Invoke(&self, requestresult: ::windows::runtime::HRESULT, bytesreturned: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(requestresult), ::std::mem::transmute(bytesreturned)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDeviceRequestCompletionCallback {
    type Vtable = IDeviceRequestCompletionCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2577116452, 39629, 17851, [134, 105, 42, 47, 192, 40, 139, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceRequestCompletionCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestresult: ::windows::runtime::HRESULT, bytesreturned: u32) -> ::windows::runtime::HRESULT,
);
