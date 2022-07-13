#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APPLICATIONTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const LOCAL_APPLICATIONTYPE: APPLICATIONTYPE = APPLICATIONTYPE(0i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const CLUSTERRESOURCE_APPLICATIONTYPE: APPLICATIONTYPE = APPLICATIONTYPE(1i32);
impl ::core::marker::Copy for APPLICATIONTYPE {}
impl ::core::clone::Clone for APPLICATIONTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPLICATIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APPLICATIONTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for APPLICATIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPLICATIONTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUTHENTICATION_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const NO_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const INCOMING_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const MUTUAL_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(2i32);
impl ::core::marker::Copy for AUTHENTICATION_LEVEL {}
impl ::core::clone::Clone for AUTHENTICATION_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHENTICATION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AUTHENTICATION_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUTHENTICATION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHENTICATION_LEVEL").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub struct BOID {
    pub rgb: [u8; 16],
}
impl ::core::marker::Copy for BOID {}
impl ::core::clone::Clone for BOID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BOID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BOID").field("rgb", &self.rgb).finish()
    }
}
unsafe impl ::windows::core::Abi for BOID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BOID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BOID>()) == 0 }
    }
}
impl ::core::cmp::Eq for BOID {}
impl ::core::default::Default for BOID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CLSID_MSDtcTransaction: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39f8d76b_0928_11d1_97df_00c04fb9618a);
pub const CLSID_MSDtcTransactionManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b18ab61_091d_11d1_97df_00c04fb9618a);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCINSTALL_E_CLIENT_ALREADY_INSTALLED: i32 = 384i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCINSTALL_E_SERVER_ALREADY_INSTALLED: i32 = 385i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type DTC_GET_TRANSACTION_MANAGER = ::core::option::Option<unsafe extern "system" fn(pszhost: ::windows::core::PCSTR, psztmname: ::windows::core::PCSTR, rid: *const ::windows::core::GUID, dwreserved1: u32, wcbreserved2: u16, pvreserved2: *mut ::core::ffi::c_void, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type DTC_GET_TRANSACTION_MANAGER_EX_A = ::core::option::Option<unsafe extern "system" fn(i_pszhost: ::windows::core::PCSTR, i_psztmname: ::windows::core::PCSTR, i_riid: *const ::windows::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type DTC_GET_TRANSACTION_MANAGER_EX_W = ::core::option::Option<unsafe extern "system" fn(i_pwszhost: ::windows::core::PCWSTR, i_pwsztmname: ::windows::core::PCWSTR, i_riid: *const ::windows::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type DTC_INSTALL_CLIENT = ::core::option::Option<unsafe extern "system" fn(i_pszremotetmhostname: *mut i8, i_dwprotocol: u32, i_dwoverwrite: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_INSTALL_OVERWRITE_CLIENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_INSTALL_OVERWRITE_SERVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DTC_STATUS_(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_UNKNOWN: DTC_STATUS_ = DTC_STATUS_(0i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_STARTING: DTC_STATUS_ = DTC_STATUS_(1i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_STARTED: DTC_STATUS_ = DTC_STATUS_(2i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_PAUSING: DTC_STATUS_ = DTC_STATUS_(3i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_PAUSED: DTC_STATUS_ = DTC_STATUS_(4i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_CONTINUING: DTC_STATUS_ = DTC_STATUS_(5i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_STOPPING: DTC_STATUS_ = DTC_STATUS_(6i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_STOPPED: DTC_STATUS_ = DTC_STATUS_(7i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_E_CANTCONTROL: DTC_STATUS_ = DTC_STATUS_(8i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_FAILED: DTC_STATUS_ = DTC_STATUS_(9i32);
impl ::core::marker::Copy for DTC_STATUS_ {}
impl ::core::clone::Clone for DTC_STATUS_ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DTC_STATUS_ {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DTC_STATUS_ {
    type Abi = Self;
}
impl ::core::fmt::Debug for DTC_STATUS_ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DTC_STATUS_").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[inline]
pub unsafe fn DtcGetTransactionManager<'a, P0, P1>(i_pszhost: P0, i_psztmname: P1, i_riid: *const ::windows::core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DtcGetTransactionManager(i_pszhost: ::windows::core::PCSTR, i_psztmname: ::windows::core::PCSTR, i_riid: *const ::windows::core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    DtcGetTransactionManager(i_pszhost.into(), i_psztmname.into(), ::core::mem::transmute(i_riid), i_dwreserved1, i_wcbreserved2, ::core::mem::transmute(i_pvreserved2), ::core::mem::transmute(o_ppvobject)).ok()
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[inline]
pub unsafe fn DtcGetTransactionManagerC<'a, P0, P1>(i_pszhost: P0, i_psztmname: P1, i_riid: *const ::windows::core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DtcGetTransactionManagerC(i_pszhost: ::windows::core::PCSTR, i_psztmname: ::windows::core::PCSTR, i_riid: *const ::windows::core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    DtcGetTransactionManagerC(i_pszhost.into(), i_psztmname.into(), ::core::mem::transmute(i_riid), i_dwreserved1, i_wcbreserved2, ::core::mem::transmute(i_pvreserved2), ::core::mem::transmute(o_ppvobject)).ok()
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[inline]
pub unsafe fn DtcGetTransactionManagerExA<'a, P0, P1>(i_pszhost: P0, i_psztmname: P1, i_riid: *const ::windows::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DtcGetTransactionManagerExA(i_pszhost: ::windows::core::PCSTR, i_psztmname: ::windows::core::PCSTR, i_riid: *const ::windows::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    DtcGetTransactionManagerExA(i_pszhost.into(), i_psztmname.into(), ::core::mem::transmute(i_riid), i_grfoptions, ::core::mem::transmute(i_pvconfigparams), ::core::mem::transmute(o_ppvobject)).ok()
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[inline]
pub unsafe fn DtcGetTransactionManagerExW<'a, P0, P1>(i_pwszhost: P0, i_pwsztmname: P1, i_riid: *const ::windows::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DtcGetTransactionManagerExW(i_pwszhost: ::windows::core::PCWSTR, i_pwsztmname: ::windows::core::PCWSTR, i_riid: *const ::windows::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    DtcGetTransactionManagerExW(i_pwszhost.into(), i_pwsztmname.into(), ::core::mem::transmute(i_riid), i_grfoptions, ::core::mem::transmute(i_pvconfigparams), ::core::mem::transmute(o_ppvobject)).ok()
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcLuConfigure(::windows::core::IUnknown);
impl IDtcLuConfigure {
    pub unsafe fn Add(&self, puclupair: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(puclupair)), puclupair.len() as _).ok()
    }
    pub unsafe fn Delete(&self, puclupair: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(puclupair)), puclupair.len() as _).ok()
    }
}
impl ::core::convert::From<IDtcLuConfigure> for ::windows::core::IUnknown {
    fn from(value: IDtcLuConfigure) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcLuConfigure> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcLuConfigure) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuConfigure> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuConfigure) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcLuConfigure {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuConfigure {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuConfigure {}
impl ::core::fmt::Debug for IDtcLuConfigure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuConfigure").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuConfigure {
    type Vtable = IDtcLuConfigure_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e760_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuConfigure_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcLuRecovery(::windows::core::IUnknown);
impl IDtcLuRecovery {}
impl ::core::convert::From<IDtcLuRecovery> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRecovery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcLuRecovery> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcLuRecovery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRecovery> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRecovery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcLuRecovery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecovery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecovery {}
impl ::core::fmt::Debug for IDtcLuRecovery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecovery").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRecovery {
    type Vtable = IDtcLuRecovery_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac2b8ad2_d6f0_11d0_b386_00a0c9083365);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecovery_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcLuRecoveryFactory(::windows::core::IUnknown);
impl IDtcLuRecoveryFactory {
    pub unsafe fn Create(&self, puclupair: &[u8]) -> ::windows::core::Result<IDtcLuRecovery> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(puclupair)), puclupair.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDtcLuRecovery>(result__)
    }
}
impl ::core::convert::From<IDtcLuRecoveryFactory> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRecoveryFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcLuRecoveryFactory> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcLuRecoveryFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRecoveryFactory> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRecoveryFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcLuRecoveryFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecoveryFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecoveryFactory {}
impl ::core::fmt::Debug for IDtcLuRecoveryFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecoveryFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRecoveryFactory {
    type Vtable = IDtcLuRecoveryFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e762_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryFactory_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32, pprecovery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByDtc(::windows::core::IUnknown);
impl IDtcLuRecoveryInitiatedByDtc {
    pub unsafe fn GetWork(&self, pwork: *mut _DtcLu_LocalRecovery_Work, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetWork)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwork), ::core::mem::transmute(ppv)).ok()
    }
}
impl ::core::convert::From<IDtcLuRecoveryInitiatedByDtc> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByDtc) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcLuRecoveryInitiatedByDtc> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcLuRecoveryInitiatedByDtc) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRecoveryInitiatedByDtc> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByDtc) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcLuRecoveryInitiatedByDtc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecoveryInitiatedByDtc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecoveryInitiatedByDtc {}
impl ::core::fmt::Debug for IDtcLuRecoveryInitiatedByDtc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecoveryInitiatedByDtc").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRecoveryInitiatedByDtc {
    type Vtable = IDtcLuRecoveryInitiatedByDtc_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e764_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByDtc_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetWork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwork: *mut _DtcLu_LocalRecovery_Work, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByDtcStatusWork(::windows::core::IUnknown);
impl IDtcLuRecoveryInitiatedByDtcStatusWork {
    pub unsafe fn HandleCheckLuStatus(&self, lrecoveryseqnum: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HandleCheckLuStatus)(::windows::core::Interface::as_raw(self), lrecoveryseqnum).ok()
    }
}
impl ::core::convert::From<IDtcLuRecoveryInitiatedByDtcStatusWork> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByDtcStatusWork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcLuRecoveryInitiatedByDtcStatusWork> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcLuRecoveryInitiatedByDtcStatusWork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRecoveryInitiatedByDtcStatusWork> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByDtcStatusWork) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcLuRecoveryInitiatedByDtcStatusWork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecoveryInitiatedByDtcStatusWork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecoveryInitiatedByDtcStatusWork {}
impl ::core::fmt::Debug for IDtcLuRecoveryInitiatedByDtcStatusWork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecoveryInitiatedByDtcStatusWork").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRecoveryInitiatedByDtcStatusWork {
    type Vtable = IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e766_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub HandleCheckLuStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lrecoveryseqnum: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByDtcTransWork(::windows::core::IUnknown);
impl IDtcLuRecoveryInitiatedByDtcTransWork {
    pub unsafe fn GetLogNameSizes(&self, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLogNameSizes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcbourlogname), ::core::mem::transmute(pcbremotelogname)).ok()
    }
    pub unsafe fn GetOurXln(&self, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOurXln)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pxln), ::core::mem::transmute(pourlogname), ::core::mem::transmute(premotelogname), ::core::mem::transmute(pdwprotocol)).ok()
    }
    pub unsafe fn HandleConfirmationFromOurXln(&self, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HandleConfirmationFromOurXln)(::windows::core::Interface::as_raw(self), confirmation).ok()
    }
    pub unsafe fn HandleTheirXlnResponse(&self, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut _DtcLu_Xln_Confirmation) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HandleTheirXlnResponse)(::windows::core::Interface::as_raw(self), xln, ::core::mem::transmute(premotelogname), cbremotelogname, dwprotocol, ::core::mem::transmute(pconfirmation)).ok()
    }
    pub unsafe fn HandleErrorFromOurXln(&self, error: _DtcLu_Xln_Error) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HandleErrorFromOurXln)(::windows::core::Interface::as_raw(self), error).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckForCompareStates(&self, fcomparestates: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CheckForCompareStates)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(fcomparestates)).ok()
    }
    pub unsafe fn GetOurTransIdSize(&self, pcbourtransid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOurTransIdSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcbourtransid)).ok()
    }
    pub unsafe fn GetOurCompareStates(&self, pourtransid: *mut u8, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOurCompareStates)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pourtransid), ::core::mem::transmute(pcomparestate)).ok()
    }
    pub unsafe fn HandleTheirCompareStatesResponse(&self, comparestate: _DtcLu_CompareState, pconfirmation: *mut _DtcLu_CompareStates_Confirmation) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HandleTheirCompareStatesResponse)(::windows::core::Interface::as_raw(self), comparestate, ::core::mem::transmute(pconfirmation)).ok()
    }
    pub unsafe fn HandleErrorFromOurCompareStates(&self, error: _DtcLu_CompareStates_Error) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HandleErrorFromOurCompareStates)(::windows::core::Interface::as_raw(self), error).ok()
    }
    pub unsafe fn ConversationLost(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConversationLost)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetRecoverySeqNum(&self, plrecoveryseqnum: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRecoverySeqNum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(plrecoveryseqnum)).ok()
    }
    pub unsafe fn ObsoleteRecoverySeqNum(&self, lnewrecoveryseqnum: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ObsoleteRecoverySeqNum)(::windows::core::Interface::as_raw(self), lnewrecoveryseqnum).ok()
    }
}
impl ::core::convert::From<IDtcLuRecoveryInitiatedByDtcTransWork> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByDtcTransWork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcLuRecoveryInitiatedByDtcTransWork> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcLuRecoveryInitiatedByDtcTransWork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRecoveryInitiatedByDtcTransWork> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByDtcTransWork) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcLuRecoveryInitiatedByDtcTransWork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecoveryInitiatedByDtcTransWork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecoveryInitiatedByDtcTransWork {}
impl ::core::fmt::Debug for IDtcLuRecoveryInitiatedByDtcTransWork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecoveryInitiatedByDtcTransWork").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRecoveryInitiatedByDtcTransWork {
    type Vtable = IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e765_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetLogNameSizes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> ::windows::core::HRESULT,
    pub GetOurXln: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::HRESULT,
    pub HandleConfirmationFromOurXln: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::HRESULT,
    pub HandleTheirXlnResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut _DtcLu_Xln_Confirmation) -> ::windows::core::HRESULT,
    pub HandleErrorFromOurXln: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, error: _DtcLu_Xln_Error) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CheckForCompareStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fcomparestates: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CheckForCompareStates: usize,
    pub GetOurTransIdSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbourtransid: *mut u32) -> ::windows::core::HRESULT,
    pub GetOurCompareStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pourtransid: *mut u8, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::HRESULT,
    pub HandleTheirCompareStatesResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparestate: _DtcLu_CompareState, pconfirmation: *mut _DtcLu_CompareStates_Confirmation) -> ::windows::core::HRESULT,
    pub HandleErrorFromOurCompareStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, error: _DtcLu_CompareStates_Error) -> ::windows::core::HRESULT,
    pub ConversationLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRecoverySeqNum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plrecoveryseqnum: *mut i32) -> ::windows::core::HRESULT,
    pub ObsoleteRecoverySeqNum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnewrecoveryseqnum: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByLu(::windows::core::IUnknown);
