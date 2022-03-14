#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_WinRT_AllJoyn\"`*"]
#[repr(transparent)]
pub struct IWindowsDevicesAllJoynBusAttachmentFactoryInterop(::windows::core::IUnknown);
impl IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    #[doc = "*Required features: `\"Win32_System_WinRT_AllJoyn\"`*"]
    pub unsafe fn CreateFromWin32Handle<T: ::windows::core::Interface>(&self, win32handle: u64, enableaboutdata: u8) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateFromWin32Handle)(::core::mem::transmute_copy(self), ::core::mem::transmute(win32handle), ::core::mem::transmute(enableaboutdata), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
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
impl ::core::fmt::Debug for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsDevicesAllJoynBusAttachmentFactoryInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    type Vtable = IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b8f7505_b239_4e7b_88af_f6682575d861);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromWin32Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, win32handle: u64, enableaboutdata: u8, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_AllJoyn\"`*"]
#[repr(transparent)]
pub struct IWindowsDevicesAllJoynBusAttachmentInterop(::windows::core::IUnknown);
impl IWindowsDevicesAllJoynBusAttachmentInterop {
    #[doc = "*Required features: `\"Win32_System_WinRT_AllJoyn\"`*"]
    pub unsafe fn Win32Handle(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Win32Handle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsDevicesAllJoynBusAttachmentInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWindowsDevicesAllJoynBusAttachmentInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
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
impl ::core::fmt::Debug for IWindowsDevicesAllJoynBusAttachmentInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsDevicesAllJoynBusAttachmentInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWindowsDevicesAllJoynBusAttachmentInterop {
    type Vtable = IWindowsDevicesAllJoynBusAttachmentInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd89c65b_b50e_4a19_9d0c_b42b783281cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusAttachmentInterop_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Win32Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_AllJoyn\"`*"]
#[repr(transparent)]
pub struct IWindowsDevicesAllJoynBusObjectFactoryInterop(::windows::core::IUnknown);
impl IWindowsDevicesAllJoynBusObjectFactoryInterop {
    #[doc = "*Required features: `\"Win32_System_WinRT_AllJoyn\"`*"]
    pub unsafe fn CreateFromWin32Handle<T: ::windows::core::Interface>(&self, win32handle: u64) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateFromWin32Handle)(::core::mem::transmute_copy(self), ::core::mem::transmute(win32handle), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsDevicesAllJoynBusObjectFactoryInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWindowsDevicesAllJoynBusObjectFactoryInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
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
impl ::core::fmt::Debug for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsDevicesAllJoynBusObjectFactoryInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    type Vtable = IWindowsDevicesAllJoynBusObjectFactoryInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6174e506_8b95_4e36_95c0_b88fed34938c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusObjectFactoryInterop_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromWin32Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, win32handle: u64, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_AllJoyn\"`*"]
#[repr(transparent)]
pub struct IWindowsDevicesAllJoynBusObjectInterop(::windows::core::IUnknown);
impl IWindowsDevicesAllJoynBusObjectInterop {
    #[doc = "*Required features: `\"Win32_System_WinRT_AllJoyn\"`*"]
    pub unsafe fn AddPropertyGetHandler<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, context: *const ::core::ffi::c_void, interfacename: Param1, callback: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddPropertyGetHandler)(::core::mem::transmute_copy(self), ::core::mem::transmute(context), interfacename.into_param().abi(), ::core::mem::transmute(callback)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_AllJoyn\"`*"]
    pub unsafe fn AddPropertySetHandler<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, context: *const ::core::ffi::c_void, interfacename: Param1, callback: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddPropertySetHandler)(::core::mem::transmute_copy(self), ::core::mem::transmute(context), interfacename.into_param().abi(), ::core::mem::transmute(callback)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_AllJoyn\"`*"]
    pub unsafe fn Win32Handle(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Win32Handle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsDevicesAllJoynBusObjectInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWindowsDevicesAllJoynBusObjectInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
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
impl ::core::fmt::Debug for IWindowsDevicesAllJoynBusObjectInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsDevicesAllJoynBusObjectInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWindowsDevicesAllJoynBusObjectInterop {
    type Vtable = IWindowsDevicesAllJoynBusObjectInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd78aa3d5_5054_428f_99f2_ec3a5de3c3bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusObjectInterop_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AddPropertyGetHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, interfacename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, callback: isize) -> ::windows::core::HRESULT,
    pub AddPropertySetHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, interfacename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, callback: isize) -> ::windows::core::HRESULT,
    pub Win32Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u64) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
