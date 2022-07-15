#[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckGamingPrivilegeSilently(privilegeid: u32, scope: &::windows::core::HSTRING, policy: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::BOOL> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CheckGamingPrivilegeSilently(privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, hasprivilege: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CheckGamingPrivilegeSilently(privilegeid, ::core::mem::transmute_copy(scope), ::core::mem::transmute_copy(policy), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Foundation::BOOL>(result__)
}
#[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckGamingPrivilegeSilentlyForUser<'a, P0>(user: P0, privilegeid: u32, scope: &::windows::core::HSTRING, policy: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::BOOL>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CheckGamingPrivilegeSilentlyForUser(user: *mut ::core::ffi::c_void, privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, hasprivilege: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CheckGamingPrivilegeSilentlyForUser(user.into().abi(), privilegeid, ::core::mem::transmute_copy(scope), ::core::mem::transmute_copy(policy), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Foundation::BOOL>(result__)
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn CheckGamingPrivilegeWithUI(privilegeid: u32, scope: &::windows::core::HSTRING, policy: &::windows::core::HSTRING, friendlymessage: &::windows::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CheckGamingPrivilegeWithUI(privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, friendlymessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    CheckGamingPrivilegeWithUI(privilegeid, ::core::mem::transmute_copy(scope), ::core::mem::transmute_copy(policy), ::core::mem::transmute_copy(friendlymessage), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn CheckGamingPrivilegeWithUIForUser<'a, P0>(user: P0, privilegeid: u32, scope: &::windows::core::HSTRING, policy: &::windows::core::HSTRING, friendlymessage: &::windows::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CheckGamingPrivilegeWithUIForUser(user: *mut ::core::ffi::c_void, privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, friendlymessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    CheckGamingPrivilegeWithUIForUser(user.into().abi(), privilegeid, ::core::mem::transmute_copy(scope), ::core::mem::transmute_copy(policy), ::core::mem::transmute_copy(friendlymessage), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetExpandedResourceExclusiveCpuCount(exclusivecpucount: *mut u32) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetExpandedResourceExclusiveCpuCount(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn GetGamingDeviceModelInformation() -> ::windows::core::Result<GAMING_DEVICE_MODEL_INFORMATION> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetGamingDeviceModelInformation(information: *mut GAMING_DEVICE_MODEL_INFORMATION) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetGamingDeviceModelInformation(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GAMING_DEVICE_MODEL_INFORMATION>(result__)
}
#[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HasExpandedResources() -> ::windows::core::Result<super::Foundation::BOOL> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HasExpandedResources(hasexpandedresources: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HasExpandedResources(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Foundation::BOOL>(result__)
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const ID_GDF_THUMBNAIL_STR: &str = "__GDF_THUMBNAIL";
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const ID_GDF_XML_STR: &str = "__GDF_XML";
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
pub struct IGameExplorer(::windows::core::IUnknown);
impl IGameExplorer {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddGame<'a, P0, P1>(&self, bstrgdfbinarypath: P0, bstrgameinstalldirectory: P1, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).AddGame)(::windows::core::Interface::as_raw(self), bstrgdfbinarypath.into().abi(), bstrgameinstalldirectory.into().abi(), installscope, ::core::mem::transmute(pguidinstanceid)).ok()
    }
    pub unsafe fn RemoveGame(&self, guidinstanceid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveGame)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guidinstanceid)).ok()
    }
    pub unsafe fn UpdateGame(&self, guidinstanceid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateGame)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guidinstanceid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VerifyAccess<'a, P0>(&self, bstrgdfbinarypath: P0) -> ::windows::core::Result<super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).VerifyAccess)(::windows::core::Interface::as_raw(self), bstrgdfbinarypath.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IGameExplorer> for ::windows::core::IUnknown {
    fn from(value: IGameExplorer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGameExplorer> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IGameExplorer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameExplorer> for ::windows::core::IUnknown {
    fn from(value: &IGameExplorer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
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
    pub unsafe fn InstallGame<'a, P0, P1>(&self, binarygdfpath: P0, installdirectory: P1, installscope: GAME_INSTALL_SCOPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).InstallGame)(::windows::core::Interface::as_raw(self), binarygdfpath.into(), installdirectory.into(), installscope).ok()
    }
    pub unsafe fn UninstallGame<'a, P0>(&self, binarygdfpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).UninstallGame)(::windows::core::Interface::as_raw(self), binarygdfpath.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckAccess<'a, P0>(&self, binarygdfpath: P0) -> ::windows::core::Result<super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CheckAccess)(::windows::core::Interface::as_raw(self), binarygdfpath.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IGameExplorer2> for ::windows::core::IUnknown {
    fn from(value: IGameExplorer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGameExplorer2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IGameExplorer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameExplorer2> for ::windows::core::IUnknown {
    fn from(value: &IGameExplorer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
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
    pub unsafe fn GetMaxCategoryLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxCategoryLength)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxNameLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxNameLength)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxValueLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxValueLength)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxCategories(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxCategories)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetMaxStatsPerCategory(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxStatsPerCategory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetCategoryTitle<'a, P0>(&self, categoryindex: u16, title: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetCategoryTitle)(::windows::core::Interface::as_raw(self), categoryindex, title.into()).ok()
    }
    pub unsafe fn GetCategoryTitle(&self, categoryindex: u16) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCategoryTitle)(::windows::core::Interface::as_raw(self), categoryindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetStatistic(&self, categoryindex: u16, statindex: u16, pname: *mut ::windows::core::PWSTR, pvalue: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatistic)(::windows::core::Interface::as_raw(self), categoryindex, statindex, ::core::mem::transmute(pname), ::core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn SetStatistic<'a, P0, P1>(&self, categoryindex: u16, statindex: u16, name: P0, value: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetStatistic)(::windows::core::Interface::as_raw(self), categoryindex, statindex, name.into(), value.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, P0>(&self, trackchanges: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Save)(::windows::core::Interface::as_raw(self), trackchanges.into()).ok()
    }
    pub unsafe fn SetLastPlayedCategory(&self, categoryindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLastPlayedCategory)(::windows::core::Interface::as_raw(self), categoryindex).ok()
    }
    pub unsafe fn GetLastPlayedCategory(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLastPlayedCategory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IGameStatistics> for ::windows::core::IUnknown {
    fn from(value: IGameStatistics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGameStatistics> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IGameStatistics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameStatistics> for ::windows::core::IUnknown {
    fn from(value: &IGameStatistics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
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
    pub unsafe fn GetGameStatistics<'a, P0>(&self, gdfbinarypath: P0, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut ::core::option::Option<IGameStatistics>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetGameStatistics)(::windows::core::Interface::as_raw(self), gdfbinarypath.into(), opentype, ::core::mem::transmute(popenresult), ::core::mem::transmute(ppistats)).ok()
    }
    pub unsafe fn RemoveGameStatistics<'a, P0>(&self, gdfbinarypath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RemoveGameStatistics)(::windows::core::Interface::as_raw(self), gdfbinarypath.into()).ok()
    }
}
impl ::core::convert::From<IGameStatisticsMgr> for ::windows::core::IUnknown {
    fn from(value: IGameStatisticsMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGameStatisticsMgr> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IGameStatisticsMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameStatisticsMgr> for ::windows::core::IUnknown {
    fn from(value: &IGameStatisticsMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetGameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gdfbinarypath: ::windows::core::PCWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveGameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gdfbinarypath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
pub struct IXblIdpAuthManager(::windows::core::IUnknown);
impl IXblIdpAuthManager {
    pub unsafe fn SetGamerAccount<'a, P0, P1>(&self, msaaccountid: P0, xuid: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetGamerAccount)(::windows::core::Interface::as_raw(self), msaaccountid.into(), xuid.into()).ok()
    }
    pub unsafe fn GetGamerAccount(&self, msaaccountid: *mut ::windows::core::PWSTR, xuid: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGamerAccount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(msaaccountid), ::core::mem::transmute(xuid)).ok()
    }
    pub unsafe fn SetAppViewInitialized<'a, P0, P1>(&self, appsid: P0, msaaccountid: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetAppViewInitialized)(::windows::core::Interface::as_raw(self), appsid.into(), msaaccountid.into()).ok()
    }
    pub unsafe fn GetEnvironment(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetEnvironment)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetSandbox(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSandbox)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTokenAndSignatureWithTokenResult<'a, P0, P1, P2, P3, P4, P5, P6, P7>(&self, msaaccountid: P0, appsid: P1, msatarget: P2, msapolicy: P3, httpmethod: P4, uri: P5, headers: P6, body: &[u8], forcerefresh: P7) -> ::windows::core::Result<IXblIdpAuthTokenResult>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<::windows::core::PCWSTR>,
        P4: ::std::convert::Into<::windows::core::PCWSTR>,
        P5: ::std::convert::Into<::windows::core::PCWSTR>,
        P6: ::std::convert::Into<::windows::core::PCWSTR>,
        P7: ::std::convert::Into<super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTokenAndSignatureWithTokenResult)(::windows::core::Interface::as_raw(self), msaaccountid.into(), appsid.into(), msatarget.into(), msapolicy.into(), httpmethod.into(), uri.into(), headers.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(body)), body.len() as _, forcerefresh.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IXblIdpAuthTokenResult>(result__)
    }
}
impl ::core::convert::From<IXblIdpAuthManager> for ::windows::core::IUnknown {
    fn from(value: IXblIdpAuthManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXblIdpAuthManager> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IXblIdpAuthManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXblIdpAuthManager> for ::windows::core::IUnknown {
    fn from(value: &IXblIdpAuthManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetGamerAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msaaccountid: ::windows::core::PCWSTR, xuid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetGamerAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msaaccountid: *mut ::windows::core::PWSTR, xuid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetAppViewInitialized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appsid: ::windows::core::PCWSTR, msaaccountid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environment: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetSandbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sandbox: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTokenAndSignatureWithTokenResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msaaccountid: ::windows::core::PCWSTR, appsid: ::windows::core::PCWSTR, msatarget: ::windows::core::PCWSTR, msapolicy: ::windows::core::PCWSTR, httpmethod: ::windows::core::PCWSTR, uri: ::windows::core::PCWSTR, headers: ::windows::core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTokenAndSignatureWithTokenResult: usize,
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
pub struct IXblIdpAuthTokenResult(::windows::core::IUnknown);
impl IXblIdpAuthTokenResult {
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<XBL_IDP_AUTH_TOKEN_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XBL_IDP_AUTH_TOKEN_STATUS>(result__)
    }
    pub unsafe fn GetErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetErrorCode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HRESULT>(result__)
    }
    pub unsafe fn GetToken(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetToken)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetSignature(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSignature)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetSandbox(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSandbox)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetEnvironment(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetEnvironment)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetMsaAccountId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMsaAccountId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetXuid(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetXuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetGamertag(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetGamertag)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetAgeGroup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetAgeGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetPrivileges(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPrivileges)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetMsaTarget(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMsaTarget)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetMsaPolicy(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMsaPolicy)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetMsaAppId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMsaAppId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetRedirect(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRedirect)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetMessage(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMessage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetHelpId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetHelpId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetEnforcementBans(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetEnforcementBans)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetRestrictions(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRestrictions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetTitleRestrictions(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTitleRestrictions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IXblIdpAuthTokenResult> for ::windows::core::IUnknown {
    fn from(value: IXblIdpAuthTokenResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXblIdpAuthTokenResult> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IXblIdpAuthTokenResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXblIdpAuthTokenResult> for ::windows::core::IUnknown {
    fn from(value: &IXblIdpAuthTokenResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
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
    pub unsafe fn GetModernGamertag(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetModernGamertag)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetModernGamertagSuffix(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetModernGamertagSuffix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetUniqueModernGamertag(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetUniqueModernGamertag)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IXblIdpAuthTokenResult2> for ::windows::core::IUnknown {
    fn from(value: IXblIdpAuthTokenResult2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXblIdpAuthTokenResult2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IXblIdpAuthTokenResult2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXblIdpAuthTokenResult2> for ::windows::core::IUnknown {
    fn from(value: &IXblIdpAuthTokenResult2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetModernGamertag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetModernGamertagSuffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetUniqueModernGamertag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
pub unsafe fn ProcessPendingGameUI<'a, P0>(waitforcompletion: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ProcessPendingGameUI(waitforcompletion: super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    ProcessPendingGameUI(waitforcompletion.into()).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ReleaseExclusiveCpuSets() -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ReleaseExclusiveCpuSets() -> ::windows::core::HRESULT;
    }
    ReleaseExclusiveCpuSets().ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowChangeFriendRelationshipUI(targetuserxuid: &::windows::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowChangeFriendRelationshipUI(targetuserxuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowChangeFriendRelationshipUI(::core::mem::transmute_copy(targetuserxuid), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowChangeFriendRelationshipUIForUser<'a, P0>(user: P0, targetuserxuid: &::windows::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowChangeFriendRelationshipUIForUser(user: *mut ::core::ffi::c_void, targetuserxuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowChangeFriendRelationshipUIForUser(user.into().abi(), ::core::mem::transmute_copy(targetuserxuid), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowCustomizeUserProfileUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowCustomizeUserProfileUI(completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowCustomizeUserProfileUI(::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowCustomizeUserProfileUIForUser<'a, P0>(user: P0, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowCustomizeUserProfileUIForUser(user: *mut ::core::ffi::c_void, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowCustomizeUserProfileUIForUser(user.into().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowFindFriendsUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowFindFriendsUI(completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowFindFriendsUI(::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowFindFriendsUIForUser<'a, P0>(user: P0, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowFindFriendsUIForUser(user: *mut ::core::ffi::c_void, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowFindFriendsUIForUser(user.into().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowGameInfoUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowGameInfoUI(titleid: u32, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowGameInfoUI(titleid, ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowGameInfoUIForUser<'a, P0>(user: P0, titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowGameInfoUIForUser(user: *mut ::core::ffi::c_void, titleid: u32, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowGameInfoUIForUser(user.into().abi(), titleid, ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowGameInviteUI(serviceconfigurationid: &::windows::core::HSTRING, sessiontemplatename: &::windows::core::HSTRING, sessionid: &::windows::core::HSTRING, invitationdisplaytext: &::windows::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowGameInviteUI(serviceconfigurationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessiontemplatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, invitationdisplaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowGameInviteUI(::core::mem::transmute_copy(serviceconfigurationid), ::core::mem::transmute_copy(sessiontemplatename), ::core::mem::transmute_copy(sessionid), ::core::mem::transmute_copy(invitationdisplaytext), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowGameInviteUIForUser<'a, P0>(user: P0, serviceconfigurationid: &::windows::core::HSTRING, sessiontemplatename: &::windows::core::HSTRING, sessionid: &::windows::core::HSTRING, invitationdisplaytext: &::windows::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowGameInviteUIForUser(user: *mut ::core::ffi::c_void, serviceconfigurationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessiontemplatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, invitationdisplaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowGameInviteUIForUser(user.into().abi(), ::core::mem::transmute_copy(serviceconfigurationid), ::core::mem::transmute_copy(sessiontemplatename), ::core::mem::transmute_copy(sessionid), ::core::mem::transmute_copy(invitationdisplaytext), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowGameInviteUIWithContext(serviceconfigurationid: &::windows::core::HSTRING, sessiontemplatename: &::windows::core::HSTRING, sessionid: &::windows::core::HSTRING, invitationdisplaytext: &::windows::core::HSTRING, customactivationcontext: &::windows::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowGameInviteUIWithContext(serviceconfigurationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessiontemplatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, invitationdisplaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, customactivationcontext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowGameInviteUIWithContext(::core::mem::transmute_copy(serviceconfigurationid), ::core::mem::transmute_copy(sessiontemplatename), ::core::mem::transmute_copy(sessionid), ::core::mem::transmute_copy(invitationdisplaytext), ::core::mem::transmute_copy(customactivationcontext), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowGameInviteUIWithContextForUser<'a, P0>(user: P0, serviceconfigurationid: &::windows::core::HSTRING, sessiontemplatename: &::windows::core::HSTRING, sessionid: &::windows::core::HSTRING, invitationdisplaytext: &::windows::core::HSTRING, customactivationcontext: &::windows::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowGameInviteUIWithContextForUser(user: *mut ::core::ffi::c_void, serviceconfigurationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessiontemplatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, invitationdisplaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, customactivationcontext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowGameInviteUIWithContextForUser(user.into().abi(), ::core::mem::transmute_copy(serviceconfigurationid), ::core::mem::transmute_copy(sessiontemplatename), ::core::mem::transmute_copy(sessionid), ::core::mem::transmute_copy(invitationdisplaytext), ::core::mem::transmute_copy(customactivationcontext), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowPlayerPickerUI(promptdisplaytext: &::windows::core::HSTRING, xuids: &[::windows::core::HSTRING], preselectedxuids: &[::windows::core::HSTRING], minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowPlayerPickerUI(promptdisplaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xuids: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xuidscount: usize, preselectedxuids: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowPlayerPickerUI(::core::mem::transmute_copy(promptdisplaytext), ::core::mem::transmute(::windows::core::as_ptr_or_null(xuids)), xuids.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(preselectedxuids)), preselectedxuids.len() as _, minselectioncount, maxselectioncount, ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowPlayerPickerUIForUser<'a, P0>(user: P0, promptdisplaytext: &::windows::core::HSTRING, xuids: &[::windows::core::HSTRING], preselectedxuids: &[::windows::core::HSTRING], minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowPlayerPickerUIForUser(user: *mut ::core::ffi::c_void, promptdisplaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xuids: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xuidscount: usize, preselectedxuids: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowPlayerPickerUIForUser(user.into().abi(), ::core::mem::transmute_copy(promptdisplaytext), ::core::mem::transmute(::windows::core::as_ptr_or_null(xuids)), xuids.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(preselectedxuids)), preselectedxuids.len() as _, minselectioncount, maxselectioncount, ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowProfileCardUI(targetuserxuid: &::windows::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowProfileCardUI(targetuserxuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowProfileCardUI(::core::mem::transmute_copy(targetuserxuid), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowProfileCardUIForUser<'a, P0>(user: P0, targetuserxuid: &::windows::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowProfileCardUIForUser(user: *mut ::core::ffi::c_void, targetuserxuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowProfileCardUIForUser(user.into().abi(), ::core::mem::transmute_copy(targetuserxuid), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowTitleAchievementsUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowTitleAchievementsUI(titleid: u32, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowTitleAchievementsUI(titleid, ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowTitleAchievementsUIForUser<'a, P0>(user: P0, titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowTitleAchievementsUIForUser(user: *mut ::core::ffi::c_void, titleid: u32, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowTitleAchievementsUIForUser(user.into().abi(), titleid, ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowUserSettingsUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowUserSettingsUI(completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowUserSettingsUI(::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[inline]
pub unsafe fn ShowUserSettingsUIForUser<'a, P0>(user: P0, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShowUserSettingsUIForUser(user: *mut ::core::ffi::c_void, completionroutine: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    ShowUserSettingsUIForUser(user.into().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TryCancelPendingGameUI() -> super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TryCancelPendingGameUI() -> super::Foundation::BOOL;
    }
    TryCancelPendingGameUI()
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
