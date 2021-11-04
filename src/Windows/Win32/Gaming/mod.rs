#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckGamingPrivilegeSilently<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(privilegeid: u32, scope: Param1, policy: Param2) -> ::windows::runtime::Result<super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckGamingPrivilegeSilently(privilegeid: u32, scope: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, policy: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, hasprivilege: *mut super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CheckGamingPrivilegeSilently(::std::mem::transmute(privilegeid), scope.into_param().abi(), policy.into_param().abi(), &mut result__).from_abi::<super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckGamingPrivilegeSilentlyForUser<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(user: Param0, privilegeid: u32, scope: Param2, policy: Param3) -> ::windows::runtime::Result<super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckGamingPrivilegeSilentlyForUser(user: ::windows::runtime::RawPtr, privilegeid: u32, scope: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, policy: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, hasprivilege: *mut super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CheckGamingPrivilegeSilentlyForUser(user.into_param().abi(), ::std::mem::transmute(privilegeid), scope.into_param().abi(), policy.into_param().abi(), &mut result__).from_abi::<super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn CheckGamingPrivilegeWithUI<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(privilegeid: u32, scope: Param1, policy: Param2, friendlymessage: Param3, completionroutine: ::std::option::Option<GameUICompletionRoutine>, context: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckGamingPrivilegeWithUI(privilegeid: u32, scope: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, policy: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, friendlymessage: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CheckGamingPrivilegeWithUI(::std::mem::transmute(privilegeid), scope.into_param().abi(), policy.into_param().abi(), friendlymessage.into_param().abi(), ::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn CheckGamingPrivilegeWithUIForUser<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
    user: Param0,
    privilegeid: u32,
    scope: Param2,
    policy: Param3,
    friendlymessage: Param4,
    completionroutine: ::std::option::Option<GameUICompletionRoutine>,
    context: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckGamingPrivilegeWithUIForUser(user: ::windows::runtime::RawPtr, privilegeid: u32, scope: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, policy: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, friendlymessage: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CheckGamingPrivilegeWithUIForUser(user.into_param().abi(), ::std::mem::transmute(privilegeid), scope.into_param().abi(), policy.into_param().abi(), friendlymessage.into_param().abi(), ::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GAMESTATS_OPEN_RESULT(pub i32);
pub const GAMESTATS_OPEN_CREATED: GAMESTATS_OPEN_RESULT = GAMESTATS_OPEN_RESULT(0i32);
pub const GAMESTATS_OPEN_OPENED: GAMESTATS_OPEN_RESULT = GAMESTATS_OPEN_RESULT(1i32);
impl ::std::convert::From<i32> for GAMESTATS_OPEN_RESULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GAMESTATS_OPEN_RESULT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GAMESTATS_OPEN_TYPE(pub i32);
pub const GAMESTATS_OPEN_OPENORCREATE: GAMESTATS_OPEN_TYPE = GAMESTATS_OPEN_TYPE(0i32);
pub const GAMESTATS_OPEN_OPENONLY: GAMESTATS_OPEN_TYPE = GAMESTATS_OPEN_TYPE(1i32);
impl ::std::convert::From<i32> for GAMESTATS_OPEN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GAMESTATS_OPEN_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GAME_INSTALL_SCOPE(pub i32);
pub const GIS_NOT_INSTALLED: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(1i32);
pub const GIS_CURRENT_USER: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(2i32);
pub const GIS_ALL_USERS: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(3i32);
impl ::std::convert::From<i32> for GAME_INSTALL_SCOPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GAME_INSTALL_SCOPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GAMING_DEVICE_DEVICE_ID(pub i32);
pub const GAMING_DEVICE_DEVICE_ID_NONE: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(0i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(1988865574i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_S: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(712204761i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(1523980231i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X_DEVKIT: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(284675555i32);
impl ::std::convert::From<i32> for GAMING_DEVICE_DEVICE_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GAMING_DEVICE_DEVICE_ID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Gaming`*"]
pub struct GAMING_DEVICE_MODEL_INFORMATION {
    pub vendorId: GAMING_DEVICE_VENDOR_ID,
    pub deviceId: GAMING_DEVICE_DEVICE_ID,
}
impl GAMING_DEVICE_MODEL_INFORMATION {}
impl ::std::default::Default for GAMING_DEVICE_MODEL_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GAMING_DEVICE_MODEL_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GAMING_DEVICE_MODEL_INFORMATION").field("vendorId", &self.vendorId).field("deviceId", &self.deviceId).finish()
    }
}
impl ::std::cmp::PartialEq for GAMING_DEVICE_MODEL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.vendorId == other.vendorId && self.deviceId == other.deviceId
    }
}
impl ::std::cmp::Eq for GAMING_DEVICE_MODEL_INFORMATION {}
unsafe impl ::windows::runtime::Abi for GAMING_DEVICE_MODEL_INFORMATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GAMING_DEVICE_VENDOR_ID(pub i32);
pub const GAMING_DEVICE_VENDOR_ID_NONE: GAMING_DEVICE_VENDOR_ID = GAMING_DEVICE_VENDOR_ID(0i32);
pub const GAMING_DEVICE_VENDOR_ID_MICROSOFT: GAMING_DEVICE_VENDOR_ID = GAMING_DEVICE_VENDOR_ID(-1024700366i32);
impl ::std::convert::From<i32> for GAMING_DEVICE_VENDOR_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GAMING_DEVICE_VENDOR_ID {
    type Abi = Self;
}
pub const GameExplorer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2589895056, 12340, 19823, [145, 40, 1, 243, 198, 16, 34, 188]);
pub const GameStatistics: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3687340588, 49372, 18785, [182, 226, 210, 139, 98, 193, 26, 212]);
pub type GameUICompletionRoutine = unsafe extern "system" fn(returncode: ::windows::runtime::HRESULT, context: *const ::std::ffi::c_void);
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn GetExpandedResourceExclusiveCpuCount() -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetExpandedResourceExclusiveCpuCount(exclusivecpucount: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetExpandedResourceExclusiveCpuCount(&mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn GetGamingDeviceModelInformation() -> ::windows::runtime::Result<GAMING_DEVICE_MODEL_INFORMATION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGamingDeviceModelInformation(information: *mut GAMING_DEVICE_MODEL_INFORMATION) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <GAMING_DEVICE_MODEL_INFORMATION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetGamingDeviceModelInformation(&mut result__).from_abi::<GAMING_DEVICE_MODEL_INFORMATION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HasExpandedResources() -> ::windows::runtime::Result<super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HasExpandedResources(hasexpandedresources: *mut super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        HasExpandedResources(&mut result__).from_abi::<super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IGameExplorer(pub ::windows::runtime::IUnknown);
impl IGameExplorer {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn AddGame<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>>(&self, bstrgdfbinarypath: Param0, bstrgameinstalldirectory: Param1, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), bstrgdfbinarypath.into_param().abi(), bstrgameinstalldirectory.into_param().abi(), ::std::mem::transmute(installscope), ::std::mem::transmute(pguidinstanceid)).ok()
    }
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub unsafe fn RemoveGame<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, guidinstanceid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), guidinstanceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub unsafe fn UpdateGame<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, guidinstanceid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), guidinstanceid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn VerifyAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>>(&self, bstrgdfbinarypath: Param0) -> ::windows::runtime::Result<super::Foundation::BOOL> {
        let mut result__: <super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), bstrgdfbinarypath.into_param().abi(), &mut result__).from_abi::<super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGameExplorer {
    type Vtable = IGameExplorer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3887266674, 55080, 18867, [165, 242, 24, 235, 245, 241, 52, 158]);
}
impl ::std::convert::From<IGameExplorer> for ::windows::runtime::IUnknown {
    fn from(value: IGameExplorer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IGameExplorer> for ::windows::runtime::IUnknown {
    fn from(value: &IGameExplorer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGameExplorer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGameExplorer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameExplorer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrgdfbinarypath: ::std::mem::ManuallyDrop<super::Foundation::BSTR>, bstrgameinstalldirectory: ::std::mem::ManuallyDrop<super::Foundation::BSTR>, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidinstanceid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidinstanceid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrgdfbinarypath: ::std::mem::ManuallyDrop<super::Foundation::BSTR>, pfhasaccess: *mut super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Gaming`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IGameExplorer2(pub ::windows::runtime::IUnknown);
impl IGameExplorer2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn InstallGame<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(&self, binarygdfpath: Param0, installdirectory: Param1, installscope: GAME_INSTALL_SCOPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), binarygdfpath.into_param().abi(), installdirectory.into_param().abi(), ::std::mem::transmute(installscope)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn UninstallGame<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(&self, binarygdfpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), binarygdfpath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn CheckAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(&self, binarygdfpath: Param0) -> ::windows::runtime::Result<super::Foundation::BOOL> {
        let mut result__: <super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), binarygdfpath.into_param().abi(), &mut result__).from_abi::<super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGameExplorer2 {
    type Vtable = IGameExplorer2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2257013415, 41453, 17677, [167, 235, 184, 158, 32, 178, 255, 243]);
}
impl ::std::convert::From<IGameExplorer2> for ::windows::runtime::IUnknown {
    fn from(value: IGameExplorer2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IGameExplorer2> for ::windows::runtime::IUnknown {
    fn from(value: &IGameExplorer2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGameExplorer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGameExplorer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameExplorer2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, binarygdfpath: super::Foundation::PWSTR, installdirectory: super::Foundation::PWSTR, installscope: GAME_INSTALL_SCOPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, binarygdfpath: super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, binarygdfpath: super::Foundation::PWSTR, phasaccess: *mut super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Gaming`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IGameStatistics(pub ::windows::runtime::IUnknown);
impl IGameStatistics {
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub unsafe fn GetMaxCategoryLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub unsafe fn GetMaxNameLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub unsafe fn GetMaxValueLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub unsafe fn GetMaxCategories(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub unsafe fn GetMaxStatsPerCategory(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn SetCategoryTitle<'a, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(&self, categoryindex: u16, title: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(categoryindex), title.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetCategoryTitle(&self, categoryindex: u16) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(categoryindex), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetStatistic(&self, categoryindex: u16, statindex: u16, pname: *mut super::Foundation::PWSTR, pvalue: *mut super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(categoryindex), ::std::mem::transmute(statindex), ::std::mem::transmute(pname), ::std::mem::transmute(pvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn SetStatistic<'a, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(&self, categoryindex: u16, statindex: u16, name: Param2, value: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(categoryindex), ::std::mem::transmute(statindex), name.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn Save<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(&self, trackchanges: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), trackchanges.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub unsafe fn SetLastPlayedCategory(&self, categoryindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(categoryindex)).ok()
    }
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub unsafe fn GetLastPlayedCategory(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGameStatistics {
    type Vtable = IGameStatistics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(948423114, 1184, 17070, [188, 76, 95, 166, 199, 114, 17, 69]);
}
impl ::std::convert::From<IGameStatistics> for ::windows::runtime::IUnknown {
    fn from(value: IGameStatistics) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IGameStatistics> for ::windows::runtime::IUnknown {
    fn from(value: &IGameStatistics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGameStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGameStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameStatistics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmax: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmax: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, categoryindex: u16, title: super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, categoryindex: u16, ptitle: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, categoryindex: u16, statindex: u16, pname: *mut super::Foundation::PWSTR, pvalue: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, categoryindex: u16, statindex: u16, name: super::Foundation::PWSTR, value: super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, trackchanges: super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, categoryindex: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcategoryindex: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Gaming`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IGameStatisticsMgr(pub ::windows::runtime::IUnknown);
impl IGameStatisticsMgr {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetGameStatistics<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(&self, gdfbinarypath: Param0, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut ::std::option::Option<IGameStatistics>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), gdfbinarypath.into_param().abi(), ::std::mem::transmute(opentype), ::std::mem::transmute(popenresult), ::std::mem::transmute(ppistats)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn RemoveGameStatistics<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(&self, gdfbinarypath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), gdfbinarypath.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGameStatisticsMgr {
    type Vtable = IGameStatisticsMgr_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2951997969, 59150, 16509, [149, 221, 53, 230, 18, 196, 28, 226]);
}
impl ::std::convert::From<IGameStatisticsMgr> for ::windows::runtime::IUnknown {
    fn from(value: IGameStatisticsMgr) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IGameStatisticsMgr> for ::windows::runtime::IUnknown {
    fn from(value: &IGameStatisticsMgr) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGameStatisticsMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGameStatisticsMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameStatisticsMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gdfbinarypath: super::Foundation::PWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gdfbinarypath: super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Gaming`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IXblIdpAuthManager(pub ::windows::runtime::IUnknown);
impl IXblIdpAuthManager {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn SetGamerAccount<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(&self, msaaccountid: Param0, xuid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), msaaccountid.into_param().abi(), xuid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetGamerAccount(&self, msaaccountid: *mut super::Foundation::PWSTR, xuid: *mut super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(msaaccountid), ::std::mem::transmute(xuid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn SetAppViewInitialized<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(&self, appsid: Param0, msaaccountid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), appsid.into_param().abi(), msaaccountid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetEnvironment(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetSandbox(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetTokenAndSignatureWithTokenResult<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
        Param6: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
        Param9: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
    >(
        &self,
        msaaccountid: Param0,
        appsid: Param1,
        msatarget: Param2,
        msapolicy: Param3,
        httpmethod: Param4,
        uri: Param5,
        headers: Param6,
        body: *const u8,
        bodysize: u32,
        forcerefresh: Param9,
    ) -> ::windows::runtime::Result<IXblIdpAuthTokenResult> {
        let mut result__: <IXblIdpAuthTokenResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            msaaccountid.into_param().abi(),
            appsid.into_param().abi(),
            msatarget.into_param().abi(),
            msapolicy.into_param().abi(),
            httpmethod.into_param().abi(),
            uri.into_param().abi(),
            headers.into_param().abi(),
            ::std::mem::transmute(body),
            ::std::mem::transmute(bodysize),
            forcerefresh.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IXblIdpAuthTokenResult>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXblIdpAuthManager {
    type Vtable = IXblIdpAuthManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3948796680, 35775, 17563, [172, 33, 176, 45, 222, 179, 177, 54]);
}
impl ::std::convert::From<IXblIdpAuthManager> for ::windows::runtime::IUnknown {
    fn from(value: IXblIdpAuthManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IXblIdpAuthManager> for ::windows::runtime::IUnknown {
    fn from(value: &IXblIdpAuthManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXblIdpAuthManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXblIdpAuthManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, msaaccountid: super::Foundation::PWSTR, xuid: super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, msaaccountid: *mut super::Foundation::PWSTR, xuid: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appsid: super::Foundation::PWSTR, msaaccountid: super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, environment: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sandbox: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, msaaccountid: super::Foundation::PWSTR, appsid: super::Foundation::PWSTR, msatarget: super::Foundation::PWSTR, msapolicy: super::Foundation::PWSTR, httpmethod: super::Foundation::PWSTR, uri: super::Foundation::PWSTR, headers: super::Foundation::PWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Gaming`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IXblIdpAuthTokenResult(pub ::windows::runtime::IUnknown);
impl IXblIdpAuthTokenResult {
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<XBL_IDP_AUTH_TOKEN_STATUS> {
        let mut result__: <XBL_IDP_AUTH_TOKEN_STATUS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<XBL_IDP_AUTH_TOKEN_STATUS>(result__)
    }
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub unsafe fn GetErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let mut result__: <::windows::runtime::HRESULT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetToken(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetSignature(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetSandbox(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetEnvironment(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetMsaAccountId(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetXuid(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetGamertag(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetAgeGroup(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetPrivileges(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetMsaTarget(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetMsaPolicy(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetMsaAppId(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetRedirect(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetMessage(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetHelpId(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetEnforcementBans(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetRestrictions(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetTitleRestrictions(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXblIdpAuthTokenResult {
    type Vtable = IXblIdpAuthTokenResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1187906085, 62055, 19816, [178, 153, 178, 118, 37, 82, 222, 193]);
}
impl ::std::convert::From<IXblIdpAuthTokenResult> for ::windows::runtime::IUnknown {
    fn from(value: IXblIdpAuthTokenResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IXblIdpAuthTokenResult> for ::windows::runtime::IUnknown {
    fn from(value: &IXblIdpAuthTokenResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXblIdpAuthTokenResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXblIdpAuthTokenResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthTokenResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: *mut XBL_IDP_AUTH_TOKEN_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, errorcode: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signature: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sandbox: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, environment: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, msaaccountid: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xuid: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gamertag: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, agegroup: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, privileges: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, msatarget: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, msapolicy: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, msaappid: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, redirect: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, helpid: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enforcementbans: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, restrictions: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, titlerestrictions: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Gaming`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IXblIdpAuthTokenResult2(pub ::windows::runtime::IUnknown);
impl IXblIdpAuthTokenResult2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetModernGamertag(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetModernGamertagSuffix(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    pub unsafe fn GetUniqueModernGamertag(&self) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXblIdpAuthTokenResult2 {
    type Vtable = IXblIdpAuthTokenResult2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1977049264, 24761, 16685, [153, 79, 38, 178, 205, 95, 120, 18]);
}
impl ::std::convert::From<IXblIdpAuthTokenResult2> for ::windows::runtime::IUnknown {
    fn from(value: IXblIdpAuthTokenResult2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IXblIdpAuthTokenResult2> for ::windows::runtime::IUnknown {
    fn from(value: &IXblIdpAuthTokenResult2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXblIdpAuthTokenResult2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXblIdpAuthTokenResult2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthTokenResult2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Gaming`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct KnownGamingPrivileges(pub i32);
pub const XPRIVILEGE_BROADCAST: KnownGamingPrivileges = KnownGamingPrivileges(190i32);
pub const XPRIVILEGE_VIEW_FRIENDS_LIST: KnownGamingPrivileges = KnownGamingPrivileges(197i32);
pub const XPRIVILEGE_GAME_DVR: KnownGamingPrivileges = KnownGamingPrivileges(198i32);
pub const XPRIVILEGE_SHARE_KINECT_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(199i32);
pub const XPRIVILEGE_MULTIPLAYER_PARTIES: KnownGamingPrivileges = KnownGamingPrivileges(203i32);
pub const XPRIVILEGE_COMMUNICATION_VOICE_INGAME: KnownGamingPrivileges = KnownGamingPrivileges(205i32);
pub const XPRIVILEGE_COMMUNICATION_VOICE_SKYPE: KnownGamingPrivileges = KnownGamingPrivileges(206i32);
pub const XPRIVILEGE_CLOUD_GAMING_MANAGE_SESSION: KnownGamingPrivileges = KnownGamingPrivileges(207i32);
pub const XPRIVILEGE_CLOUD_GAMING_JOIN_SESSION: KnownGamingPrivileges = KnownGamingPrivileges(208i32);
pub const XPRIVILEGE_CLOUD_SAVED_GAMES: KnownGamingPrivileges = KnownGamingPrivileges(209i32);
pub const XPRIVILEGE_SHARE_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(211i32);
pub const XPRIVILEGE_PREMIUM_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(214i32);
pub const XPRIVILEGE_SUBSCRIPTION_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(219i32);
pub const XPRIVILEGE_SOCIAL_NETWORK_SHARING: KnownGamingPrivileges = KnownGamingPrivileges(220i32);
pub const XPRIVILEGE_PREMIUM_VIDEO: KnownGamingPrivileges = KnownGamingPrivileges(224i32);
pub const XPRIVILEGE_VIDEO_COMMUNICATIONS: KnownGamingPrivileges = KnownGamingPrivileges(235i32);
pub const XPRIVILEGE_PURCHASE_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(245i32);
pub const XPRIVILEGE_USER_CREATED_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(247i32);
pub const XPRIVILEGE_PROFILE_VIEWING: KnownGamingPrivileges = KnownGamingPrivileges(249i32);
pub const XPRIVILEGE_COMMUNICATIONS: KnownGamingPrivileges = KnownGamingPrivileges(252i32);
pub const XPRIVILEGE_MULTIPLAYER_SESSIONS: KnownGamingPrivileges = KnownGamingPrivileges(254i32);
pub const XPRIVILEGE_ADD_FRIEND: KnownGamingPrivileges = KnownGamingPrivileges(255i32);
impl ::std::convert::From<i32> for KnownGamingPrivileges {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KnownGamingPrivileges {
    type Abi = Self;
}
pub type PlayerPickerUICompletionRoutine = unsafe extern "system" fn(returncode: ::windows::runtime::HRESULT, context: *const ::std::ffi::c_void, selectedxuids: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, selectedxuidscount: usize);
#[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ProcessPendingGameUI<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(waitforcompletion: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessPendingGameUI(waitforcompletion: super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        ProcessPendingGameUI(waitforcompletion.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ReleaseExclusiveCpuSets() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseExclusiveCpuSets() -> ::windows::runtime::HRESULT;
        }
        ReleaseExclusiveCpuSets().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowChangeFriendRelationshipUI<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(targetuserxuid: Param0, completionroutine: ::std::option::Option<GameUICompletionRoutine>, context: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowChangeFriendRelationshipUI(targetuserxuid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ShowChangeFriendRelationshipUI(targetuserxuid.into_param().abi(), ::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowChangeFriendRelationshipUIForUser<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(user: Param0, targetuserxuid: Param1, completionroutine: ::std::option::Option<GameUICompletionRoutine>, context: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowChangeFriendRelationshipUIForUser(user: ::windows::runtime::RawPtr, targetuserxuid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ShowChangeFriendRelationshipUIForUser(user.into_param().abi(), targetuserxuid.into_param().abi(), ::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowCustomizeUserProfileUI(completionroutine: ::std::option::Option<GameUICompletionRoutine>, context: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowCustomizeUserProfileUI(completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ShowCustomizeUserProfileUI(::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowCustomizeUserProfileUIForUser<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(user: Param0, completionroutine: ::std::option::Option<GameUICompletionRoutine>, context: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowCustomizeUserProfileUIForUser(user: ::windows::runtime::RawPtr, completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ShowCustomizeUserProfileUIForUser(user.into_param().abi(), ::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowFindFriendsUI(completionroutine: ::std::option::Option<GameUICompletionRoutine>, context: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowFindFriendsUI(completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ShowFindFriendsUI(::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowFindFriendsUIForUser<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(user: Param0, completionroutine: ::std::option::Option<GameUICompletionRoutine>, context: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowFindFriendsUIForUser(user: ::windows::runtime::RawPtr, completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ShowFindFriendsUIForUser(user.into_param().abi(), ::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowGameInfoUI(titleid: u32, completionroutine: ::std::option::Option<GameUICompletionRoutine>, context: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInfoUI(titleid: u32, completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ShowGameInfoUI(::std::mem::transmute(titleid), ::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowGameInfoUIForUser<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(user: Param0, titleid: u32, completionroutine: ::std::option::Option<GameUICompletionRoutine>, context: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInfoUIForUser(user: ::windows::runtime::RawPtr, titleid: u32, completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ShowGameInfoUIForUser(user.into_param().abi(), ::std::mem::transmute(titleid), ::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowGameInviteUI<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
    serviceconfigurationid: Param0,
    sessiontemplatename: Param1,
    sessionid: Param2,
    invitationdisplaytext: Param3,
    completionroutine: ::std::option::Option<GameUICompletionRoutine>,
    context: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInviteUI(serviceconfigurationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, sessiontemplatename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, sessionid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, invitationdisplaytext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ShowGameInviteUI(serviceconfigurationid.into_param().abi(), sessiontemplatename.into_param().abi(), sessionid.into_param().abi(), invitationdisplaytext.into_param().abi(), ::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowGameInviteUIForUser<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
    user: Param0,
    serviceconfigurationid: Param1,
    sessiontemplatename: Param2,
    sessionid: Param3,
    invitationdisplaytext: Param4,
    completionroutine: ::std::option::Option<GameUICompletionRoutine>,
    context: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInviteUIForUser(user: ::windows::runtime::RawPtr, serviceconfigurationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, sessiontemplatename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, sessionid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, invitationdisplaytext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ShowGameInviteUIForUser(user.into_param().abi(), serviceconfigurationid.into_param().abi(), sessiontemplatename.into_param().abi(), sessionid.into_param().abi(), invitationdisplaytext.into_param().abi(), ::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowGameInviteUIWithContext<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
    serviceconfigurationid: Param0,
    sessiontemplatename: Param1,
    sessionid: Param2,
    invitationdisplaytext: Param3,
    customactivationcontext: Param4,
    completionroutine: ::std::option::Option<GameUICompletionRoutine>,
    context: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInviteUIWithContext(
                serviceconfigurationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
                sessiontemplatename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
                sessionid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
                invitationdisplaytext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
                customactivationcontext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
                completionroutine: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        ShowGameInviteUIWithContext(serviceconfigurationid.into_param().abi(), sessiontemplatename.into_param().abi(), sessionid.into_param().abi(), invitationdisplaytext.into_param().abi(), customactivationcontext.into_param().abi(), ::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowGameInviteUIWithContextForUser<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param5: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
    user: Param0,
    serviceconfigurationid: Param1,
    sessiontemplatename: Param2,
    sessionid: Param3,
    invitationdisplaytext: Param4,
    customactivationcontext: Param5,
    completionroutine: ::std::option::Option<GameUICompletionRoutine>,
    context: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInviteUIWithContextForUser(
                user: ::windows::runtime::RawPtr,
                serviceconfigurationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
                sessiontemplatename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
                sessionid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
                invitationdisplaytext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
                customactivationcontext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
                completionroutine: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        ShowGameInviteUIWithContextForUser(user.into_param().abi(), serviceconfigurationid.into_param().abi(), sessiontemplatename.into_param().abi(), sessionid.into_param().abi(), invitationdisplaytext.into_param().abi(), customactivationcontext.into_param().abi(), ::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowPlayerPickerUI<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(promptdisplaytext: Param0, xuids: *const ::windows::runtime::HSTRING, xuidscount: usize, preselectedxuids: *const ::windows::runtime::HSTRING, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: ::std::option::Option<PlayerPickerUICompletionRoutine>, context: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowPlayerPickerUI(promptdisplaytext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, xuids: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, xuidscount: usize, preselectedxuids: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ShowPlayerPickerUI(
            promptdisplaytext.into_param().abi(),
            ::std::mem::transmute(xuids),
            ::std::mem::transmute(xuidscount),
            ::std::mem::transmute(preselectedxuids),
            ::std::mem::transmute(preselectedxuidscount),
            ::std::mem::transmute(minselectioncount),
            ::std::mem::transmute(maxselectioncount),
            ::std::mem::transmute(completionroutine),
            ::std::mem::transmute(context),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowPlayerPickerUIForUser<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
    user: Param0,
    promptdisplaytext: Param1,
    xuids: *const ::windows::runtime::HSTRING,
    xuidscount: usize,
    preselectedxuids: *const ::windows::runtime::HSTRING,
    preselectedxuidscount: usize,
    minselectioncount: usize,
    maxselectioncount: usize,
    completionroutine: ::std::option::Option<PlayerPickerUICompletionRoutine>,
    context: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowPlayerPickerUIForUser(
                user: ::windows::runtime::RawPtr,
                promptdisplaytext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
                xuids: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
                xuidscount: usize,
                preselectedxuids: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
                preselectedxuidscount: usize,
                minselectioncount: usize,
                maxselectioncount: usize,
                completionroutine: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        ShowPlayerPickerUIForUser(
            user.into_param().abi(),
            promptdisplaytext.into_param().abi(),
            ::std::mem::transmute(xuids),
            ::std::mem::transmute(xuidscount),
            ::std::mem::transmute(preselectedxuids),
            ::std::mem::transmute(preselectedxuidscount),
            ::std::mem::transmute(minselectioncount),
            ::std::mem::transmute(maxselectioncount),
            ::std::mem::transmute(completionroutine),
            ::std::mem::transmute(context),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowProfileCardUI<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(targetuserxuid: Param0, completionroutine: ::std::option::Option<GameUICompletionRoutine>, context: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowProfileCardUI(targetuserxuid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ShowProfileCardUI(targetuserxuid.into_param().abi(), ::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowProfileCardUIForUser<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(user: Param0, targetuserxuid: Param1, completionroutine: ::std::option::Option<GameUICompletionRoutine>, context: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowProfileCardUIForUser(user: ::windows::runtime::RawPtr, targetuserxuid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ShowProfileCardUIForUser(user.into_param().abi(), targetuserxuid.into_param().abi(), ::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowTitleAchievementsUI(titleid: u32, completionroutine: ::std::option::Option<GameUICompletionRoutine>, context: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowTitleAchievementsUI(titleid: u32, completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ShowTitleAchievementsUI(::std::mem::transmute(titleid), ::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowTitleAchievementsUIForUser<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(user: Param0, titleid: u32, completionroutine: ::std::option::Option<GameUICompletionRoutine>, context: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowTitleAchievementsUIForUser(user: ::windows::runtime::RawPtr, titleid: u32, completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ShowTitleAchievementsUIForUser(user.into_param().abi(), ::std::mem::transmute(titleid), ::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowUserSettingsUI(completionroutine: ::std::option::Option<GameUICompletionRoutine>, context: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowUserSettingsUI(completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ShowUserSettingsUI(::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[inline]
pub unsafe fn ShowUserSettingsUIForUser<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(user: Param0, completionroutine: ::std::option::Option<GameUICompletionRoutine>, context: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowUserSettingsUIForUser(user: ::windows::runtime::RawPtr, completionroutine: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ShowUserSettingsUIForUser(user.into_param().abi(), ::std::mem::transmute(completionroutine), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TryCancelPendingGameUI() -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TryCancelPendingGameUI() -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(TryCancelPendingGameUI())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Gaming`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XBL_IDP_AUTH_TOKEN_STATUS(pub i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_SUCCESS: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(0i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_SUCCESS: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(1i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_NO_ACCOUNT_SET: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(2i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_LOAD_MSA_ACCOUNT_FAILED: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(3i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_XBOX_VETO: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(4i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_MSA_INTERRUPT: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(5i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_NO_CONSENT: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(6i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_VIEW_NOT_SET: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(7i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_UNKNOWN: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(-1i32);
impl ::std::convert::From<i32> for XBL_IDP_AUTH_TOKEN_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XBL_IDP_AUTH_TOKEN_STATUS {
    type Abi = Self;
}
pub const XblIdpAuthManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3458421579, 22232, 18808, [134, 162, 126, 229, 112, 100, 4, 104]);
pub const XblIdpAuthTokenResult: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2672374849, 29770, 16652, [174, 43, 154, 34, 247, 199, 115, 31]);
