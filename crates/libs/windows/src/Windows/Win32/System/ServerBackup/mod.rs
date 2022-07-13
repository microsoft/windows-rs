#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
#[repr(transparent)]
pub struct IWsbApplicationAsync(::windows::core::IUnknown);
impl IWsbApplicationAsync {
    pub unsafe fn QueryStatus(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueryStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HRESULT>(result__)
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Abort)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWsbApplicationAsync> for ::windows::core::IUnknown {
    fn from(value: IWsbApplicationAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWsbApplicationAsync> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWsbApplicationAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWsbApplicationAsync> for ::windows::core::IUnknown {
    fn from(value: &IWsbApplicationAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWsbApplicationAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWsbApplicationAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWsbApplicationAsync {}
impl ::core::fmt::Debug for IWsbApplicationAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWsbApplicationAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWsbApplicationAsync {
    type Vtable = IWsbApplicationAsync_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0843f6f7_895c_44a6_b0c2_05a5022aa3a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWsbApplicationAsync_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub QueryStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
#[repr(transparent)]
pub struct IWsbApplicationBackupSupport(::windows::core::IUnknown);
impl IWsbApplicationBackupSupport {
    pub unsafe fn CheckConsistency<'a, P0, P1, P2>(&self, wszwritermetadata: P0, wszcomponentname: P1, wszcomponentlogicalpath: P2, cvolumes: u32, rgwszsourcevolumepath: *const ::windows::core::PWSTR, rgwszsnapshotvolumepath: *const ::windows::core::PWSTR) -> ::windows::core::Result<IWsbApplicationAsync>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CheckConsistency)(::windows::core::Interface::as_raw(self), wszwritermetadata.into(), wszcomponentname.into(), wszcomponentlogicalpath.into(), ::core::mem::transmute(cvolumes), ::core::mem::transmute(rgwszsourcevolumepath), ::core::mem::transmute(rgwszsnapshotvolumepath), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWsbApplicationAsync>(result__)
    }
}
impl ::core::convert::From<IWsbApplicationBackupSupport> for ::windows::core::IUnknown {
    fn from(value: IWsbApplicationBackupSupport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWsbApplicationBackupSupport> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWsbApplicationBackupSupport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWsbApplicationBackupSupport> for ::windows::core::IUnknown {
    fn from(value: &IWsbApplicationBackupSupport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWsbApplicationBackupSupport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWsbApplicationBackupSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWsbApplicationBackupSupport {}
impl ::core::fmt::Debug for IWsbApplicationBackupSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWsbApplicationBackupSupport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWsbApplicationBackupSupport {
    type Vtable = IWsbApplicationBackupSupport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1eff3510_4a27_46ad_b9e0_08332f0f4f6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWsbApplicationBackupSupport_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub CheckConsistency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszwritermetadata: ::windows::core::PCWSTR, wszcomponentname: ::windows::core::PCWSTR, wszcomponentlogicalpath: ::windows::core::PCWSTR, cvolumes: u32, rgwszsourcevolumepath: *const ::windows::core::PWSTR, rgwszsnapshotvolumepath: *const ::windows::core::PWSTR, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
#[repr(transparent)]
pub struct IWsbApplicationRestoreSupport(::windows::core::IUnknown);
impl IWsbApplicationRestoreSupport {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreRestore<'a, P0, P1, P2, P3>(&self, wszwritermetadata: P0, wszcomponentname: P1, wszcomponentlogicalpath: P2, bnorollforward: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<super::super::Foundation::BOOLEAN>,
    {
        (::windows::core::Interface::vtable(self).PreRestore)(::windows::core::Interface::as_raw(self), wszwritermetadata.into(), wszcomponentname.into(), wszcomponentlogicalpath.into(), bnorollforward.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostRestore<'a, P0, P1, P2, P3>(&self, wszwritermetadata: P0, wszcomponentname: P1, wszcomponentlogicalpath: P2, bnorollforward: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<super::super::Foundation::BOOLEAN>,
    {
        (::windows::core::Interface::vtable(self).PostRestore)(::windows::core::Interface::as_raw(self), wszwritermetadata.into(), wszcomponentname.into(), wszcomponentlogicalpath.into(), bnorollforward.into()).ok()
    }
    pub unsafe fn OrderComponents(&self, ccomponents: u32, rgcomponentname: *const ::windows::core::PWSTR, rgcomponentlogicalpaths: *const ::windows::core::PWSTR, prgcomponentname: *mut *mut ::windows::core::PWSTR, prgcomponentlogicalpath: *mut *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OrderComponents)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ccomponents), ::core::mem::transmute(rgcomponentname), ::core::mem::transmute(rgcomponentlogicalpaths), ::core::mem::transmute(prgcomponentname), ::core::mem::transmute(prgcomponentlogicalpath)).ok()
    }
    pub unsafe fn IsRollForwardSupported(&self) -> ::windows::core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsRollForwardSupported)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u8>(result__)
    }
}
impl ::core::convert::From<IWsbApplicationRestoreSupport> for ::windows::core::IUnknown {
    fn from(value: IWsbApplicationRestoreSupport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWsbApplicationRestoreSupport> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWsbApplicationRestoreSupport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWsbApplicationRestoreSupport> for ::windows::core::IUnknown {
    fn from(value: &IWsbApplicationRestoreSupport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWsbApplicationRestoreSupport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWsbApplicationRestoreSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWsbApplicationRestoreSupport {}
impl ::core::fmt::Debug for IWsbApplicationRestoreSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWsbApplicationRestoreSupport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWsbApplicationRestoreSupport {
    type Vtable = IWsbApplicationRestoreSupport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d3bdb38_4ee8_4718_85f9_c7dbc4ab77aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWsbApplicationRestoreSupport_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub PreRestore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszwritermetadata: ::windows::core::PCWSTR, wszcomponentname: ::windows::core::PCWSTR, wszcomponentlogicalpath: ::windows::core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PreRestore: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PostRestore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszwritermetadata: ::windows::core::PCWSTR, wszcomponentname: ::windows::core::PCWSTR, wszcomponentlogicalpath: ::windows::core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PostRestore: usize,
    pub OrderComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccomponents: u32, rgcomponentname: *const ::windows::core::PWSTR, rgcomponentlogicalpaths: *const ::windows::core::PWSTR, prgcomponentname: *mut *mut ::windows::core::PWSTR, prgcomponentlogicalpath: *mut *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub IsRollForwardSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbrollforwardsupported: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSBAPP_ASYNC_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(7995396i32);
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSB_MAX_OB_STATUS_ENTRY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSB_MAX_OB_STATUS_VALUE_TYPE_PAIR: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSB_OB_REGISTRATION_INFO {
    pub m_wszResourceDLL: ::windows::core::PWSTR,
    pub m_guidSnapinId: ::windows::core::GUID,
    pub m_dwProviderName: u32,
    pub m_dwProviderIcon: u32,
    pub m_bSupportsRemoting: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WSB_OB_REGISTRATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSB_OB_REGISTRATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSB_OB_REGISTRATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSB_OB_REGISTRATION_INFO").field("m_wszResourceDLL", &self.m_wszResourceDLL).field("m_guidSnapinId", &self.m_guidSnapinId).field("m_dwProviderName", &self.m_dwProviderName).field("m_dwProviderIcon", &self.m_dwProviderIcon).field("m_bSupportsRemoting", &self.m_bSupportsRemoting).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSB_OB_REGISTRATION_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSB_OB_REGISTRATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSB_OB_REGISTRATION_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSB_OB_REGISTRATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSB_OB_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub struct WSB_OB_STATUS_ENTRY {
    pub m_dwIcon: u32,
    pub m_dwStatusEntryName: u32,
    pub m_dwStatusEntryValue: u32,
    pub m_cValueTypePair: u32,
    pub m_rgValueTypePair: *mut WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR,
}
impl ::core::marker::Copy for WSB_OB_STATUS_ENTRY {}
impl ::core::clone::Clone for WSB_OB_STATUS_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSB_OB_STATUS_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSB_OB_STATUS_ENTRY").field("m_dwIcon", &self.m_dwIcon).field("m_dwStatusEntryName", &self.m_dwStatusEntryName).field("m_dwStatusEntryValue", &self.m_dwStatusEntryValue).field("m_cValueTypePair", &self.m_cValueTypePair).field("m_rgValueTypePair", &self.m_rgValueTypePair).finish()
    }
}
unsafe impl ::windows::core::Abi for WSB_OB_STATUS_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSB_OB_STATUS_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSB_OB_STATUS_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSB_OB_STATUS_ENTRY {}
impl ::core::default::Default for WSB_OB_STATUS_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSB_OB_STATUS_ENTRY_PAIR_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSB_OB_ET_UNDEFINED: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSB_OB_ET_STRING: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSB_OB_ET_NUMBER: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSB_OB_ET_DATETIME: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSB_OB_ET_TIME: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSB_OB_ET_SIZE: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSB_OB_ET_MAX: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(6i32);
impl ::core::marker::Copy for WSB_OB_STATUS_ENTRY_PAIR_TYPE {}
impl ::core::clone::Clone for WSB_OB_STATUS_ENTRY_PAIR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSB_OB_STATUS_ENTRY_PAIR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WSB_OB_STATUS_ENTRY_PAIR_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSB_OB_STATUS_ENTRY_PAIR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSB_OB_STATUS_ENTRY_PAIR_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub struct WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    pub m_wszObStatusEntryPairValue: ::windows::core::PWSTR,
    pub m_ObStatusEntryPairType: WSB_OB_STATUS_ENTRY_PAIR_TYPE,
}
impl ::core::marker::Copy for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {}
impl ::core::clone::Clone for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR").field("m_wszObStatusEntryPairValue", &self.m_wszObStatusEntryPairValue).field("m_ObStatusEntryPairType", &self.m_ObStatusEntryPairType).finish()
    }
}
unsafe impl ::windows::core::Abi for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {}
impl ::core::default::Default for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub struct WSB_OB_STATUS_INFO {
    pub m_guidSnapinId: ::windows::core::GUID,
    pub m_cStatusEntry: u32,
    pub m_rgStatusEntry: *mut WSB_OB_STATUS_ENTRY,
}
impl ::core::marker::Copy for WSB_OB_STATUS_INFO {}
impl ::core::clone::Clone for WSB_OB_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSB_OB_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSB_OB_STATUS_INFO").field("m_guidSnapinId", &self.m_guidSnapinId).field("m_cStatusEntry", &self.m_cStatusEntry).field("m_rgStatusEntry", &self.m_rgStatusEntry).finish()
    }
}
unsafe impl ::windows::core::Abi for WSB_OB_STATUS_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSB_OB_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSB_OB_STATUS_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSB_OB_STATUS_INFO {}
impl ::core::default::Default for WSB_OB_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
