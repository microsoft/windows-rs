#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HANDLE_ACCESS_OPTIONS(pub u32);
pub const HAO_NONE: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(0u32);
pub const HAO_READ_ATTRIBUTES: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(128u32);
pub const HAO_READ: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(1179785u32);
pub const HAO_WRITE: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(1179926u32);
pub const HAO_DELETE: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(65536u32);
impl ::core::convert::From<u32> for HANDLE_ACCESS_OPTIONS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for HANDLE_ACCESS_OPTIONS {
    type Abi = Self;
}
impl ::core::ops::BitOr for HANDLE_ACCESS_OPTIONS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for HANDLE_ACCESS_OPTIONS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for HANDLE_ACCESS_OPTIONS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for HANDLE_ACCESS_OPTIONS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for HANDLE_ACCESS_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HANDLE_CREATION_OPTIONS(pub i32);
pub const HCO_CREATE_NEW: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(1i32);
pub const HCO_CREATE_ALWAYS: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(2i32);
pub const HCO_OPEN_EXISTING: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(3i32);
pub const HCO_OPEN_ALWAYS: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(4i32);
pub const HCO_TRUNCATE_EXISTING: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(5i32);
impl ::core::convert::From<i32> for HANDLE_CREATION_OPTIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for HANDLE_CREATION_OPTIONS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HANDLE_OPTIONS(pub u32);
pub const HO_NONE: HANDLE_OPTIONS = HANDLE_OPTIONS(0u32);
pub const HO_OPEN_REQUIRING_OPLOCK: HANDLE_OPTIONS = HANDLE_OPTIONS(262144u32);
pub const HO_DELETE_ON_CLOSE: HANDLE_OPTIONS = HANDLE_OPTIONS(67108864u32);
pub const HO_SEQUENTIAL_SCAN: HANDLE_OPTIONS = HANDLE_OPTIONS(134217728u32);
pub const HO_RANDOM_ACCESS: HANDLE_OPTIONS = HANDLE_OPTIONS(268435456u32);
pub const HO_NO_BUFFERING: HANDLE_OPTIONS = HANDLE_OPTIONS(536870912u32);
pub const HO_OVERLAPPED: HANDLE_OPTIONS = HANDLE_OPTIONS(1073741824u32);
pub const HO_WRITE_THROUGH: HANDLE_OPTIONS = HANDLE_OPTIONS(2147483648u32);
impl ::core::convert::From<u32> for HANDLE_OPTIONS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for HANDLE_OPTIONS {
    type Abi = Self;
}
impl ::core::ops::BitOr for HANDLE_OPTIONS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for HANDLE_OPTIONS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for HANDLE_OPTIONS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for HANDLE_OPTIONS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for HANDLE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HANDLE_SHARING_OPTIONS(pub u32);
pub const HSO_SHARE_NONE: HANDLE_SHARING_OPTIONS = HANDLE_SHARING_OPTIONS(0u32);
pub const HSO_SHARE_READ: HANDLE_SHARING_OPTIONS = HANDLE_SHARING_OPTIONS(1u32);
pub const HSO_SHARE_WRITE: HANDLE_SHARING_OPTIONS = HANDLE_SHARING_OPTIONS(2u32);
pub const HSO_SHARE_DELETE: HANDLE_SHARING_OPTIONS = HANDLE_SHARING_OPTIONS(4u32);
impl ::core::convert::From<u32> for HANDLE_SHARING_OPTIONS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for HANDLE_SHARING_OPTIONS {
    type Abi = Self;
}
impl ::core::ops::BitOr for HANDLE_SHARING_OPTIONS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for HANDLE_SHARING_OPTIONS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for HANDLE_SHARING_OPTIONS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for HANDLE_SHARING_OPTIONS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for HANDLE_SHARING_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOplockBreakingHandler(pub ::windows::core::IUnknown);
impl IOplockBreakingHandler {
    pub unsafe fn OplockBreaking(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IOplockBreakingHandler {
    type Vtable = IOplockBreakingHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x826abe3d_3acd_47d3_84f2_88aaedcf6304);
}
impl ::core::convert::From<IOplockBreakingHandler> for ::windows::core::IUnknown {
    fn from(value: IOplockBreakingHandler) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOplockBreakingHandler> for ::windows::core::IUnknown {
    fn from(value: &IOplockBreakingHandler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOplockBreakingHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOplockBreakingHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOplockBreakingHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRandomAccessStreamFileAccessMode(pub ::windows::core::IUnknown);
impl IRandomAccessStreamFileAccessMode {
    pub unsafe fn GetMode(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRandomAccessStreamFileAccessMode {
    type Vtable = IRandomAccessStreamFileAccessMode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x332e5848_2e15_458e_85c4_c911c0c3d6f4);
}
impl ::core::convert::From<IRandomAccessStreamFileAccessMode> for ::windows::core::IUnknown {
    fn from(value: IRandomAccessStreamFileAccessMode) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRandomAccessStreamFileAccessMode> for ::windows::core::IUnknown {
    fn from(value: &IRandomAccessStreamFileAccessMode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRandomAccessStreamFileAccessMode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRandomAccessStreamFileAccessMode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamFileAccessMode_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fileaccessmode: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IStorageFolderHandleAccess(pub ::windows::core::IUnknown);
impl IStorageFolderHandleAccess {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, IOplockBreakingHandler>>(&self, filename: Param0, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: Param5) -> ::windows::core::Result<super::super::super::Foundation::HANDLE> {
        let mut result__: <super::super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(creationoptions), ::core::mem::transmute(accessoptions), ::core::mem::transmute(sharingoptions), ::core::mem::transmute(options), oplockbreakinghandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::HANDLE>(result__)
    }
}
unsafe impl ::windows::core::Interface for IStorageFolderHandleAccess {
    type Vtable = IStorageFolderHandleAccess_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf19938f_5462_48a0_be65_d2a3271a08d6);
}
impl ::core::convert::From<IStorageFolderHandleAccess> for ::windows::core::IUnknown {
    fn from(value: IStorageFolderHandleAccess) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageFolderHandleAccess> for ::windows::core::IUnknown {
    fn from(value: &IStorageFolderHandleAccess) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageFolderHandleAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageFolderHandleAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderHandleAccess_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: super::super::super::Foundation::PWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::windows::core::RawPtr, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IStorageItemHandleAccess(pub ::windows::core::IUnknown);
impl IStorageItemHandleAccess {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, Param3: ::windows::core::IntoParam<'a, IOplockBreakingHandler>>(&self, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: Param3) -> ::windows::core::Result<super::super::super::Foundation::HANDLE> {
        let mut result__: <super::super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(accessoptions), ::core::mem::transmute(sharingoptions), ::core::mem::transmute(options), oplockbreakinghandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::HANDLE>(result__)
    }
}
unsafe impl ::windows::core::Interface for IStorageItemHandleAccess {
    type Vtable = IStorageItemHandleAccess_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ca296b2_2c25_4d22_b785_b885c8201e6a);
}
impl ::core::convert::From<IStorageItemHandleAccess> for ::windows::core::IUnknown {
    fn from(value: IStorageItemHandleAccess) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageItemHandleAccess> for ::windows::core::IUnknown {
    fn from(value: &IStorageItemHandleAccess) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageItemHandleAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageItemHandleAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemHandleAccess_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::windows::core::RawPtr, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUnbufferedFileHandleOplockCallback(pub ::windows::core::IUnknown);
impl IUnbufferedFileHandleOplockCallback {
    pub unsafe fn OnBrokenCallback(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IUnbufferedFileHandleOplockCallback {
    type Vtable = IUnbufferedFileHandleOplockCallback_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1019a0e_6243_4329_8497_2e75894d7710);
}
impl ::core::convert::From<IUnbufferedFileHandleOplockCallback> for ::windows::core::IUnknown {
    fn from(value: IUnbufferedFileHandleOplockCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUnbufferedFileHandleOplockCallback> for ::windows::core::IUnknown {
    fn from(value: &IUnbufferedFileHandleOplockCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUnbufferedFileHandleOplockCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUnbufferedFileHandleOplockCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnbufferedFileHandleOplockCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUnbufferedFileHandleProvider(pub ::windows::core::IUnknown);
impl IUnbufferedFileHandleProvider {
    pub unsafe fn OpenUnbufferedFileHandle<'a, Param0: ::windows::core::IntoParam<'a, IUnbufferedFileHandleOplockCallback>>(&self, oplockbreakcallback: Param0) -> ::windows::core::Result<usize> {
        let mut result__: <usize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), oplockbreakcallback.into_param().abi(), &mut result__).from_abi::<usize>(result__)
    }
    pub unsafe fn CloseUnbufferedFileHandle(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IUnbufferedFileHandleProvider {
    type Vtable = IUnbufferedFileHandleProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa65c9109_42ab_4b94_a7b1_dd2e4e68515e);
}
impl ::core::convert::From<IUnbufferedFileHandleProvider> for ::windows::core::IUnknown {
    fn from(value: IUnbufferedFileHandleProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUnbufferedFileHandleProvider> for ::windows::core::IUnknown {
    fn from(value: &IUnbufferedFileHandleProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUnbufferedFileHandleProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUnbufferedFileHandleProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnbufferedFileHandleProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, oplockbreakcallback: ::windows::core::RawPtr, filehandle: *mut usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
