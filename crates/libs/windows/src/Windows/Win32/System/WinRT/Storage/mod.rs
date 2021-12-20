#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub type HANDLE_ACCESS_OPTIONS = u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HAO_NONE: HANDLE_ACCESS_OPTIONS = 0u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HAO_READ_ATTRIBUTES: HANDLE_ACCESS_OPTIONS = 128u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HAO_READ: HANDLE_ACCESS_OPTIONS = 1179785u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HAO_WRITE: HANDLE_ACCESS_OPTIONS = 1179926u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HAO_DELETE: HANDLE_ACCESS_OPTIONS = 65536u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub type HANDLE_CREATION_OPTIONS = i32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HCO_CREATE_NEW: HANDLE_CREATION_OPTIONS = 1i32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HCO_CREATE_ALWAYS: HANDLE_CREATION_OPTIONS = 2i32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HCO_OPEN_EXISTING: HANDLE_CREATION_OPTIONS = 3i32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HCO_OPEN_ALWAYS: HANDLE_CREATION_OPTIONS = 4i32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HCO_TRUNCATE_EXISTING: HANDLE_CREATION_OPTIONS = 5i32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub type HANDLE_OPTIONS = u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HO_NONE: HANDLE_OPTIONS = 0u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HO_OPEN_REQUIRING_OPLOCK: HANDLE_OPTIONS = 262144u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HO_DELETE_ON_CLOSE: HANDLE_OPTIONS = 67108864u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HO_SEQUENTIAL_SCAN: HANDLE_OPTIONS = 134217728u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HO_RANDOM_ACCESS: HANDLE_OPTIONS = 268435456u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HO_NO_BUFFERING: HANDLE_OPTIONS = 536870912u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HO_OVERLAPPED: HANDLE_OPTIONS = 1073741824u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HO_WRITE_THROUGH: HANDLE_OPTIONS = 2147483648u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub type HANDLE_SHARING_OPTIONS = u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HSO_SHARE_NONE: HANDLE_SHARING_OPTIONS = 0u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HSO_SHARE_READ: HANDLE_SHARING_OPTIONS = 1u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HSO_SHARE_WRITE: HANDLE_SHARING_OPTIONS = 2u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
pub const HSO_SHARE_DELETE: HANDLE_SHARING_OPTIONS = 4u32;
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
#[repr(transparent)]
pub struct IOplockBreakingHandler(::windows::core::IUnknown);
impl IOplockBreakingHandler {
    #[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
    pub unsafe fn OplockBreaking(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IOplockBreakingHandler> for ::windows::core::IUnknown {
    fn from(value: IOplockBreakingHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IOplockBreakingHandler> for ::windows::core::IUnknown {
    fn from(value: &IOplockBreakingHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOplockBreakingHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IOplockBreakingHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IOplockBreakingHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOplockBreakingHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOplockBreakingHandler {}
impl ::core::fmt::Debug for IOplockBreakingHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOplockBreakingHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOplockBreakingHandler {
    type Vtable = IOplockBreakingHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x826abe3d_3acd_47d3_84f2_88aaedcf6304);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOplockBreakingHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
#[repr(transparent)]
pub struct IRandomAccessStreamFileAccessMode(::windows::core::IUnknown);
impl IRandomAccessStreamFileAccessMode {
    #[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
    pub unsafe fn GetMode(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IRandomAccessStreamFileAccessMode> for ::windows::core::IUnknown {
    fn from(value: IRandomAccessStreamFileAccessMode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRandomAccessStreamFileAccessMode> for ::windows::core::IUnknown {
    fn from(value: &IRandomAccessStreamFileAccessMode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRandomAccessStreamFileAccessMode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRandomAccessStreamFileAccessMode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRandomAccessStreamFileAccessMode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRandomAccessStreamFileAccessMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRandomAccessStreamFileAccessMode {}
impl ::core::fmt::Debug for IRandomAccessStreamFileAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRandomAccessStreamFileAccessMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRandomAccessStreamFileAccessMode {
    type Vtable = IRandomAccessStreamFileAccessModeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x332e5848_2e15_458e_85c4_c911c0c3d6f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamFileAccessModeVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileaccessmode: *mut u32) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
#[repr(transparent)]
pub struct IStorageFolderHandleAccess(::windows::core::IUnknown);
impl IStorageFolderHandleAccess {
    #[doc = "*Required features: 'Win32_System_WinRT_Storage', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, IOplockBreakingHandler>>(&self, filename: Param0, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: Param5) -> ::windows::core::Result<super::super::super::Foundation::HANDLE> {
        let mut result__: super::super::super::Foundation::HANDLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(creationoptions), ::core::mem::transmute(accessoptions), ::core::mem::transmute(sharingoptions), ::core::mem::transmute(options), oplockbreakinghandler.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::HANDLE>(result__)
    }
}
impl ::core::convert::From<IStorageFolderHandleAccess> for ::windows::core::IUnknown {
    fn from(value: IStorageFolderHandleAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageFolderHandleAccess> for ::windows::core::IUnknown {
    fn from(value: &IStorageFolderHandleAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageFolderHandleAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IStorageFolderHandleAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageFolderHandleAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageFolderHandleAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageFolderHandleAccess {}
impl ::core::fmt::Debug for IStorageFolderHandleAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageFolderHandleAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IStorageFolderHandleAccess {
    type Vtable = IStorageFolderHandleAccessVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf19938f_5462_48a0_be65_d2a3271a08d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderHandleAccessVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::windows::core::RawPtr, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
#[repr(transparent)]
pub struct IStorageItemHandleAccess(::windows::core::IUnknown);
impl IStorageItemHandleAccess {
    #[doc = "*Required features: 'Win32_System_WinRT_Storage', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, Param3: ::windows::core::IntoParam<'a, IOplockBreakingHandler>>(&self, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: Param3) -> ::windows::core::Result<super::super::super::Foundation::HANDLE> {
        let mut result__: super::super::super::Foundation::HANDLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(accessoptions), ::core::mem::transmute(sharingoptions), ::core::mem::transmute(options), oplockbreakinghandler.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::HANDLE>(result__)
    }
}
impl ::core::convert::From<IStorageItemHandleAccess> for ::windows::core::IUnknown {
    fn from(value: IStorageItemHandleAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItemHandleAccess> for ::windows::core::IUnknown {
    fn from(value: &IStorageItemHandleAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageItemHandleAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IStorageItemHandleAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageItemHandleAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageItemHandleAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageItemHandleAccess {}
impl ::core::fmt::Debug for IStorageItemHandleAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageItemHandleAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IStorageItemHandleAccess {
    type Vtable = IStorageItemHandleAccessVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ca296b2_2c25_4d22_b785_b885c8201e6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemHandleAccessVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::windows::core::RawPtr, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
#[repr(transparent)]
pub struct IUnbufferedFileHandleOplockCallback(::windows::core::IUnknown);
impl IUnbufferedFileHandleOplockCallback {
    #[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
    pub unsafe fn OnBrokenCallback(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IUnbufferedFileHandleOplockCallback> for ::windows::core::IUnknown {
    fn from(value: IUnbufferedFileHandleOplockCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUnbufferedFileHandleOplockCallback> for ::windows::core::IUnknown {
    fn from(value: &IUnbufferedFileHandleOplockCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUnbufferedFileHandleOplockCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUnbufferedFileHandleOplockCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUnbufferedFileHandleOplockCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUnbufferedFileHandleOplockCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUnbufferedFileHandleOplockCallback {}
impl ::core::fmt::Debug for IUnbufferedFileHandleOplockCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUnbufferedFileHandleOplockCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUnbufferedFileHandleOplockCallback {
    type Vtable = IUnbufferedFileHandleOplockCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1019a0e_6243_4329_8497_2e75894d7710);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnbufferedFileHandleOplockCallbackVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
#[repr(transparent)]
pub struct IUnbufferedFileHandleProvider(::windows::core::IUnknown);
impl IUnbufferedFileHandleProvider {
    #[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
    pub unsafe fn OpenUnbufferedFileHandle<'a, Param0: ::windows::core::IntoParam<'a, IUnbufferedFileHandleOplockCallback>>(&self, oplockbreakcallback: Param0) -> ::windows::core::Result<usize> {
        let mut result__: usize = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), oplockbreakcallback.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<usize>(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Storage'*"]
    pub unsafe fn CloseUnbufferedFileHandle(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IUnbufferedFileHandleProvider> for ::windows::core::IUnknown {
    fn from(value: IUnbufferedFileHandleProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUnbufferedFileHandleProvider> for ::windows::core::IUnknown {
    fn from(value: &IUnbufferedFileHandleProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUnbufferedFileHandleProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUnbufferedFileHandleProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUnbufferedFileHandleProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUnbufferedFileHandleProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUnbufferedFileHandleProvider {}
impl ::core::fmt::Debug for IUnbufferedFileHandleProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUnbufferedFileHandleProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUnbufferedFileHandleProvider {
    type Vtable = IUnbufferedFileHandleProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa65c9109_42ab_4b94_a7b1_dd2e4e68515e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnbufferedFileHandleProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oplockbreakcallback: ::windows::core::RawPtr, filehandle: *mut usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
