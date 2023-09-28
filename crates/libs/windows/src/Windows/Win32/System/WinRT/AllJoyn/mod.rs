#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsDevicesAllJoynBusAttachmentFactoryInterop(::windows_core::IUnknown);
impl IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    pub unsafe fn CreateFromWin32Handle<T>(&self, win32handle: u64, enableaboutdata: u8) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).CreateFromWin32Handle)(::windows_core::Interface::as_raw(self), win32handle, enableaboutdata, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWindowsDevicesAllJoynBusAttachmentFactoryInterop, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::windows_core::Interface for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    type Vtable = IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b8f7505_b239_4e7b_88af_f6682575d861);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromWin32Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, win32handle: u64, enableaboutdata: u8, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsDevicesAllJoynBusAttachmentInterop(::windows_core::IUnknown);
impl IWindowsDevicesAllJoynBusAttachmentInterop {
    pub unsafe fn Win32Handle(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Win32Handle)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWindowsDevicesAllJoynBusAttachmentInterop, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::windows_core::Interface for IWindowsDevicesAllJoynBusAttachmentInterop {
    type Vtable = IWindowsDevicesAllJoynBusAttachmentInterop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsDevicesAllJoynBusAttachmentInterop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd89c65b_b50e_4a19_9d0c_b42b783281cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusAttachmentInterop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Win32Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsDevicesAllJoynBusObjectFactoryInterop(::windows_core::IUnknown);
impl IWindowsDevicesAllJoynBusObjectFactoryInterop {
    pub unsafe fn CreateFromWin32Handle<T>(&self, win32handle: u64) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).CreateFromWin32Handle)(::windows_core::Interface::as_raw(self), win32handle, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWindowsDevicesAllJoynBusObjectFactoryInterop, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::windows_core::Interface for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    type Vtable = IWindowsDevicesAllJoynBusObjectFactoryInterop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6174e506_8b95_4e36_95c0_b88fed34938c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusObjectFactoryInterop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromWin32Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, win32handle: u64, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsDevicesAllJoynBusObjectInterop(::windows_core::IUnknown);
impl IWindowsDevicesAllJoynBusObjectInterop {
    pub unsafe fn AddPropertyGetHandler(&self, context: *const ::core::ffi::c_void, interfacename: &::windows_core::HSTRING, callback: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPropertyGetHandler)(::windows_core::Interface::as_raw(self), context, ::core::mem::transmute_copy(interfacename), callback).ok()
    }
    pub unsafe fn AddPropertySetHandler(&self, context: *const ::core::ffi::c_void, interfacename: &::windows_core::HSTRING, callback: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPropertySetHandler)(::windows_core::Interface::as_raw(self), context, ::core::mem::transmute_copy(interfacename), callback).ok()
    }
    pub unsafe fn Win32Handle(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Win32Handle)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWindowsDevicesAllJoynBusObjectInterop, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::windows_core::Interface for IWindowsDevicesAllJoynBusObjectInterop {
    type Vtable = IWindowsDevicesAllJoynBusObjectInterop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsDevicesAllJoynBusObjectInterop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd78aa3d5_5054_428f_99f2_ec3a5de3c3bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusObjectInterop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AddPropertyGetHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, interfacename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, callback: isize) -> ::windows_core::HRESULT,
    pub AddPropertySetHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, interfacename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, callback: isize) -> ::windows_core::HRESULT,
    pub Win32Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u64) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
