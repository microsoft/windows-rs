#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_ServerBackup`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWsbApplicationAsync(pub ::windows::core::IUnknown);
impl IWsbApplicationAsync {
    #[doc = "*Required features: `Win32_System_ServerBackup`*"]
    pub unsafe fn QueryStatus(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__: <::windows::core::HRESULT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
    }
    #[doc = "*Required features: `Win32_System_ServerBackup`*"]
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWsbApplicationAsync {
    type Vtable = IWsbApplicationAsync_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0843f6f7_895c_44a6_b0c2_05a5022aa3a1);
}
impl ::core::convert::From<IWsbApplicationAsync> for ::windows::core::IUnknown {
    fn from(value: IWsbApplicationAsync) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWsbApplicationAsync> for ::windows::core::IUnknown {
    fn from(value: &IWsbApplicationAsync) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWsbApplicationAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWsbApplicationAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWsbApplicationAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phrresult: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_ServerBackup`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWsbApplicationBackupSupport(pub ::windows::core::IUnknown);
impl IWsbApplicationBackupSupport {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ServerBackup`, `Win32_Foundation`*"]
    pub unsafe fn CheckConsistency<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        wszwritermetadata: Param0,
        wszcomponentname: Param1,
        wszcomponentlogicalpath: Param2,
        cvolumes: u32,
        rgwszsourcevolumepath: *const super::super::Foundation::PWSTR,
        rgwszsnapshotvolumepath: *const super::super::Foundation::PWSTR,
    ) -> ::windows::core::Result<IWsbApplicationAsync> {
        let mut result__: <IWsbApplicationAsync as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wszwritermetadata.into_param().abi(), wszcomponentname.into_param().abi(), wszcomponentlogicalpath.into_param().abi(), ::core::mem::transmute(cvolumes), ::core::mem::transmute(rgwszsourcevolumepath), ::core::mem::transmute(rgwszsnapshotvolumepath), &mut result__).from_abi::<IWsbApplicationAsync>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWsbApplicationBackupSupport {
    type Vtable = IWsbApplicationBackupSupport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1eff3510_4a27_46ad_b9e0_08332f0f4f6d);
}
impl ::core::convert::From<IWsbApplicationBackupSupport> for ::windows::core::IUnknown {
    fn from(value: IWsbApplicationBackupSupport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWsbApplicationBackupSupport> for ::windows::core::IUnknown {
    fn from(value: &IWsbApplicationBackupSupport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWsbApplicationBackupSupport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWsbApplicationBackupSupport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWsbApplicationBackupSupport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszwritermetadata: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcomponentlogicalpath: super::super::Foundation::PWSTR, cvolumes: u32, rgwszsourcevolumepath: *const super::super::Foundation::PWSTR, rgwszsnapshotvolumepath: *const super::super::Foundation::PWSTR, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_ServerBackup`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWsbApplicationRestoreSupport(pub ::windows::core::IUnknown);
impl IWsbApplicationRestoreSupport {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ServerBackup`, `Win32_Foundation`*"]
    pub unsafe fn PreRestore<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(&self, wszwritermetadata: Param0, wszcomponentname: Param1, wszcomponentlogicalpath: Param2, bnorollforward: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wszwritermetadata.into_param().abi(), wszcomponentname.into_param().abi(), wszcomponentlogicalpath.into_param().abi(), bnorollforward.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ServerBackup`, `Win32_Foundation`*"]
    pub unsafe fn PostRestore<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(&self, wszwritermetadata: Param0, wszcomponentname: Param1, wszcomponentlogicalpath: Param2, bnorollforward: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), wszwritermetadata.into_param().abi(), wszcomponentname.into_param().abi(), wszcomponentlogicalpath.into_param().abi(), bnorollforward.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_ServerBackup`, `Win32_Foundation`*"]
    pub unsafe fn OrderComponents(&self, ccomponents: u32, rgcomponentname: *const super::super::Foundation::PWSTR, rgcomponentlogicalpaths: *const super::super::Foundation::PWSTR, prgcomponentname: *mut *mut super::super::Foundation::PWSTR, prgcomponentlogicalpath: *mut *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccomponents), ::core::mem::transmute(rgcomponentname), ::core::mem::transmute(rgcomponentlogicalpaths), ::core::mem::transmute(prgcomponentname), ::core::mem::transmute(prgcomponentlogicalpath)).ok()
    }
    #[doc = "*Required features: `Win32_System_ServerBackup`*"]
    pub unsafe fn IsRollForwardSupported(&self) -> ::windows::core::Result<u8> {
        let mut result__: <u8 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u8>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWsbApplicationRestoreSupport {
    type Vtable = IWsbApplicationRestoreSupport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d3bdb38_4ee8_4718_85f9_c7dbc4ab77aa);
}
impl ::core::convert::From<IWsbApplicationRestoreSupport> for ::windows::core::IUnknown {
    fn from(value: IWsbApplicationRestoreSupport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWsbApplicationRestoreSupport> for ::windows::core::IUnknown {
    fn from(value: &IWsbApplicationRestoreSupport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWsbApplicationRestoreSupport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWsbApplicationRestoreSupport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWsbApplicationRestoreSupport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszwritermetadata: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcomponentlogicalpath: super::super::Foundation::PWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszwritermetadata: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcomponentlogicalpath: super::super::Foundation::PWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ccomponents: u32, rgcomponentname: *const super::super::Foundation::PWSTR, rgcomponentlogicalpaths: *const super::super::Foundation::PWSTR, prgcomponentname: *mut *mut super::super::Foundation::PWSTR, prgcomponentlogicalpath: *mut *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbrollforwardsupported: *mut u8) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_ServerBackup`*"]
pub const WSBAPP_ASYNC_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(7995396i32 as _);
#[doc = "*Required features: `Win32_System_ServerBackup`*"]
pub const WSB_MAX_OB_STATUS_ENTRY: u32 = 5u32;
#[doc = "*Required features: `Win32_System_ServerBackup`*"]
pub const WSB_MAX_OB_STATUS_VALUE_TYPE_PAIR: u32 = 5u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ServerBackup`, `Win32_Foundation`*"]
pub struct WSB_OB_REGISTRATION_INFO {
    pub m_wszResourceDLL: super::super::Foundation::PWSTR,
    pub m_guidSnapinId: ::windows::core::GUID,
    pub m_dwProviderName: u32,
    pub m_dwProviderIcon: u32,
    pub m_bSupportsRemoting: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl WSB_OB_REGISTRATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSB_OB_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSB_OB_REGISTRATION_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSB_OB_REGISTRATION_INFO")
            .field("m_wszResourceDLL", &self.m_wszResourceDLL)
            .field("m_guidSnapinId", &self.m_guidSnapinId)
            .field("m_dwProviderName", &self.m_dwProviderName)
            .field("m_dwProviderIcon", &self.m_dwProviderIcon)
            .field("m_bSupportsRemoting", &self.m_bSupportsRemoting)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSB_OB_REGISTRATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.m_wszResourceDLL == other.m_wszResourceDLL && self.m_guidSnapinId == other.m_guidSnapinId && self.m_dwProviderName == other.m_dwProviderName && self.m_dwProviderIcon == other.m_dwProviderIcon && self.m_bSupportsRemoting == other.m_bSupportsRemoting
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSB_OB_REGISTRATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSB_OB_REGISTRATION_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ServerBackup`, `Win32_Foundation`*"]
pub struct WSB_OB_STATUS_ENTRY {
    pub m_dwIcon: u32,
    pub m_dwStatusEntryName: u32,
    pub m_dwStatusEntryValue: u32,
    pub m_cValueTypePair: u32,
    pub m_rgValueTypePair: *mut WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSB_OB_STATUS_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSB_OB_STATUS_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSB_OB_STATUS_ENTRY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSB_OB_STATUS_ENTRY").field("m_dwIcon", &self.m_dwIcon).field("m_dwStatusEntryName", &self.m_dwStatusEntryName).field("m_dwStatusEntryValue", &self.m_dwStatusEntryValue).field("m_cValueTypePair", &self.m_cValueTypePair).field("m_rgValueTypePair", &self.m_rgValueTypePair).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSB_OB_STATUS_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.m_dwIcon == other.m_dwIcon && self.m_dwStatusEntryName == other.m_dwStatusEntryName && self.m_dwStatusEntryValue == other.m_dwStatusEntryValue && self.m_cValueTypePair == other.m_cValueTypePair && self.m_rgValueTypePair == other.m_rgValueTypePair
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSB_OB_STATUS_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSB_OB_STATUS_ENTRY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ServerBackup`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSB_OB_STATUS_ENTRY_PAIR_TYPE(pub i32);
pub const WSB_OB_ET_UNDEFINED: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(0i32);
pub const WSB_OB_ET_STRING: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(1i32);
pub const WSB_OB_ET_NUMBER: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(2i32);
pub const WSB_OB_ET_DATETIME: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(3i32);
pub const WSB_OB_ET_TIME: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(4i32);
pub const WSB_OB_ET_SIZE: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(5i32);
pub const WSB_OB_ET_MAX: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(6i32);
impl ::core::convert::From<i32> for WSB_OB_STATUS_ENTRY_PAIR_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WSB_OB_STATUS_ENTRY_PAIR_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ServerBackup`, `Win32_Foundation`*"]
pub struct WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    pub m_wszObStatusEntryPairValue: super::super::Foundation::PWSTR,
    pub m_ObStatusEntryPairType: WSB_OB_STATUS_ENTRY_PAIR_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR").field("m_wszObStatusEntryPairValue", &self.m_wszObStatusEntryPairValue).field("m_ObStatusEntryPairType", &self.m_ObStatusEntryPairType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.m_wszObStatusEntryPairValue == other.m_wszObStatusEntryPairValue && self.m_ObStatusEntryPairType == other.m_ObStatusEntryPairType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ServerBackup`, `Win32_Foundation`*"]
pub struct WSB_OB_STATUS_INFO {
    pub m_guidSnapinId: ::windows::core::GUID,
    pub m_cStatusEntry: u32,
    pub m_rgStatusEntry: *mut WSB_OB_STATUS_ENTRY,
}
#[cfg(feature = "Win32_Foundation")]
impl WSB_OB_STATUS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSB_OB_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSB_OB_STATUS_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WSB_OB_STATUS_INFO").field("m_guidSnapinId", &self.m_guidSnapinId).field("m_cStatusEntry", &self.m_cStatusEntry).field("m_rgStatusEntry", &self.m_rgStatusEntry).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSB_OB_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.m_guidSnapinId == other.m_guidSnapinId && self.m_cStatusEntry == other.m_cStatusEntry && self.m_rgStatusEntry == other.m_rgStatusEntry
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSB_OB_STATUS_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSB_OB_STATUS_INFO {
    type Abi = Self;
}
