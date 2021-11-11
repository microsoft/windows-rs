#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_WinRT_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWindowsDevicesAllJoynBusAttachmentFactoryInterop(pub ::windows::core::IUnknown);
impl IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    #[doc = "*Required features: `Win32_System_WinRT_AllJoyn`*"]
    pub unsafe fn CreateFromWin32Handle<T: ::windows::core::Interface>(&self, win32handle: u64, enableaboutdata: u8) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(win32handle), ::core::mem::transmute(enableaboutdata), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    type Vtable = IWindowsDevicesAllJoynBusAttachmentFactoryInterop_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b8f7505_b239_4e7b_88af_f6682575d861);
}
impl ::core::convert::From<IWindowsDevicesAllJoynBusAttachmentFactoryInterop> for ::windows::core::IUnknown {
    fn from(value: IWindowsDevicesAllJoynBusAttachmentFactoryInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWindowsDevicesAllJoynBusAttachmentFactoryInterop> for ::windows::core::IUnknown {
    fn from(value: &IWindowsDevicesAllJoynBusAttachmentFactoryInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusAttachmentFactoryInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, win32handle: u64, enableaboutdata: u8, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWindowsDevicesAllJoynBusAttachmentInterop(pub ::windows::core::IUnknown);
impl IWindowsDevicesAllJoynBusAttachmentInterop {
    #[doc = "*Required features: `Win32_System_WinRT_AllJoyn`*"]
    pub unsafe fn Win32Handle(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWindowsDevicesAllJoynBusAttachmentInterop {
    type Vtable = IWindowsDevicesAllJoynBusAttachmentInterop_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd89c65b_b50e_4a19_9d0c_b42b783281cd);
}
impl ::core::convert::From<IWindowsDevicesAllJoynBusAttachmentInterop> for ::windows::core::IUnknown {
    fn from(value: IWindowsDevicesAllJoynBusAttachmentInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWindowsDevicesAllJoynBusAttachmentInterop> for ::windows::core::IUnknown {
    fn from(value: &IWindowsDevicesAllJoynBusAttachmentInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsDevicesAllJoynBusAttachmentInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsDevicesAllJoynBusAttachmentInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusAttachmentInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut u64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWindowsDevicesAllJoynBusObjectFactoryInterop(pub ::windows::core::IUnknown);
impl IWindowsDevicesAllJoynBusObjectFactoryInterop {
    #[doc = "*Required features: `Win32_System_WinRT_AllJoyn`*"]
    pub unsafe fn CreateFromWin32Handle<T: ::windows::core::Interface>(&self, win32handle: u64) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(win32handle), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    type Vtable = IWindowsDevicesAllJoynBusObjectFactoryInterop_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6174e506_8b95_4e36_95c0_b88fed34938c);
}
impl ::core::convert::From<IWindowsDevicesAllJoynBusObjectFactoryInterop> for ::windows::core::IUnknown {
    fn from(value: IWindowsDevicesAllJoynBusObjectFactoryInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWindowsDevicesAllJoynBusObjectFactoryInterop> for ::windows::core::IUnknown {
    fn from(value: &IWindowsDevicesAllJoynBusObjectFactoryInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsDevicesAllJoynBusObjectFactoryInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusObjectFactoryInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, win32handle: u64, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWindowsDevicesAllJoynBusObjectInterop(pub ::windows::core::IUnknown);
impl IWindowsDevicesAllJoynBusObjectInterop {
    #[doc = "*Required features: `Win32_System_WinRT_AllJoyn`*"]
    pub unsafe fn AddPropertyGetHandler<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, context: *const ::core::ffi::c_void, interfacename: Param1, callback: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(context), interfacename.into_param().abi(), ::core::mem::transmute(callback)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_AllJoyn`*"]
    pub unsafe fn AddPropertySetHandler<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, context: *const ::core::ffi::c_void, interfacename: Param1, callback: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(context), interfacename.into_param().abi(), ::core::mem::transmute(callback)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_AllJoyn`*"]
    pub unsafe fn Win32Handle(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWindowsDevicesAllJoynBusObjectInterop {
    type Vtable = IWindowsDevicesAllJoynBusObjectInterop_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd78aa3d5_5054_428f_99f2_ec3a5de3c3bc);
}
impl ::core::convert::From<IWindowsDevicesAllJoynBusObjectInterop> for ::windows::core::IUnknown {
    fn from(value: IWindowsDevicesAllJoynBusObjectInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWindowsDevicesAllJoynBusObjectInterop> for ::windows::core::IUnknown {
    fn from(value: &IWindowsDevicesAllJoynBusObjectInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsDevicesAllJoynBusObjectInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsDevicesAllJoynBusObjectInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusObjectInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, interfacename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, callback: isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, interfacename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, callback: isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut u64) -> ::windows::core::HRESULT,
);
