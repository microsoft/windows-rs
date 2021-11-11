#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_DeviceIoControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12d3e372_874b_457d_9fdf_73977778686c);
#[doc = "*Required features: `Win32_Devices_DeviceAccess`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDeviceAccessInstance<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(deviceinterfacepath: Param0, desiredaccess: u32) -> ::windows::core::Result<ICreateDeviceAccessAsync> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDeviceAccessInstance(deviceinterfacepath: super::super::Foundation::PWSTR, desiredaccess: u32, createasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <ICreateDeviceAccessAsync as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateDeviceAccessInstance(deviceinterfacepath.into_param().abi(), ::core::mem::transmute(desiredaccess), &mut result__).from_abi::<ICreateDeviceAccessAsync>(result__)
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICreateDeviceAccessAsync(pub ::windows::core::IUnknown);
impl ICreateDeviceAccessAsync {
    #[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
    pub unsafe fn Wait(&self, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(timeout)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
    pub unsafe fn GetResult<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for ICreateDeviceAccessAsync {
    type Vtable = ICreateDeviceAccessAsync_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3474628f_683d_42d2_abcb_db018c6503bc);
}
impl ::core::convert::From<ICreateDeviceAccessAsync> for ::windows::core::IUnknown {
    fn from(value: ICreateDeviceAccessAsync) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICreateDeviceAccessAsync> for ::windows::core::IUnknown {
    fn from(value: &ICreateDeviceAccessAsync) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICreateDeviceAccessAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICreateDeviceAccessAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateDeviceAccessAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timeout: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, deviceaccess: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDeviceIoControl(pub ::windows::core::IUnknown);
impl IDeviceIoControl {
    #[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
    pub unsafe fn DeviceIoControlSync(&self, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(iocontrolcode), ::core::mem::transmute(inputbuffer), ::core::mem::transmute(inputbuffersize), ::core::mem::transmute(outputbuffer), ::core::mem::transmute(outputbuffersize), ::core::mem::transmute(bytesreturned)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
    pub unsafe fn DeviceIoControlAsync<'a, Param5: ::windows::core::IntoParam<'a, IDeviceRequestCompletionCallback>>(&self, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, requestcompletioncallback: Param5, cancelcontext: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(iocontrolcode),
            ::core::mem::transmute(inputbuffer),
            ::core::mem::transmute(inputbuffersize),
            ::core::mem::transmute(outputbuffer),
            ::core::mem::transmute(outputbuffersize),
            requestcompletioncallback.into_param().abi(),
            ::core::mem::transmute(cancelcontext),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
    pub unsafe fn CancelOperation(&self, cancelcontext: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cancelcontext)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDeviceIoControl {
    type Vtable = IDeviceIoControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9eefe161_23ab_4f18_9b49_991b586ae970);
}
impl ::core::convert::From<IDeviceIoControl> for ::windows::core::IUnknown {
    fn from(value: IDeviceIoControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDeviceIoControl> for ::windows::core::IUnknown {
    fn from(value: &IDeviceIoControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDeviceIoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDeviceIoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceIoControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, requestcompletioncallback: ::windows::core::RawPtr, cancelcontext: *mut usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cancelcontext: usize) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDeviceRequestCompletionCallback(pub ::windows::core::IUnknown);
impl IDeviceRequestCompletionCallback {
    #[doc = "*Required features: `Win32_Devices_DeviceAccess`*"]
    pub unsafe fn Invoke(&self, requestresult: ::windows::core::HRESULT, bytesreturned: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(requestresult), ::core::mem::transmute(bytesreturned)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDeviceRequestCompletionCallback {
    type Vtable = IDeviceRequestCompletionCallback_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x999bad24_9acd_45bb_8669_2a2fc0288b04);
}
impl ::core::convert::From<IDeviceRequestCompletionCallback> for ::windows::core::IUnknown {
    fn from(value: IDeviceRequestCompletionCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDeviceRequestCompletionCallback> for ::windows::core::IUnknown {
    fn from(value: &IDeviceRequestCompletionCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDeviceRequestCompletionCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDeviceRequestCompletionCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceRequestCompletionCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requestresult: ::windows::core::HRESULT, bytesreturned: u32) -> ::windows::core::HRESULT,
);
