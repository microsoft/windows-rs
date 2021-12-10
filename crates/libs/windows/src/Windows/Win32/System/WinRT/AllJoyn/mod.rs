#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_System_WinRT_AllJoyn'*"]
#[repr(transparent)]
pub struct IWindowsDevicesAllJoynBusAttachmentFactoryInterop(::windows::core::IUnknown);
impl IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    #[doc = "*Required features: 'Win32_System_WinRT_AllJoyn'*"]
    pub unsafe fn CreateFromWin32Handle<T: ::windows::core::Interface>(&self, win32handle: u64, enableaboutdata: u8) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(win32handle), ::core::mem::transmute(enableaboutdata), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IWindowsDevicesAllJoynBusAttachmentFactoryInterop> for ::windows::core::IInspectable {
    fn from(value: IWindowsDevicesAllJoynBusAttachmentFactoryInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWindowsDevicesAllJoynBusAttachmentFactoryInterop> for ::windows::core::IInspectable {
    fn from(value: &IWindowsDevicesAllJoynBusAttachmentFactoryInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWindowsDevicesAllJoynBusAttachmentFactoryInterop> for ::windows::core::IUnknown {
    fn from(value: IWindowsDevicesAllJoynBusAttachmentFactoryInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWindowsDevicesAllJoynBusAttachmentFactoryInterop> for ::windows::core::IUnknown {
    fn from(value: &IWindowsDevicesAllJoynBusAttachmentFactoryInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {}
unsafe impl ::windows::core::Interface for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    type Vtable = IWindowsDevicesAllJoynBusAttachmentFactoryInteropVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b8f7505_b239_4e7b_88af_f6682575d861);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusAttachmentFactoryInteropVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, win32handle: u64, enableaboutdata: u8, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_WinRT_AllJoyn'*"]
#[repr(transparent)]
pub struct IWindowsDevicesAllJoynBusAttachmentInterop(::windows::core::IUnknown);
impl IWindowsDevicesAllJoynBusAttachmentInterop {
    #[doc = "*Required features: 'Win32_System_WinRT_AllJoyn'*"]
    pub unsafe fn Win32Handle(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IWindowsDevicesAllJoynBusAttachmentInterop> for ::windows::core::IInspectable {
    fn from(value: IWindowsDevicesAllJoynBusAttachmentInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWindowsDevicesAllJoynBusAttachmentInterop> for ::windows::core::IInspectable {
    fn from(value: &IWindowsDevicesAllJoynBusAttachmentInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWindowsDevicesAllJoynBusAttachmentInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IWindowsDevicesAllJoynBusAttachmentInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWindowsDevicesAllJoynBusAttachmentInterop> for ::windows::core::IUnknown {
    fn from(value: IWindowsDevicesAllJoynBusAttachmentInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWindowsDevicesAllJoynBusAttachmentInterop> for ::windows::core::IUnknown {
    fn from(value: &IWindowsDevicesAllJoynBusAttachmentInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsDevicesAllJoynBusAttachmentInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWindowsDevicesAllJoynBusAttachmentInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWindowsDevicesAllJoynBusAttachmentInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWindowsDevicesAllJoynBusAttachmentInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWindowsDevicesAllJoynBusAttachmentInterop {}
unsafe impl ::windows::core::Interface for IWindowsDevicesAllJoynBusAttachmentInterop {
    type Vtable = IWindowsDevicesAllJoynBusAttachmentInteropVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd89c65b_b50e_4a19_9d0c_b42b783281cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusAttachmentInteropVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_WinRT_AllJoyn'*"]
#[repr(transparent)]
pub struct IWindowsDevicesAllJoynBusObjectFactoryInterop(::windows::core::IUnknown);
impl IWindowsDevicesAllJoynBusObjectFactoryInterop {
    #[doc = "*Required features: 'Win32_System_WinRT_AllJoyn'*"]
    pub unsafe fn CreateFromWin32Handle<T: ::windows::core::Interface>(&self, win32handle: u64) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(win32handle), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IWindowsDevicesAllJoynBusObjectFactoryInterop> for ::windows::core::IInspectable {
    fn from(value: IWindowsDevicesAllJoynBusObjectFactoryInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWindowsDevicesAllJoynBusObjectFactoryInterop> for ::windows::core::IInspectable {
    fn from(value: &IWindowsDevicesAllJoynBusObjectFactoryInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IWindowsDevicesAllJoynBusObjectFactoryInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWindowsDevicesAllJoynBusObjectFactoryInterop> for ::windows::core::IUnknown {
    fn from(value: IWindowsDevicesAllJoynBusObjectFactoryInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWindowsDevicesAllJoynBusObjectFactoryInterop> for ::windows::core::IUnknown {
    fn from(value: &IWindowsDevicesAllJoynBusObjectFactoryInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWindowsDevicesAllJoynBusObjectFactoryInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWindowsDevicesAllJoynBusObjectFactoryInterop {}
unsafe impl ::windows::core::Interface for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    type Vtable = IWindowsDevicesAllJoynBusObjectFactoryInteropVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6174e506_8b95_4e36_95c0_b88fed34938c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusObjectFactoryInteropVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, win32handle: u64, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_WinRT_AllJoyn'*"]
#[repr(transparent)]
pub struct IWindowsDevicesAllJoynBusObjectInterop(::windows::core::IUnknown);
impl IWindowsDevicesAllJoynBusObjectInterop {
    #[doc = "*Required features: 'Win32_System_WinRT_AllJoyn'*"]
    pub unsafe fn AddPropertyGetHandler<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, context: *const ::core::ffi::c_void, interfacename: Param1, callback: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(context), interfacename.into_param().abi(), ::core::mem::transmute(callback)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_AllJoyn'*"]
    pub unsafe fn AddPropertySetHandler<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, context: *const ::core::ffi::c_void, interfacename: Param1, callback: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(context), interfacename.into_param().abi(), ::core::mem::transmute(callback)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_AllJoyn'*"]
    pub unsafe fn Win32Handle(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IWindowsDevicesAllJoynBusObjectInterop> for ::windows::core::IInspectable {
    fn from(value: IWindowsDevicesAllJoynBusObjectInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWindowsDevicesAllJoynBusObjectInterop> for ::windows::core::IInspectable {
    fn from(value: &IWindowsDevicesAllJoynBusObjectInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWindowsDevicesAllJoynBusObjectInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IWindowsDevicesAllJoynBusObjectInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWindowsDevicesAllJoynBusObjectInterop> for ::windows::core::IUnknown {
    fn from(value: IWindowsDevicesAllJoynBusObjectInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWindowsDevicesAllJoynBusObjectInterop> for ::windows::core::IUnknown {
    fn from(value: &IWindowsDevicesAllJoynBusObjectInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsDevicesAllJoynBusObjectInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWindowsDevicesAllJoynBusObjectInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWindowsDevicesAllJoynBusObjectInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWindowsDevicesAllJoynBusObjectInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWindowsDevicesAllJoynBusObjectInterop {}
unsafe impl ::windows::core::Interface for IWindowsDevicesAllJoynBusObjectInterop {
    type Vtable = IWindowsDevicesAllJoynBusObjectInteropVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd78aa3d5_5054_428f_99f2_ec3a5de3c3bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusObjectInteropVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, interfacename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, callback: isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, interfacename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, callback: isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u64) -> ::windows::core::HRESULT,
);
