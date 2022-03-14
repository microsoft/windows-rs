#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckGamingPrivilegeSilently<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(privilegeid: u32, scope: Param1, policy: Param2) -> ::windows::core::Result<super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckGamingPrivilegeSilently(privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, hasprivilege: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: super::Foundation::BOOL = ::core::mem::zeroed();
        CheckGamingPrivilegeSilently(::core::mem::transmute(privilegeid), scope.into_param().abi(), policy.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckGamingPrivilegeSilentlyForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, privilegeid: u32, scope: Param2, policy: Param3) -> ::windows::core::Result<super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckGamingPrivilegeSilentlyForUser(user: *mut ::core::ffi::c_void, privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, hasprivilege: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: super::Foundation::BOOL = ::core::mem::zeroed();
        CheckGamingPrivilegeSilentlyForUser(user.into_param().abi(), ::core::mem::transmute(privilegeid), scope.into_param().abi(), policy.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn CheckGamingPrivilegeWithUI<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(privilegeid: u32, scope: Param1, policy: Param2, friendlymessage: Param3, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckGamingPrivilegeWithUI(privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, friendlymessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CheckGamingPrivilegeWithUI(::core::mem::transmute(privilegeid), scope.into_param().abi(), policy.into_param().abi(), friendlymessage.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn CheckGamingPrivilegeWithUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, privilegeid: u32, scope: Param2, policy: Param3, friendlymessage: Param4, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckGamingPrivilegeWithUIForUser(user: *mut ::core::ffi::c_void, privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, friendlymessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CheckGamingPrivilegeWithUIForUser(user.into_param().abi(), ::core::mem::transmute(privilegeid), scope.into_param().abi(), policy.into_param().abi(), friendlymessage.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GAMESTATS_OPEN_RESULT(pub i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMESTATS_OPEN_CREATED: GAMESTATS_OPEN_RESULT = GAMESTATS_OPEN_RESULT(0i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMESTATS_OPEN_OPENED: GAMESTATS_OPEN_RESULT = GAMESTATS_OPEN_RESULT(1i32);
impl ::core::marker::Copy for GAMESTATS_OPEN_RESULT {}
impl ::core::clone::Clone for GAMESTATS_OPEN_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GAMESTATS_OPEN_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GAMESTATS_OPEN_RESULT {
    type Abi = Self;
}
impl ::core::fmt::Debug for GAMESTATS_OPEN_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GAMESTATS_OPEN_RESULT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GAMESTATS_OPEN_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMESTATS_OPEN_OPENORCREATE: GAMESTATS_OPEN_TYPE = GAMESTATS_OPEN_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMESTATS_OPEN_OPENONLY: GAMESTATS_OPEN_TYPE = GAMESTATS_OPEN_TYPE(1i32);
impl ::core::marker::Copy for GAMESTATS_OPEN_TYPE {}
impl ::core::clone::Clone for GAMESTATS_OPEN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GAMESTATS_OPEN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GAMESTATS_OPEN_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for GAMESTATS_OPEN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GAMESTATS_OPEN_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GAME_INSTALL_SCOPE(pub i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GIS_NOT_INSTALLED: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(1i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GIS_CURRENT_USER: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(2i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GIS_ALL_USERS: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(3i32);
impl ::core::marker::Copy for GAME_INSTALL_SCOPE {}
impl ::core::clone::Clone for GAME_INSTALL_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GAME_INSTALL_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GAME_INSTALL_SCOPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for GAME_INSTALL_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GAME_INSTALL_SCOPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GAMING_DEVICE_DEVICE_ID(pub i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_DEVICE_ID_NONE: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(0i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(1988865574i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_S: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(712204761i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(1523980231i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X_DEVKIT: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(284675555i32);
impl ::core::marker::Copy for GAMING_DEVICE_DEVICE_ID {}
impl ::core::clone::Clone for GAMING_DEVICE_DEVICE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GAMING_DEVICE_DEVICE_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GAMING_DEVICE_DEVICE_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for GAMING_DEVICE_DEVICE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GAMING_DEVICE_DEVICE_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub struct GAMING_DEVICE_MODEL_INFORMATION {
    pub vendorId: GAMING_DEVICE_VENDOR_ID,
    pub deviceId: GAMING_DEVICE_DEVICE_ID,
}
impl ::core::marker::Copy for GAMING_DEVICE_MODEL_INFORMATION {}
impl ::core::clone::Clone for GAMING_DEVICE_MODEL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GAMING_DEVICE_MODEL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GAMING_DEVICE_MODEL_INFORMATION").field("vendorId", &self.vendorId).field("deviceId", &self.deviceId).finish()
    }
}
unsafe impl ::windows::core::Abi for GAMING_DEVICE_MODEL_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GAMING_DEVICE_MODEL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GAMING_DEVICE_MODEL_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for GAMING_DEVICE_MODEL_INFORMATION {}
impl ::core::default::Default for GAMING_DEVICE_MODEL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GAMING_DEVICE_VENDOR_ID(pub i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_VENDOR_ID_NONE: GAMING_DEVICE_VENDOR_ID = GAMING_DEVICE_VENDOR_ID(0i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_VENDOR_ID_MICROSOFT: GAMING_DEVICE_VENDOR_ID = GAMING_DEVICE_VENDOR_ID(-1024700366i32);
impl ::core::marker::Copy for GAMING_DEVICE_VENDOR_ID {}
impl ::core::clone::Clone for GAMING_DEVICE_VENDOR_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GAMING_DEVICE_VENDOR_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GAMING_DEVICE_VENDOR_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for GAMING_DEVICE_VENDOR_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GAMING_DEVICE_VENDOR_ID").field(&self.0).finish()
    }
}
pub const GameExplorer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a5ea990_3034_4d6f_9128_01f3c61022bc);
pub const GameStatistics: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbc85a2c_c0dc_4961_b6e2_d28b62c11ad4);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub type GameUICompletionRoutine = ::core::option::Option<unsafe extern "system" fn(returncode: ::windows::core::HRESULT, context: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn GetExpandedResourceExclusiveCpuCount() -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetExpandedResourceExclusiveCpuCount(exclusivecpucount: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        GetExpandedResourceExclusiveCpuCount(::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn GetGamingDeviceModelInformation() -> ::windows::core::Result<GAMING_DEVICE_MODEL_INFORMATION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGamingDeviceModelInformation(information: *mut GAMING_DEVICE_MODEL_INFORMATION) -> ::windows::core::HRESULT;
        }
        let mut result__: GAMING_DEVICE_MODEL_INFORMATION = ::core::mem::zeroed();
        GetGamingDeviceModelInformation(::core::mem::transmute(&mut result__)).from_abi::<GAMING_DEVICE_MODEL_INFORMATION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HasExpandedResources() -> ::windows::core::Result<super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HasExpandedResources(hasexpandedresources: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: super::Foundation::BOOL = ::core::mem::zeroed();
        HasExpandedResources(::core::mem::transmute(&mut result__)).from_abi::<super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const ID_GDF_THUMBNAIL_STR: &'static str = "__GDF_THUMBNAIL";
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const ID_GDF_XML_STR: &'static str = "__GDF_XML";
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
pub struct IGameExplorer(::windows::core::IUnknown);
impl IGameExplorer {
    #[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddGame<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::Foundation::BSTR>>(&self, bstrgdfbinarypath: Param0, bstrgameinstalldirectory: Param1, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddGame)(::core::mem::transmute_copy(self), bstrgdfbinarypath.into_param().abi(), bstrgameinstalldirectory.into_param().abi(), ::core::mem::transmute(installscope), ::core::mem::transmute(pguidinstanceid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn RemoveGame<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidinstanceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveGame)(::core::mem::transmute_copy(self), guidinstanceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn UpdateGame<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidinstanceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateGame)(::core::mem::transmute_copy(self), guidinstanceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VerifyAccess<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::BSTR>>(&self, bstrgdfbinarypath: Param0) -> ::windows::core::Result<super::Foundation::BOOL> {
        let mut result__: super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).VerifyAccess)(::core::mem::transmute_copy(self), bstrgdfbinarypath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IGameExplorer> for ::windows::core::IUnknown {
    fn from(value: IGameExplorer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameExplorer> for ::windows::core::IUnknown {
    fn from(value: &IGameExplorer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGameExplorer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGameExplorer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGameExplorer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGameExplorer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameExplorer {}
impl ::core::fmt::Debug for IGameExplorer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameExplorer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGameExplorer {
    type Vtable = IGameExplorer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7b2fb72_d728_49b3_a5f2_18ebf5f1349e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameExplorer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AddGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgdfbinarypath: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, bstrgameinstalldirectory: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddGame: usize,
    pub RemoveGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidinstanceid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub UpdateGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidinstanceid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub VerifyAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgdfbinarypath: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, pfhasaccess: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VerifyAccess: usize,
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
pub struct IGameExplorer2(::windows::core::IUnknown);
impl IGameExplorer2 {
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn InstallGame<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, binarygdfpath: Param0, installdirectory: Param1, installscope: GAME_INSTALL_SCOPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InstallGame)(::core::mem::transmute_copy(self), binarygdfpath.into_param().abi(), installdirectory.into_param().abi(), ::core::mem::transmute(installscope)).ok()
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn UninstallGame<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, binarygdfpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UninstallGame)(::core::mem::transmute_copy(self), binarygdfpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckAccess<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, binarygdfpath: Param0) -> ::windows::core::Result<super::Foundation::BOOL> {
        let mut result__: super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CheckAccess)(::core::mem::transmute_copy(self), binarygdfpath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IGameExplorer2> for ::windows::core::IUnknown {
    fn from(value: IGameExplorer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameExplorer2> for ::windows::core::IUnknown {
    fn from(value: &IGameExplorer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGameExplorer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGameExplorer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGameExplorer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGameExplorer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameExplorer2 {}
impl ::core::fmt::Debug for IGameExplorer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameExplorer2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGameExplorer2 {
    type Vtable = IGameExplorer2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86874aa7_a1ed_450d_a7eb_b89e20b2fff3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameExplorer2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub InstallGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binarygdfpath: ::windows::core::PCWSTR, installdirectory: ::windows::core::PCWSTR, installscope: GAME_INSTALL_SCOPE) -> ::windows::core::HRESULT,
    pub UninstallGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binarygdfpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CheckAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binarygdfpath: ::windows::core::PCWSTR, phasaccess: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CheckAccess: usize,
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
pub struct IGameStatistics(::windows::core::IUnknown);
impl IGameStatistics {
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetMaxCategoryLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxCategoryLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetMaxNameLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxNameLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetMaxValueLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxValueLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetMaxCategories(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxCategories)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetMaxStatsPerCategory(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxStatsPerCategory)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn SetCategoryTitle<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, categoryindex: u16, title: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCategoryTitle)(::core::mem::transmute_copy(self), ::core::mem::transmute(categoryindex), title.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetCategoryTitle(&self, categoryindex: u16) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCategoryTitle)(::core::mem::transmute_copy(self), ::core::mem::transmute(categoryindex), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetStatistic(&self, categoryindex: u16, statindex: u16, pname: *mut ::windows::core::PWSTR, pvalue: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatistic)(::core::mem::transmute_copy(self), ::core::mem::transmute(categoryindex), ::core::mem::transmute(statindex), ::core::mem::transmute(pname), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn SetStatistic<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, categoryindex: u16, statindex: u16, name: Param2, value: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStatistic)(::core::mem::transmute_copy(self), ::core::mem::transmute(categoryindex), ::core::mem::transmute(statindex), name.into_param().abi(), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::BOOL>>(&self, trackchanges: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self), trackchanges.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn SetLastPlayedCategory(&self, categoryindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLastPlayedCategory)(::core::mem::transmute_copy(self), ::core::mem::transmute(categoryindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetLastPlayedCategory(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLastPlayedCategory)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IGameStatistics> for ::windows::core::IUnknown {
    fn from(value: IGameStatistics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameStatistics> for ::windows::core::IUnknown {
    fn from(value: &IGameStatistics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGameStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGameStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGameStatistics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGameStatistics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameStatistics {}
impl ::core::fmt::Debug for IGameStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameStatistics").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGameStatistics {
    type Vtable = IGameStatistics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3887c9ca_04a0_42ae_bc4c_5fa6c7721145);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameStatistics_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetMaxCategoryLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows::core::HRESULT,
    pub GetMaxNameLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows::core::HRESULT,
    pub GetMaxValueLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows::core::HRESULT,
    pub GetMaxCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmax: *mut u16) -> ::windows::core::HRESULT,
    pub GetMaxStatsPerCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmax: *mut u16) -> ::windows::core::HRESULT,
    pub SetCategoryTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, categoryindex: u16, title: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetCategoryTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, categoryindex: u16, ptitle: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetStatistic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, categoryindex: u16, statindex: u16, pname: *mut ::windows::core::PWSTR, pvalue: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetStatistic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, categoryindex: u16, statindex: u16, name: ::windows::core::PCWSTR, value: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, trackchanges: super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub SetLastPlayedCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, categoryindex: u32) -> ::windows::core::HRESULT,
    pub GetLastPlayedCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcategoryindex: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
pub struct IGameStatisticsMgr(::windows::core::IUnknown);
impl IGameStatisticsMgr {
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetGameStatistics<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, gdfbinarypath: Param0, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut ::core::option::Option<IGameStatistics>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGameStatistics)(::core::mem::transmute_copy(self), gdfbinarypath.into_param().abi(), ::core::mem::transmute(opentype), ::core::mem::transmute(popenresult), ::core::mem::transmute(ppistats)).ok()
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn RemoveGameStatistics<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, gdfbinarypath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveGameStatistics)(::core::mem::transmute_copy(self), gdfbinarypath.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IGameStatisticsMgr> for ::windows::core::IUnknown {
    fn from(value: IGameStatisticsMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameStatisticsMgr> for ::windows::core::IUnknown {
    fn from(value: &IGameStatisticsMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGameStatisticsMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGameStatisticsMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGameStatisticsMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGameStatisticsMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameStatisticsMgr {}
impl ::core::fmt::Debug for IGameStatisticsMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameStatisticsMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGameStatisticsMgr {
    type Vtable = IGameStatisticsMgr_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaff3ea11_e70e_407d_95dd_35e612c41ce2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameStatisticsMgr_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetGameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gdfbinarypath: ::windows::core::PCWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveGameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gdfbinarypath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
pub struct IXblIdpAuthManager(::windows::core::IUnknown);
impl IXblIdpAuthManager {
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn SetGamerAccount<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, msaaccountid: Param0, xuid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGamerAccount)(::core::mem::transmute_copy(self), msaaccountid.into_param().abi(), xuid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetGamerAccount(&self, msaaccountid: *mut ::windows::core::PWSTR, xuid: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGamerAccount)(::core::mem::transmute_copy(self), ::core::mem::transmute(msaaccountid), ::core::mem::transmute(xuid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn SetAppViewInitialized<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, appsid: Param0, msaaccountid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAppViewInitialized)(::core::mem::transmute_copy(self), appsid.into_param().abi(), msaaccountid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetEnvironment(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEnvironment)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetSandbox(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSandbox)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTokenAndSignatureWithTokenResult<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param6: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param9: ::windows::core::IntoParam<'a, super::Foundation::BOOL>>(&self, msaaccountid: Param0, appsid: Param1, msatarget: Param2, msapolicy: Param3, httpmethod: Param4, uri: Param5, headers: Param6, body: &[u8], forcerefresh: Param9) -> ::windows::core::Result<IXblIdpAuthTokenResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTokenAndSignatureWithTokenResult)(::core::mem::transmute_copy(self), msaaccountid.into_param().abi(), appsid.into_param().abi(), msatarget.into_param().abi(), msapolicy.into_param().abi(), httpmethod.into_param().abi(), uri.into_param().abi(), headers.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(body)), body.len() as _, forcerefresh.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IXblIdpAuthTokenResult>(result__)
    }
}
impl ::core::convert::From<IXblIdpAuthManager> for ::windows::core::IUnknown {
    fn from(value: IXblIdpAuthManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXblIdpAuthManager> for ::windows::core::IUnknown {
    fn from(value: &IXblIdpAuthManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXblIdpAuthManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXblIdpAuthManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXblIdpAuthManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXblIdpAuthManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXblIdpAuthManager {}
impl ::core::fmt::Debug for IXblIdpAuthManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXblIdpAuthManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXblIdpAuthManager {
    type Vtable = IXblIdpAuthManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb5ddb08_8bbf_449b_ac21_b02ddeb3b136);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetGamerAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msaaccountid: ::windows::core::PCWSTR, xuid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetGamerAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msaaccountid: *mut ::windows::core::PWSTR, xuid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetAppViewInitialized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appsid: ::windows::core::PCWSTR, msaaccountid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environment: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetSandbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sandbox: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTokenAndSignatureWithTokenResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msaaccountid: ::windows::core::PCWSTR, appsid: ::windows::core::PCWSTR, msatarget: ::windows::core::PCWSTR, msapolicy: ::windows::core::PCWSTR, httpmethod: ::windows::core::PCWSTR, uri: ::windows::core::PCWSTR, headers: ::windows::core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTokenAndSignatureWithTokenResult: usize,
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
pub struct IXblIdpAuthTokenResult(::windows::core::IUnknown);
impl IXblIdpAuthTokenResult {
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<XBL_IDP_AUTH_TOKEN_STATUS> {
        let mut result__: XBL_IDP_AUTH_TOKEN_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<XBL_IDP_AUTH_TOKEN_STATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetErrorCode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::HRESULT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetToken(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetToken)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetSignature(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSignature)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetSandbox(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSandbox)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetEnvironment(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEnvironment)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetMsaAccountId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMsaAccountId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetXuid(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetXuid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetGamertag(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGamertag)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetAgeGroup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAgeGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetPrivileges(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPrivileges)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetMsaTarget(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMsaTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetMsaPolicy(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMsaPolicy)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetMsaAppId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMsaAppId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetRedirect(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRedirect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetMessage(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMessage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetHelpId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetHelpId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetEnforcementBans(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEnforcementBans)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetRestrictions(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRestrictions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetTitleRestrictions(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTitleRestrictions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IXblIdpAuthTokenResult> for ::windows::core::IUnknown {
    fn from(value: IXblIdpAuthTokenResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXblIdpAuthTokenResult> for ::windows::core::IUnknown {
    fn from(value: &IXblIdpAuthTokenResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXblIdpAuthTokenResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXblIdpAuthTokenResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXblIdpAuthTokenResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXblIdpAuthTokenResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXblIdpAuthTokenResult {}
impl ::core::fmt::Debug for IXblIdpAuthTokenResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXblIdpAuthTokenResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXblIdpAuthTokenResult {
    type Vtable = IXblIdpAuthTokenResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46ce0225_f267_4d68_b299_b2762552dec1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthTokenResult_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut XBL_IDP_AUTH_TOKEN_STATUS) -> ::windows::core::HRESULT,
    pub GetErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub GetToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signature: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetSandbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sandbox: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environment: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetMsaAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msaaccountid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetXuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xuid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetGamertag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamertag: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetAgeGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, agegroup: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetPrivileges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, privileges: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetMsaTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msatarget: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetMsaPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msapolicy: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetMsaAppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msaappid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetRedirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, redirect: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetHelpId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, helpid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetEnforcementBans: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enforcementbans: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restrictions: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetTitleRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, titlerestrictions: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
pub struct IXblIdpAuthTokenResult2(::windows::core::IUnknown);
impl IXblIdpAuthTokenResult2 {
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetModernGamertag(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetModernGamertag)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetModernGamertagSuffix(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetModernGamertagSuffix)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub unsafe fn GetUniqueModernGamertag(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUniqueModernGamertag)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IXblIdpAuthTokenResult2> for ::windows::core::IUnknown {
    fn from(value: IXblIdpAuthTokenResult2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXblIdpAuthTokenResult2> for ::windows::core::IUnknown {
    fn from(value: &IXblIdpAuthTokenResult2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXblIdpAuthTokenResult2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXblIdpAuthTokenResult2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXblIdpAuthTokenResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXblIdpAuthTokenResult2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXblIdpAuthTokenResult2 {}
impl ::core::fmt::Debug for IXblIdpAuthTokenResult2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXblIdpAuthTokenResult2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXblIdpAuthTokenResult2 {
    type Vtable = IXblIdpAuthTokenResult2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75d760b0_60b9_412d_994f_26b2cd5f7812);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthTokenResult2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetModernGamertag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetModernGamertagSuffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetUniqueModernGamertag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KnownGamingPrivileges(pub i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_BROADCAST: KnownGamingPrivileges = KnownGamingPrivileges(190i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_VIEW_FRIENDS_LIST: KnownGamingPrivileges = KnownGamingPrivileges(197i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_GAME_DVR: KnownGamingPrivileges = KnownGamingPrivileges(198i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_SHARE_KINECT_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(199i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_MULTIPLAYER_PARTIES: KnownGamingPrivileges = KnownGamingPrivileges(203i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_COMMUNICATION_VOICE_INGAME: KnownGamingPrivileges = KnownGamingPrivileges(205i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_COMMUNICATION_VOICE_SKYPE: KnownGamingPrivileges = KnownGamingPrivileges(206i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_CLOUD_GAMING_MANAGE_SESSION: KnownGamingPrivileges = KnownGamingPrivileges(207i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_CLOUD_GAMING_JOIN_SESSION: KnownGamingPrivileges = KnownGamingPrivileges(208i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_CLOUD_SAVED_GAMES: KnownGamingPrivileges = KnownGamingPrivileges(209i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_SHARE_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(211i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_PREMIUM_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(214i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_SUBSCRIPTION_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(219i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_SOCIAL_NETWORK_SHARING: KnownGamingPrivileges = KnownGamingPrivileges(220i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_PREMIUM_VIDEO: KnownGamingPrivileges = KnownGamingPrivileges(224i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_VIDEO_COMMUNICATIONS: KnownGamingPrivileges = KnownGamingPrivileges(235i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_PURCHASE_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(245i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_USER_CREATED_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(247i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_PROFILE_VIEWING: KnownGamingPrivileges = KnownGamingPrivileges(249i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_COMMUNICATIONS: KnownGamingPrivileges = KnownGamingPrivileges(252i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_MULTIPLAYER_SESSIONS: KnownGamingPrivileges = KnownGamingPrivileges(254i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_ADD_FRIEND: KnownGamingPrivileges = KnownGamingPrivileges(255i32);
impl ::core::marker::Copy for KnownGamingPrivileges {}
impl ::core::clone::Clone for KnownGamingPrivileges {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KnownGamingPrivileges {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KnownGamingPrivileges {
    type Abi = Self;
}
impl ::core::fmt::Debug for KnownGamingPrivileges {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KnownGamingPrivileges").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub type PlayerPickerUICompletionRoutine = ::core::option::Option<unsafe extern "system" fn(returncode: ::windows::core::HRESULT, context: *const ::core::ffi::c_void, selectedxuids: *const ::windows::core::HSTRING, selectedxuidscount: usize)>;
#[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ProcessPendingGameUI<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::BOOL>>(waitforcompletion: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessPendingGameUI(waitforcompletion: super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        ProcessPendingGameUI(waitforcompletion.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ReleaseExclusiveCpuSets() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseExclusiveCpuSets() -> ::windows::core::HRESULT;
        }
        ReleaseExclusiveCpuSets().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowChangeFriendRelationshipUI<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(targetuserxuid: Param0, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowChangeFriendRelationshipUI(targetuserxuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowChangeFriendRelationshipUI(targetuserxuid.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowChangeFriendRelationshipUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, targetuserxuid: Param1, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowChangeFriendRelationshipUIForUser(user: *mut ::core::ffi::c_void, targetuserxuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowChangeFriendRelationshipUIForUser(user.into_param().abi(), targetuserxuid.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowCustomizeUserProfileUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowCustomizeUserProfileUI(completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowCustomizeUserProfileUI(::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowCustomizeUserProfileUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(user: Param0, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowCustomizeUserProfileUIForUser(user: *mut ::core::ffi::c_void, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowCustomizeUserProfileUIForUser(user.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowFindFriendsUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowFindFriendsUI(completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowFindFriendsUI(::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowFindFriendsUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(user: Param0, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowFindFriendsUIForUser(user: *mut ::core::ffi::c_void, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowFindFriendsUIForUser(user.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowGameInfoUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInfoUI(titleid: u32, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowGameInfoUI(::core::mem::transmute(titleid), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowGameInfoUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(user: Param0, titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInfoUIForUser(user: *mut ::core::ffi::c_void, titleid: u32, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowGameInfoUIForUser(user.into_param().abi(), ::core::mem::transmute(titleid), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowGameInviteUI<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(serviceconfigurationid: Param0, sessiontemplatename: Param1, sessionid: Param2, invitationdisplaytext: Param3, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInviteUI(serviceconfigurationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessiontemplatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, invitationdisplaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowGameInviteUI(serviceconfigurationid.into_param().abi(), sessiontemplatename.into_param().abi(), sessionid.into_param().abi(), invitationdisplaytext.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowGameInviteUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, serviceconfigurationid: Param1, sessiontemplatename: Param2, sessionid: Param3, invitationdisplaytext: Param4, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInviteUIForUser(user: *mut ::core::ffi::c_void, serviceconfigurationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessiontemplatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, invitationdisplaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowGameInviteUIForUser(user.into_param().abi(), serviceconfigurationid.into_param().abi(), sessiontemplatename.into_param().abi(), sessionid.into_param().abi(), invitationdisplaytext.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowGameInviteUIWithContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(serviceconfigurationid: Param0, sessiontemplatename: Param1, sessionid: Param2, invitationdisplaytext: Param3, customactivationcontext: Param4, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInviteUIWithContext(serviceconfigurationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessiontemplatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, invitationdisplaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, customactivationcontext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowGameInviteUIWithContext(serviceconfigurationid.into_param().abi(), sessiontemplatename.into_param().abi(), sessionid.into_param().abi(), invitationdisplaytext.into_param().abi(), customactivationcontext.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowGameInviteUIWithContextForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, serviceconfigurationid: Param1, sessiontemplatename: Param2, sessionid: Param3, invitationdisplaytext: Param4, customactivationcontext: Param5, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInviteUIWithContextForUser(user: *mut ::core::ffi::c_void, serviceconfigurationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessiontemplatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, invitationdisplaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, customactivationcontext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowGameInviteUIWithContextForUser(user.into_param().abi(), serviceconfigurationid.into_param().abi(), sessiontemplatename.into_param().abi(), sessionid.into_param().abi(), invitationdisplaytext.into_param().abi(), customactivationcontext.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowPlayerPickerUI<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(promptdisplaytext: Param0, xuids: &[::windows::core::HSTRING], preselectedxuids: &[::windows::core::HSTRING], minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowPlayerPickerUI(promptdisplaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xuids: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xuidscount: usize, preselectedxuids: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowPlayerPickerUI(promptdisplaytext.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(xuids)), xuids.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(preselectedxuids)), preselectedxuids.len() as _, ::core::mem::transmute(minselectioncount), ::core::mem::transmute(maxselectioncount), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowPlayerPickerUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, promptdisplaytext: Param1, xuids: &[::windows::core::HSTRING], preselectedxuids: &[::windows::core::HSTRING], minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowPlayerPickerUIForUser(user: *mut ::core::ffi::c_void, promptdisplaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xuids: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xuidscount: usize, preselectedxuids: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowPlayerPickerUIForUser(user.into_param().abi(), promptdisplaytext.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(xuids)), xuids.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(preselectedxuids)), preselectedxuids.len() as _, ::core::mem::transmute(minselectioncount), ::core::mem::transmute(maxselectioncount), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowProfileCardUI<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(targetuserxuid: Param0, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowProfileCardUI(targetuserxuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowProfileCardUI(targetuserxuid.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowProfileCardUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, targetuserxuid: Param1, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowProfileCardUIForUser(user: *mut ::core::ffi::c_void, targetuserxuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowProfileCardUIForUser(user.into_param().abi(), targetuserxuid.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowTitleAchievementsUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowTitleAchievementsUI(titleid: u32, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowTitleAchievementsUI(::core::mem::transmute(titleid), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowTitleAchievementsUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(user: Param0, titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowTitleAchievementsUIForUser(user: *mut ::core::ffi::c_void, titleid: u32, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowTitleAchievementsUIForUser(user.into_param().abi(), ::core::mem::transmute(titleid), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowUserSettingsUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowUserSettingsUI(completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowUserSettingsUI(::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowUserSettingsUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(user: Param0, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowUserSettingsUIForUser(user: *mut ::core::ffi::c_void, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowUserSettingsUIForUser(user.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TryCancelPendingGameUI() -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TryCancelPendingGameUI() -> super::Foundation::BOOL;
        }
        ::core::mem::transmute(TryCancelPendingGameUI())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XBL_IDP_AUTH_TOKEN_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_SUCCESS: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_SUCCESS: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_NO_ACCOUNT_SET: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_LOAD_MSA_ACCOUNT_FAILED: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_XBOX_VETO: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_MSA_INTERRUPT: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(5i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_NO_CONSENT: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(6i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_VIEW_NOT_SET: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(7i32);
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_UNKNOWN: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(-1i32);
impl ::core::marker::Copy for XBL_IDP_AUTH_TOKEN_STATUS {}
impl ::core::clone::Clone for XBL_IDP_AUTH_TOKEN_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XBL_IDP_AUTH_TOKEN_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XBL_IDP_AUTH_TOKEN_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for XBL_IDP_AUTH_TOKEN_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XBL_IDP_AUTH_TOKEN_STATUS").field(&self.0).finish()
    }
}
pub const XblIdpAuthManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce23534b_56d8_4978_86a2_7ee570640468);
pub const XblIdpAuthTokenResult: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f493441_744a_410c_ae2b_9a22f7c7731f);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
