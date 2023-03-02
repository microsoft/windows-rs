#[doc = "*Required features: `\"Win32_System_WinRT_AllJoyn\"`*"]
#[repr(transparent)]
pub struct IWindowsDevicesAllJoynBusAttachmentFactoryInterop(::windows::core::IUnknown);
impl IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    pub unsafe fn CreateFromWin32Handle<T>(&self, win32handle: u64, enableaboutdata: u8) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).CreateFromWin32Handle)(::windows::core::Interface::as_raw(self), win32handle, enableaboutdata, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWindowsDevicesAllJoynBusAttachmentFactoryInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
}
impl ::core::clone::Clone for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b8f7505_b239_4e7b_88af_f6682575d861);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromWin32Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, win32handle: u64, enableaboutdata: u8, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_AllJoyn\"`*"]
#[repr(transparent)]
pub struct IWindowsDevicesAllJoynBusAttachmentInterop(::windows::core::IUnknown);
impl IWindowsDevicesAllJoynBusAttachmentInterop {
    pub unsafe fn Win32Handle(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).Win32Handle)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWindowsDevicesAllJoynBusAttachmentInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
}
impl ::core::clone::Clone for IWindowsDevicesAllJoynBusAttachmentInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWindowsDevicesAllJoynBusAttachmentInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd89c65b_b50e_4a19_9d0c_b42b783281cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusAttachmentInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Win32Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_AllJoyn\"`*"]
#[repr(transparent)]
pub struct IWindowsDevicesAllJoynBusObjectFactoryInterop(::windows::core::IUnknown);
impl IWindowsDevicesAllJoynBusObjectFactoryInterop {
    pub unsafe fn CreateFromWin32Handle<T>(&self, win32handle: u64) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).CreateFromWin32Handle)(::windows::core::Interface::as_raw(self), win32handle, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWindowsDevicesAllJoynBusObjectFactoryInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
}
impl ::core::clone::Clone for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6174e506_8b95_4e36_95c0_b88fed34938c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusObjectFactoryInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromWin32Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, win32handle: u64, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_AllJoyn\"`*"]
#[repr(transparent)]
pub struct IWindowsDevicesAllJoynBusObjectInterop(::windows::core::IUnknown);
impl IWindowsDevicesAllJoynBusObjectInterop {
    pub unsafe fn AddPropertyGetHandler(&self, context: *const ::core::ffi::c_void, interfacename: &::windows::core::HSTRING, callback: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddPropertyGetHandler)(::windows::core::Interface::as_raw(self), context, ::core::mem::transmute_copy(interfacename), callback).ok()
    }
    pub unsafe fn AddPropertySetHandler(&self, context: *const ::core::ffi::c_void, interfacename: &::windows::core::HSTRING, callback: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddPropertySetHandler)(::windows::core::Interface::as_raw(self), context, ::core::mem::transmute_copy(interfacename), callback).ok()
    }
    pub unsafe fn Win32Handle(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).Win32Handle)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWindowsDevicesAllJoynBusObjectInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
}
impl ::core::clone::Clone for IWindowsDevicesAllJoynBusObjectInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWindowsDevicesAllJoynBusObjectInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd78aa3d5_5054_428f_99f2_ec3a5de3c3bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusObjectInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AddPropertyGetHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, interfacename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, callback: isize) -> ::windows::core::HRESULT,
    pub AddPropertySetHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, interfacename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, callback: isize) -> ::windows::core::HRESULT,
    pub Win32Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u64) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
