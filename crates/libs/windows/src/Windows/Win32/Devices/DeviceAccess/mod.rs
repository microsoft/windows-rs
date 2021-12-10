#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const CLSID_DeviceIoControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12d3e372_874b_457d_9fdf_73977778686c);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDeviceAccessInstance<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(deviceinterfacepath: Param0, desiredaccess: u32) -> ::windows::core::Result<ICreateDeviceAccessAsync> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDeviceAccessInstance(deviceinterfacepath: super::super::Foundation::PWSTR, desiredaccess: u32, createasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateDeviceAccessInstance(deviceinterfacepath.into_param().abi(), ::core::mem::transmute(desiredaccess), ::core::mem::transmute(&mut result__)).from_abi::<ICreateDeviceAccessAsync>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DEV_PORT_1394: u32 = 8u32;
pub const DEV_PORT_ARTI: u32 = 7u32;
pub const DEV_PORT_COM1: u32 = 2u32;
pub const DEV_PORT_COM2: u32 = 3u32;
pub const DEV_PORT_COM3: u32 = 4u32;
pub const DEV_PORT_COM4: u32 = 5u32;
pub const DEV_PORT_DIAQ: u32 = 6u32;
pub const DEV_PORT_MAX: u32 = 9u32;
pub const DEV_PORT_MIN: u32 = 1u32;
pub const DEV_PORT_SIM: u32 = 1u32;
pub const DEV_PORT_USB: u32 = 9u32;
pub const ED_AUDIO_1: i32 = 1i32;
pub const ED_AUDIO_10: i32 = 512i32;
pub const ED_AUDIO_11: i32 = 1024i32;
pub const ED_AUDIO_12: i32 = 2048i32;
pub const ED_AUDIO_13: i32 = 4096i32;
pub const ED_AUDIO_14: i32 = 8192i32;
pub const ED_AUDIO_15: i32 = 16384i32;
pub const ED_AUDIO_16: i32 = 32768i32;
pub const ED_AUDIO_17: i32 = 65536i32;
pub const ED_AUDIO_18: i32 = 131072i32;
pub const ED_AUDIO_19: i32 = 262144i32;
pub const ED_AUDIO_2: i32 = 2i32;
pub const ED_AUDIO_20: i32 = 524288i32;
pub const ED_AUDIO_21: i32 = 1048576i32;
pub const ED_AUDIO_22: i32 = 2097152i32;
pub const ED_AUDIO_23: i32 = 4194304i32;
pub const ED_AUDIO_24: i32 = 8388608i32;
pub const ED_AUDIO_3: i32 = 4i32;
pub const ED_AUDIO_4: i32 = 8i32;
pub const ED_AUDIO_5: i32 = 16i32;
pub const ED_AUDIO_6: i32 = 32i32;
pub const ED_AUDIO_7: i32 = 64i32;
pub const ED_AUDIO_8: i32 = 128i32;
pub const ED_AUDIO_9: i32 = 256i32;
pub const ED_AUDIO_ALL: u32 = 268435456u32;
pub const ED_BASE: i32 = 4096i32;
pub const ED_BOTTOM: u32 = 4u32;
pub const ED_CENTER: u32 = 512u32;
pub const ED_LEFT: u32 = 256u32;
pub const ED_MIDDLE: u32 = 2u32;
pub const ED_RIGHT: u32 = 1024u32;
pub const ED_TOP: u32 = 1u32;
pub const ED_VIDEO: i32 = 33554432i32;
#[repr(transparent)]
pub struct ICreateDeviceAccessAsync(::windows::core::IUnknown);
impl ICreateDeviceAccessAsync {
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Wait(&self, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(timeout)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetResult<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ICreateDeviceAccessAsync> for ::windows::core::IUnknown {
    fn from(value: ICreateDeviceAccessAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICreateDeviceAccessAsync> for ::windows::core::IUnknown {
    fn from(value: &ICreateDeviceAccessAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICreateDeviceAccessAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICreateDeviceAccessAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICreateDeviceAccessAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICreateDeviceAccessAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateDeviceAccessAsync {}
unsafe impl ::windows::core::Interface for ICreateDeviceAccessAsync {
    type Vtable = ICreateDeviceAccessAsyncVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3474628f_683d_42d2_abcb_db018c6503bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateDeviceAccessAsyncVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, deviceaccess: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDeviceIoControl(::windows::core::IUnknown);
impl IDeviceIoControl {
    pub unsafe fn DeviceIoControlSync(&self, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(iocontrolcode), ::core::mem::transmute(inputbuffer), ::core::mem::transmute(inputbuffersize), ::core::mem::transmute(outputbuffer), ::core::mem::transmute(outputbuffersize), ::core::mem::transmute(bytesreturned)).ok()
    }
    pub unsafe fn DeviceIoControlAsync<'a, Param5: ::windows::core::IntoParam<'a, IDeviceRequestCompletionCallback>>(&self, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, requestcompletioncallback: Param5, cancelcontext: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(iocontrolcode), ::core::mem::transmute(inputbuffer), ::core::mem::transmute(inputbuffersize), ::core::mem::transmute(outputbuffer), ::core::mem::transmute(outputbuffersize), requestcompletioncallback.into_param().abi(), ::core::mem::transmute(cancelcontext)).ok()
    }
    pub unsafe fn CancelOperation(&self, cancelcontext: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cancelcontext)).ok()
    }
}
impl ::core::convert::From<IDeviceIoControl> for ::windows::core::IUnknown {
    fn from(value: IDeviceIoControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDeviceIoControl> for ::windows::core::IUnknown {
    fn from(value: &IDeviceIoControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDeviceIoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDeviceIoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDeviceIoControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDeviceIoControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceIoControl {}
unsafe impl ::windows::core::Interface for IDeviceIoControl {
    type Vtable = IDeviceIoControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9eefe161_23ab_4f18_9b49_991b586ae970);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceIoControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, requestcompletioncallback: ::windows::core::RawPtr, cancelcontext: *mut usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cancelcontext: usize) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDeviceRequestCompletionCallback(::windows::core::IUnknown);
impl IDeviceRequestCompletionCallback {
    pub unsafe fn Invoke(&self, requestresult: ::windows::core::HRESULT, bytesreturned: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(requestresult), ::core::mem::transmute(bytesreturned)).ok()
    }
}
impl ::core::convert::From<IDeviceRequestCompletionCallback> for ::windows::core::IUnknown {
    fn from(value: IDeviceRequestCompletionCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDeviceRequestCompletionCallback> for ::windows::core::IUnknown {
    fn from(value: &IDeviceRequestCompletionCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDeviceRequestCompletionCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDeviceRequestCompletionCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDeviceRequestCompletionCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDeviceRequestCompletionCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceRequestCompletionCallback {}
unsafe impl ::windows::core::Interface for IDeviceRequestCompletionCallback {
    type Vtable = IDeviceRequestCompletionCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x999bad24_9acd_45bb_8669_2a2fc0288b04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceRequestCompletionCallbackVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestresult: ::windows::core::HRESULT, bytesreturned: u32) -> ::windows::core::HRESULT);