impl IDtcLuRecoveryInitiatedByLu {
    pub unsafe fn GetObjectToHandleWorkFromLu(&self) -> ::windows::core::Result<IDtcLuRecoveryInitiatedByLuWork> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetObjectToHandleWorkFromLu)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDtcLuRecoveryInitiatedByLuWork>(result__)
    }
}
impl ::core::convert::From<IDtcLuRecoveryInitiatedByLu> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByLu) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcLuRecoveryInitiatedByLu> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcLuRecoveryInitiatedByLu) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRecoveryInitiatedByLu> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByLu) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcLuRecoveryInitiatedByLu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecoveryInitiatedByLu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecoveryInitiatedByLu {}
impl ::core::fmt::Debug for IDtcLuRecoveryInitiatedByLu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecoveryInitiatedByLu").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRecoveryInitiatedByLu {
    type Vtable = IDtcLuRecoveryInitiatedByLu_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e768_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByLu_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetObjectToHandleWorkFromLu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwork: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByLuWork(::windows::core::IUnknown);
impl IDtcLuRecoveryInitiatedByLuWork {
    pub unsafe fn HandleTheirXln(&self, lrecoveryseqnum: i32, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut _DtcLu_Xln_Response) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HandleTheirXln)(::windows::core::Interface::as_raw(self), lrecoveryseqnum, xln, ::core::mem::transmute(premotelogname), cbremotelogname, ::core::mem::transmute(pourlogname), cbourlogname, dwprotocol, ::core::mem::transmute(presponse)).ok()
    }
    pub unsafe fn GetOurLogNameSize(&self, pcbourlogname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOurLogNameSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcbourlogname)).ok()
    }
    pub unsafe fn GetOurXln(&self, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOurXln)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pxln), ::core::mem::transmute(pourlogname), ::core::mem::transmute(pdwprotocol)).ok()
    }
    pub unsafe fn HandleConfirmationOfOurXln(&self, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HandleConfirmationOfOurXln)(::windows::core::Interface::as_raw(self), confirmation).ok()
    }
    pub unsafe fn HandleTheirCompareStates(&self, premotetransid: *mut u8, cbremotetransid: u32, comparestate: _DtcLu_CompareState, presponse: *mut _DtcLu_CompareStates_Response, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HandleTheirCompareStates)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(premotetransid), cbremotetransid, comparestate, ::core::mem::transmute(presponse), ::core::mem::transmute(pcomparestate)).ok()
    }
    pub unsafe fn HandleConfirmationOfOurCompareStates(&self, confirmation: _DtcLu_CompareStates_Confirmation) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HandleConfirmationOfOurCompareStates)(::windows::core::Interface::as_raw(self), confirmation).ok()
    }
    pub unsafe fn HandleErrorFromOurCompareStates(&self, error: _DtcLu_CompareStates_Error) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HandleErrorFromOurCompareStates)(::windows::core::Interface::as_raw(self), error).ok()
    }
    pub unsafe fn ConversationLost(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConversationLost)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDtcLuRecoveryInitiatedByLuWork> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByLuWork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcLuRecoveryInitiatedByLuWork> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcLuRecoveryInitiatedByLuWork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRecoveryInitiatedByLuWork> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByLuWork) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcLuRecoveryInitiatedByLuWork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecoveryInitiatedByLuWork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecoveryInitiatedByLuWork {}
impl ::core::fmt::Debug for IDtcLuRecoveryInitiatedByLuWork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecoveryInitiatedByLuWork").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRecoveryInitiatedByLuWork {
    type Vtable = IDtcLuRecoveryInitiatedByLuWork_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac2b8ad1_d6f0_11d0_b386_00a0c9083365);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByLuWork_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub HandleTheirXln: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lrecoveryseqnum: i32, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut _DtcLu_Xln_Response) -> ::windows::core::HRESULT,
    pub GetOurLogNameSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbourlogname: *mut u32) -> ::windows::core::HRESULT,
    pub GetOurXln: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::HRESULT,
    pub HandleConfirmationOfOurXln: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::HRESULT,
    pub HandleTheirCompareStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, premotetransid: *mut u8, cbremotetransid: u32, comparestate: _DtcLu_CompareState, presponse: *mut _DtcLu_CompareStates_Response, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::HRESULT,
    pub HandleConfirmationOfOurCompareStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, confirmation: _DtcLu_CompareStates_Confirmation) -> ::windows::core::HRESULT,
    pub HandleErrorFromOurCompareStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, error: _DtcLu_CompareStates_Error) -> ::windows::core::HRESULT,
    pub ConversationLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcLuRmEnlistment(::windows::core::IUnknown);
impl IDtcLuRmEnlistment {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unplug<'a, P0>(&self, fconversationlost: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Unplug)(::windows::core::Interface::as_raw(self), fconversationlost.into()).ok()
    }
    pub unsafe fn BackedOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BackedOut)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BackOut)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Committed(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Committed)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Forget(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Forget)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestCommit)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDtcLuRmEnlistment> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRmEnlistment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcLuRmEnlistment> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcLuRmEnlistment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRmEnlistment> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRmEnlistment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcLuRmEnlistment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRmEnlistment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRmEnlistment {}
impl ::core::fmt::Debug for IDtcLuRmEnlistment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRmEnlistment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRmEnlistment {
    type Vtable = IDtcLuRmEnlistment_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e769_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRmEnlistment_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Unplug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fconversationlost: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unplug: usize,
    pub BackedOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BackOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Committed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Forget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcLuRmEnlistmentFactory(::windows::core::IUnknown);
impl IDtcLuRmEnlistmentFactory {
    pub unsafe fn Create<'a, P0, P1>(&self, puclupair: *mut u8, cblupair: u32, pitransaction: P0, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: P1, pprmenlistment: *mut ::core::option::Option<IDtcLuRmEnlistment>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITransaction>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IDtcLuRmEnlistmentSink>>,
    {
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(puclupair), cblupair, pitransaction.into().abi(), ::core::mem::transmute(ptransid), cbtransid, prmenlistmentsink.into().abi(), ::core::mem::transmute(pprmenlistment)).ok()
    }
}
impl ::core::convert::From<IDtcLuRmEnlistmentFactory> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRmEnlistmentFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcLuRmEnlistmentFactory> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcLuRmEnlistmentFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRmEnlistmentFactory> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRmEnlistmentFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcLuRmEnlistmentFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRmEnlistmentFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRmEnlistmentFactory {}
impl ::core::fmt::Debug for IDtcLuRmEnlistmentFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRmEnlistmentFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRmEnlistmentFactory {
    type Vtable = IDtcLuRmEnlistmentFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e771_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRmEnlistmentFactory_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puclupair: *mut u8, cblupair: u32, pitransaction: *mut ::core::ffi::c_void, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: *mut ::core::ffi::c_void, pprmenlistment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcLuRmEnlistmentSink(::windows::core::IUnknown);
impl IDtcLuRmEnlistmentSink {
    pub unsafe fn AckUnplug(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AckUnplug)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TmDown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TmDown)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SessionLost(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SessionLost)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackedOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BackedOut)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BackOut)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Committed(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Committed)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Forget(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Forget)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Prepare(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Prepare)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestCommit)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDtcLuRmEnlistmentSink> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRmEnlistmentSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcLuRmEnlistmentSink> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcLuRmEnlistmentSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRmEnlistmentSink> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRmEnlistmentSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcLuRmEnlistmentSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRmEnlistmentSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRmEnlistmentSink {}
impl ::core::fmt::Debug for IDtcLuRmEnlistmentSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRmEnlistmentSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRmEnlistmentSink {
    type Vtable = IDtcLuRmEnlistmentSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e770_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRmEnlistmentSink_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub AckUnplug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TmDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SessionLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BackedOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BackOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Committed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Forget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Prepare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcLuSubordinateDtc(::windows::core::IUnknown);
impl IDtcLuSubordinateDtc {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unplug<'a, P0>(&self, fconversationlost: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Unplug)(::windows::core::Interface::as_raw(self), fconversationlost.into()).ok()
    }
    pub unsafe fn BackedOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BackedOut)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BackOut)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Committed(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Committed)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Forget(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Forget)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Prepare(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Prepare)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestCommit)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDtcLuSubordinateDtc> for ::windows::core::IUnknown {
    fn from(value: IDtcLuSubordinateDtc) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcLuSubordinateDtc> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcLuSubordinateDtc) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuSubordinateDtc> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuSubordinateDtc) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcLuSubordinateDtc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuSubordinateDtc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuSubordinateDtc {}
impl ::core::fmt::Debug for IDtcLuSubordinateDtc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuSubordinateDtc").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuSubordinateDtc {
    type Vtable = IDtcLuSubordinateDtc_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e773_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuSubordinateDtc_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Unplug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fconversationlost: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unplug: usize,
    pub BackedOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BackOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Committed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Forget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Prepare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcLuSubordinateDtcFactory(::windows::core::IUnknown);
