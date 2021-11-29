#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckGamingPrivilegeSilently<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(privilegeid: u32, scope: Param1, policy: Param2) -> ::windows::core::Result<super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckGamingPrivilegeSilently(privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, hasprivilege: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CheckGamingPrivilegeSilently(::core::mem::transmute(privilegeid), scope.into_param().abi(), policy.into_param().abi(), &mut result__).from_abi::<super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckGamingPrivilegeSilentlyForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, privilegeid: u32, scope: Param2, policy: Param3) -> ::windows::core::Result<super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckGamingPrivilegeSilentlyForUser(user: ::windows::core::RawPtr, privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, hasprivilege: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CheckGamingPrivilegeSilentlyForUser(user.into_param().abi(), ::core::mem::transmute(privilegeid), scope.into_param().abi(), policy.into_param().abi(), &mut result__).from_abi::<super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[inline]
pub unsafe fn CheckGamingPrivilegeWithUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
    user: Param0,
    privilegeid: u32,
    scope: Param2,
    policy: Param3,
    friendlymessage: Param4,
    completionroutine: GameUICompletionRoutine,
    context: *const ::core::ffi::c_void,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckGamingPrivilegeWithUIForUser(user: ::windows::core::RawPtr, privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, friendlymessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CheckGamingPrivilegeWithUIForUser(user.into_param().abi(), ::core::mem::transmute(privilegeid), scope.into_param().abi(), policy.into_param().abi(), friendlymessage.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GAMESTATS_OPEN_RESULT(pub i32);
pub const GAMESTATS_OPEN_CREATED: GAMESTATS_OPEN_RESULT = GAMESTATS_OPEN_RESULT(0i32);
pub const GAMESTATS_OPEN_OPENED: GAMESTATS_OPEN_RESULT = GAMESTATS_OPEN_RESULT(1i32);
impl ::core::convert::From<i32> for GAMESTATS_OPEN_RESULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GAMESTATS_OPEN_RESULT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GAMESTATS_OPEN_TYPE(pub i32);
pub const GAMESTATS_OPEN_OPENORCREATE: GAMESTATS_OPEN_TYPE = GAMESTATS_OPEN_TYPE(0i32);
pub const GAMESTATS_OPEN_OPENONLY: GAMESTATS_OPEN_TYPE = GAMESTATS_OPEN_TYPE(1i32);
impl ::core::convert::From<i32> for GAMESTATS_OPEN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GAMESTATS_OPEN_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GAME_INSTALL_SCOPE(pub i32);
pub const GIS_NOT_INSTALLED: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(1i32);
pub const GIS_CURRENT_USER: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(2i32);
pub const GIS_ALL_USERS: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(3i32);
impl ::core::convert::From<i32> for GAME_INSTALL_SCOPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GAME_INSTALL_SCOPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GAMING_DEVICE_DEVICE_ID(pub i32);
pub const GAMING_DEVICE_DEVICE_ID_NONE: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(0i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(1988865574i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_S: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(712204761i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(1523980231i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X_DEVKIT: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(284675555i32);
impl ::core::convert::From<i32> for GAMING_DEVICE_DEVICE_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GAMING_DEVICE_DEVICE_ID {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GAMING_DEVICE_MODEL_INFORMATION {
    pub vendorId: GAMING_DEVICE_VENDOR_ID,
    pub deviceId: GAMING_DEVICE_DEVICE_ID,
}
impl GAMING_DEVICE_MODEL_INFORMATION {}
impl ::core::default::Default for GAMING_DEVICE_MODEL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GAMING_DEVICE_MODEL_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GAMING_DEVICE_MODEL_INFORMATION").field("vendorId", &self.vendorId).field("deviceId", &self.deviceId).finish()
    }
}
impl ::core::cmp::PartialEq for GAMING_DEVICE_MODEL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.vendorId == other.vendorId && self.deviceId == other.deviceId
    }
}
impl ::core::cmp::Eq for GAMING_DEVICE_MODEL_INFORMATION {}
unsafe impl ::windows::core::Abi for GAMING_DEVICE_MODEL_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GAMING_DEVICE_VENDOR_ID(pub i32);
pub const GAMING_DEVICE_VENDOR_ID_NONE: GAMING_DEVICE_VENDOR_ID = GAMING_DEVICE_VENDOR_ID(0i32);
pub const GAMING_DEVICE_VENDOR_ID_MICROSOFT: GAMING_DEVICE_VENDOR_ID = GAMING_DEVICE_VENDOR_ID(-1024700366i32);
impl ::core::convert::From<i32> for GAMING_DEVICE_VENDOR_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GAMING_DEVICE_VENDOR_ID {
    type Abi = Self;
}
pub const GameExplorer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a5ea990_3034_4d6f_9128_01f3c61022bc);
pub const GameStatistics: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbc85a2c_c0dc_4961_b6e2_d28b62c11ad4);
pub type GameUICompletionRoutine = ::core::option::Option<unsafe extern "system" fn(returncode: ::windows::core::HRESULT, context: *const ::core::ffi::c_void)>;
#[inline]
pub unsafe fn GetExpandedResourceExclusiveCpuCount() -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetExpandedResourceExclusiveCpuCount(exclusivecpucount: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        GetExpandedResourceExclusiveCpuCount(&mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetGamingDeviceModelInformation() -> ::windows::core::Result<GAMING_DEVICE_MODEL_INFORMATION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGamingDeviceModelInformation(information: *mut GAMING_DEVICE_MODEL_INFORMATION) -> ::windows::core::HRESULT;
        }
        let mut result__: <GAMING_DEVICE_MODEL_INFORMATION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        GetGamingDeviceModelInformation(&mut result__).from_abi::<GAMING_DEVICE_MODEL_INFORMATION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HasExpandedResources() -> ::windows::core::Result<super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HasExpandedResources(hasexpandedresources: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        HasExpandedResources(&mut result__).from_abi::<super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGameExplorer(pub ::windows::core::IUnknown);
impl IGameExplorer {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddGame<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::Foundation::BSTR>>(&self, bstrgdfbinarypath: Param0, bstrgameinstalldirectory: Param1, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrgdfbinarypath.into_param().abi(), bstrgameinstalldirectory.into_param().abi(), ::core::mem::transmute(installscope), ::core::mem::transmute(pguidinstanceid)).ok()
    }
    pub unsafe fn RemoveGame<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidinstanceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), guidinstanceid.into_param().abi()).ok()
    }
    pub unsafe fn UpdateGame<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidinstanceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), guidinstanceid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VerifyAccess<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::BSTR>>(&self, bstrgdfbinarypath: Param0) -> ::windows::core::Result<super::Foundation::BOOL> {
        let mut result__: <super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bstrgdfbinarypath.into_param().abi(), &mut result__).from_abi::<super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IGameExplorer {
    type Vtable = IGameExplorer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7b2fb72_d728_49b3_a5f2_18ebf5f1349e);
}
impl ::core::convert::From<IGameExplorer> for ::windows::core::IUnknown {
    fn from(value: IGameExplorer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGameExplorer> for ::windows::core::IUnknown {
    fn from(value: &IGameExplorer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGameExplorer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGameExplorer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameExplorer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrgdfbinarypath: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, bstrgameinstalldirectory: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guidinstanceid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guidinstanceid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrgdfbinarypath: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, pfhasaccess: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGameExplorer2(pub ::windows::core::IUnknown);
impl IGameExplorer2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallGame<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>>(&self, binarygdfpath: Param0, installdirectory: Param1, installscope: GAME_INSTALL_SCOPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), binarygdfpath.into_param().abi(), installdirectory.into_param().abi(), ::core::mem::transmute(installscope)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UninstallGame<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>>(&self, binarygdfpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), binarygdfpath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckAccess<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>>(&self, binarygdfpath: Param0) -> ::windows::core::Result<super::Foundation::BOOL> {
        let mut result__: <super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), binarygdfpath.into_param().abi(), &mut result__).from_abi::<super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IGameExplorer2 {
    type Vtable = IGameExplorer2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86874aa7_a1ed_450d_a7eb_b89e20b2fff3);
}
impl ::core::convert::From<IGameExplorer2> for ::windows::core::IUnknown {
    fn from(value: IGameExplorer2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGameExplorer2> for ::windows::core::IUnknown {
    fn from(value: &IGameExplorer2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGameExplorer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGameExplorer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameExplorer2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, binarygdfpath: super::Foundation::PWSTR, installdirectory: super::Foundation::PWSTR, installscope: GAME_INSTALL_SCOPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, binarygdfpath: super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, binarygdfpath: super::Foundation::PWSTR, phasaccess: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGameStatistics(pub ::windows::core::IUnknown);
impl IGameStatistics {
    pub unsafe fn GetMaxCategoryLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxNameLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxValueLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxCategories(&self) -> ::windows::core::Result<u16> {
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    pub unsafe fn GetMaxStatsPerCategory(&self) -> ::windows::core::Result<u16> {
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCategoryTitle<'a, Param1: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>>(&self, categoryindex: u16, title: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(categoryindex), title.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCategoryTitle(&self, categoryindex: u16) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(categoryindex), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStatistic(&self, categoryindex: u16, statindex: u16, pname: *mut super::Foundation::PWSTR, pvalue: *mut super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(categoryindex), ::core::mem::transmute(statindex), ::core::mem::transmute(pname), ::core::mem::transmute(pvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStatistic<'a, Param2: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>>(&self, categoryindex: u16, statindex: u16, name: Param2, value: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(categoryindex), ::core::mem::transmute(statindex), name.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::BOOL>>(&self, trackchanges: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), trackchanges.into_param().abi()).ok()
    }
    pub unsafe fn SetLastPlayedCategory(&self, categoryindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(categoryindex)).ok()
    }
    pub unsafe fn GetLastPlayedCategory(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IGameStatistics {
    type Vtable = IGameStatistics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3887c9ca_04a0_42ae_bc4c_5fa6c7721145);
}
impl ::core::convert::From<IGameStatistics> for ::windows::core::IUnknown {
    fn from(value: IGameStatistics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGameStatistics> for ::windows::core::IUnknown {
    fn from(value: &IGameStatistics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGameStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGameStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameStatistics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cch: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cch: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cch: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmax: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmax: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, categoryindex: u16, title: super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, categoryindex: u16, ptitle: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, categoryindex: u16, statindex: u16, pname: *mut super::Foundation::PWSTR, pvalue: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, categoryindex: u16, statindex: u16, name: super::Foundation::PWSTR, value: super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, trackchanges: super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, categoryindex: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcategoryindex: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGameStatisticsMgr(pub ::windows::core::IUnknown);
impl IGameStatisticsMgr {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGameStatistics<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>>(&self, gdfbinarypath: Param0, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut ::core::option::Option<IGameStatistics>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), gdfbinarypath.into_param().abi(), ::core::mem::transmute(opentype), ::core::mem::transmute(popenresult), ::core::mem::transmute(ppistats)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveGameStatistics<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>>(&self, gdfbinarypath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), gdfbinarypath.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IGameStatisticsMgr {
    type Vtable = IGameStatisticsMgr_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaff3ea11_e70e_407d_95dd_35e612c41ce2);
}
impl ::core::convert::From<IGameStatisticsMgr> for ::windows::core::IUnknown {
    fn from(value: IGameStatisticsMgr) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGameStatisticsMgr> for ::windows::core::IUnknown {
    fn from(value: &IGameStatisticsMgr) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGameStatisticsMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGameStatisticsMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameStatisticsMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, gdfbinarypath: super::Foundation::PWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, gdfbinarypath: super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXblIdpAuthManager(pub ::windows::core::IUnknown);
impl IXblIdpAuthManager {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGamerAccount<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>>(&self, msaaccountid: Param0, xuid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), msaaccountid.into_param().abi(), xuid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGamerAccount(&self, msaaccountid: *mut super::Foundation::PWSTR, xuid: *mut super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(msaaccountid), ::core::mem::transmute(xuid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAppViewInitialized<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>>(&self, appsid: Param0, msaaccountid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), appsid.into_param().abi(), msaaccountid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnvironment(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSandbox(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTokenAndSignatureWithTokenResult<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>,
        Param1: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>,
        Param2: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>,
        Param3: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>,
        Param4: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>,
        Param5: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>,
        Param6: ::windows::core::IntoParam<'a, super::Foundation::PWSTR>,
        Param9: ::windows::core::IntoParam<'a, super::Foundation::BOOL>,
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
    ) -> ::windows::core::Result<IXblIdpAuthTokenResult> {
        let mut result__: <IXblIdpAuthTokenResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(
            ::core::mem::transmute_copy(self),
            msaaccountid.into_param().abi(),
            appsid.into_param().abi(),
            msatarget.into_param().abi(),
            msapolicy.into_param().abi(),
            httpmethod.into_param().abi(),
            uri.into_param().abi(),
            headers.into_param().abi(),
            ::core::mem::transmute(body),
            ::core::mem::transmute(bodysize),
            forcerefresh.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IXblIdpAuthTokenResult>(result__)
    }
}
unsafe impl ::windows::core::Interface for IXblIdpAuthManager {
    type Vtable = IXblIdpAuthManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb5ddb08_8bbf_449b_ac21_b02ddeb3b136);
}
impl ::core::convert::From<IXblIdpAuthManager> for ::windows::core::IUnknown {
    fn from(value: IXblIdpAuthManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXblIdpAuthManager> for ::windows::core::IUnknown {
    fn from(value: &IXblIdpAuthManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXblIdpAuthManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXblIdpAuthManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, msaaccountid: super::Foundation::PWSTR, xuid: super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, msaaccountid: *mut super::Foundation::PWSTR, xuid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appsid: super::Foundation::PWSTR, msaaccountid: super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, environment: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sandbox: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, msaaccountid: super::Foundation::PWSTR, appsid: super::Foundation::PWSTR, msatarget: super::Foundation::PWSTR, msapolicy: super::Foundation::PWSTR, httpmethod: super::Foundation::PWSTR, uri: super::Foundation::PWSTR, headers: super::Foundation::PWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXblIdpAuthTokenResult(pub ::windows::core::IUnknown);
impl IXblIdpAuthTokenResult {
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<XBL_IDP_AUTH_TOKEN_STATUS> {
        let mut result__: <XBL_IDP_AUTH_TOKEN_STATUS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XBL_IDP_AUTH_TOKEN_STATUS>(result__)
    }
    pub unsafe fn GetErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__: <::windows::core::HRESULT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetToken(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSignature(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSandbox(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnvironment(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMsaAccountId(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXuid(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGamertag(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAgeGroup(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPrivileges(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMsaTarget(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMsaPolicy(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMsaAppId(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRedirect(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMessage(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHelpId(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnforcementBans(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestrictions(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTitleRestrictions(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IXblIdpAuthTokenResult {
    type Vtable = IXblIdpAuthTokenResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46ce0225_f267_4d68_b299_b2762552dec1);
}
impl ::core::convert::From<IXblIdpAuthTokenResult> for ::windows::core::IUnknown {
    fn from(value: IXblIdpAuthTokenResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXblIdpAuthTokenResult> for ::windows::core::IUnknown {
    fn from(value: &IXblIdpAuthTokenResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXblIdpAuthTokenResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXblIdpAuthTokenResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthTokenResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, status: *mut XBL_IDP_AUTH_TOKEN_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, errorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signature: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sandbox: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, environment: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, msaaccountid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xuid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, gamertag: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, agegroup: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, privileges: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, msatarget: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, msapolicy: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, msaappid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, redirect: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, message: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, helpid: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enforcementbans: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, restrictions: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, titlerestrictions: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXblIdpAuthTokenResult2(pub ::windows::core::IUnknown);
impl IXblIdpAuthTokenResult2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetModernGamertag(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetModernGamertagSuffix(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUniqueModernGamertag(&self) -> ::windows::core::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IXblIdpAuthTokenResult2 {
    type Vtable = IXblIdpAuthTokenResult2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75d760b0_60b9_412d_994f_26b2cd5f7812);
}
impl ::core::convert::From<IXblIdpAuthTokenResult2> for ::windows::core::IUnknown {
    fn from(value: IXblIdpAuthTokenResult2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXblIdpAuthTokenResult2> for ::windows::core::IUnknown {
    fn from(value: &IXblIdpAuthTokenResult2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXblIdpAuthTokenResult2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXblIdpAuthTokenResult2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthTokenResult2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for KnownGamingPrivileges {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for KnownGamingPrivileges {
    type Abi = Self;
}
pub type PlayerPickerUICompletionRoutine = ::core::option::Option<unsafe extern "system" fn(returncode: ::windows::core::HRESULT, context: *const ::core::ffi::c_void, selectedxuids: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, selectedxuidscount: usize)>;
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
#[inline]
pub unsafe fn ShowChangeFriendRelationshipUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, targetuserxuid: Param1, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowChangeFriendRelationshipUIForUser(user: ::windows::core::RawPtr, targetuserxuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowChangeFriendRelationshipUIForUser(user.into_param().abi(), targetuserxuid.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[inline]
pub unsafe fn ShowCustomizeUserProfileUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(user: Param0, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowCustomizeUserProfileUIForUser(user: ::windows::core::RawPtr, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowCustomizeUserProfileUIForUser(user.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[inline]
pub unsafe fn ShowFindFriendsUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(user: Param0, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowFindFriendsUIForUser(user: ::windows::core::RawPtr, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowFindFriendsUIForUser(user.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[inline]
pub unsafe fn ShowGameInfoUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(user: Param0, titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInfoUIForUser(user: ::windows::core::RawPtr, titleid: u32, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowGameInfoUIForUser(user.into_param().abi(), ::core::mem::transmute(titleid), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowGameInviteUI<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
    serviceconfigurationid: Param0,
    sessiontemplatename: Param1,
    sessionid: Param2,
    invitationdisplaytext: Param3,
    completionroutine: GameUICompletionRoutine,
    context: *const ::core::ffi::c_void,
) -> ::windows::core::Result<()> {
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
#[inline]
pub unsafe fn ShowGameInviteUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
    user: Param0,
    serviceconfigurationid: Param1,
    sessiontemplatename: Param2,
    sessionid: Param3,
    invitationdisplaytext: Param4,
    completionroutine: GameUICompletionRoutine,
    context: *const ::core::ffi::c_void,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInviteUIForUser(user: ::windows::core::RawPtr, serviceconfigurationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessiontemplatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, invitationdisplaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowGameInviteUIForUser(user.into_param().abi(), serviceconfigurationid.into_param().abi(), sessiontemplatename.into_param().abi(), sessionid.into_param().abi(), invitationdisplaytext.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowGameInviteUIWithContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
    serviceconfigurationid: Param0,
    sessiontemplatename: Param1,
    sessionid: Param2,
    invitationdisplaytext: Param3,
    customactivationcontext: Param4,
    completionroutine: GameUICompletionRoutine,
    context: *const ::core::ffi::c_void,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInviteUIWithContext(
                serviceconfigurationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
                sessiontemplatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
                sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
                invitationdisplaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
                customactivationcontext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
                completionroutine: ::windows::core::RawPtr,
                context: *const ::core::ffi::c_void,
            ) -> ::windows::core::HRESULT;
        }
        ShowGameInviteUIWithContext(serviceconfigurationid.into_param().abi(), sessiontemplatename.into_param().abi(), sessionid.into_param().abi(), invitationdisplaytext.into_param().abi(), customactivationcontext.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowGameInviteUIWithContextForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
    user: Param0,
    serviceconfigurationid: Param1,
    sessiontemplatename: Param2,
    sessionid: Param3,
    invitationdisplaytext: Param4,
    customactivationcontext: Param5,
    completionroutine: GameUICompletionRoutine,
    context: *const ::core::ffi::c_void,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInviteUIWithContextForUser(
                user: ::windows::core::RawPtr,
                serviceconfigurationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
                sessiontemplatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
                sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
                invitationdisplaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
                customactivationcontext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
                completionroutine: ::windows::core::RawPtr,
                context: *const ::core::ffi::c_void,
            ) -> ::windows::core::HRESULT;
        }
        ShowGameInviteUIWithContextForUser(user.into_param().abi(), serviceconfigurationid.into_param().abi(), sessiontemplatename.into_param().abi(), sessionid.into_param().abi(), invitationdisplaytext.into_param().abi(), customactivationcontext.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowPlayerPickerUI<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(promptdisplaytext: Param0, xuids: *const ::windows::core::HSTRING, xuidscount: usize, preselectedxuids: *const ::windows::core::HSTRING, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowPlayerPickerUI(promptdisplaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xuids: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xuidscount: usize, preselectedxuids: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowPlayerPickerUI(
            promptdisplaytext.into_param().abi(),
            ::core::mem::transmute(xuids),
            ::core::mem::transmute(xuidscount),
            ::core::mem::transmute(preselectedxuids),
            ::core::mem::transmute(preselectedxuidscount),
            ::core::mem::transmute(minselectioncount),
            ::core::mem::transmute(maxselectioncount),
            ::core::mem::transmute(completionroutine),
            ::core::mem::transmute(context),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowPlayerPickerUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
    user: Param0,
    promptdisplaytext: Param1,
    xuids: *const ::windows::core::HSTRING,
    xuidscount: usize,
    preselectedxuids: *const ::windows::core::HSTRING,
    preselectedxuidscount: usize,
    minselectioncount: usize,
    maxselectioncount: usize,
    completionroutine: PlayerPickerUICompletionRoutine,
    context: *const ::core::ffi::c_void,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowPlayerPickerUIForUser(user: ::windows::core::RawPtr, promptdisplaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xuids: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xuidscount: usize, preselectedxuids: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowPlayerPickerUIForUser(
            user.into_param().abi(),
            promptdisplaytext.into_param().abi(),
            ::core::mem::transmute(xuids),
            ::core::mem::transmute(xuidscount),
            ::core::mem::transmute(preselectedxuids),
            ::core::mem::transmute(preselectedxuidscount),
            ::core::mem::transmute(minselectioncount),
            ::core::mem::transmute(maxselectioncount),
            ::core::mem::transmute(completionroutine),
            ::core::mem::transmute(context),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[inline]
pub unsafe fn ShowProfileCardUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, targetuserxuid: Param1, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowProfileCardUIForUser(user: ::windows::core::RawPtr, targetuserxuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowProfileCardUIForUser(user.into_param().abi(), targetuserxuid.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[inline]
pub unsafe fn ShowTitleAchievementsUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(user: Param0, titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowTitleAchievementsUIForUser(user: ::windows::core::RawPtr, titleid: u32, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowTitleAchievementsUIForUser(user.into_param().abi(), ::core::mem::transmute(titleid), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[inline]
pub unsafe fn ShowUserSettingsUIForUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(user: Param0, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowUserSettingsUIForUser(user: ::windows::core::RawPtr, completionroutine: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        ShowUserSettingsUIForUser(user.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for XBL_IDP_AUTH_TOKEN_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XBL_IDP_AUTH_TOKEN_STATUS {
    type Abi = Self;
}
pub const XblIdpAuthManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce23534b_56d8_4978_86a2_7ee570640468);
pub const XblIdpAuthTokenResult: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f493441_744a_410c_ae2b_9a22f7c7731f);