impl IDtcLuSubordinateDtcFactory {
    pub unsafe fn Create<'a, P0, P1, P2>(&self, puclupair: *mut u8, cblupair: u32, punktransactionouter: P0, isolevel: i32, isoflags: u32, poptions: P1, pptransaction: *mut ::core::option::Option<ITransaction>, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: P2, ppsubordinatedtc: *mut ::core::option::Option<IDtcLuSubordinateDtc>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ITransactionOptions>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, IDtcLuSubordinateDtcSink>>,
    {
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(puclupair), cblupair, punktransactionouter.into().abi(), isolevel, isoflags, poptions.into().abi(), ::core::mem::transmute(pptransaction), ::core::mem::transmute(ptransid), cbtransid, psubordinatedtcsink.into().abi(), ::core::mem::transmute(ppsubordinatedtc)).ok()
    }
}
impl ::core::convert::From<IDtcLuSubordinateDtcFactory> for ::windows::core::IUnknown {
    fn from(value: IDtcLuSubordinateDtcFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcLuSubordinateDtcFactory> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcLuSubordinateDtcFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuSubordinateDtcFactory> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuSubordinateDtcFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcLuSubordinateDtcFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuSubordinateDtcFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuSubordinateDtcFactory {}
impl ::core::fmt::Debug for IDtcLuSubordinateDtcFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuSubordinateDtcFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuSubordinateDtcFactory {
    type Vtable = IDtcLuSubordinateDtcFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e775_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuSubordinateDtcFactory_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puclupair: *mut u8, cblupair: u32, punktransactionouter: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: *mut ::core::ffi::c_void, pptransaction: *mut *mut ::core::ffi::c_void, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: *mut ::core::ffi::c_void, ppsubordinatedtc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcLuSubordinateDtcSink(::windows::core::IUnknown);
impl IDtcLuSubordinateDtcSink {
    pub unsafe fn AckUnplug(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AckUnplug)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TmDown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TmDown)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SessionLost(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SessionLost)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackedOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BackedOut)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BackOut)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Committed(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Committed)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Forget(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Forget)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestCommit)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDtcLuSubordinateDtcSink> for ::windows::core::IUnknown {
    fn from(value: IDtcLuSubordinateDtcSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcLuSubordinateDtcSink> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcLuSubordinateDtcSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuSubordinateDtcSink> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuSubordinateDtcSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcLuSubordinateDtcSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuSubordinateDtcSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuSubordinateDtcSink {}
impl ::core::fmt::Debug for IDtcLuSubordinateDtcSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuSubordinateDtcSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuSubordinateDtcSink {
    type Vtable = IDtcLuSubordinateDtcSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e774_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuSubordinateDtcSink_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub AckUnplug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TmDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SessionLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BackedOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BackOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Committed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Forget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcNetworkAccessConfig(::windows::core::IUnknown);
impl IDtcNetworkAccessConfig {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnyNetworkAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetAnyNetworkAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAnyNetworkAccess<'a, P0>(&self, banynetworkaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetAnyNetworkAccess)(::windows::core::Interface::as_raw(self), banynetworkaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkAdministrationAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetNetworkAdministrationAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkAdministrationAccess<'a, P0>(&self, bnetworkadministrationaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetNetworkAdministrationAccess)(::windows::core::Interface::as_raw(self), bnetworkadministrationaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTransactionAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetNetworkTransactionAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTransactionAccess<'a, P0>(&self, bnetworktransactionaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetNetworkTransactionAccess)(::windows::core::Interface::as_raw(self), bnetworktransactionaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkClientAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetNetworkClientAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkClientAccess<'a, P0>(&self, bnetworkclientaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetNetworkClientAccess)(::windows::core::Interface::as_raw(self), bnetworkclientaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTIPAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetNetworkTIPAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTIPAccess<'a, P0>(&self, bnetworktipaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetNetworkTIPAccess)(::windows::core::Interface::as_raw(self), bnetworktipaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXAAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetXAAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXAAccess<'a, P0>(&self, bxaaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetXAAccess)(::windows::core::Interface::as_raw(self), bxaaccess.into()).ok()
    }
    pub unsafe fn RestartDtcService(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RestartDtcService)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDtcNetworkAccessConfig> for ::windows::core::IUnknown {
    fn from(value: IDtcNetworkAccessConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcNetworkAccessConfig> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcNetworkAccessConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcNetworkAccessConfig> for ::windows::core::IUnknown {
    fn from(value: &IDtcNetworkAccessConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcNetworkAccessConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcNetworkAccessConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcNetworkAccessConfig {}
impl ::core::fmt::Debug for IDtcNetworkAccessConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcNetworkAccessConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcNetworkAccessConfig {
    type Vtable = IDtcNetworkAccessConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9797c15d_a428_4291_87b6_0995031a678d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcNetworkAccessConfig_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAnyNetworkAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbanynetworkaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAnyNetworkAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAnyNetworkAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, banynetworkaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAnyNetworkAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNetworkAdministrationAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbnetworkadministrationaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNetworkAdministrationAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNetworkAdministrationAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNetworkAdministrationAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNetworkTransactionAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbnetworktransactionaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNetworkTransactionAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNetworkTransactionAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bnetworktransactionaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNetworkTransactionAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNetworkClientAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbnetworkclientaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNetworkClientAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNetworkClientAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bnetworkclientaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNetworkClientAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNetworkTIPAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbnetworktipaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNetworkTIPAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNetworkTIPAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bnetworktipaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNetworkTIPAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetXAAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbxaaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetXAAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetXAAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bxaaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetXAAccess: usize,
    pub RestartDtcService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcNetworkAccessConfig2(::windows::core::IUnknown);
impl IDtcNetworkAccessConfig2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnyNetworkAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetAnyNetworkAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAnyNetworkAccess<'a, P0>(&self, banynetworkaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetAnyNetworkAccess)(::windows::core::Interface::as_raw(self), banynetworkaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkAdministrationAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetNetworkAdministrationAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkAdministrationAccess<'a, P0>(&self, bnetworkadministrationaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetNetworkAdministrationAccess)(::windows::core::Interface::as_raw(self), bnetworkadministrationaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTransactionAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetNetworkTransactionAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTransactionAccess<'a, P0>(&self, bnetworktransactionaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetNetworkTransactionAccess)(::windows::core::Interface::as_raw(self), bnetworktransactionaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkClientAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetNetworkClientAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkClientAccess<'a, P0>(&self, bnetworkclientaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetNetworkClientAccess)(::windows::core::Interface::as_raw(self), bnetworkclientaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTIPAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetNetworkTIPAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTIPAccess<'a, P0>(&self, bnetworktipaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetNetworkTIPAccess)(::windows::core::Interface::as_raw(self), bnetworktipaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXAAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetXAAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXAAccess<'a, P0>(&self, bxaaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetXAAccess)(::windows::core::Interface::as_raw(self), bxaaccess.into()).ok()
    }
    pub unsafe fn RestartDtcService(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RestartDtcService)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkInboundAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetNetworkInboundAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkOutboundAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetNetworkOutboundAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkInboundAccess<'a, P0>(&self, binbound: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetNetworkInboundAccess)(::windows::core::Interface::as_raw(self), binbound.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkOutboundAccess<'a, P0>(&self, boutbound: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetNetworkOutboundAccess)(::windows::core::Interface::as_raw(self), boutbound.into()).ok()
    }
    pub unsafe fn GetAuthenticationLevel(&self) -> ::windows::core::Result<AUTHENTICATION_LEVEL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetAuthenticationLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<AUTHENTICATION_LEVEL>(result__)
    }
    pub unsafe fn SetAuthenticationLevel(&self, authlevel: AUTHENTICATION_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAuthenticationLevel)(::windows::core::Interface::as_raw(self), authlevel).ok()
    }
}
impl ::core::convert::From<IDtcNetworkAccessConfig2> for ::windows::core::IUnknown {
    fn from(value: IDtcNetworkAccessConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcNetworkAccessConfig2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcNetworkAccessConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcNetworkAccessConfig2> for ::windows::core::IUnknown {
    fn from(value: &IDtcNetworkAccessConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IDtcNetworkAccessConfig2> for IDtcNetworkAccessConfig {
    fn from(value: IDtcNetworkAccessConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcNetworkAccessConfig2> for &'a IDtcNetworkAccessConfig {
    fn from(value: &'a IDtcNetworkAccessConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcNetworkAccessConfig2> for IDtcNetworkAccessConfig {
    fn from(value: &IDtcNetworkAccessConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcNetworkAccessConfig2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcNetworkAccessConfig2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcNetworkAccessConfig2 {}
impl ::core::fmt::Debug for IDtcNetworkAccessConfig2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcNetworkAccessConfig2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcNetworkAccessConfig2 {
    type Vtable = IDtcNetworkAccessConfig2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7aa013b_eb7d_4f42_b41c_b2dec09ae034);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcNetworkAccessConfig2_Vtbl {
    pub base__: IDtcNetworkAccessConfig_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNetworkInboundAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbinbound: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNetworkInboundAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNetworkOutboundAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pboutbound: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNetworkOutboundAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNetworkInboundAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binbound: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNetworkInboundAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNetworkOutboundAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boutbound: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNetworkOutboundAccess: usize,
    pub GetAuthenticationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pauthlevel: *mut AUTHENTICATION_LEVEL) -> ::windows::core::HRESULT,
    pub SetAuthenticationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, authlevel: AUTHENTICATION_LEVEL) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcNetworkAccessConfig3(::windows::core::IUnknown);
impl IDtcNetworkAccessConfig3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnyNetworkAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetAnyNetworkAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAnyNetworkAccess<'a, P0>(&self, banynetworkaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetAnyNetworkAccess)(::windows::core::Interface::as_raw(self), banynetworkaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkAdministrationAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetNetworkAdministrationAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkAdministrationAccess<'a, P0>(&self, bnetworkadministrationaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNetworkAdministrationAccess)(::windows::core::Interface::as_raw(self), bnetworkadministrationaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTransactionAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetNetworkTransactionAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTransactionAccess<'a, P0>(&self, bnetworktransactionaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNetworkTransactionAccess)(::windows::core::Interface::as_raw(self), bnetworktransactionaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkClientAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetNetworkClientAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkClientAccess<'a, P0>(&self, bnetworkclientaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNetworkClientAccess)(::windows::core::Interface::as_raw(self), bnetworkclientaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTIPAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetNetworkTIPAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTIPAccess<'a, P0>(&self, bnetworktipaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNetworkTIPAccess)(::windows::core::Interface::as_raw(self), bnetworktipaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXAAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetXAAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXAAccess<'a, P0>(&self, bxaaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetXAAccess)(::windows::core::Interface::as_raw(self), bxaaccess.into()).ok()
    }
    pub unsafe fn RestartDtcService(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RestartDtcService)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkInboundAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetNetworkInboundAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkOutboundAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetNetworkOutboundAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkInboundAccess<'a, P0>(&self, binbound: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetNetworkInboundAccess)(::windows::core::Interface::as_raw(self), binbound.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkOutboundAccess<'a, P0>(&self, boutbound: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetNetworkOutboundAccess)(::windows::core::Interface::as_raw(self), boutbound.into()).ok()
    }
    pub unsafe fn GetAuthenticationLevel(&self) -> ::windows::core::Result<AUTHENTICATION_LEVEL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetAuthenticationLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<AUTHENTICATION_LEVEL>(result__)
    }
    pub unsafe fn SetAuthenticationLevel(&self, authlevel: AUTHENTICATION_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetAuthenticationLevel)(::windows::core::Interface::as_raw(self), authlevel).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLUAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLUAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLUAccess<'a, P0>(&self, bluaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetLUAccess)(::windows::core::Interface::as_raw(self), bluaccess.into()).ok()
    }
}
impl ::core::convert::From<IDtcNetworkAccessConfig3> for ::windows::core::IUnknown {
    fn from(value: IDtcNetworkAccessConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcNetworkAccessConfig3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcNetworkAccessConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcNetworkAccessConfig3> for ::windows::core::IUnknown {
    fn from(value: &IDtcNetworkAccessConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IDtcNetworkAccessConfig3> for IDtcNetworkAccessConfig {
    fn from(value: IDtcNetworkAccessConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcNetworkAccessConfig3> for &'a IDtcNetworkAccessConfig {
    fn from(value: &'a IDtcNetworkAccessConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcNetworkAccessConfig3> for IDtcNetworkAccessConfig {
    fn from(value: &IDtcNetworkAccessConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IDtcNetworkAccessConfig3> for IDtcNetworkAccessConfig2 {
    fn from(value: IDtcNetworkAccessConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcNetworkAccessConfig3> for &'a IDtcNetworkAccessConfig2 {
    fn from(value: &'a IDtcNetworkAccessConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcNetworkAccessConfig3> for IDtcNetworkAccessConfig2 {
    fn from(value: &IDtcNetworkAccessConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcNetworkAccessConfig3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcNetworkAccessConfig3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcNetworkAccessConfig3 {}
impl ::core::fmt::Debug for IDtcNetworkAccessConfig3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcNetworkAccessConfig3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcNetworkAccessConfig3 {
    type Vtable = IDtcNetworkAccessConfig3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76e4b4f3_2ca5_466b_89d5_fd218ee75b49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcNetworkAccessConfig3_Vtbl {
    pub base__: IDtcNetworkAccessConfig2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLUAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbluaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLUAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLUAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLUAccess: usize,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcToXaHelper(::windows::core::IUnknown);
impl IDtcToXaHelper {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Close<'a, P0>(&self, i_fdorecovery: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self), i_fdorecovery.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateTridToXid<'a, P0>(&self, pitransaction: P0, pguidbqual: *const ::windows::core::GUID) -> ::windows::core::Result<xid_t>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITransaction>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TranslateTridToXid)(::windows::core::Interface::as_raw(self), pitransaction.into().abi(), ::core::mem::transmute(pguidbqual), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<xid_t>(result__)
    }
}
impl ::core::convert::From<IDtcToXaHelper> for ::windows::core::IUnknown {
    fn from(value: IDtcToXaHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcToXaHelper> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcToXaHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcToXaHelper> for ::windows::core::IUnknown {
    fn from(value: &IDtcToXaHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcToXaHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcToXaHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcToXaHelper {}
impl ::core::fmt::Debug for IDtcToXaHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcToXaHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcToXaHelper {
    type Vtable = IDtcToXaHelper_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9861611_304a_11d1_9813_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaHelper_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i_fdorecovery: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Close: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TranslateTridToXid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitransaction: *mut ::core::ffi::c_void, pguidbqual: *const ::windows::core::GUID, pxid: *mut xid_t) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TranslateTridToXid: usize,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcToXaHelperFactory(::windows::core::IUnknown);
impl IDtcToXaHelperFactory {
    pub unsafe fn Create<'a, P0, P1>(&self, pszdsn: P0, pszclientdllname: P1, pguidrm: *mut ::windows::core::GUID, ppxahelper: *mut ::core::option::Option<IDtcToXaHelper>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
        P1: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), pszdsn.into(), pszclientdllname.into(), ::core::mem::transmute(pguidrm), ::core::mem::transmute(ppxahelper)).ok()
    }
}
impl ::core::convert::From<IDtcToXaHelperFactory> for ::windows::core::IUnknown {
    fn from(value: IDtcToXaHelperFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcToXaHelperFactory> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcToXaHelperFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcToXaHelperFactory> for ::windows::core::IUnknown {
    fn from(value: &IDtcToXaHelperFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcToXaHelperFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcToXaHelperFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcToXaHelperFactory {}
impl ::core::fmt::Debug for IDtcToXaHelperFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcToXaHelperFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcToXaHelperFactory {
    type Vtable = IDtcToXaHelperFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9861610_304a_11d1_9813_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaHelperFactory_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdsn: ::windows::core::PCSTR, pszclientdllname: ::windows::core::PCSTR, pguidrm: *mut ::windows::core::GUID, ppxahelper: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcToXaHelperSinglePipe(::windows::core::IUnknown);
impl IDtcToXaHelperSinglePipe {
    pub unsafe fn XARMCreate<'a, P0, P1>(&self, pszdsn: P0, pszclientdll: P1, pdwrmcookie: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
        P1: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).XARMCreate)(::windows::core::Interface::as_raw(self), pszdsn.into(), pszclientdll.into(), ::core::mem::transmute(pdwrmcookie)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConvertTridToXID(&self, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConvertTridToXID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwitrans), dwrmcookie, ::core::mem::transmute(pxid)).ok()
    }
    pub unsafe fn EnlistWithRM<'a, P0, P1>(&self, dwrmcookie: u32, i_pitransaction: P0, i_pitransres: P1) -> ::windows::core::Result<ITransactionEnlistmentAsync>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITransaction>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ITransactionResourceAsync>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnlistWithRM)(::windows::core::Interface::as_raw(self), dwrmcookie, i_pitransaction.into().abi(), i_pitransres.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransactionEnlistmentAsync>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseRMCookie<'a, P0>(&self, i_dwrmcookie: u32, i_fnormal: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).ReleaseRMCookie)(::windows::core::Interface::as_raw(self), i_dwrmcookie, i_fnormal.into())
    }
}
impl ::core::convert::From<IDtcToXaHelperSinglePipe> for ::windows::core::IUnknown {
    fn from(value: IDtcToXaHelperSinglePipe) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcToXaHelperSinglePipe> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcToXaHelperSinglePipe) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcToXaHelperSinglePipe> for ::windows::core::IUnknown {
    fn from(value: &IDtcToXaHelperSinglePipe) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcToXaHelperSinglePipe {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcToXaHelperSinglePipe {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcToXaHelperSinglePipe {}
impl ::core::fmt::Debug for IDtcToXaHelperSinglePipe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcToXaHelperSinglePipe").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcToXaHelperSinglePipe {
    type Vtable = IDtcToXaHelperSinglePipe_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47ed4971_53b3_11d1_bbb9_00c04fd658f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaHelperSinglePipe_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub XARMCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdsn: ::windows::core::PCSTR, pszclientdll: ::windows::core::PCSTR, pdwrmcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ConvertTridToXID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConvertTridToXID: usize,
    pub EnlistWithRM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwrmcookie: u32, i_pitransaction: *mut ::core::ffi::c_void, i_pitransres: *mut ::core::ffi::c_void, o_ppitransenslitment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseRMCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i_dwrmcookie: u32, i_fnormal: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseRMCookie: usize,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IDtcToXaMapper(::windows::core::IUnknown);
impl IDtcToXaMapper {
    pub unsafe fn RequestNewResourceManager<'a, P0, P1>(&self, pszdsn: P0, pszclientdllname: P1, pdwrmcookie: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
        P1: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).RequestNewResourceManager)(::windows::core::Interface::as_raw(self), pszdsn.into(), pszclientdllname.into(), ::core::mem::transmute(pdwrmcookie)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateTridToXid(&self, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TranslateTridToXid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwitransaction), dwrmcookie, ::core::mem::transmute(pxid)).ok()
    }
    pub unsafe fn EnlistResourceManager(&self, dwrmcookie: u32, pdwitransaction: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnlistResourceManager)(::windows::core::Interface::as_raw(self), dwrmcookie, ::core::mem::transmute(pdwitransaction)).ok()
    }
    pub unsafe fn ReleaseResourceManager(&self, dwrmcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseResourceManager)(::windows::core::Interface::as_raw(self), dwrmcookie).ok()
    }
}
impl ::core::convert::From<IDtcToXaMapper> for ::windows::core::IUnknown {
    fn from(value: IDtcToXaMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDtcToXaMapper> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDtcToXaMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcToXaMapper> for ::windows::core::IUnknown {
    fn from(value: &IDtcToXaMapper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDtcToXaMapper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcToXaMapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcToXaMapper {}
impl ::core::fmt::Debug for IDtcToXaMapper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcToXaMapper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDtcToXaMapper {
    type Vtable = IDtcToXaMapper_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64ffabe0_7ce9_11d0_8ce6_00c04fdc877e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaMapper_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub RequestNewResourceManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdsn: ::windows::core::PCSTR, pszclientdllname: ::windows::core::PCSTR, pdwrmcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TranslateTridToXid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TranslateTridToXid: usize,
    pub EnlistResourceManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwrmcookie: u32, pdwitransaction: *const u32) -> ::windows::core::HRESULT,
    pub ReleaseResourceManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwrmcookie: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IGetDispenser(::windows::core::IUnknown);
impl IGetDispenser {
    pub unsafe fn GetDispenser(&self, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDispenser)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject)).ok()
    }
}
impl ::core::convert::From<IGetDispenser> for ::windows::core::IUnknown {
    fn from(value: IGetDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGetDispenser> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IGetDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGetDispenser> for ::windows::core::IUnknown {
    fn from(value: &IGetDispenser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IGetDispenser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGetDispenser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetDispenser {}
impl ::core::fmt::Debug for IGetDispenser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetDispenser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGetDispenser {
    type Vtable = IGetDispenser_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc23cc370_87ef_11ce_8081_0080c758527e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetDispenser_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetDispenser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IKernelTransaction(::windows::core::IUnknown);
impl IKernelTransaction {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetHandle)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
}
impl ::core::convert::From<IKernelTransaction> for ::windows::core::IUnknown {
    fn from(value: IKernelTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IKernelTransaction> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IKernelTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IKernelTransaction> for ::windows::core::IUnknown {
    fn from(value: &IKernelTransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IKernelTransaction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IKernelTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKernelTransaction {}
impl ::core::fmt::Debug for IKernelTransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKernelTransaction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IKernelTransaction {
    type Vtable = IKernelTransaction_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79427a2b_f895_40e0_be79_b57dc82ed231);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKernelTransaction_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHandle: usize,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ILastResourceManager(::windows::core::IUnknown);
impl ILastResourceManager {
    pub unsafe fn TransactionCommitted(&self, pprepinfo: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TransactionCommitted)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pprepinfo)), pprepinfo.len() as _).ok()
    }
    pub unsafe fn RecoveryDone(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RecoveryDone)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ILastResourceManager> for ::windows::core::IUnknown {
    fn from(value: ILastResourceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILastResourceManager> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ILastResourceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILastResourceManager> for ::windows::core::IUnknown {
    fn from(value: &ILastResourceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ILastResourceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILastResourceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILastResourceManager {}
impl ::core::fmt::Debug for ILastResourceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILastResourceManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILastResourceManager {
    type Vtable = ILastResourceManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d964ad4_5b33_11d3_8a91_00c04f79eb6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILastResourceManager_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub TransactionCommitted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32) -> ::windows::core::HRESULT,
    pub RecoveryDone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IPrepareInfo(::windows::core::IUnknown);
impl IPrepareInfo {
    pub unsafe fn GetPrepareInfoSize(&self, pcbprepinfo: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPrepareInfoSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcbprepinfo)).ok()
    }
    pub unsafe fn GetPrepareInfo(&self, pprepinfo: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPrepareInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pprepinfo)).ok()
    }
}
impl ::core::convert::From<IPrepareInfo> for ::windows::core::IUnknown {
    fn from(value: IPrepareInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrepareInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPrepareInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrepareInfo> for ::windows::core::IUnknown {
    fn from(value: &IPrepareInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPrepareInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrepareInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrepareInfo {}
impl ::core::fmt::Debug for IPrepareInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrepareInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrepareInfo {
    type Vtable = IPrepareInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80c7bfd0_87ee_11ce_8081_0080c758527e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrepareInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetPrepareInfoSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbprepinfo: *mut u32) -> ::windows::core::HRESULT,
    pub GetPrepareInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprepinfo: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IPrepareInfo2(::windows::core::IUnknown);
impl IPrepareInfo2 {
    pub unsafe fn GetPrepareInfoSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPrepareInfoSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPrepareInfo(&self, pprepinfo: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPrepareInfo)(::windows::core::Interface::as_raw(self), pprepinfo.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pprepinfo))).ok()
    }
}
impl ::core::convert::From<IPrepareInfo2> for ::windows::core::IUnknown {
    fn from(value: IPrepareInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrepareInfo2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPrepareInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrepareInfo2> for ::windows::core::IUnknown {
    fn from(value: &IPrepareInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPrepareInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrepareInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrepareInfo2 {}
impl ::core::fmt::Debug for IPrepareInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrepareInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrepareInfo2 {
    type Vtable = IPrepareInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fab2547_9779_11d1_b886_00c04fb9618a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrepareInfo2_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetPrepareInfoSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbprepinfo: *mut u32) -> ::windows::core::HRESULT,
    pub GetPrepareInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbprepareinfo: u32, pprepinfo: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IRMHelper(::windows::core::IUnknown);
impl IRMHelper {
    pub unsafe fn RMCount(&self, dwctotalnumberofrms: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RMCount)(::windows::core::Interface::as_raw(self), dwctotalnumberofrms).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RMInfo<'a, P0, P1, P2>(&self, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: P0, pszopenstring: P1, pszclosestring: P2, guidrmrecovery: ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::PCSTR>,
        P2: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).RMInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pxa_switch), fcdeclcallingconv.into(), pszopenstring.into(), pszclosestring.into(), ::core::mem::transmute(guidrmrecovery)).ok()
    }
}
impl ::core::convert::From<IRMHelper> for ::windows::core::IUnknown {
    fn from(value: IRMHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRMHelper> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRMHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRMHelper> for ::windows::core::IUnknown {
    fn from(value: &IRMHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRMHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRMHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRMHelper {}
impl ::core::fmt::Debug for IRMHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRMHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRMHelper {
    type Vtable = IRMHelper_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe793f6d1_f53d_11cf_a60d_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRMHelper_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub RMCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwctotalnumberofrms: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RMInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: super::super::Foundation::BOOL, pszopenstring: ::windows::core::PCSTR, pszclosestring: ::windows::core::PCSTR, guidrmrecovery: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RMInfo: usize,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IResourceManager(::windows::core::IUnknown);
impl IResourceManager {
    pub unsafe fn Enlist<'a, P0, P1>(&self, ptransaction: P0, pres: P1, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITransaction>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ITransactionResourceAsync>>,
    {
        (::windows::core::Interface::vtable(self).Enlist)(::windows::core::Interface::as_raw(self), ptransaction.into().abi(), pres.into().abi(), ::core::mem::transmute(puow), ::core::mem::transmute(pisolevel), ::core::mem::transmute(ppenlist)).ok()
    }
    pub unsafe fn Reenlist(&self, pprepinfo: &[u8], ltimeout: u32) -> ::windows::core::Result<XACTSTAT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Reenlist)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pprepinfo)), pprepinfo.len() as _, ltimeout, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTSTAT>(result__)
    }
    pub unsafe fn ReenlistmentComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReenlistmentComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDistributedTransactionManager(&self, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDistributedTransactionManager)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject)).ok()
    }
}
impl ::core::convert::From<IResourceManager> for ::windows::core::IUnknown {
    fn from(value: IResourceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IResourceManager> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IResourceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManager> for ::windows::core::IUnknown {
    fn from(value: &IResourceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IResourceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResourceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManager {}
impl ::core::fmt::Debug for IResourceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IResourceManager {
    type Vtable = IResourceManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13741d21_87eb_11ce_8081_0080c758527e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Enlist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, pres: *mut ::core::ffi::c_void, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reenlist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT,
    pub ReenlistmentComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDistributedTransactionManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IResourceManager2(::windows::core::IUnknown);
impl IResourceManager2 {
    pub unsafe fn Enlist<'a, P0, P1>(&self, ptransaction: P0, pres: P1, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITransaction>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ITransactionResourceAsync>>,
    {
        (::windows::core::Interface::vtable(self).base__.Enlist)(::windows::core::Interface::as_raw(self), ptransaction.into().abi(), pres.into().abi(), ::core::mem::transmute(puow), ::core::mem::transmute(pisolevel), ::core::mem::transmute(ppenlist)).ok()
    }
    pub unsafe fn Reenlist(&self, pprepinfo: &[u8], ltimeout: u32) -> ::windows::core::Result<XACTSTAT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Reenlist)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pprepinfo)), pprepinfo.len() as _, ltimeout, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTSTAT>(result__)
    }
    pub unsafe fn ReenlistmentComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ReenlistmentComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDistributedTransactionManager(&self, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDistributedTransactionManager)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enlist2<'a, P0, P1>(&self, ptransaction: P0, presasync: P1, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITransaction>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ITransactionResourceAsync>>,
    {
        (::windows::core::Interface::vtable(self).Enlist2)(::windows::core::Interface::as_raw(self), ptransaction.into().abi(), presasync.into().abi(), ::core::mem::transmute(puow), ::core::mem::transmute(pisolevel), ::core::mem::transmute(pxid), ::core::mem::transmute(ppenlist)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Reenlist2(&self, pxid: *const xid_t, dwtimeout: u32) -> ::windows::core::Result<XACTSTAT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Reenlist2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pxid), dwtimeout, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTSTAT>(result__)
    }
}
impl ::core::convert::From<IResourceManager2> for ::windows::core::IUnknown {
    fn from(value: IResourceManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IResourceManager2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IResourceManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManager2> for ::windows::core::IUnknown {
    fn from(value: &IResourceManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IResourceManager2> for IResourceManager {
    fn from(value: IResourceManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IResourceManager2> for &'a IResourceManager {
    fn from(value: &'a IResourceManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManager2> for IResourceManager {
    fn from(value: &IResourceManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IResourceManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResourceManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManager2 {}
impl ::core::fmt::Debug for IResourceManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManager2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IResourceManager2 {
    type Vtable = IResourceManager2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd136c69a_f749_11d1_8f47_00c04f8ee57d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager2_Vtbl {
    pub base__: IResourceManager_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Enlist2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, presasync: *mut ::core::ffi::c_void, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enlist2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Reenlist2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxid: *const xid_t, dwtimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Reenlist2: usize,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IResourceManagerFactory(::windows::core::IUnknown);
impl IResourceManagerFactory {
    pub unsafe fn Create<'a, P0, P1>(&self, pguidrm: *const ::windows::core::GUID, pszrmname: P0, piresmgrsink: P1) -> ::windows::core::Result<IResourceManager>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IResourceManagerSink>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidrm), pszrmname.into(), piresmgrsink.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IResourceManager>(result__)
    }
}
impl ::core::convert::From<IResourceManagerFactory> for ::windows::core::IUnknown {
    fn from(value: IResourceManagerFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IResourceManagerFactory> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IResourceManagerFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManagerFactory> for ::windows::core::IUnknown {
    fn from(value: &IResourceManagerFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IResourceManagerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResourceManagerFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManagerFactory {}
impl ::core::fmt::Debug for IResourceManagerFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManagerFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IResourceManagerFactory {
    type Vtable = IResourceManagerFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13741d20_87eb_11ce_8081_0080c758527e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerFactory_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidrm: *const ::windows::core::GUID, pszrmname: ::windows::core::PCSTR, piresmgrsink: *mut ::core::ffi::c_void, ppresmgr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IResourceManagerFactory2(::windows::core::IUnknown);
impl IResourceManagerFactory2 {
    pub unsafe fn Create<'a, P0, P1>(&self, pguidrm: *const ::windows::core::GUID, pszrmname: P0, piresmgrsink: P1) -> ::windows::core::Result<IResourceManager>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IResourceManagerSink>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Create)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidrm), pszrmname.into(), piresmgrsink.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IResourceManager>(result__)
    }
    pub unsafe fn CreateEx<'a, P0, P1>(&self, pguidrm: *const ::windows::core::GUID, pszrmname: P0, piresmgrsink: P1, riidrequested: *const ::windows::core::GUID, ppvresmgr: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IResourceManagerSink>>,
    {
        (::windows::core::Interface::vtable(self).CreateEx)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidrm), pszrmname.into(), piresmgrsink.into().abi(), ::core::mem::transmute(riidrequested), ::core::mem::transmute(ppvresmgr)).ok()
    }
}
impl ::core::convert::From<IResourceManagerFactory2> for ::windows::core::IUnknown {
    fn from(value: IResourceManagerFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IResourceManagerFactory2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IResourceManagerFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManagerFactory2> for ::windows::core::IUnknown {
    fn from(value: &IResourceManagerFactory2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IResourceManagerFactory2> for IResourceManagerFactory {
    fn from(value: IResourceManagerFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IResourceManagerFactory2> for &'a IResourceManagerFactory {
    fn from(value: &'a IResourceManagerFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManagerFactory2> for IResourceManagerFactory {
    fn from(value: &IResourceManagerFactory2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IResourceManagerFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResourceManagerFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManagerFactory2 {}
impl ::core::fmt::Debug for IResourceManagerFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManagerFactory2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IResourceManagerFactory2 {
    type Vtable = IResourceManagerFactory2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b369c21_fbd2_11d1_8f47_00c04f8ee57d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerFactory2_Vtbl {
    pub base__: IResourceManagerFactory_Vtbl,
    pub CreateEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidrm: *const ::windows::core::GUID, pszrmname: ::windows::core::PCSTR, piresmgrsink: *mut ::core::ffi::c_void, riidrequested: *const ::windows::core::GUID, ppvresmgr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IResourceManagerRejoinable(::windows::core::IUnknown);
impl IResourceManagerRejoinable {
    pub unsafe fn Enlist<'a, P0, P1>(&self, ptransaction: P0, pres: P1, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITransaction>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ITransactionResourceAsync>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Enlist)(::windows::core::Interface::as_raw(self), ptransaction.into().abi(), pres.into().abi(), ::core::mem::transmute(puow), ::core::mem::transmute(pisolevel), ::core::mem::transmute(ppenlist)).ok()
    }
    pub unsafe fn Reenlist(&self, pprepinfo: &[u8], ltimeout: u32) -> ::windows::core::Result<XACTSTAT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.Reenlist)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pprepinfo)), pprepinfo.len() as _, ltimeout, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTSTAT>(result__)
    }
    pub unsafe fn ReenlistmentComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ReenlistmentComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDistributedTransactionManager(&self, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDistributedTransactionManager)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enlist2<'a, P0, P1>(&self, ptransaction: P0, presasync: P1, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITransaction>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ITransactionResourceAsync>>,
    {
        (::windows::core::Interface::vtable(self).base__.Enlist2)(::windows::core::Interface::as_raw(self), ptransaction.into().abi(), presasync.into().abi(), ::core::mem::transmute(puow), ::core::mem::transmute(pisolevel), ::core::mem::transmute(pxid), ::core::mem::transmute(ppenlist)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Reenlist2(&self, pxid: *const xid_t, dwtimeout: u32) -> ::windows::core::Result<XACTSTAT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Reenlist2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pxid), dwtimeout, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTSTAT>(result__)
    }
    pub unsafe fn Rejoin(&self, pprepinfo: &[u8], ltimeout: u32) -> ::windows::core::Result<XACTSTAT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Rejoin)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pprepinfo)), pprepinfo.len() as _, ltimeout, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTSTAT>(result__)
    }
}
impl ::core::convert::From<IResourceManagerRejoinable> for ::windows::core::IUnknown {
    fn from(value: IResourceManagerRejoinable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IResourceManagerRejoinable> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IResourceManagerRejoinable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManagerRejoinable> for ::windows::core::IUnknown {
    fn from(value: &IResourceManagerRejoinable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IResourceManagerRejoinable> for IResourceManager {
    fn from(value: IResourceManagerRejoinable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IResourceManagerRejoinable> for &'a IResourceManager {
    fn from(value: &'a IResourceManagerRejoinable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManagerRejoinable> for IResourceManager {
    fn from(value: &IResourceManagerRejoinable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IResourceManagerRejoinable> for IResourceManager2 {
    fn from(value: IResourceManagerRejoinable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IResourceManagerRejoinable> for &'a IResourceManager2 {
    fn from(value: &'a IResourceManagerRejoinable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManagerRejoinable> for IResourceManager2 {
    fn from(value: &IResourceManagerRejoinable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IResourceManagerRejoinable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResourceManagerRejoinable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManagerRejoinable {}
impl ::core::fmt::Debug for IResourceManagerRejoinable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManagerRejoinable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IResourceManagerRejoinable {
    type Vtable = IResourceManagerRejoinable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f6de620_b5df_4f3e_9cfa_c8aebd05172b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerRejoinable_Vtbl {
    pub base__: IResourceManager2_Vtbl,
    pub Rejoin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IResourceManagerSink(::windows::core::IUnknown);
impl IResourceManagerSink {
    pub unsafe fn TMDown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TMDown)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IResourceManagerSink> for ::windows::core::IUnknown {
    fn from(value: IResourceManagerSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IResourceManagerSink> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IResourceManagerSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManagerSink> for ::windows::core::IUnknown {
    fn from(value: &IResourceManagerSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IResourceManagerSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResourceManagerSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManagerSink {}
impl ::core::fmt::Debug for IResourceManagerSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManagerSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IResourceManagerSink {
    type Vtable = IResourceManagerSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d563181_defb_11ce_aed1_00aa0051e2c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerSink_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub TMDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ISOFLAG(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_COMMIT_DC: ISOFLAG = ISOFLAG(1i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_COMMIT: ISOFLAG = ISOFLAG(2i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_COMMIT_NO: ISOFLAG = ISOFLAG(3i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_ABORT_DC: ISOFLAG = ISOFLAG(4i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_ABORT: ISOFLAG = ISOFLAG(8i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_ABORT_NO: ISOFLAG = ISOFLAG(12i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_DONTCARE: ISOFLAG = ISOFLAG(5i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_BOTH: ISOFLAG = ISOFLAG(10i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_NONE: ISOFLAG = ISOFLAG(15i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_OPTIMISTIC: ISOFLAG = ISOFLAG(16i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_READONLY: ISOFLAG = ISOFLAG(32i32);
impl ::core::marker::Copy for ISOFLAG {}
impl ::core::clone::Clone for ISOFLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ISOFLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ISOFLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for ISOFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISOFLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ISOLATIONLEVEL(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_UNSPECIFIED: ISOLATIONLEVEL = ISOLATIONLEVEL(-1i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_CHAOS: ISOLATIONLEVEL = ISOLATIONLEVEL(16i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_READUNCOMMITTED: ISOLATIONLEVEL = ISOLATIONLEVEL(256i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_BROWSE: ISOLATIONLEVEL = ISOLATIONLEVEL(256i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_CURSORSTABILITY: ISOLATIONLEVEL = ISOLATIONLEVEL(4096i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_READCOMMITTED: ISOLATIONLEVEL = ISOLATIONLEVEL(4096i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_REPEATABLEREAD: ISOLATIONLEVEL = ISOLATIONLEVEL(65536i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_SERIALIZABLE: ISOLATIONLEVEL = ISOLATIONLEVEL(1048576i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_ISOLATED: ISOLATIONLEVEL = ISOLATIONLEVEL(1048576i32);
impl ::core::marker::Copy for ISOLATIONLEVEL {}
impl ::core::clone::Clone for ISOLATIONLEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ISOLATIONLEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ISOLATIONLEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for ISOLATIONLEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISOLATIONLEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITipHelper(::windows::core::IUnknown);
impl ITipHelper {
    pub unsafe fn Pull(&self, i_psztxurl: *const u8) -> ::windows::core::Result<ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Pull)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(i_psztxurl), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransaction>(result__)
    }
    pub unsafe fn PullAsync<'a, P0>(&self, i_psztxurl: *const u8, i_ptippullsink: P0) -> ::windows::core::Result<ITransaction>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITipPullSink>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PullAsync)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(i_psztxurl), i_ptippullsink.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransaction>(result__)
    }
    pub unsafe fn GetLocalTmUrl(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLocalTmUrl)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u8>(result__)
    }
}
impl ::core::convert::From<ITipHelper> for ::windows::core::IUnknown {
    fn from(value: ITipHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITipHelper> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITipHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITipHelper> for ::windows::core::IUnknown {
    fn from(value: &ITipHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITipHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITipHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITipHelper {}
impl ::core::fmt::Debug for ITipHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITipHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITipHelper {
    type Vtable = ITipHelper_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17cf72d1_bac5_11d1_b1bf_00c04fc2f3ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipHelper_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i_psztxurl: *const u8, o_ppitransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PullAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i_psztxurl: *const u8, i_ptippullsink: *mut ::core::ffi::c_void, o_ppitransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLocalTmUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, o_ppszlocaltmurl: *mut *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITipPullSink(::windows::core::IUnknown);
impl ITipPullSink {
    pub unsafe fn PullComplete(&self, i_hrpull: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PullComplete)(::windows::core::Interface::as_raw(self), i_hrpull).ok()
    }
}
impl ::core::convert::From<ITipPullSink> for ::windows::core::IUnknown {
    fn from(value: ITipPullSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITipPullSink> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITipPullSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITipPullSink> for ::windows::core::IUnknown {
    fn from(value: &ITipPullSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITipPullSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITipPullSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITipPullSink {}
impl ::core::fmt::Debug for ITipPullSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITipPullSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITipPullSink {
    type Vtable = ITipPullSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17cf72d2_bac5_11d1_b1bf_00c04fc2f3ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipPullSink_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub PullComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i_hrpull: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITipTransaction(::windows::core::IUnknown);
impl ITipTransaction {
    pub unsafe fn Push(&self, i_pszremotetmurl: *const u8) -> ::windows::core::Result<::windows::core::PSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Push)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(i_pszremotetmurl), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PSTR>(result__)
    }
    pub unsafe fn GetTransactionUrl(&self) -> ::windows::core::Result<::windows::core::PSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTransactionUrl)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PSTR>(result__)
    }
}
impl ::core::convert::From<ITipTransaction> for ::windows::core::IUnknown {
    fn from(value: ITipTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITipTransaction> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITipTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITipTransaction> for ::windows::core::IUnknown {
    fn from(value: &ITipTransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITipTransaction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITipTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITipTransaction {}
impl ::core::fmt::Debug for ITipTransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITipTransaction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITipTransaction {
    type Vtable = ITipTransaction_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17cf72d0_bac5_11d1_b1bf_00c04fc2f3ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipTransaction_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i_pszremotetmurl: *const u8, o_ppszremotetxurl: *mut ::windows::core::PSTR) -> ::windows::core::HRESULT,
    pub GetTransactionUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, o_ppszlocaltxurl: *mut ::windows::core::PSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITmNodeName(::windows::core::IUnknown);
impl ITmNodeName {
    pub unsafe fn GetNodeNameSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetNodeNameSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetNodeName(&self, cbnodenamebuffersize: u32, pnodenamebuffer: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNodeName)(::windows::core::Interface::as_raw(self), cbnodenamebuffersize, ::core::mem::transmute(pnodenamebuffer)).ok()
    }
}
impl ::core::convert::From<ITmNodeName> for ::windows::core::IUnknown {
    fn from(value: ITmNodeName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITmNodeName> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITmNodeName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITmNodeName> for ::windows::core::IUnknown {
    fn from(value: &ITmNodeName) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITmNodeName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITmNodeName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITmNodeName {}
impl ::core::fmt::Debug for ITmNodeName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITmNodeName").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITmNodeName {
    type Vtable = ITmNodeName_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30274f88_6ee4_474e_9b95_7807bc9ef8cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITmNodeName_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetNodeNameSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbnodenamesize: *mut u32) -> ::windows::core::HRESULT,
    pub GetNodeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbnodenamebuffersize: u32, pnodenamebuffer: ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransaction(::windows::core::IUnknown);
impl ITransaction {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Commit<'a, P0>(&self, fretaining: P0, grftc: u32, grfrm: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self), fretaining.into(), grftc, grfrm).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Abort<'a, P0, P1>(&self, pboidreason: *const BOID, fretaining: P0, fasync: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Abort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pboidreason), fretaining.into(), fasync.into()).ok()
    }
    pub unsafe fn GetTransactionInfo(&self) -> ::windows::core::Result<XACTTRANSINFO> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTransactionInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTTRANSINFO>(result__)
    }
}
impl ::core::convert::From<ITransaction> for ::windows::core::IUnknown {
    fn from(value: ITransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransaction> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransaction> for ::windows::core::IUnknown {
    fn from(value: &ITransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransaction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransaction {}
impl ::core::fmt::Debug for ITransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransaction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransaction {
    type Vtable = ITransaction_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fb15084_af41_11ce_bd2b_204c4f4f5020);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransaction_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Commit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Abort: usize,
    pub GetTransactionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransaction2(::windows::core::IUnknown);
impl ITransaction2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Commit<'a, P0>(&self, fretaining: P0, grftc: u32, grfrm: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Commit)(::windows::core::Interface::as_raw(self), fretaining.into(), grftc, grfrm).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Abort<'a, P0, P1>(&self, pboidreason: *const BOID, fretaining: P0, fasync: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Abort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pboidreason), fretaining.into(), fasync.into()).ok()
    }
    pub unsafe fn GetTransactionInfo(&self) -> ::windows::core::Result<XACTTRANSINFO> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetTransactionInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTTRANSINFO>(result__)
    }
    pub unsafe fn CloneWithCommitDisabled(&self) -> ::windows::core::Result<ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CloneWithCommitDisabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransaction>(result__)
    }
    pub unsafe fn GetTransactionInfo2(&self) -> ::windows::core::Result<XACTTRANSINFO> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTransactionInfo2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTTRANSINFO>(result__)
    }
}
impl ::core::convert::From<ITransaction2> for ::windows::core::IUnknown {
    fn from(value: ITransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransaction2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransaction2> for ::windows::core::IUnknown {
    fn from(value: &ITransaction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ITransaction2> for ITransaction {
    fn from(value: ITransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransaction2> for &'a ITransaction {
    fn from(value: &'a ITransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransaction2> for ITransaction {
    fn from(value: &ITransaction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ITransaction2> for ITransactionCloner {
    fn from(value: ITransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransaction2> for &'a ITransactionCloner {
    fn from(value: &'a ITransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransaction2> for ITransactionCloner {
    fn from(value: &ITransaction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransaction2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransaction2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransaction2 {}
impl ::core::fmt::Debug for ITransaction2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransaction2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransaction2 {
    type Vtable = ITransaction2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34021548_0065_11d3_bac1_00c04f797be2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransaction2_Vtbl {
    pub base__: ITransactionCloner_Vtbl,
    pub GetTransactionInfo2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionCloner(::windows::core::IUnknown);
impl ITransactionCloner {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Commit<'a, P0>(&self, fretaining: P0, grftc: u32, grfrm: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Commit)(::windows::core::Interface::as_raw(self), fretaining.into(), grftc, grfrm).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Abort<'a, P0, P1>(&self, pboidreason: *const BOID, fretaining: P0, fasync: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Abort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pboidreason), fretaining.into(), fasync.into()).ok()
    }
    pub unsafe fn GetTransactionInfo(&self) -> ::windows::core::Result<XACTTRANSINFO> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetTransactionInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTTRANSINFO>(result__)
    }
    pub unsafe fn CloneWithCommitDisabled(&self) -> ::windows::core::Result<ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CloneWithCommitDisabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransaction>(result__)
    }
}
impl ::core::convert::From<ITransactionCloner> for ::windows::core::IUnknown {
    fn from(value: ITransactionCloner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionCloner> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionCloner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionCloner> for ::windows::core::IUnknown {
    fn from(value: &ITransactionCloner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ITransactionCloner> for ITransaction {
    fn from(value: ITransactionCloner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionCloner> for &'a ITransaction {
    fn from(value: &'a ITransactionCloner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionCloner> for ITransaction {
    fn from(value: &ITransactionCloner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionCloner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionCloner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionCloner {}
impl ::core::fmt::Debug for ITransactionCloner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionCloner").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionCloner {
    type Vtable = ITransactionCloner_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02656950_2152_11d0_944c_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionCloner_Vtbl {
    pub base__: ITransaction_Vtbl,
    pub CloneWithCommitDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionDispenser(::windows::core::IUnknown);
impl ITransactionDispenser {
    pub unsafe fn GetOptionsObject(&self) -> ::windows::core::Result<ITransactionOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOptionsObject)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransactionOptions>(result__)
    }
    pub unsafe fn BeginTransaction<'a, P0, P1>(&self, punkouter: P0, isolevel: i32, isoflags: u32, poptions: P1) -> ::windows::core::Result<ITransaction>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ITransactionOptions>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BeginTransaction)(::windows::core::Interface::as_raw(self), punkouter.into().abi(), isolevel, isoflags, poptions.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransaction>(result__)
    }
}
impl ::core::convert::From<ITransactionDispenser> for ::windows::core::IUnknown {
    fn from(value: ITransactionDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionDispenser> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionDispenser> for ::windows::core::IUnknown {
    fn from(value: &ITransactionDispenser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionDispenser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionDispenser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionDispenser {}
impl ::core::fmt::Debug for ITransactionDispenser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionDispenser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionDispenser {
    type Vtable = ITransactionDispenser_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a6ad9e1_23b9_11cf_ad60_00aa00a74ccd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionDispenser_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetOptionsObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BeginTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: *mut ::core::ffi::c_void, pptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionEnlistmentAsync(::windows::core::IUnknown);
impl ITransactionEnlistmentAsync {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrepareRequestDone<'a, P0>(&self, hr: ::windows::core::HRESULT, pmk: P0, pboidreason: *const BOID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::IMoniker>>,
    {
        (::windows::core::Interface::vtable(self).PrepareRequestDone)(::windows::core::Interface::as_raw(self), hr, pmk.into().abi(), ::core::mem::transmute(pboidreason)).ok()
    }
    pub unsafe fn CommitRequestDone(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CommitRequestDone)(::windows::core::Interface::as_raw(self), hr).ok()
    }
    pub unsafe fn AbortRequestDone(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AbortRequestDone)(::windows::core::Interface::as_raw(self), hr).ok()
    }
}
impl ::core::convert::From<ITransactionEnlistmentAsync> for ::windows::core::IUnknown {
    fn from(value: ITransactionEnlistmentAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionEnlistmentAsync> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionEnlistmentAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionEnlistmentAsync> for ::windows::core::IUnknown {
    fn from(value: &ITransactionEnlistmentAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionEnlistmentAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionEnlistmentAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionEnlistmentAsync {}
impl ::core::fmt::Debug for ITransactionEnlistmentAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionEnlistmentAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionEnlistmentAsync {
    type Vtable = ITransactionEnlistmentAsync_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fb15081_af41_11ce_bd2b_204c4f4f5020);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionEnlistmentAsync_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub PrepareRequestDone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, pmk: *mut ::core::ffi::c_void, pboidreason: *const BOID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrepareRequestDone: usize,
    pub CommitRequestDone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub AbortRequestDone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionExport(::windows::core::IUnknown);
impl ITransactionExport {
    pub unsafe fn Export<'a, P0>(&self, punktransaction: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Export)(::windows::core::Interface::as_raw(self), punktransaction.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetTransactionCookie<'a, P0>(&self, punktransaction: P0, rgbtransactioncookie: &mut [u8], pcbused: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).GetTransactionCookie)(::windows::core::Interface::as_raw(self), punktransaction.into().abi(), rgbtransactioncookie.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgbtransactioncookie)), ::core::mem::transmute(pcbused)).ok()
    }
}
impl ::core::convert::From<ITransactionExport> for ::windows::core::IUnknown {
    fn from(value: ITransactionExport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionExport> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionExport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionExport> for ::windows::core::IUnknown {
    fn from(value: &ITransactionExport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionExport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionExport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionExport {}
impl ::core::fmt::Debug for ITransactionExport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionExport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionExport {
    type Vtable = ITransactionExport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0141fda5_8fc0_11ce_bd18_204c4f4f5020);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionExport_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Export: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punktransaction: *mut ::core::ffi::c_void, pcbtransactioncookie: *mut u32) -> ::windows::core::HRESULT,
    pub GetTransactionCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punktransaction: *mut ::core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionExportFactory(::windows::core::IUnknown);
impl ITransactionExportFactory {
    pub unsafe fn GetRemoteClassId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRemoteClassId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn Create(&self, rgbwhereabouts: &[u8]) -> ::windows::core::Result<ITransactionExport> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), rgbwhereabouts.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgbwhereabouts)), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransactionExport>(result__)
    }
}
impl ::core::convert::From<ITransactionExportFactory> for ::windows::core::IUnknown {
    fn from(value: ITransactionExportFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionExportFactory> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionExportFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionExportFactory> for ::windows::core::IUnknown {
    fn from(value: &ITransactionExportFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionExportFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionExportFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionExportFactory {}
impl ::core::fmt::Debug for ITransactionExportFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionExportFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionExportFactory {
    type Vtable = ITransactionExportFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1cf9b53_8745_11ce_a9ba_00aa006c3706);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionExportFactory_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetRemoteClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbwhereabouts: u32, rgbwhereabouts: *const u8, ppexport: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionImport(::windows::core::IUnknown);
impl ITransactionImport {
    pub unsafe fn Import<T>(&self, rgbtransactioncookie: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).Import)(::windows::core::Interface::as_raw(self), rgbtransactioncookie.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgbtransactioncookie)), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ITransactionImport> for ::windows::core::IUnknown {
    fn from(value: ITransactionImport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionImport> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionImport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionImport> for ::windows::core::IUnknown {
    fn from(value: &ITransactionImport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionImport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionImport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionImport {}
impl ::core::fmt::Debug for ITransactionImport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionImport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionImport {
    type Vtable = ITransactionImport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1cf9b5a_8745_11ce_a9ba_00aa006c3706);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionImport_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *const u8, piid: *const ::windows::core::GUID, ppvtransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionImportWhereabouts(::windows::core::IUnknown);
impl ITransactionImportWhereabouts {
    pub unsafe fn GetWhereaboutsSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetWhereaboutsSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetWhereabouts(&self, rgbwhereabouts: &mut [u8], pcbused: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetWhereabouts)(::windows::core::Interface::as_raw(self), rgbwhereabouts.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgbwhereabouts)), ::core::mem::transmute(pcbused)).ok()
    }
}
impl ::core::convert::From<ITransactionImportWhereabouts> for ::windows::core::IUnknown {
    fn from(value: ITransactionImportWhereabouts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionImportWhereabouts> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionImportWhereabouts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionImportWhereabouts> for ::windows::core::IUnknown {
    fn from(value: &ITransactionImportWhereabouts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionImportWhereabouts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionImportWhereabouts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionImportWhereabouts {}
impl ::core::fmt::Debug for ITransactionImportWhereabouts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionImportWhereabouts").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionImportWhereabouts {
    type Vtable = ITransactionImportWhereabouts_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0141fda4_8fc0_11ce_bd18_204c4f4f5020);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionImportWhereabouts_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetWhereaboutsSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbwhereabouts: *mut u32) -> ::windows::core::HRESULT,
    pub GetWhereabouts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionLastEnlistmentAsync(::windows::core::IUnknown);
impl ITransactionLastEnlistmentAsync {
    pub unsafe fn TransactionOutcome(&self, xactstat: XACTSTAT, pboidreason: *const BOID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TransactionOutcome)(::windows::core::Interface::as_raw(self), xactstat, ::core::mem::transmute(pboidreason)).ok()
    }
}
impl ::core::convert::From<ITransactionLastEnlistmentAsync> for ::windows::core::IUnknown {
    fn from(value: ITransactionLastEnlistmentAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionLastEnlistmentAsync> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionLastEnlistmentAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionLastEnlistmentAsync> for ::windows::core::IUnknown {
    fn from(value: &ITransactionLastEnlistmentAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionLastEnlistmentAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionLastEnlistmentAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionLastEnlistmentAsync {}
impl ::core::fmt::Debug for ITransactionLastEnlistmentAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionLastEnlistmentAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionLastEnlistmentAsync {
    type Vtable = ITransactionLastEnlistmentAsync_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc82bd533_5b30_11d3_8a91_00c04f79eb6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionLastEnlistmentAsync_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub TransactionOutcome: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xactstat: XACTSTAT, pboidreason: *const BOID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionLastResourceAsync(::windows::core::IUnknown);
impl ITransactionLastResourceAsync {
    pub unsafe fn DelegateCommit(&self, grfrm: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DelegateCommit)(::windows::core::Interface::as_raw(self), grfrm).ok()
    }
    pub unsafe fn ForgetRequest(&self, pnewuow: *const BOID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ForgetRequest)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pnewuow)).ok()
    }
}
impl ::core::convert::From<ITransactionLastResourceAsync> for ::windows::core::IUnknown {
    fn from(value: ITransactionLastResourceAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionLastResourceAsync> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionLastResourceAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionLastResourceAsync> for ::windows::core::IUnknown {
    fn from(value: &ITransactionLastResourceAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionLastResourceAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionLastResourceAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionLastResourceAsync {}
impl ::core::fmt::Debug for ITransactionLastResourceAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionLastResourceAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionLastResourceAsync {
    type Vtable = ITransactionLastResourceAsync_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc82bd532_5b30_11d3_8a91_00c04f79eb6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionLastResourceAsync_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub DelegateCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfrm: u32) -> ::windows::core::HRESULT,
    pub ForgetRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewuow: *const BOID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionOptions(::windows::core::IUnknown);
impl ITransactionOptions {
    pub unsafe fn SetOptions(&self, poptions: *const XACTOPT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(poptions)).ok()
    }
    pub unsafe fn GetOptions(&self, poptions: *mut XACTOPT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(poptions)).ok()
    }
}
impl ::core::convert::From<ITransactionOptions> for ::windows::core::IUnknown {
    fn from(value: ITransactionOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionOptions> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionOptions> for ::windows::core::IUnknown {
    fn from(value: &ITransactionOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionOptions {}
impl ::core::fmt::Debug for ITransactionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionOptions {
    type Vtable = ITransactionOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a6ad9e0_23b9_11cf_ad60_00aa00a74ccd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionOptions_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poptions: *const XACTOPT) -> ::windows::core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poptions: *mut XACTOPT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionOutcomeEvents(::windows::core::IUnknown);
impl ITransactionOutcomeEvents {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Committed<'a, P0>(&self, fretaining: P0, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Committed)(::windows::core::Interface::as_raw(self), fretaining.into(), ::core::mem::transmute(pnewuow), hr).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Aborted<'a, P0>(&self, pboidreason: *const BOID, fretaining: P0, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Aborted)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pboidreason), fretaining.into(), ::core::mem::transmute(pnewuow), hr).ok()
    }
    pub unsafe fn HeuristicDecision(&self, dwdecision: u32, pboidreason: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HeuristicDecision)(::windows::core::Interface::as_raw(self), dwdecision, ::core::mem::transmute(pboidreason), hr).ok()
    }
    pub unsafe fn Indoubt(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Indoubt)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ITransactionOutcomeEvents> for ::windows::core::IUnknown {
    fn from(value: ITransactionOutcomeEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionOutcomeEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionOutcomeEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionOutcomeEvents> for ::windows::core::IUnknown {
    fn from(value: &ITransactionOutcomeEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionOutcomeEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionOutcomeEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionOutcomeEvents {}
impl ::core::fmt::Debug for ITransactionOutcomeEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionOutcomeEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionOutcomeEvents {
    type Vtable = ITransactionOutcomeEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a6ad9e2_23b9_11cf_ad60_00aa00a74ccd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionOutcomeEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Committed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Committed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Aborted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Aborted: usize,
    pub HeuristicDecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdecision: u32, pboidreason: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub Indoubt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionPhase0EnlistmentAsync(::windows::core::IUnknown);
impl ITransactionPhase0EnlistmentAsync {
    pub unsafe fn Enable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Enable)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForEnlistment(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WaitForEnlistment)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Phase0Done(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Phase0Done)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unenlist(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unenlist)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetTransaction(&self) -> ::windows::core::Result<ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransaction>(result__)
    }
}
impl ::core::convert::From<ITransactionPhase0EnlistmentAsync> for ::windows::core::IUnknown {
    fn from(value: ITransactionPhase0EnlistmentAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionPhase0EnlistmentAsync> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionPhase0EnlistmentAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionPhase0EnlistmentAsync> for ::windows::core::IUnknown {
    fn from(value: &ITransactionPhase0EnlistmentAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionPhase0EnlistmentAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionPhase0EnlistmentAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionPhase0EnlistmentAsync {}
impl ::core::fmt::Debug for ITransactionPhase0EnlistmentAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionPhase0EnlistmentAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionPhase0EnlistmentAsync {
    type Vtable = ITransactionPhase0EnlistmentAsync_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82dc88e1_a954_11d1_8f88_00600895e7d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionPhase0EnlistmentAsync_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WaitForEnlistment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Phase0Done: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unenlist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionPhase0Factory(::windows::core::IUnknown);
impl ITransactionPhase0Factory {
    pub unsafe fn Create<'a, P0>(&self, pphase0notify: P0) -> ::windows::core::Result<ITransactionPhase0EnlistmentAsync>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITransactionPhase0NotifyAsync>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), pphase0notify.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransactionPhase0EnlistmentAsync>(result__)
    }
}
impl ::core::convert::From<ITransactionPhase0Factory> for ::windows::core::IUnknown {
    fn from(value: ITransactionPhase0Factory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionPhase0Factory> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionPhase0Factory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionPhase0Factory> for ::windows::core::IUnknown {
    fn from(value: &ITransactionPhase0Factory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionPhase0Factory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionPhase0Factory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionPhase0Factory {}
impl ::core::fmt::Debug for ITransactionPhase0Factory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionPhase0Factory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionPhase0Factory {
    type Vtable = ITransactionPhase0Factory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82dc88e0_a954_11d1_8f88_00600895e7d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionPhase0Factory_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphase0notify: *mut ::core::ffi::c_void, ppphase0enlistment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionPhase0NotifyAsync(::windows::core::IUnknown);
impl ITransactionPhase0NotifyAsync {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Phase0Request<'a, P0>(&self, fabortinghint: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Phase0Request)(::windows::core::Interface::as_raw(self), fabortinghint.into()).ok()
    }
    pub unsafe fn EnlistCompleted(&self, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnlistCompleted)(::windows::core::Interface::as_raw(self), status).ok()
    }
}
impl ::core::convert::From<ITransactionPhase0NotifyAsync> for ::windows::core::IUnknown {
    fn from(value: ITransactionPhase0NotifyAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionPhase0NotifyAsync> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionPhase0NotifyAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionPhase0NotifyAsync> for ::windows::core::IUnknown {
    fn from(value: &ITransactionPhase0NotifyAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionPhase0NotifyAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionPhase0NotifyAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionPhase0NotifyAsync {}
impl ::core::fmt::Debug for ITransactionPhase0NotifyAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionPhase0NotifyAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionPhase0NotifyAsync {
    type Vtable = ITransactionPhase0NotifyAsync_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef081809_0c76_11d2_87a6_00c04f990f34);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionPhase0NotifyAsync_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Phase0Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fabortinghint: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Phase0Request: usize,
    pub EnlistCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionReceiver(::windows::core::IUnknown);
impl ITransactionReceiver {
    pub unsafe fn UnmarshalPropagationToken(&self, rgbtoken: &[u8]) -> ::windows::core::Result<ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).UnmarshalPropagationToken)(::windows::core::Interface::as_raw(self), rgbtoken.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgbtoken)), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransaction>(result__)
    }
    pub unsafe fn GetReturnTokenSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetReturnTokenSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn MarshalReturnToken(&self, rgbreturntoken: &mut [u8], pcbused: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MarshalReturnToken)(::windows::core::Interface::as_raw(self), rgbreturntoken.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgbreturntoken)), ::core::mem::transmute(pcbused)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ITransactionReceiver> for ::windows::core::IUnknown {
    fn from(value: ITransactionReceiver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionReceiver> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionReceiver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionReceiver> for ::windows::core::IUnknown {
    fn from(value: &ITransactionReceiver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionReceiver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionReceiver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionReceiver {}
impl ::core::fmt::Debug for ITransactionReceiver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionReceiver").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionReceiver {
    type Vtable = ITransactionReceiver_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59313e03_b36c_11cf_a539_00aa006887c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionReceiver_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub UnmarshalPropagationToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbtoken: u32, rgbtoken: *const u8, pptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetReturnTokenSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbreturntoken: *mut u32) -> ::windows::core::HRESULT,
    pub MarshalReturnToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionReceiverFactory(::windows::core::IUnknown);
impl ITransactionReceiverFactory {
    pub unsafe fn Create(&self) -> ::windows::core::Result<ITransactionReceiver> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransactionReceiver>(result__)
    }
}
impl ::core::convert::From<ITransactionReceiverFactory> for ::windows::core::IUnknown {
    fn from(value: ITransactionReceiverFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionReceiverFactory> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionReceiverFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionReceiverFactory> for ::windows::core::IUnknown {
    fn from(value: &ITransactionReceiverFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionReceiverFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionReceiverFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionReceiverFactory {}
impl ::core::fmt::Debug for ITransactionReceiverFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionReceiverFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionReceiverFactory {
    type Vtable = ITransactionReceiverFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59313e02_b36c_11cf_a539_00aa006887c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionReceiverFactory_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppreceiver: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionResource(::windows::core::IUnknown);
impl ITransactionResource {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PrepareRequest<'a, P0, P1, P2>(&self, fretaining: P0, grfrm: u32, fwantmoniker: P1, fsinglephase: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).PrepareRequest)(::windows::core::Interface::as_raw(self), fretaining.into(), grfrm, fwantmoniker.into(), fsinglephase.into()).ok()
    }
    pub unsafe fn CommitRequest(&self, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CommitRequest)(::windows::core::Interface::as_raw(self), grfrm, ::core::mem::transmute(pnewuow)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AbortRequest<'a, P0>(&self, pboidreason: *const BOID, fretaining: P0, pnewuow: *const BOID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).AbortRequest)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pboidreason), fretaining.into(), ::core::mem::transmute(pnewuow)).ok()
    }
    pub unsafe fn TMDown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TMDown)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ITransactionResource> for ::windows::core::IUnknown {
    fn from(value: ITransactionResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionResource> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionResource> for ::windows::core::IUnknown {
    fn from(value: &ITransactionResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionResource {}
impl ::core::fmt::Debug for ITransactionResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionResource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionResource {
    type Vtable = ITransactionResource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee5ff7b3_4572_11d0_9452_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionResource_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub PrepareRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PrepareRequest: usize,
    pub CommitRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AbortRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AbortRequest: usize,
    pub TMDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionResourceAsync(::windows::core::IUnknown);
impl ITransactionResourceAsync {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PrepareRequest<'a, P0, P1, P2>(&self, fretaining: P0, grfrm: u32, fwantmoniker: P1, fsinglephase: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).PrepareRequest)(::windows::core::Interface::as_raw(self), fretaining.into(), grfrm, fwantmoniker.into(), fsinglephase.into()).ok()
    }
    pub unsafe fn CommitRequest(&self, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CommitRequest)(::windows::core::Interface::as_raw(self), grfrm, ::core::mem::transmute(pnewuow)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AbortRequest<'a, P0>(&self, pboidreason: *const BOID, fretaining: P0, pnewuow: *const BOID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).AbortRequest)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pboidreason), fretaining.into(), ::core::mem::transmute(pnewuow)).ok()
    }
    pub unsafe fn TMDown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TMDown)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ITransactionResourceAsync> for ::windows::core::IUnknown {
    fn from(value: ITransactionResourceAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionResourceAsync> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionResourceAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionResourceAsync> for ::windows::core::IUnknown {
    fn from(value: &ITransactionResourceAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionResourceAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionResourceAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionResourceAsync {}
impl ::core::fmt::Debug for ITransactionResourceAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionResourceAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionResourceAsync {
    type Vtable = ITransactionResourceAsync_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69e971f0_23ce_11cf_ad60_00aa00a74ccd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionResourceAsync_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub PrepareRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PrepareRequest: usize,
    pub CommitRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AbortRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AbortRequest: usize,
    pub TMDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionTransmitter(::windows::core::IUnknown);
impl ITransactionTransmitter {
    pub unsafe fn Set<'a, P0>(&self, ptransaction: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITransaction>>,
    {
        (::windows::core::Interface::vtable(self).Set)(::windows::core::Interface::as_raw(self), ptransaction.into().abi()).ok()
    }
    pub unsafe fn GetPropagationTokenSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPropagationTokenSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn MarshalPropagationToken(&self, rgbtoken: &mut [u8], pcbused: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MarshalPropagationToken)(::windows::core::Interface::as_raw(self), rgbtoken.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgbtoken)), ::core::mem::transmute(pcbused)).ok()
    }
    pub unsafe fn UnmarshalReturnToken(&self, rgbreturntoken: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnmarshalReturnToken)(::windows::core::Interface::as_raw(self), rgbreturntoken.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgbreturntoken))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ITransactionTransmitter> for ::windows::core::IUnknown {
    fn from(value: ITransactionTransmitter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionTransmitter> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionTransmitter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionTransmitter> for ::windows::core::IUnknown {
    fn from(value: &ITransactionTransmitter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionTransmitter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionTransmitter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionTransmitter {}
impl ::core::fmt::Debug for ITransactionTransmitter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionTransmitter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionTransmitter {
    type Vtable = ITransactionTransmitter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59313e01_b36c_11cf_a539_00aa006887c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionTransmitter_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Set: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPropagationTokenSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbtoken: *mut u32) -> ::windows::core::HRESULT,
    pub MarshalPropagationToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT,
    pub UnmarshalReturnToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbreturntoken: u32, rgbreturntoken: *const u8) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionTransmitterFactory(::windows::core::IUnknown);
impl ITransactionTransmitterFactory {
    pub unsafe fn Create(&self) -> ::windows::core::Result<ITransactionTransmitter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransactionTransmitter>(result__)
    }
}
impl ::core::convert::From<ITransactionTransmitterFactory> for ::windows::core::IUnknown {
    fn from(value: ITransactionTransmitterFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionTransmitterFactory> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionTransmitterFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionTransmitterFactory> for ::windows::core::IUnknown {
    fn from(value: &ITransactionTransmitterFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionTransmitterFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionTransmitterFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionTransmitterFactory {}
impl ::core::fmt::Debug for ITransactionTransmitterFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionTransmitterFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionTransmitterFactory {
    type Vtable = ITransactionTransmitterFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59313e00_b36c_11cf_a539_00aa006887c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionTransmitterFactory_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptransmitter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionVoterBallotAsync2(::windows::core::IUnknown);
impl ITransactionVoterBallotAsync2 {
    pub unsafe fn VoteRequestDone(&self, hr: ::windows::core::HRESULT, pboidreason: *const BOID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).VoteRequestDone)(::windows::core::Interface::as_raw(self), hr, ::core::mem::transmute(pboidreason)).ok()
    }
}
impl ::core::convert::From<ITransactionVoterBallotAsync2> for ::windows::core::IUnknown {
    fn from(value: ITransactionVoterBallotAsync2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionVoterBallotAsync2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionVoterBallotAsync2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionVoterBallotAsync2> for ::windows::core::IUnknown {
    fn from(value: &ITransactionVoterBallotAsync2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionVoterBallotAsync2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionVoterBallotAsync2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionVoterBallotAsync2 {}
impl ::core::fmt::Debug for ITransactionVoterBallotAsync2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionVoterBallotAsync2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionVoterBallotAsync2 {
    type Vtable = ITransactionVoterBallotAsync2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5433376c_414d_11d3_b206_00c04fc2f3ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionVoterBallotAsync2_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub VoteRequestDone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, pboidreason: *const BOID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionVoterFactory2(::windows::core::IUnknown);
impl ITransactionVoterFactory2 {
    pub unsafe fn Create<'a, P0, P1>(&self, ptransaction: P0, pvoternotify: P1) -> ::windows::core::Result<ITransactionVoterBallotAsync2>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITransaction>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ITransactionVoterNotifyAsync2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), ptransaction.into().abi(), pvoternotify.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransactionVoterBallotAsync2>(result__)
    }
}
impl ::core::convert::From<ITransactionVoterFactory2> for ::windows::core::IUnknown {
    fn from(value: ITransactionVoterFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionVoterFactory2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionVoterFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionVoterFactory2> for ::windows::core::IUnknown {
    fn from(value: &ITransactionVoterFactory2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionVoterFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionVoterFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionVoterFactory2 {}
impl ::core::fmt::Debug for ITransactionVoterFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionVoterFactory2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionVoterFactory2 {
    type Vtable = ITransactionVoterFactory2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5433376a_414d_11d3_b206_00c04fc2f3ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionVoterFactory2_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, pvoternotify: *mut ::core::ffi::c_void, ppvoterballot: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct ITransactionVoterNotifyAsync2(::windows::core::IUnknown);
impl ITransactionVoterNotifyAsync2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Committed<'a, P0>(&self, fretaining: P0, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Committed)(::windows::core::Interface::as_raw(self), fretaining.into(), ::core::mem::transmute(pnewuow), hr).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Aborted<'a, P0>(&self, pboidreason: *const BOID, fretaining: P0, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Aborted)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pboidreason), fretaining.into(), ::core::mem::transmute(pnewuow), hr).ok()
    }
    pub unsafe fn HeuristicDecision(&self, dwdecision: u32, pboidreason: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.HeuristicDecision)(::windows::core::Interface::as_raw(self), dwdecision, ::core::mem::transmute(pboidreason), hr).ok()
    }
    pub unsafe fn Indoubt(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Indoubt)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn VoteRequest(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).VoteRequest)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ITransactionVoterNotifyAsync2> for ::windows::core::IUnknown {
    fn from(value: ITransactionVoterNotifyAsync2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionVoterNotifyAsync2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITransactionVoterNotifyAsync2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionVoterNotifyAsync2> for ::windows::core::IUnknown {
    fn from(value: &ITransactionVoterNotifyAsync2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ITransactionVoterNotifyAsync2> for ITransactionOutcomeEvents {
    fn from(value: ITransactionVoterNotifyAsync2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITransactionVoterNotifyAsync2> for &'a ITransactionOutcomeEvents {
    fn from(value: &'a ITransactionVoterNotifyAsync2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionVoterNotifyAsync2> for ITransactionOutcomeEvents {
    fn from(value: &ITransactionVoterNotifyAsync2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITransactionVoterNotifyAsync2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionVoterNotifyAsync2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionVoterNotifyAsync2 {}
impl ::core::fmt::Debug for ITransactionVoterNotifyAsync2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionVoterNotifyAsync2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransactionVoterNotifyAsync2 {
    type Vtable = ITransactionVoterNotifyAsync2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5433376b_414d_11d3_b206_00c04fc2f3ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionVoterNotifyAsync2_Vtbl {
    pub base__: ITransactionOutcomeEvents_Vtbl,
    pub VoteRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IXAConfig(::windows::core::IUnknown);
impl IXAConfig {
    pub unsafe fn Initialize(&self, clsidhelperdll: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(clsidhelperdll)).ok()
    }
    pub unsafe fn Terminate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Terminate)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IXAConfig> for ::windows::core::IUnknown {
    fn from(value: IXAConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXAConfig> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IXAConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXAConfig> for ::windows::core::IUnknown {
    fn from(value: &IXAConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IXAConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXAConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXAConfig {}
impl ::core::fmt::Debug for IXAConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXAConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXAConfig {
    type Vtable = IXAConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8a6e3a1_9a8c_11cf_a308_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAConfig_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsidhelperdll: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IXAObtainRMInfo(::windows::core::IUnknown);
impl IXAObtainRMInfo {
    pub unsafe fn ObtainRMInfo<'a, P0>(&self, pirmhelper: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRMHelper>>,
    {
        (::windows::core::Interface::vtable(self).ObtainRMInfo)(::windows::core::Interface::as_raw(self), pirmhelper.into().abi()).ok()
    }
}
impl ::core::convert::From<IXAObtainRMInfo> for ::windows::core::IUnknown {
    fn from(value: IXAObtainRMInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXAObtainRMInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IXAObtainRMInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXAObtainRMInfo> for ::windows::core::IUnknown {
    fn from(value: &IXAObtainRMInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IXAObtainRMInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXAObtainRMInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXAObtainRMInfo {}
impl ::core::fmt::Debug for IXAObtainRMInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXAObtainRMInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXAObtainRMInfo {
    type Vtable = IXAObtainRMInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe793f6d2_f53d_11cf_a60d_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAObtainRMInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ObtainRMInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirmhelper: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IXATransLookup(::windows::core::IUnknown);
impl IXATransLookup {
    pub unsafe fn Lookup(&self) -> ::windows::core::Result<ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Lookup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransaction>(result__)
    }
}
impl ::core::convert::From<IXATransLookup> for ::windows::core::IUnknown {
    fn from(value: IXATransLookup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXATransLookup> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IXATransLookup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXATransLookup> for ::windows::core::IUnknown {
    fn from(value: &IXATransLookup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IXATransLookup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXATransLookup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXATransLookup {}
impl ::core::fmt::Debug for IXATransLookup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXATransLookup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXATransLookup {
    type Vtable = IXATransLookup_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3b1f131_eeda_11ce_aed4_00aa0051e2c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXATransLookup_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Lookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
pub struct IXATransLookup2(::windows::core::IUnknown);
impl IXATransLookup2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Lookup(&self, pxid: *const xid_t) -> ::windows::core::Result<ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Lookup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pxid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransaction>(result__)
    }
}
impl ::core::convert::From<IXATransLookup2> for ::windows::core::IUnknown {
    fn from(value: IXATransLookup2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXATransLookup2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IXATransLookup2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXATransLookup2> for ::windows::core::IUnknown {
    fn from(value: &IXATransLookup2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IXATransLookup2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXATransLookup2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXATransLookup2 {}
impl ::core::fmt::Debug for IXATransLookup2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXATransLookup2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXATransLookup2 {
    type Vtable = IXATransLookup2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf193c85_0d1a_4290_b88f_d2cb8873d1e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXATransLookup2_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Lookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxid: *const xid_t, pptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Lookup: usize,
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const MAXBQUALSIZE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const MAXGTRIDSIZE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const MAXINFOSIZE: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub struct OLE_TM_CONFIG_PARAMS_V1 {
    pub dwVersion: u32,
    pub dwcConcurrencyHint: u32,
}
impl ::core::marker::Copy for OLE_TM_CONFIG_PARAMS_V1 {}
impl ::core::clone::Clone for OLE_TM_CONFIG_PARAMS_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OLE_TM_CONFIG_PARAMS_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLE_TM_CONFIG_PARAMS_V1").field("dwVersion", &self.dwVersion).field("dwcConcurrencyHint", &self.dwcConcurrencyHint).finish()
    }
}
unsafe impl ::windows::core::Abi for OLE_TM_CONFIG_PARAMS_V1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OLE_TM_CONFIG_PARAMS_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OLE_TM_CONFIG_PARAMS_V1>()) == 0 }
    }
}
impl ::core::cmp::Eq for OLE_TM_CONFIG_PARAMS_V1 {}
impl ::core::default::Default for OLE_TM_CONFIG_PARAMS_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub struct OLE_TM_CONFIG_PARAMS_V2 {
    pub dwVersion: u32,
    pub dwcConcurrencyHint: u32,
    pub applicationType: APPLICATIONTYPE,
    pub clusterResourceId: ::windows::core::GUID,
}
impl ::core::marker::Copy for OLE_TM_CONFIG_PARAMS_V2 {}
impl ::core::clone::Clone for OLE_TM_CONFIG_PARAMS_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OLE_TM_CONFIG_PARAMS_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLE_TM_CONFIG_PARAMS_V2").field("dwVersion", &self.dwVersion).field("dwcConcurrencyHint", &self.dwcConcurrencyHint).field("applicationType", &self.applicationType).field("clusterResourceId", &self.clusterResourceId).finish()
    }
}
unsafe impl ::windows::core::Abi for OLE_TM_CONFIG_PARAMS_V2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OLE_TM_CONFIG_PARAMS_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OLE_TM_CONFIG_PARAMS_V2>()) == 0 }
    }
}
impl ::core::cmp::Eq for OLE_TM_CONFIG_PARAMS_V2 {}
impl ::core::default::Default for OLE_TM_CONFIG_PARAMS_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_CONFIG_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_CONFIG_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_FLAG_INTERNAL_TO_TM: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_FLAG_NOAGILERECOVERY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_FLAG_NODEMANDSTART: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_FLAG_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_FLAG_QUERY_SERVICE_LOCKSTATUS: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const RMNAMESZ: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMASYNC: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMENDRSCAN: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMER_INVAL: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMER_PROTO: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMER_TMERR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMFAIL: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMJOIN: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMMIGRATE: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMMULTIPLE: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMNOFLAGS: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMNOMIGRATE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMNOWAIT: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMONEPHASE: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMREGISTER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMRESUME: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMSTARTRSCAN: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMSUCCESS: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMSUSPEND: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMUSEASYNC: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TM_JOIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TM_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TM_RESUME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TX_MISC_CONSTANTS(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const MAX_TRAN_DESC: TX_MISC_CONSTANTS = TX_MISC_CONSTANTS(40i32);
impl ::core::marker::Copy for TX_MISC_CONSTANTS {}
impl ::core::clone::Clone for TX_MISC_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TX_MISC_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TX_MISC_CONSTANTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TX_MISC_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TX_MISC_CONSTANTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XACTCONST(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTCONST_TIMEOUTINFINITE: XACTCONST = XACTCONST(0i32);
impl ::core::marker::Copy for XACTCONST {}
impl ::core::clone::Clone for XACTCONST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XACTCONST {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XACTCONST {
    type Abi = Self;
}
impl ::core::fmt::Debug for XACTCONST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACTCONST").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XACTHEURISTIC(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTHEURISTIC_ABORT: XACTHEURISTIC = XACTHEURISTIC(1i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTHEURISTIC_COMMIT: XACTHEURISTIC = XACTHEURISTIC(2i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTHEURISTIC_DAMAGE: XACTHEURISTIC = XACTHEURISTIC(3i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTHEURISTIC_DANGER: XACTHEURISTIC = XACTHEURISTIC(4i32);
impl ::core::marker::Copy for XACTHEURISTIC {}
impl ::core::clone::Clone for XACTHEURISTIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XACTHEURISTIC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XACTHEURISTIC {
    type Abi = Self;
}
impl ::core::fmt::Debug for XACTHEURISTIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACTHEURISTIC").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub struct XACTOPT {
    pub ulTimeout: u32,
    pub szDescription: [u8; 40],
}
impl ::core::marker::Copy for XACTOPT {}
impl ::core::clone::Clone for XACTOPT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XACTOPT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XACTOPT").field("ulTimeout", &self.ulTimeout).field("szDescription", &self.szDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for XACTOPT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XACTOPT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XACTOPT>()) == 0 }
    }
}
impl ::core::cmp::Eq for XACTOPT {}
impl ::core::default::Default for XACTOPT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XACTRM(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTRM_OPTIMISTICLASTWINS: XACTRM = XACTRM(1i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTRM_NOREADONLYPREPARES: XACTRM = XACTRM(2i32);
impl ::core::marker::Copy for XACTRM {}
impl ::core::clone::Clone for XACTRM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XACTRM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XACTRM {
    type Abi = Self;
}
impl ::core::fmt::Debug for XACTRM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACTRM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XACTSTAT(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_NONE: XACTSTAT = XACTSTAT(0i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_OPENNORMAL: XACTSTAT = XACTSTAT(1i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_OPENREFUSED: XACTSTAT = XACTSTAT(2i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_PREPARING: XACTSTAT = XACTSTAT(4i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_PREPARED: XACTSTAT = XACTSTAT(8i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_PREPARERETAINING: XACTSTAT = XACTSTAT(16i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_PREPARERETAINED: XACTSTAT = XACTSTAT(32i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_COMMITTING: XACTSTAT = XACTSTAT(64i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_COMMITRETAINING: XACTSTAT = XACTSTAT(128i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_ABORTING: XACTSTAT = XACTSTAT(256i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_ABORTED: XACTSTAT = XACTSTAT(512i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_COMMITTED: XACTSTAT = XACTSTAT(1024i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_HEURISTIC_ABORT: XACTSTAT = XACTSTAT(2048i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_HEURISTIC_COMMIT: XACTSTAT = XACTSTAT(4096i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_HEURISTIC_DAMAGE: XACTSTAT = XACTSTAT(8192i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_HEURISTIC_DANGER: XACTSTAT = XACTSTAT(16384i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_FORCED_ABORT: XACTSTAT = XACTSTAT(32768i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_FORCED_COMMIT: XACTSTAT = XACTSTAT(65536i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_INDOUBT: XACTSTAT = XACTSTAT(131072i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_CLOSED: XACTSTAT = XACTSTAT(262144i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_OPEN: XACTSTAT = XACTSTAT(3i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_NOTPREPARED: XACTSTAT = XACTSTAT(524227i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_ALL: XACTSTAT = XACTSTAT(524287i32);
impl ::core::marker::Copy for XACTSTAT {}
impl ::core::clone::Clone for XACTSTAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XACTSTAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XACTSTAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for XACTSTAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACTSTAT").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct XACTSTATS {
    pub cOpen: u32,
    pub cCommitting: u32,
    pub cCommitted: u32,
    pub cAborting: u32,
    pub cAborted: u32,
    pub cInDoubt: u32,
    pub cHeuristicDecision: u32,
    pub timeTransactionsUp: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for XACTSTATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for XACTSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for XACTSTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XACTSTATS").field("cOpen", &self.cOpen).field("cCommitting", &self.cCommitting).field("cCommitted", &self.cCommitted).field("cAborting", &self.cAborting).field("cAborted", &self.cAborted).field("cInDoubt", &self.cInDoubt).field("cHeuristicDecision", &self.cHeuristicDecision).field("timeTransactionsUp", &self.timeTransactionsUp).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for XACTSTATS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for XACTSTATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XACTSTATS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for XACTSTATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for XACTSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XACTTC(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTTC_NONE: XACTTC = XACTTC(0i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTTC_SYNC_PHASEONE: XACTTC = XACTTC(1i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTTC_SYNC_PHASETWO: XACTTC = XACTTC(2i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTTC_SYNC: XACTTC = XACTTC(2i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTTC_ASYNC_PHASEONE: XACTTC = XACTTC(4i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTTC_ASYNC: XACTTC = XACTTC(4i32);
impl ::core::marker::Copy for XACTTC {}
impl ::core::clone::Clone for XACTTC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XACTTC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XACTTC {
    type Abi = Self;
}
impl ::core::fmt::Debug for XACTTC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACTTC").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub struct XACTTRANSINFO {
    pub uow: BOID,
    pub isoLevel: i32,
    pub isoFlags: u32,
    pub grfTCSupported: u32,
    pub grfRMSupported: u32,
    pub grfTCSupportedRetaining: u32,
    pub grfRMSupportedRetaining: u32,
}
impl ::core::marker::Copy for XACTTRANSINFO {}
impl ::core::clone::Clone for XACTTRANSINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XACTTRANSINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XACTTRANSINFO").field("uow", &self.uow).field("isoLevel", &self.isoLevel).field("isoFlags", &self.isoFlags).field("grfTCSupported", &self.grfTCSupported).field("grfRMSupported", &self.grfRMSupported).field("grfTCSupportedRetaining", &self.grfTCSupportedRetaining).field("grfRMSupportedRetaining", &self.grfRMSupportedRetaining).finish()
    }
}
unsafe impl ::windows::core::Abi for XACTTRANSINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XACTTRANSINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XACTTRANSINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for XACTTRANSINFO {}
impl ::core::default::Default for XACTTRANSINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XACT_DTC_CONSTANTS(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_CONNECTION_REQUEST_DENIED: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147168000i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_TOOMANY_ENLISTMENTS: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167999i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_DUPLICATE_GUID: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167998i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_NOTSINGLEPHASE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167997i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_RECOVERYALREADYDONE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167996i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_PROTOCOL: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167995i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_RM_FAILURE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167994i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_RECOVERY_FAILED: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167993i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_NOT_FOUND: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167992i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_DUPLICATE_LU: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167991i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_NOT_CONNECTED: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167990i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_DUPLICATE_TRANSID: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167989i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_BUSY: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167988i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_NO_RECOVERY_PROCESS: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167987i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_DOWN: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167986i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_RECOVERING: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167985i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_RECOVERY_MISMATCH: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167984i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_RM_UNAVAILABLE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167983i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LRMRECOVERYALREADYDONE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167982i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_NOLASTRESOURCEINTERFACE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167981i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_S_NONOTIFY: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(315648i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_OK_NONOTIFY: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(315649i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const dwUSER_MS_SQLSERVER: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(65535i32);
impl ::core::marker::Copy for XACT_DTC_CONSTANTS {}
impl ::core::clone::Clone for XACT_DTC_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XACT_DTC_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XACT_DTC_CONSTANTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for XACT_DTC_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACT_DTC_CONSTANTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_ASYNC: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_DUPID: i32 = -8i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_INVAL: i32 = -5i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_NOTA: i32 = -4i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_OUTSIDE: i32 = -9i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_PROTO: i32 = -6i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_RMERR: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_RMFAIL: i32 = -7i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XA_CLOSE_EPT = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_COMMIT_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XA_COMPLETE_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut i32, param1: *mut i32, param2: i32, param3: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_END_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_FMTID_DTC: u32 = 4478019u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_FMTID_DTC_VER1: u32 = 21255235u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_FORGET_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_HEURCOM: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_HEURHAZ: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_HEURMIX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_HEURRB: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_NOMIGRATE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XA_OPEN_EPT = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_PREPARE_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBBASE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBCOMMFAIL: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBDEADLOCK: u32 = 102u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBEND: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBINTEGRITY: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBOTHER: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBPROTO: u32 = 105u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBROLLBACK: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBTIMEOUT: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBTRANSIENT: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RDONLY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_RECOVER_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32, param3: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RETRY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_ROLLBACK_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_START_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_SWITCH_F_DTC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XIDDATASIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DtcLu_CompareState(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATE_COMMITTED: _DtcLu_CompareState = _DtcLu_CompareState(1i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATE_HEURISTICCOMMITTED: _DtcLu_CompareState = _DtcLu_CompareState(2i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATE_HEURISTICMIXED: _DtcLu_CompareState = _DtcLu_CompareState(3i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATE_HEURISTICRESET: _DtcLu_CompareState = _DtcLu_CompareState(4i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATE_INDOUBT: _DtcLu_CompareState = _DtcLu_CompareState(5i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATE_RESET: _DtcLu_CompareState = _DtcLu_CompareState(6i32);
impl ::core::marker::Copy for _DtcLu_CompareState {}
impl ::core::clone::Clone for _DtcLu_CompareState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DtcLu_CompareState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DtcLu_CompareState {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DtcLu_CompareState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DtcLu_CompareState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DtcLu_CompareStates_Confirmation(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATESCONFIRMATION_CONFIRM: _DtcLu_CompareStates_Confirmation = _DtcLu_CompareStates_Confirmation(1i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATESCONFIRMATION_PROTOCOL: _DtcLu_CompareStates_Confirmation = _DtcLu_CompareStates_Confirmation(2i32);
impl ::core::marker::Copy for _DtcLu_CompareStates_Confirmation {}
impl ::core::clone::Clone for _DtcLu_CompareStates_Confirmation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DtcLu_CompareStates_Confirmation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DtcLu_CompareStates_Confirmation {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DtcLu_CompareStates_Confirmation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DtcLu_CompareStates_Confirmation").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DtcLu_CompareStates_Error(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATESERROR_PROTOCOL: _DtcLu_CompareStates_Error = _DtcLu_CompareStates_Error(1i32);
impl ::core::marker::Copy for _DtcLu_CompareStates_Error {}
impl ::core::clone::Clone for _DtcLu_CompareStates_Error {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DtcLu_CompareStates_Error {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DtcLu_CompareStates_Error {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DtcLu_CompareStates_Error {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DtcLu_CompareStates_Error").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DtcLu_CompareStates_Response(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATESRESPONSE_OK: _DtcLu_CompareStates_Response = _DtcLu_CompareStates_Response(1i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATESRESPONSE_PROTOCOL: _DtcLu_CompareStates_Response = _DtcLu_CompareStates_Response(2i32);
impl ::core::marker::Copy for _DtcLu_CompareStates_Response {}
impl ::core::clone::Clone for _DtcLu_CompareStates_Response {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DtcLu_CompareStates_Response {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DtcLu_CompareStates_Response {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DtcLu_CompareStates_Response {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DtcLu_CompareStates_Response").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DtcLu_LocalRecovery_Work(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCINITIATEDRECOVERYWORK_CHECKLUSTATUS: _DtcLu_LocalRecovery_Work = _DtcLu_LocalRecovery_Work(1i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCINITIATEDRECOVERYWORK_TRANS: _DtcLu_LocalRecovery_Work = _DtcLu_LocalRecovery_Work(2i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCINITIATEDRECOVERYWORK_TMDOWN: _DtcLu_LocalRecovery_Work = _DtcLu_LocalRecovery_Work(3i32);
impl ::core::marker::Copy for _DtcLu_LocalRecovery_Work {}
impl ::core::clone::Clone for _DtcLu_LocalRecovery_Work {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DtcLu_LocalRecovery_Work {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DtcLu_LocalRecovery_Work {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DtcLu_LocalRecovery_Work {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DtcLu_LocalRecovery_Work").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DtcLu_Xln(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLN_COLD: _DtcLu_Xln = _DtcLu_Xln(1i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLN_WARM: _DtcLu_Xln = _DtcLu_Xln(2i32);
impl ::core::marker::Copy for _DtcLu_Xln {}
impl ::core::clone::Clone for _DtcLu_Xln {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DtcLu_Xln {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DtcLu_Xln {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DtcLu_Xln {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DtcLu_Xln").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DtcLu_Xln_Confirmation(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNCONFIRMATION_CONFIRM: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(1i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNCONFIRMATION_LOGNAMEMISMATCH: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(2i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNCONFIRMATION_COLDWARMMISMATCH: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(3i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNCONFIRMATION_OBSOLETE: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(4i32);
impl ::core::marker::Copy for _DtcLu_Xln_Confirmation {}
impl ::core::clone::Clone for _DtcLu_Xln_Confirmation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DtcLu_Xln_Confirmation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DtcLu_Xln_Confirmation {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DtcLu_Xln_Confirmation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DtcLu_Xln_Confirmation").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DtcLu_Xln_Error(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNERROR_PROTOCOL: _DtcLu_Xln_Error = _DtcLu_Xln_Error(1i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNERROR_LOGNAMEMISMATCH: _DtcLu_Xln_Error = _DtcLu_Xln_Error(2i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNERROR_COLDWARMMISMATCH: _DtcLu_Xln_Error = _DtcLu_Xln_Error(3i32);
impl ::core::marker::Copy for _DtcLu_Xln_Error {}
impl ::core::clone::Clone for _DtcLu_Xln_Error {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DtcLu_Xln_Error {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DtcLu_Xln_Error {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DtcLu_Xln_Error {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DtcLu_Xln_Error").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DtcLu_Xln_Response(pub i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNRESPONSE_OK_SENDOURXLNBACK: _DtcLu_Xln_Response = _DtcLu_Xln_Response(1i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNRESPONSE_OK_SENDCONFIRMATION: _DtcLu_Xln_Response = _DtcLu_Xln_Response(2i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNRESPONSE_LOGNAMEMISMATCH: _DtcLu_Xln_Response = _DtcLu_Xln_Response(3i32);
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNRESPONSE_COLDWARMMISMATCH: _DtcLu_Xln_Response = _DtcLu_Xln_Response(4i32);
impl ::core::marker::Copy for _DtcLu_Xln_Response {}
impl ::core::clone::Clone for _DtcLu_Xln_Response {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DtcLu_Xln_Response {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DtcLu_Xln_Response {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DtcLu_Xln_Response {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DtcLu_Xln_Response").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub struct _ProxyConfigParams {
    pub wcThreadsMax: u16,
}
impl ::core::marker::Copy for _ProxyConfigParams {}
impl ::core::clone::Clone for _ProxyConfigParams {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for _ProxyConfigParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_ProxyConfigParams").field("wcThreadsMax", &self.wcThreadsMax).finish()
    }
}
unsafe impl ::windows::core::Abi for _ProxyConfigParams {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for _ProxyConfigParams {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_ProxyConfigParams>()) == 0 }
    }
}
impl ::core::cmp::Eq for _ProxyConfigParams {}
impl ::core::default::Default for _ProxyConfigParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct xa_switch_t {
    pub name: [super::super::Foundation::CHAR; 32],
    pub flags: i32,
    pub version: i32,
    pub xa_open_entry: isize,
    pub xa_close_entry: isize,
    pub xa_start_entry: isize,
    pub xa_end_entry: isize,
    pub xa_rollback_entry: isize,
    pub xa_prepare_entry: isize,
    pub xa_commit_entry: isize,
    pub xa_recover_entry: isize,
    pub xa_forget_entry: isize,
    pub xa_complete_entry: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for xa_switch_t {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for xa_switch_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for xa_switch_t {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("xa_switch_t")
            .field("name", &self.name)
            .field("flags", &self.flags)
            .field("version", &self.version)
            .field("xa_open_entry", &self.xa_open_entry)
            .field("xa_close_entry", &self.xa_close_entry)
            .field("xa_start_entry", &self.xa_start_entry)
            .field("xa_end_entry", &self.xa_end_entry)
            .field("xa_rollback_entry", &self.xa_rollback_entry)
            .field("xa_prepare_entry", &self.xa_prepare_entry)
            .field("xa_commit_entry", &self.xa_commit_entry)
            .field("xa_recover_entry", &self.xa_recover_entry)
            .field("xa_forget_entry", &self.xa_forget_entry)
            .field("xa_complete_entry", &self.xa_complete_entry)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for xa_switch_t {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for xa_switch_t {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<xa_switch_t>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for xa_switch_t {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for xa_switch_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct xid_t {
    pub formatID: i32,
    pub gtrid_length: i32,
    pub bqual_length: i32,
    pub data: [super::super::Foundation::CHAR; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for xid_t {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for xid_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for xid_t {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("xid_t").field("formatID", &self.formatID).field("gtrid_length", &self.gtrid_length).field("bqual_length", &self.bqual_length).field("data", &self.data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for xid_t {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for xid_t {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<xid_t>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for xid_t {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for xid_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
