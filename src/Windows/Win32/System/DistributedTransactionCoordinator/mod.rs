#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPLICATIONTYPE(pub i32);
pub const LOCAL_APPLICATIONTYPE: APPLICATIONTYPE = APPLICATIONTYPE(0i32);
pub const CLUSTERRESOURCE_APPLICATIONTYPE: APPLICATIONTYPE = APPLICATIONTYPE(1i32);
impl ::core::convert::From<i32> for APPLICATIONTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for APPLICATIONTYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AUTHENTICATION_LEVEL(pub i32);
pub const NO_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(0i32);
pub const INCOMING_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(1i32);
pub const MUTUAL_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(2i32);
impl ::core::convert::From<i32> for AUTHENTICATION_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AUTHENTICATION_LEVEL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct BOID {
    pub rgb: [u8; 16],
}
impl BOID {}
impl ::core::default::Default for BOID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BOID {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BOID").field("rgb", &self.rgb).finish()
    }
}
impl ::core::cmp::PartialEq for BOID {
    fn eq(&self, other: &Self) -> bool {
        self.rgb == other.rgb
    }
}
impl ::core::cmp::Eq for BOID {}
unsafe impl ::windows::core::Abi for BOID {
    type Abi = Self;
}
pub const CLSID_MSDtcTransaction: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39f8d76b_0928_11d1_97df_00c04fb9618a);
pub const CLSID_MSDtcTransactionManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b18ab61_091d_11d1_97df_00c04fb9618a);
pub const DTCINSTALL_E_CLIENT_ALREADY_INSTALLED: i32 = 384i32;
pub const DTCINSTALL_E_SERVER_ALREADY_INSTALLED: i32 = 385i32;
#[cfg(feature = "Win32_Foundation")]
pub type DTC_GET_TRANSACTION_MANAGER = unsafe extern "system" fn(pszhost: super::super::Foundation::PSTR, psztmname: super::super::Foundation::PSTR, rid: *const ::windows::core::GUID, dwreserved1: u32, wcbreserved2: u16, pvreserved2: *mut ::core::ffi::c_void, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type DTC_GET_TRANSACTION_MANAGER_EX_A = unsafe extern "system" fn(i_pszhost: super::super::Foundation::PSTR, i_psztmname: super::super::Foundation::PSTR, i_riid: *const ::windows::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type DTC_GET_TRANSACTION_MANAGER_EX_W = unsafe extern "system" fn(i_pwszhost: super::super::Foundation::PWSTR, i_pwsztmname: super::super::Foundation::PWSTR, i_riid: *const ::windows::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
pub type DTC_INSTALL_CLIENT = unsafe extern "system" fn(i_pszremotetmhostname: *mut i8, i_dwprotocol: u32, i_dwoverwrite: u32) -> ::windows::core::HRESULT;
pub const DTC_INSTALL_OVERWRITE_CLIENT: u32 = 1u32;
pub const DTC_INSTALL_OVERWRITE_SERVER: u32 = 2u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DTC_STATUS_(pub i32);
pub const DTC_STATUS_UNKNOWN: DTC_STATUS_ = DTC_STATUS_(0i32);
pub const DTC_STATUS_STARTING: DTC_STATUS_ = DTC_STATUS_(1i32);
pub const DTC_STATUS_STARTED: DTC_STATUS_ = DTC_STATUS_(2i32);
pub const DTC_STATUS_PAUSING: DTC_STATUS_ = DTC_STATUS_(3i32);
pub const DTC_STATUS_PAUSED: DTC_STATUS_ = DTC_STATUS_(4i32);
pub const DTC_STATUS_CONTINUING: DTC_STATUS_ = DTC_STATUS_(5i32);
pub const DTC_STATUS_STOPPING: DTC_STATUS_ = DTC_STATUS_(6i32);
pub const DTC_STATUS_STOPPED: DTC_STATUS_ = DTC_STATUS_(7i32);
pub const DTC_STATUS_E_CANTCONTROL: DTC_STATUS_ = DTC_STATUS_(8i32);
pub const DTC_STATUS_FAILED: DTC_STATUS_ = DTC_STATUS_(9i32);
impl ::core::convert::From<i32> for DTC_STATUS_ {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DTC_STATUS_ {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DtcGetTransactionManager<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(i_pszhost: Param0, i_psztmname: Param1, i_riid: *const ::windows::core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DtcGetTransactionManager(i_pszhost: super::super::Foundation::PSTR, i_psztmname: super::super::Foundation::PSTR, i_riid: *const ::windows::core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DtcGetTransactionManager(i_pszhost.into_param().abi(), i_psztmname.into_param().abi(), ::core::mem::transmute(i_riid), ::core::mem::transmute(i_dwreserved1), ::core::mem::transmute(i_wcbreserved2), ::core::mem::transmute(i_pvreserved2), ::core::mem::transmute(o_ppvobject)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DtcGetTransactionManagerC<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(i_pszhost: Param0, i_psztmname: Param1, i_riid: *const ::windows::core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DtcGetTransactionManagerC(i_pszhost: super::super::Foundation::PSTR, i_psztmname: super::super::Foundation::PSTR, i_riid: *const ::windows::core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DtcGetTransactionManagerC(i_pszhost.into_param().abi(), i_psztmname.into_param().abi(), ::core::mem::transmute(i_riid), ::core::mem::transmute(i_dwreserved1), ::core::mem::transmute(i_wcbreserved2), ::core::mem::transmute(i_pvreserved2), ::core::mem::transmute(o_ppvobject)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DtcGetTransactionManagerExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(i_pszhost: Param0, i_psztmname: Param1, i_riid: *const ::windows::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DtcGetTransactionManagerExA(i_pszhost: super::super::Foundation::PSTR, i_psztmname: super::super::Foundation::PSTR, i_riid: *const ::windows::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DtcGetTransactionManagerExA(i_pszhost.into_param().abi(), i_psztmname.into_param().abi(), ::core::mem::transmute(i_riid), ::core::mem::transmute(i_grfoptions), ::core::mem::transmute(i_pvconfigparams), ::core::mem::transmute(o_ppvobject)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DtcGetTransactionManagerExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(i_pwszhost: Param0, i_pwsztmname: Param1, i_riid: *const ::windows::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DtcGetTransactionManagerExW(i_pwszhost: super::super::Foundation::PWSTR, i_pwsztmname: super::super::Foundation::PWSTR, i_riid: *const ::windows::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DtcGetTransactionManagerExW(i_pwszhost.into_param().abi(), i_pwsztmname.into_param().abi(), ::core::mem::transmute(i_riid), ::core::mem::transmute(i_grfoptions), ::core::mem::transmute(i_pvconfigparams), ::core::mem::transmute(o_ppvobject)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcLuConfigure(pub ::windows::core::IUnknown);
impl IDtcLuConfigure {
    pub unsafe fn Add(&self, puclupair: *const u8, cblupair: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(puclupair), ::core::mem::transmute(cblupair)).ok()
    }
    pub unsafe fn Delete(&self, puclupair: *const u8, cblupair: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(puclupair), ::core::mem::transmute(cblupair)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuConfigure {
    type Vtable = IDtcLuConfigure_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e760_1aea_11d0_944b_00a0c905416e);
}
impl ::core::convert::From<IDtcLuConfigure> for ::windows::core::IUnknown {
    fn from(value: IDtcLuConfigure) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcLuConfigure> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuConfigure) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcLuConfigure {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcLuConfigure {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuConfigure_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puclupair: *const u8, cblupair: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puclupair: *const u8, cblupair: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcLuRecovery(pub ::windows::core::IUnknown);
impl IDtcLuRecovery {}
unsafe impl ::windows::core::Interface for IDtcLuRecovery {
    type Vtable = IDtcLuRecovery_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac2b8ad2_d6f0_11d0_b386_00a0c9083365);
}
impl ::core::convert::From<IDtcLuRecovery> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRecovery) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcLuRecovery> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRecovery) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcLuRecovery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcLuRecovery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecovery_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcLuRecoveryFactory(pub ::windows::core::IUnknown);
impl IDtcLuRecoveryFactory {
    pub unsafe fn Create(&self, puclupair: *const u8, cblupair: u32) -> ::windows::core::Result<IDtcLuRecovery> {
        let mut result__: <IDtcLuRecovery as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(puclupair), ::core::mem::transmute(cblupair), &mut result__).from_abi::<IDtcLuRecovery>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRecoveryFactory {
    type Vtable = IDtcLuRecoveryFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e762_1aea_11d0_944b_00a0c905416e);
}
impl ::core::convert::From<IDtcLuRecoveryFactory> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRecoveryFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcLuRecoveryFactory> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRecoveryFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcLuRecoveryFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcLuRecoveryFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puclupair: *const u8, cblupair: u32, pprecovery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcLuRecoveryInitiatedByDtc(pub ::windows::core::IUnknown);
impl IDtcLuRecoveryInitiatedByDtc {
    pub unsafe fn GetWork(&self, pwork: *mut _DtcLu_LocalRecovery_Work, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwork), ::core::mem::transmute(ppv)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRecoveryInitiatedByDtc {
    type Vtable = IDtcLuRecoveryInitiatedByDtc_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e764_1aea_11d0_944b_00a0c905416e);
}
impl ::core::convert::From<IDtcLuRecoveryInitiatedByDtc> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByDtc) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcLuRecoveryInitiatedByDtc> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByDtc) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcLuRecoveryInitiatedByDtc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcLuRecoveryInitiatedByDtc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByDtc_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwork: *mut _DtcLu_LocalRecovery_Work, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcLuRecoveryInitiatedByDtcStatusWork(pub ::windows::core::IUnknown);
impl IDtcLuRecoveryInitiatedByDtcStatusWork {
    pub unsafe fn HandleCheckLuStatus(&self, lrecoveryseqnum: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lrecoveryseqnum)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRecoveryInitiatedByDtcStatusWork {
    type Vtable = IDtcLuRecoveryInitiatedByDtcStatusWork_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e766_1aea_11d0_944b_00a0c905416e);
}
impl ::core::convert::From<IDtcLuRecoveryInitiatedByDtcStatusWork> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByDtcStatusWork) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcLuRecoveryInitiatedByDtcStatusWork> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByDtcStatusWork) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcLuRecoveryInitiatedByDtcStatusWork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcLuRecoveryInitiatedByDtcStatusWork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByDtcStatusWork_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lrecoveryseqnum: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcLuRecoveryInitiatedByDtcTransWork(pub ::windows::core::IUnknown);
impl IDtcLuRecoveryInitiatedByDtcTransWork {
    pub unsafe fn GetLogNameSizes(&self, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcbourlogname), ::core::mem::transmute(pcbremotelogname)).ok()
    }
    pub unsafe fn GetOurXln(&self, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pxln), ::core::mem::transmute(pourlogname), ::core::mem::transmute(premotelogname), ::core::mem::transmute(pdwprotocol)).ok()
    }
    pub unsafe fn HandleConfirmationFromOurXln(&self, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(confirmation)).ok()
    }
    pub unsafe fn HandleTheirXlnResponse(&self, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut _DtcLu_Xln_Confirmation) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(xln), ::core::mem::transmute(premotelogname), ::core::mem::transmute(cbremotelogname), ::core::mem::transmute(dwprotocol), ::core::mem::transmute(pconfirmation)).ok()
    }
    pub unsafe fn HandleErrorFromOurXln(&self, error: _DtcLu_Xln_Error) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(error)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckForCompareStates(&self, fcomparestates: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(fcomparestates)).ok()
    }
    pub unsafe fn GetOurTransIdSize(&self, pcbourtransid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcbourtransid)).ok()
    }
    pub unsafe fn GetOurCompareStates(&self, pourtransid: *mut u8, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pourtransid), ::core::mem::transmute(pcomparestate)).ok()
    }
    pub unsafe fn HandleTheirCompareStatesResponse(&self, comparestate: _DtcLu_CompareState, pconfirmation: *mut _DtcLu_CompareStates_Confirmation) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(comparestate), ::core::mem::transmute(pconfirmation)).ok()
    }
    pub unsafe fn HandleErrorFromOurCompareStates(&self, error: _DtcLu_CompareStates_Error) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(error)).ok()
    }
    pub unsafe fn ConversationLost(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetRecoverySeqNum(&self, plrecoveryseqnum: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(plrecoveryseqnum)).ok()
    }
    pub unsafe fn ObsoleteRecoverySeqNum(&self, lnewrecoveryseqnum: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(lnewrecoveryseqnum)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRecoveryInitiatedByDtcTransWork {
    type Vtable = IDtcLuRecoveryInitiatedByDtcTransWork_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e765_1aea_11d0_944b_00a0c905416e);
}
impl ::core::convert::From<IDtcLuRecoveryInitiatedByDtcTransWork> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByDtcTransWork) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcLuRecoveryInitiatedByDtcTransWork> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByDtcTransWork) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcLuRecoveryInitiatedByDtcTransWork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcLuRecoveryInitiatedByDtcTransWork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByDtcTransWork_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut _DtcLu_Xln_Confirmation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, error: _DtcLu_Xln_Error) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fcomparestates: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbourtransid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pourtransid: *mut u8, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, comparestate: _DtcLu_CompareState, pconfirmation: *mut _DtcLu_CompareStates_Confirmation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, error: _DtcLu_CompareStates_Error) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plrecoveryseqnum: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lnewrecoveryseqnum: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcLuRecoveryInitiatedByLu(pub ::windows::core::IUnknown);
impl IDtcLuRecoveryInitiatedByLu {
    pub unsafe fn GetObjectToHandleWorkFromLu(&self) -> ::windows::core::Result<IDtcLuRecoveryInitiatedByLuWork> {
        let mut result__: <IDtcLuRecoveryInitiatedByLuWork as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDtcLuRecoveryInitiatedByLuWork>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRecoveryInitiatedByLu {
    type Vtable = IDtcLuRecoveryInitiatedByLu_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e768_1aea_11d0_944b_00a0c905416e);
}
impl ::core::convert::From<IDtcLuRecoveryInitiatedByLu> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByLu) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcLuRecoveryInitiatedByLu> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByLu) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcLuRecoveryInitiatedByLu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcLuRecoveryInitiatedByLu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByLu_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcLuRecoveryInitiatedByLuWork(pub ::windows::core::IUnknown);
impl IDtcLuRecoveryInitiatedByLuWork {
    pub unsafe fn HandleTheirXln(&self, lrecoveryseqnum: i32, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut _DtcLu_Xln_Response) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(lrecoveryseqnum),
            ::core::mem::transmute(xln),
            ::core::mem::transmute(premotelogname),
            ::core::mem::transmute(cbremotelogname),
            ::core::mem::transmute(pourlogname),
            ::core::mem::transmute(cbourlogname),
            ::core::mem::transmute(dwprotocol),
            ::core::mem::transmute(presponse),
        )
        .ok()
    }
    pub unsafe fn GetOurLogNameSize(&self, pcbourlogname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcbourlogname)).ok()
    }
    pub unsafe fn GetOurXln(&self, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pxln), ::core::mem::transmute(pourlogname), ::core::mem::transmute(pdwprotocol)).ok()
    }
    pub unsafe fn HandleConfirmationOfOurXln(&self, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(confirmation)).ok()
    }
    pub unsafe fn HandleTheirCompareStates(&self, premotetransid: *mut u8, cbremotetransid: u32, comparestate: _DtcLu_CompareState, presponse: *mut _DtcLu_CompareStates_Response, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(premotetransid), ::core::mem::transmute(cbremotetransid), ::core::mem::transmute(comparestate), ::core::mem::transmute(presponse), ::core::mem::transmute(pcomparestate)).ok()
    }
    pub unsafe fn HandleConfirmationOfOurCompareStates(&self, confirmation: _DtcLu_CompareStates_Confirmation) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(confirmation)).ok()
    }
    pub unsafe fn HandleErrorFromOurCompareStates(&self, error: _DtcLu_CompareStates_Error) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(error)).ok()
    }
    pub unsafe fn ConversationLost(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRecoveryInitiatedByLuWork {
    type Vtable = IDtcLuRecoveryInitiatedByLuWork_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac2b8ad1_d6f0_11d0_b386_00a0c9083365);
}
impl ::core::convert::From<IDtcLuRecoveryInitiatedByLuWork> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByLuWork) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcLuRecoveryInitiatedByLuWork> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByLuWork) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcLuRecoveryInitiatedByLuWork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcLuRecoveryInitiatedByLuWork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByLuWork_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lrecoveryseqnum: i32, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut _DtcLu_Xln_Response) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbourlogname: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, premotetransid: *mut u8, cbremotetransid: u32, comparestate: _DtcLu_CompareState, presponse: *mut _DtcLu_CompareStates_Response, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, confirmation: _DtcLu_CompareStates_Confirmation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, error: _DtcLu_CompareStates_Error) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcLuRmEnlistment(pub ::windows::core::IUnknown);
impl IDtcLuRmEnlistment {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unplug<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fconversationlost: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fconversationlost.into_param().abi()).ok()
    }
    pub unsafe fn BackedOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Committed(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Forget(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRmEnlistment {
    type Vtable = IDtcLuRmEnlistment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e769_1aea_11d0_944b_00a0c905416e);
}
impl ::core::convert::From<IDtcLuRmEnlistment> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRmEnlistment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcLuRmEnlistment> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRmEnlistment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcLuRmEnlistment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcLuRmEnlistment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRmEnlistment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fconversationlost: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcLuRmEnlistmentFactory(pub ::windows::core::IUnknown);
impl IDtcLuRmEnlistmentFactory {
    pub unsafe fn Create<'a, Param2: ::windows::core::IntoParam<'a, ITransaction>, Param5: ::windows::core::IntoParam<'a, IDtcLuRmEnlistmentSink>>(&self, puclupair: *mut u8, cblupair: u32, pitransaction: Param2, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: Param5, pprmenlistment: *mut ::core::option::Option<IDtcLuRmEnlistment>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(puclupair), ::core::mem::transmute(cblupair), pitransaction.into_param().abi(), ::core::mem::transmute(ptransid), ::core::mem::transmute(cbtransid), prmenlistmentsink.into_param().abi(), ::core::mem::transmute(pprmenlistment)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRmEnlistmentFactory {
    type Vtable = IDtcLuRmEnlistmentFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e771_1aea_11d0_944b_00a0c905416e);
}
impl ::core::convert::From<IDtcLuRmEnlistmentFactory> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRmEnlistmentFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcLuRmEnlistmentFactory> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRmEnlistmentFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcLuRmEnlistmentFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcLuRmEnlistmentFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRmEnlistmentFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puclupair: *mut u8, cblupair: u32, pitransaction: ::windows::core::RawPtr, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: ::windows::core::RawPtr, pprmenlistment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcLuRmEnlistmentSink(pub ::windows::core::IUnknown);
impl IDtcLuRmEnlistmentSink {
    pub unsafe fn AckUnplug(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn TmDown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SessionLost(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BackedOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Committed(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Forget(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Prepare(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuRmEnlistmentSink {
    type Vtable = IDtcLuRmEnlistmentSink_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e770_1aea_11d0_944b_00a0c905416e);
}
impl ::core::convert::From<IDtcLuRmEnlistmentSink> for ::windows::core::IUnknown {
    fn from(value: IDtcLuRmEnlistmentSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcLuRmEnlistmentSink> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuRmEnlistmentSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcLuRmEnlistmentSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcLuRmEnlistmentSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRmEnlistmentSink_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcLuSubordinateDtc(pub ::windows::core::IUnknown);
impl IDtcLuSubordinateDtc {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unplug<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fconversationlost: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fconversationlost.into_param().abi()).ok()
    }
    pub unsafe fn BackedOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Committed(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Forget(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Prepare(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuSubordinateDtc {
    type Vtable = IDtcLuSubordinateDtc_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e773_1aea_11d0_944b_00a0c905416e);
}
impl ::core::convert::From<IDtcLuSubordinateDtc> for ::windows::core::IUnknown {
    fn from(value: IDtcLuSubordinateDtc) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcLuSubordinateDtc> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuSubordinateDtc) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcLuSubordinateDtc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcLuSubordinateDtc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuSubordinateDtc_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fconversationlost: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcLuSubordinateDtcFactory(pub ::windows::core::IUnknown);
impl IDtcLuSubordinateDtcFactory {
    pub unsafe fn Create<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param5: ::windows::core::IntoParam<'a, ITransactionOptions>, Param9: ::windows::core::IntoParam<'a, IDtcLuSubordinateDtcSink>>(
        &self,
        puclupair: *mut u8,
        cblupair: u32,
        punktransactionouter: Param2,
        isolevel: i32,
        isoflags: u32,
        poptions: Param5,
        pptransaction: *mut ::core::option::Option<ITransaction>,
        ptransid: *mut u8,
        cbtransid: u32,
        psubordinatedtcsink: Param9,
        ppsubordinatedtc: *mut ::core::option::Option<IDtcLuSubordinateDtc>,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(puclupair),
            ::core::mem::transmute(cblupair),
            punktransactionouter.into_param().abi(),
            ::core::mem::transmute(isolevel),
            ::core::mem::transmute(isoflags),
            poptions.into_param().abi(),
            ::core::mem::transmute(pptransaction),
            ::core::mem::transmute(ptransid),
            ::core::mem::transmute(cbtransid),
            psubordinatedtcsink.into_param().abi(),
            ::core::mem::transmute(ppsubordinatedtc),
        )
        .ok()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuSubordinateDtcFactory {
    type Vtable = IDtcLuSubordinateDtcFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e775_1aea_11d0_944b_00a0c905416e);
}
impl ::core::convert::From<IDtcLuSubordinateDtcFactory> for ::windows::core::IUnknown {
    fn from(value: IDtcLuSubordinateDtcFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcLuSubordinateDtcFactory> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuSubordinateDtcFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcLuSubordinateDtcFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcLuSubordinateDtcFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuSubordinateDtcFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puclupair: *mut u8, cblupair: u32, punktransactionouter: ::windows::core::RawPtr, isolevel: i32, isoflags: u32, poptions: ::windows::core::RawPtr, pptransaction: *mut ::windows::core::RawPtr, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: ::windows::core::RawPtr, ppsubordinatedtc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcLuSubordinateDtcSink(pub ::windows::core::IUnknown);
impl IDtcLuSubordinateDtcSink {
    pub unsafe fn AckUnplug(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn TmDown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SessionLost(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BackedOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Committed(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Forget(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDtcLuSubordinateDtcSink {
    type Vtable = IDtcLuSubordinateDtcSink_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4131e774_1aea_11d0_944b_00a0c905416e);
}
impl ::core::convert::From<IDtcLuSubordinateDtcSink> for ::windows::core::IUnknown {
    fn from(value: IDtcLuSubordinateDtcSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcLuSubordinateDtcSink> for ::windows::core::IUnknown {
    fn from(value: &IDtcLuSubordinateDtcSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcLuSubordinateDtcSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcLuSubordinateDtcSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuSubordinateDtcSink_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcNetworkAccessConfig(pub ::windows::core::IUnknown);
impl IDtcNetworkAccessConfig {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnyNetworkAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAnyNetworkAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, banynetworkaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), banynetworkaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkAdministrationAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkAdministrationAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworkadministrationaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bnetworkadministrationaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTransactionAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTransactionAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworktransactionaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bnetworktransactionaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkClientAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkClientAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworkclientaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), bnetworkclientaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTIPAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTIPAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworktipaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), bnetworktipaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXAAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXAAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bxaaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), bxaaccess.into_param().abi()).ok()
    }
    pub unsafe fn RestartDtcService(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDtcNetworkAccessConfig {
    type Vtable = IDtcNetworkAccessConfig_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9797c15d_a428_4291_87b6_0995031a678d);
}
impl ::core::convert::From<IDtcNetworkAccessConfig> for ::windows::core::IUnknown {
    fn from(value: IDtcNetworkAccessConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcNetworkAccessConfig> for ::windows::core::IUnknown {
    fn from(value: &IDtcNetworkAccessConfig) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcNetworkAccessConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcNetworkAccessConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcNetworkAccessConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbanynetworkaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, banynetworkaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbnetworkadministrationaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbnetworktransactionaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bnetworktransactionaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbnetworkclientaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bnetworkclientaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbnetworktipaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bnetworktipaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbxaaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bxaaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcNetworkAccessConfig2(pub ::windows::core::IUnknown);
impl IDtcNetworkAccessConfig2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnyNetworkAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAnyNetworkAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, banynetworkaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), banynetworkaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkAdministrationAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkAdministrationAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworkadministrationaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bnetworkadministrationaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTransactionAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTransactionAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworktransactionaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bnetworktransactionaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkClientAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkClientAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworkclientaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), bnetworkclientaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTIPAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTIPAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworktipaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), bnetworktipaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXAAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXAAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bxaaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), bxaaccess.into_param().abi()).ok()
    }
    pub unsafe fn RestartDtcService(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkInboundAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkOutboundAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkInboundAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, binbound: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), binbound.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkOutboundAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, boutbound: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), boutbound.into_param().abi()).ok()
    }
    pub unsafe fn GetAuthenticationLevel(&self) -> ::windows::core::Result<AUTHENTICATION_LEVEL> {
        let mut result__: <AUTHENTICATION_LEVEL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AUTHENTICATION_LEVEL>(result__)
    }
    pub unsafe fn SetAuthenticationLevel(&self, authlevel: AUTHENTICATION_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(authlevel)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDtcNetworkAccessConfig2 {
    type Vtable = IDtcNetworkAccessConfig2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7aa013b_eb7d_4f42_b41c_b2dec09ae034);
}
impl ::core::convert::From<IDtcNetworkAccessConfig2> for ::windows::core::IUnknown {
    fn from(value: IDtcNetworkAccessConfig2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcNetworkAccessConfig2> for ::windows::core::IUnknown {
    fn from(value: &IDtcNetworkAccessConfig2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcNetworkAccessConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcNetworkAccessConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDtcNetworkAccessConfig2> for IDtcNetworkAccessConfig {
    fn from(value: IDtcNetworkAccessConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcNetworkAccessConfig2> for IDtcNetworkAccessConfig {
    fn from(value: &IDtcNetworkAccessConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDtcNetworkAccessConfig> for IDtcNetworkAccessConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDtcNetworkAccessConfig> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDtcNetworkAccessConfig> for &IDtcNetworkAccessConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDtcNetworkAccessConfig> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcNetworkAccessConfig2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbanynetworkaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, banynetworkaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbnetworkadministrationaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbnetworktransactionaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bnetworktransactionaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbnetworkclientaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bnetworkclientaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbnetworktipaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bnetworktipaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbxaaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bxaaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbinbound: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pboutbound: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, binbound: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, boutbound: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pauthlevel: *mut AUTHENTICATION_LEVEL) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, authlevel: AUTHENTICATION_LEVEL) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcNetworkAccessConfig3(pub ::windows::core::IUnknown);
impl IDtcNetworkAccessConfig3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnyNetworkAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAnyNetworkAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, banynetworkaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), banynetworkaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkAdministrationAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkAdministrationAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworkadministrationaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bnetworkadministrationaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTransactionAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTransactionAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworktransactionaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bnetworktransactionaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkClientAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkClientAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworkclientaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), bnetworkclientaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTIPAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTIPAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworktipaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), bnetworktipaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXAAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXAAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bxaaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), bxaaccess.into_param().abi()).ok()
    }
    pub unsafe fn RestartDtcService(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkInboundAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkOutboundAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkInboundAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, binbound: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), binbound.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkOutboundAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, boutbound: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), boutbound.into_param().abi()).ok()
    }
    pub unsafe fn GetAuthenticationLevel(&self) -> ::windows::core::Result<AUTHENTICATION_LEVEL> {
        let mut result__: <AUTHENTICATION_LEVEL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AUTHENTICATION_LEVEL>(result__)
    }
    pub unsafe fn SetAuthenticationLevel(&self, authlevel: AUTHENTICATION_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(authlevel)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLUAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLUAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bluaccess: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), bluaccess.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IDtcNetworkAccessConfig3 {
    type Vtable = IDtcNetworkAccessConfig3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76e4b4f3_2ca5_466b_89d5_fd218ee75b49);
}
impl ::core::convert::From<IDtcNetworkAccessConfig3> for ::windows::core::IUnknown {
    fn from(value: IDtcNetworkAccessConfig3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcNetworkAccessConfig3> for ::windows::core::IUnknown {
    fn from(value: &IDtcNetworkAccessConfig3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcNetworkAccessConfig3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcNetworkAccessConfig3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDtcNetworkAccessConfig3> for IDtcNetworkAccessConfig2 {
    fn from(value: IDtcNetworkAccessConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcNetworkAccessConfig3> for IDtcNetworkAccessConfig2 {
    fn from(value: &IDtcNetworkAccessConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDtcNetworkAccessConfig2> for IDtcNetworkAccessConfig3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDtcNetworkAccessConfig2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDtcNetworkAccessConfig2> for &IDtcNetworkAccessConfig3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDtcNetworkAccessConfig2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDtcNetworkAccessConfig3> for IDtcNetworkAccessConfig {
    fn from(value: IDtcNetworkAccessConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcNetworkAccessConfig3> for IDtcNetworkAccessConfig {
    fn from(value: &IDtcNetworkAccessConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDtcNetworkAccessConfig> for IDtcNetworkAccessConfig3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDtcNetworkAccessConfig> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDtcNetworkAccessConfig> for &IDtcNetworkAccessConfig3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDtcNetworkAccessConfig> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcNetworkAccessConfig3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbanynetworkaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, banynetworkaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbnetworkadministrationaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbnetworktransactionaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bnetworktransactionaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbnetworkclientaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bnetworkclientaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbnetworktipaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bnetworktipaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbxaaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bxaaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbinbound: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pboutbound: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, binbound: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, boutbound: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pauthlevel: *mut AUTHENTICATION_LEVEL) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, authlevel: AUTHENTICATION_LEVEL) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbluaccess: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bluaccess: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcToXaHelper(pub ::windows::core::IUnknown);
impl IDtcToXaHelper {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Close<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, i_fdorecovery: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), i_fdorecovery.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateTridToXid<'a, Param0: ::windows::core::IntoParam<'a, ITransaction>>(&self, pitransaction: Param0, pguidbqual: *const ::windows::core::GUID) -> ::windows::core::Result<xid_t> {
        let mut result__: <xid_t as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pitransaction.into_param().abi(), ::core::mem::transmute(pguidbqual), &mut result__).from_abi::<xid_t>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDtcToXaHelper {
    type Vtable = IDtcToXaHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9861611_304a_11d1_9813_00a0c905416e);
}
impl ::core::convert::From<IDtcToXaHelper> for ::windows::core::IUnknown {
    fn from(value: IDtcToXaHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcToXaHelper> for ::windows::core::IUnknown {
    fn from(value: &IDtcToXaHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcToXaHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcToXaHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, i_fdorecovery: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pitransaction: ::windows::core::RawPtr, pguidbqual: *const ::windows::core::GUID, pxid: *mut xid_t) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcToXaHelperFactory(pub ::windows::core::IUnknown);
impl IDtcToXaHelperFactory {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, pszdsn: Param0, pszclientdllname: Param1, pguidrm: *mut ::windows::core::GUID, ppxahelper: *mut ::core::option::Option<IDtcToXaHelper>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszdsn.into_param().abi(), pszclientdllname.into_param().abi(), ::core::mem::transmute(pguidrm), ::core::mem::transmute(ppxahelper)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDtcToXaHelperFactory {
    type Vtable = IDtcToXaHelperFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9861610_304a_11d1_9813_00a0c905416e);
}
impl ::core::convert::From<IDtcToXaHelperFactory> for ::windows::core::IUnknown {
    fn from(value: IDtcToXaHelperFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcToXaHelperFactory> for ::windows::core::IUnknown {
    fn from(value: &IDtcToXaHelperFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcToXaHelperFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcToXaHelperFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaHelperFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszdsn: super::super::Foundation::PSTR, pszclientdllname: super::super::Foundation::PSTR, pguidrm: *mut ::windows::core::GUID, ppxahelper: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcToXaHelperSinglePipe(pub ::windows::core::IUnknown);
impl IDtcToXaHelperSinglePipe {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn XARMCreate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, pszdsn: Param0, pszclientdll: Param1, pdwrmcookie: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszdsn.into_param().abi(), pszclientdll.into_param().abi(), ::core::mem::transmute(pdwrmcookie)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConvertTridToXID(&self, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwitrans), ::core::mem::transmute(dwrmcookie), ::core::mem::transmute(pxid)).ok()
    }
    pub unsafe fn EnlistWithRM<'a, Param1: ::windows::core::IntoParam<'a, ITransaction>, Param2: ::windows::core::IntoParam<'a, ITransactionResourceAsync>>(&self, dwrmcookie: u32, i_pitransaction: Param1, i_pitransres: Param2) -> ::windows::core::Result<ITransactionEnlistmentAsync> {
        let mut result__: <ITransactionEnlistmentAsync as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwrmcookie), i_pitransaction.into_param().abi(), i_pitransres.into_param().abi(), &mut result__).from_abi::<ITransactionEnlistmentAsync>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseRMCookie<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, i_dwrmcookie: u32, i_fnormal: Param1) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(i_dwrmcookie), i_fnormal.into_param().abi()))
    }
}
unsafe impl ::windows::core::Interface for IDtcToXaHelperSinglePipe {
    type Vtable = IDtcToXaHelperSinglePipe_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47ed4971_53b3_11d1_bbb9_00c04fd658f6);
}
impl ::core::convert::From<IDtcToXaHelperSinglePipe> for ::windows::core::IUnknown {
    fn from(value: IDtcToXaHelperSinglePipe) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcToXaHelperSinglePipe> for ::windows::core::IUnknown {
    fn from(value: &IDtcToXaHelperSinglePipe) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcToXaHelperSinglePipe {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcToXaHelperSinglePipe {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaHelperSinglePipe_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszdsn: super::super::Foundation::PSTR, pszclientdll: super::super::Foundation::PSTR, pdwrmcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwrmcookie: u32, i_pitransaction: ::windows::core::RawPtr, i_pitransres: ::windows::core::RawPtr, o_ppitransenslitment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, i_dwrmcookie: u32, i_fnormal: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDtcToXaMapper(pub ::windows::core::IUnknown);
impl IDtcToXaMapper {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestNewResourceManager<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, pszdsn: Param0, pszclientdllname: Param1, pdwrmcookie: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszdsn.into_param().abi(), pszclientdllname.into_param().abi(), ::core::mem::transmute(pdwrmcookie)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateTridToXid(&self, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwitransaction), ::core::mem::transmute(dwrmcookie), ::core::mem::transmute(pxid)).ok()
    }
    pub unsafe fn EnlistResourceManager(&self, dwrmcookie: u32, pdwitransaction: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwrmcookie), ::core::mem::transmute(pdwitransaction)).ok()
    }
    pub unsafe fn ReleaseResourceManager(&self, dwrmcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwrmcookie)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDtcToXaMapper {
    type Vtable = IDtcToXaMapper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64ffabe0_7ce9_11d0_8ce6_00c04fdc877e);
}
impl ::core::convert::From<IDtcToXaMapper> for ::windows::core::IUnknown {
    fn from(value: IDtcToXaMapper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDtcToXaMapper> for ::windows::core::IUnknown {
    fn from(value: &IDtcToXaMapper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDtcToXaMapper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDtcToXaMapper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaMapper_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszdsn: super::super::Foundation::PSTR, pszclientdllname: super::super::Foundation::PSTR, pdwrmcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwrmcookie: u32, pdwitransaction: *const u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwrmcookie: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGetDispenser(pub ::windows::core::IUnknown);
impl IGetDispenser {
    pub unsafe fn GetDispenser(&self, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject)).ok()
    }
}
unsafe impl ::windows::core::Interface for IGetDispenser {
    type Vtable = IGetDispenser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc23cc370_87ef_11ce_8081_0080c758527e);
}
impl ::core::convert::From<IGetDispenser> for ::windows::core::IUnknown {
    fn from(value: IGetDispenser) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGetDispenser> for ::windows::core::IUnknown {
    fn from(value: &IGetDispenser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGetDispenser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGetDispenser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetDispenser_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IKernelTransaction(pub ::windows::core::IUnknown);
impl IKernelTransaction {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
}
unsafe impl ::windows::core::Interface for IKernelTransaction {
    type Vtable = IKernelTransaction_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79427a2b_f895_40e0_be79_b57dc82ed231);
}
impl ::core::convert::From<IKernelTransaction> for ::windows::core::IUnknown {
    fn from(value: IKernelTransaction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IKernelTransaction> for ::windows::core::IUnknown {
    fn from(value: &IKernelTransaction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IKernelTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IKernelTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKernelTransaction_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ILastResourceManager(pub ::windows::core::IUnknown);
impl ILastResourceManager {
    pub unsafe fn TransactionCommitted(&self, pprepinfo: *const u8, cbprepinfo: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprepinfo), ::core::mem::transmute(cbprepinfo)).ok()
    }
    pub unsafe fn RecoveryDone(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ILastResourceManager {
    type Vtable = ILastResourceManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d964ad4_5b33_11d3_8a91_00c04f79eb6d);
}
impl ::core::convert::From<ILastResourceManager> for ::windows::core::IUnknown {
    fn from(value: ILastResourceManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILastResourceManager> for ::windows::core::IUnknown {
    fn from(value: &ILastResourceManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILastResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILastResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILastResourceManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprepinfo: *const u8, cbprepinfo: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrepareInfo(pub ::windows::core::IUnknown);
impl IPrepareInfo {
    pub unsafe fn GetPrepareInfoSize(&self, pcbprepinfo: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcbprepinfo)).ok()
    }
    pub unsafe fn GetPrepareInfo(&self, pprepinfo: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprepinfo)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPrepareInfo {
    type Vtable = IPrepareInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80c7bfd0_87ee_11ce_8081_0080c758527e);
}
impl ::core::convert::From<IPrepareInfo> for ::windows::core::IUnknown {
    fn from(value: IPrepareInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrepareInfo> for ::windows::core::IUnknown {
    fn from(value: &IPrepareInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrepareInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrepareInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrepareInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbprepinfo: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprepinfo: *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrepareInfo2(pub ::windows::core::IUnknown);
impl IPrepareInfo2 {
    pub unsafe fn GetPrepareInfoSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPrepareInfo(&self, cbprepareinfo: u32, pprepinfo: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbprepareinfo), ::core::mem::transmute(pprepinfo)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPrepareInfo2 {
    type Vtable = IPrepareInfo2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fab2547_9779_11d1_b886_00c04fb9618a);
}
impl ::core::convert::From<IPrepareInfo2> for ::windows::core::IUnknown {
    fn from(value: IPrepareInfo2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrepareInfo2> for ::windows::core::IUnknown {
    fn from(value: &IPrepareInfo2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrepareInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrepareInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrepareInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbprepinfo: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbprepareinfo: u32, pprepinfo: *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRMHelper(pub ::windows::core::IUnknown);
impl IRMHelper {
    pub unsafe fn RMCount(&self, dwctotalnumberofrms: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwctotalnumberofrms)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RMInfo<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: Param1, pszopenstring: Param2, pszclosestring: Param3, guidrmrecovery: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pxa_switch), fcdeclcallingconv.into_param().abi(), pszopenstring.into_param().abi(), pszclosestring.into_param().abi(), guidrmrecovery.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IRMHelper {
    type Vtable = IRMHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe793f6d1_f53d_11cf_a60d_00a0c905416e);
}
impl ::core::convert::From<IRMHelper> for ::windows::core::IUnknown {
    fn from(value: IRMHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRMHelper> for ::windows::core::IUnknown {
    fn from(value: &IRMHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRMHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRMHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRMHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwctotalnumberofrms: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: super::super::Foundation::BOOL, pszopenstring: super::super::Foundation::PSTR, pszclosestring: super::super::Foundation::PSTR, guidrmrecovery: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IResourceManager(pub ::windows::core::IUnknown);
impl IResourceManager {
    pub unsafe fn Enlist<'a, Param0: ::windows::core::IntoParam<'a, ITransaction>, Param1: ::windows::core::IntoParam<'a, ITransactionResourceAsync>>(&self, ptransaction: Param0, pres: Param1, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ptransaction.into_param().abi(), pres.into_param().abi(), ::core::mem::transmute(puow), ::core::mem::transmute(pisolevel), ::core::mem::transmute(ppenlist)).ok()
    }
    pub unsafe fn Reenlist(&self, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32) -> ::windows::core::Result<XACTSTAT> {
        let mut result__: <XACTSTAT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprepinfo), ::core::mem::transmute(cbprepinfo), ::core::mem::transmute(ltimeout), &mut result__).from_abi::<XACTSTAT>(result__)
    }
    pub unsafe fn ReenlistmentComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetDistributedTransactionManager(&self, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject)).ok()
    }
}
unsafe impl ::windows::core::Interface for IResourceManager {
    type Vtable = IResourceManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13741d21_87eb_11ce_8081_0080c758527e);
}
impl ::core::convert::From<IResourceManager> for ::windows::core::IUnknown {
    fn from(value: IResourceManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IResourceManager> for ::windows::core::IUnknown {
    fn from(value: &IResourceManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptransaction: ::windows::core::RawPtr, pres: ::windows::core::RawPtr, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IResourceManager2(pub ::windows::core::IUnknown);
impl IResourceManager2 {
    pub unsafe fn Enlist<'a, Param0: ::windows::core::IntoParam<'a, ITransaction>, Param1: ::windows::core::IntoParam<'a, ITransactionResourceAsync>>(&self, ptransaction: Param0, pres: Param1, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ptransaction.into_param().abi(), pres.into_param().abi(), ::core::mem::transmute(puow), ::core::mem::transmute(pisolevel), ::core::mem::transmute(ppenlist)).ok()
    }
    pub unsafe fn Reenlist(&self, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32) -> ::windows::core::Result<XACTSTAT> {
        let mut result__: <XACTSTAT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprepinfo), ::core::mem::transmute(cbprepinfo), ::core::mem::transmute(ltimeout), &mut result__).from_abi::<XACTSTAT>(result__)
    }
    pub unsafe fn ReenlistmentComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetDistributedTransactionManager(&self, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enlist2<'a, Param0: ::windows::core::IntoParam<'a, ITransaction>, Param1: ::windows::core::IntoParam<'a, ITransactionResourceAsync>>(&self, ptransaction: Param0, presasync: Param1, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ptransaction.into_param().abi(), presasync.into_param().abi(), ::core::mem::transmute(puow), ::core::mem::transmute(pisolevel), ::core::mem::transmute(pxid), ::core::mem::transmute(ppenlist)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Reenlist2(&self, pxid: *const xid_t, dwtimeout: u32) -> ::windows::core::Result<XACTSTAT> {
        let mut result__: <XACTSTAT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pxid), ::core::mem::transmute(dwtimeout), &mut result__).from_abi::<XACTSTAT>(result__)
    }
}
unsafe impl ::windows::core::Interface for IResourceManager2 {
    type Vtable = IResourceManager2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd136c69a_f749_11d1_8f47_00c04f8ee57d);
}
impl ::core::convert::From<IResourceManager2> for ::windows::core::IUnknown {
    fn from(value: IResourceManager2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IResourceManager2> for ::windows::core::IUnknown {
    fn from(value: &IResourceManager2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IResourceManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IResourceManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IResourceManager2> for IResourceManager {
    fn from(value: IResourceManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManager2> for IResourceManager {
    fn from(value: &IResourceManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IResourceManager> for IResourceManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, IResourceManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IResourceManager> for &IResourceManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, IResourceManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptransaction: ::windows::core::RawPtr, pres: ::windows::core::RawPtr, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptransaction: ::windows::core::RawPtr, presasync: ::windows::core::RawPtr, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pxid: *const xid_t, dwtimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IResourceManagerFactory(pub ::windows::core::IUnknown);
impl IResourceManagerFactory {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, IResourceManagerSink>>(&self, pguidrm: *const ::windows::core::GUID, pszrmname: Param1, piresmgrsink: Param2) -> ::windows::core::Result<IResourceManager> {
        let mut result__: <IResourceManager as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidrm), pszrmname.into_param().abi(), piresmgrsink.into_param().abi(), &mut result__).from_abi::<IResourceManager>(result__)
    }
}
unsafe impl ::windows::core::Interface for IResourceManagerFactory {
    type Vtable = IResourceManagerFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13741d20_87eb_11ce_8081_0080c758527e);
}
impl ::core::convert::From<IResourceManagerFactory> for ::windows::core::IUnknown {
    fn from(value: IResourceManagerFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IResourceManagerFactory> for ::windows::core::IUnknown {
    fn from(value: &IResourceManagerFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IResourceManagerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IResourceManagerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pguidrm: *const ::windows::core::GUID, pszrmname: super::super::Foundation::PSTR, piresmgrsink: ::windows::core::RawPtr, ppresmgr: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IResourceManagerFactory2(pub ::windows::core::IUnknown);
impl IResourceManagerFactory2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, IResourceManagerSink>>(&self, pguidrm: *const ::windows::core::GUID, pszrmname: Param1, piresmgrsink: Param2) -> ::windows::core::Result<IResourceManager> {
        let mut result__: <IResourceManager as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidrm), pszrmname.into_param().abi(), piresmgrsink.into_param().abi(), &mut result__).from_abi::<IResourceManager>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateEx<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, IResourceManagerSink>>(&self, pguidrm: *const ::windows::core::GUID, pszrmname: Param1, piresmgrsink: Param2, riidrequested: *const ::windows::core::GUID, ppvresmgr: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidrm), pszrmname.into_param().abi(), piresmgrsink.into_param().abi(), ::core::mem::transmute(riidrequested), ::core::mem::transmute(ppvresmgr)).ok()
    }
}
unsafe impl ::windows::core::Interface for IResourceManagerFactory2 {
    type Vtable = IResourceManagerFactory2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b369c21_fbd2_11d1_8f47_00c04f8ee57d);
}
impl ::core::convert::From<IResourceManagerFactory2> for ::windows::core::IUnknown {
    fn from(value: IResourceManagerFactory2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IResourceManagerFactory2> for ::windows::core::IUnknown {
    fn from(value: &IResourceManagerFactory2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IResourceManagerFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IResourceManagerFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IResourceManagerFactory2> for IResourceManagerFactory {
    fn from(value: IResourceManagerFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManagerFactory2> for IResourceManagerFactory {
    fn from(value: &IResourceManagerFactory2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IResourceManagerFactory> for IResourceManagerFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, IResourceManagerFactory> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IResourceManagerFactory> for &IResourceManagerFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, IResourceManagerFactory> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pguidrm: *const ::windows::core::GUID, pszrmname: super::super::Foundation::PSTR, piresmgrsink: ::windows::core::RawPtr, ppresmgr: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pguidrm: *const ::windows::core::GUID, pszrmname: super::super::Foundation::PSTR, piresmgrsink: ::windows::core::RawPtr, riidrequested: *const ::windows::core::GUID, ppvresmgr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IResourceManagerRejoinable(pub ::windows::core::IUnknown);
impl IResourceManagerRejoinable {
    pub unsafe fn Enlist<'a, Param0: ::windows::core::IntoParam<'a, ITransaction>, Param1: ::windows::core::IntoParam<'a, ITransactionResourceAsync>>(&self, ptransaction: Param0, pres: Param1, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ptransaction.into_param().abi(), pres.into_param().abi(), ::core::mem::transmute(puow), ::core::mem::transmute(pisolevel), ::core::mem::transmute(ppenlist)).ok()
    }
    pub unsafe fn Reenlist(&self, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32) -> ::windows::core::Result<XACTSTAT> {
        let mut result__: <XACTSTAT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprepinfo), ::core::mem::transmute(cbprepinfo), ::core::mem::transmute(ltimeout), &mut result__).from_abi::<XACTSTAT>(result__)
    }
    pub unsafe fn ReenlistmentComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetDistributedTransactionManager(&self, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enlist2<'a, Param0: ::windows::core::IntoParam<'a, ITransaction>, Param1: ::windows::core::IntoParam<'a, ITransactionResourceAsync>>(&self, ptransaction: Param0, presasync: Param1, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ptransaction.into_param().abi(), presasync.into_param().abi(), ::core::mem::transmute(puow), ::core::mem::transmute(pisolevel), ::core::mem::transmute(pxid), ::core::mem::transmute(ppenlist)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Reenlist2(&self, pxid: *const xid_t, dwtimeout: u32) -> ::windows::core::Result<XACTSTAT> {
        let mut result__: <XACTSTAT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pxid), ::core::mem::transmute(dwtimeout), &mut result__).from_abi::<XACTSTAT>(result__)
    }
    pub unsafe fn Rejoin(&self, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32) -> ::windows::core::Result<XACTSTAT> {
        let mut result__: <XACTSTAT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprepinfo), ::core::mem::transmute(cbprepinfo), ::core::mem::transmute(ltimeout), &mut result__).from_abi::<XACTSTAT>(result__)
    }
}
unsafe impl ::windows::core::Interface for IResourceManagerRejoinable {
    type Vtable = IResourceManagerRejoinable_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f6de620_b5df_4f3e_9cfa_c8aebd05172b);
}
impl ::core::convert::From<IResourceManagerRejoinable> for ::windows::core::IUnknown {
    fn from(value: IResourceManagerRejoinable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IResourceManagerRejoinable> for ::windows::core::IUnknown {
    fn from(value: &IResourceManagerRejoinable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IResourceManagerRejoinable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IResourceManagerRejoinable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IResourceManagerRejoinable> for IResourceManager2 {
    fn from(value: IResourceManagerRejoinable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManagerRejoinable> for IResourceManager2 {
    fn from(value: &IResourceManagerRejoinable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IResourceManager2> for IResourceManagerRejoinable {
    fn into_param(self) -> ::windows::core::Param<'a, IResourceManager2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IResourceManager2> for &IResourceManagerRejoinable {
    fn into_param(self) -> ::windows::core::Param<'a, IResourceManager2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IResourceManagerRejoinable> for IResourceManager {
    fn from(value: IResourceManagerRejoinable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManagerRejoinable> for IResourceManager {
    fn from(value: &IResourceManagerRejoinable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IResourceManager> for IResourceManagerRejoinable {
    fn into_param(self) -> ::windows::core::Param<'a, IResourceManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IResourceManager> for &IResourceManagerRejoinable {
    fn into_param(self) -> ::windows::core::Param<'a, IResourceManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerRejoinable_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptransaction: ::windows::core::RawPtr, pres: ::windows::core::RawPtr, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptransaction: ::windows::core::RawPtr, presasync: ::windows::core::RawPtr, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pxid: *const xid_t, dwtimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IResourceManagerSink(pub ::windows::core::IUnknown);
impl IResourceManagerSink {
    pub unsafe fn TMDown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IResourceManagerSink {
    type Vtable = IResourceManagerSink_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d563181_defb_11ce_aed1_00aa0051e2c4);
}
impl ::core::convert::From<IResourceManagerSink> for ::windows::core::IUnknown {
    fn from(value: IResourceManagerSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IResourceManagerSink> for ::windows::core::IUnknown {
    fn from(value: &IResourceManagerSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IResourceManagerSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IResourceManagerSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerSink_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ISOFLAG(pub i32);
pub const ISOFLAG_RETAIN_COMMIT_DC: ISOFLAG = ISOFLAG(1i32);
pub const ISOFLAG_RETAIN_COMMIT: ISOFLAG = ISOFLAG(2i32);
pub const ISOFLAG_RETAIN_COMMIT_NO: ISOFLAG = ISOFLAG(3i32);
pub const ISOFLAG_RETAIN_ABORT_DC: ISOFLAG = ISOFLAG(4i32);
pub const ISOFLAG_RETAIN_ABORT: ISOFLAG = ISOFLAG(8i32);
pub const ISOFLAG_RETAIN_ABORT_NO: ISOFLAG = ISOFLAG(12i32);
pub const ISOFLAG_RETAIN_DONTCARE: ISOFLAG = ISOFLAG(5i32);
pub const ISOFLAG_RETAIN_BOTH: ISOFLAG = ISOFLAG(10i32);
pub const ISOFLAG_RETAIN_NONE: ISOFLAG = ISOFLAG(15i32);
pub const ISOFLAG_OPTIMISTIC: ISOFLAG = ISOFLAG(16i32);
pub const ISOFLAG_READONLY: ISOFLAG = ISOFLAG(32i32);
impl ::core::convert::From<i32> for ISOFLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ISOFLAG {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ISOLATIONLEVEL(pub i32);
pub const ISOLATIONLEVEL_UNSPECIFIED: ISOLATIONLEVEL = ISOLATIONLEVEL(-1i32);
pub const ISOLATIONLEVEL_CHAOS: ISOLATIONLEVEL = ISOLATIONLEVEL(16i32);
pub const ISOLATIONLEVEL_READUNCOMMITTED: ISOLATIONLEVEL = ISOLATIONLEVEL(256i32);
pub const ISOLATIONLEVEL_BROWSE: ISOLATIONLEVEL = ISOLATIONLEVEL(256i32);
pub const ISOLATIONLEVEL_CURSORSTABILITY: ISOLATIONLEVEL = ISOLATIONLEVEL(4096i32);
pub const ISOLATIONLEVEL_READCOMMITTED: ISOLATIONLEVEL = ISOLATIONLEVEL(4096i32);
pub const ISOLATIONLEVEL_REPEATABLEREAD: ISOLATIONLEVEL = ISOLATIONLEVEL(65536i32);
pub const ISOLATIONLEVEL_SERIALIZABLE: ISOLATIONLEVEL = ISOLATIONLEVEL(1048576i32);
pub const ISOLATIONLEVEL_ISOLATED: ISOLATIONLEVEL = ISOLATIONLEVEL(1048576i32);
impl ::core::convert::From<i32> for ISOLATIONLEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ISOLATIONLEVEL {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITipHelper(pub ::windows::core::IUnknown);
impl ITipHelper {
    pub unsafe fn Pull(&self, i_psztxurl: *const u8) -> ::windows::core::Result<ITransaction> {
        let mut result__: <ITransaction as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(i_psztxurl), &mut result__).from_abi::<ITransaction>(result__)
    }
    pub unsafe fn PullAsync<'a, Param1: ::windows::core::IntoParam<'a, ITipPullSink>>(&self, i_psztxurl: *const u8, i_ptippullsink: Param1) -> ::windows::core::Result<ITransaction> {
        let mut result__: <ITransaction as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(i_psztxurl), i_ptippullsink.into_param().abi(), &mut result__).from_abi::<ITransaction>(result__)
    }
    pub unsafe fn GetLocalTmUrl(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__: <*mut u8 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut u8>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITipHelper {
    type Vtable = ITipHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17cf72d1_bac5_11d1_b1bf_00c04fc2f3ef);
}
impl ::core::convert::From<ITipHelper> for ::windows::core::IUnknown {
    fn from(value: ITipHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITipHelper> for ::windows::core::IUnknown {
    fn from(value: &ITipHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITipHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITipHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, i_psztxurl: *const u8, o_ppitransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, i_psztxurl: *const u8, i_ptippullsink: ::windows::core::RawPtr, o_ppitransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, o_ppszlocaltmurl: *mut *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITipPullSink(pub ::windows::core::IUnknown);
impl ITipPullSink {
    pub unsafe fn PullComplete(&self, i_hrpull: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(i_hrpull)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITipPullSink {
    type Vtable = ITipPullSink_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17cf72d2_bac5_11d1_b1bf_00c04fc2f3ef);
}
impl ::core::convert::From<ITipPullSink> for ::windows::core::IUnknown {
    fn from(value: ITipPullSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITipPullSink> for ::windows::core::IUnknown {
    fn from(value: &ITipPullSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITipPullSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITipPullSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipPullSink_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, i_hrpull: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITipTransaction(pub ::windows::core::IUnknown);
impl ITipTransaction {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Push(&self, i_pszremotetmurl: *const u8) -> ::windows::core::Result<super::super::Foundation::PSTR> {
        let mut result__: <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(i_pszremotetmurl), &mut result__).from_abi::<super::super::Foundation::PSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransactionUrl(&self) -> ::windows::core::Result<super::super::Foundation::PSTR> {
        let mut result__: <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITipTransaction {
    type Vtable = ITipTransaction_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17cf72d0_bac5_11d1_b1bf_00c04fc2f3ef);
}
impl ::core::convert::From<ITipTransaction> for ::windows::core::IUnknown {
    fn from(value: ITipTransaction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITipTransaction> for ::windows::core::IUnknown {
    fn from(value: &ITipTransaction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITipTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITipTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipTransaction_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, i_pszremotetmurl: *const u8, o_ppszremotetxurl: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, o_ppszlocaltxurl: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITmNodeName(pub ::windows::core::IUnknown);
impl ITmNodeName {
    pub unsafe fn GetNodeNameSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNodeName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cbnodenamebuffersize: u32, pnodenamebuffer: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbnodenamebuffersize), pnodenamebuffer.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for ITmNodeName {
    type Vtable = ITmNodeName_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30274f88_6ee4_474e_9b95_7807bc9ef8cf);
}
impl ::core::convert::From<ITmNodeName> for ::windows::core::IUnknown {
    fn from(value: ITmNodeName) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITmNodeName> for ::windows::core::IUnknown {
    fn from(value: &ITmNodeName) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITmNodeName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITmNodeName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITmNodeName_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbnodenamesize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbnodenamebuffersize: u32, pnodenamebuffer: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransaction(pub ::windows::core::IUnknown);
impl ITransaction {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Commit<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, grftc: u32, grfrm: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fretaining.into_param().abi(), ::core::mem::transmute(grftc), ::core::mem::transmute(grfrm)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Abort<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, fasync: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pboidreason), fretaining.into_param().abi(), fasync.into_param().abi()).ok()
    }
    pub unsafe fn GetTransactionInfo(&self) -> ::windows::core::Result<XACTTRANSINFO> {
        let mut result__: <XACTTRANSINFO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XACTTRANSINFO>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITransaction {
    type Vtable = ITransaction_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fb15084_af41_11ce_bd2b_204c4f4f5020);
}
impl ::core::convert::From<ITransaction> for ::windows::core::IUnknown {
    fn from(value: ITransaction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransaction> for ::windows::core::IUnknown {
    fn from(value: &ITransaction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransaction_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinfo: *mut XACTTRANSINFO) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransaction2(pub ::windows::core::IUnknown);
impl ITransaction2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Commit<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, grftc: u32, grfrm: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fretaining.into_param().abi(), ::core::mem::transmute(grftc), ::core::mem::transmute(grfrm)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Abort<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, fasync: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pboidreason), fretaining.into_param().abi(), fasync.into_param().abi()).ok()
    }
    pub unsafe fn GetTransactionInfo(&self) -> ::windows::core::Result<XACTTRANSINFO> {
        let mut result__: <XACTTRANSINFO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XACTTRANSINFO>(result__)
    }
    pub unsafe fn CloneWithCommitDisabled(&self) -> ::windows::core::Result<ITransaction> {
        let mut result__: <ITransaction as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITransaction>(result__)
    }
    pub unsafe fn GetTransactionInfo2(&self) -> ::windows::core::Result<XACTTRANSINFO> {
        let mut result__: <XACTTRANSINFO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XACTTRANSINFO>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITransaction2 {
    type Vtable = ITransaction2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34021548_0065_11d3_bac1_00c04f797be2);
}
impl ::core::convert::From<ITransaction2> for ::windows::core::IUnknown {
    fn from(value: ITransaction2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransaction2> for ::windows::core::IUnknown {
    fn from(value: &ITransaction2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransaction2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransaction2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITransaction2> for ITransactionCloner {
    fn from(value: ITransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransaction2> for ITransactionCloner {
    fn from(value: &ITransaction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITransactionCloner> for ITransaction2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITransactionCloner> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITransactionCloner> for &ITransaction2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITransactionCloner> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITransaction2> for ITransaction {
    fn from(value: ITransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransaction2> for ITransaction {
    fn from(value: &ITransaction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITransaction> for ITransaction2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITransaction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITransaction> for &ITransaction2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITransaction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransaction2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinfo: *mut XACTTRANSINFO) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppitransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinfo: *mut XACTTRANSINFO) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionCloner(pub ::windows::core::IUnknown);
impl ITransactionCloner {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Commit<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, grftc: u32, grfrm: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fretaining.into_param().abi(), ::core::mem::transmute(grftc), ::core::mem::transmute(grfrm)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Abort<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, fasync: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pboidreason), fretaining.into_param().abi(), fasync.into_param().abi()).ok()
    }
    pub unsafe fn GetTransactionInfo(&self) -> ::windows::core::Result<XACTTRANSINFO> {
        let mut result__: <XACTTRANSINFO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XACTTRANSINFO>(result__)
    }
    pub unsafe fn CloneWithCommitDisabled(&self) -> ::windows::core::Result<ITransaction> {
        let mut result__: <ITransaction as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITransaction>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITransactionCloner {
    type Vtable = ITransactionCloner_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02656950_2152_11d0_944c_00a0c905416e);
}
impl ::core::convert::From<ITransactionCloner> for ::windows::core::IUnknown {
    fn from(value: ITransactionCloner) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionCloner> for ::windows::core::IUnknown {
    fn from(value: &ITransactionCloner) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionCloner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionCloner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITransactionCloner> for ITransaction {
    fn from(value: ITransactionCloner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionCloner> for ITransaction {
    fn from(value: &ITransactionCloner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITransaction> for ITransactionCloner {
    fn into_param(self) -> ::windows::core::Param<'a, ITransaction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITransaction> for &ITransactionCloner {
    fn into_param(self) -> ::windows::core::Param<'a, ITransaction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionCloner_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinfo: *mut XACTTRANSINFO) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppitransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionDispenser(pub ::windows::core::IUnknown);
impl ITransactionDispenser {
    pub unsafe fn GetOptionsObject(&self) -> ::windows::core::Result<ITransactionOptions> {
        let mut result__: <ITransactionOptions as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITransactionOptions>(result__)
    }
    pub unsafe fn BeginTransaction<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, ITransactionOptions>>(&self, punkouter: Param0, isolevel: i32, isoflags: u32, poptions: Param3) -> ::windows::core::Result<ITransaction> {
        let mut result__: <ITransaction as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), punkouter.into_param().abi(), ::core::mem::transmute(isolevel), ::core::mem::transmute(isoflags), poptions.into_param().abi(), &mut result__).from_abi::<ITransaction>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITransactionDispenser {
    type Vtable = ITransactionDispenser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a6ad9e1_23b9_11cf_ad60_00aa00a74ccd);
}
impl ::core::convert::From<ITransactionDispenser> for ::windows::core::IUnknown {
    fn from(value: ITransactionDispenser) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionDispenser> for ::windows::core::IUnknown {
    fn from(value: &ITransactionDispenser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionDispenser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionDispenser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionDispenser_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, punkouter: ::windows::core::RawPtr, isolevel: i32, isoflags: u32, poptions: ::windows::core::RawPtr, pptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionEnlistmentAsync(pub ::windows::core::IUnknown);
impl ITransactionEnlistmentAsync {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrepareRequestDone<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IMoniker>>(&self, hr: ::windows::core::HRESULT, pmk: Param1, pboidreason: *const BOID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr), pmk.into_param().abi(), ::core::mem::transmute(pboidreason)).ok()
    }
    pub unsafe fn CommitRequestDone(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
    pub unsafe fn AbortRequestDone(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITransactionEnlistmentAsync {
    type Vtable = ITransactionEnlistmentAsync_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fb15081_af41_11ce_bd2b_204c4f4f5020);
}
impl ::core::convert::From<ITransactionEnlistmentAsync> for ::windows::core::IUnknown {
    fn from(value: ITransactionEnlistmentAsync) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionEnlistmentAsync> for ::windows::core::IUnknown {
    fn from(value: &ITransactionEnlistmentAsync) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionEnlistmentAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionEnlistmentAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionEnlistmentAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hr: ::windows::core::HRESULT, pmk: ::windows::core::RawPtr, pboidreason: *const BOID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionExport(pub ::windows::core::IUnknown);
impl ITransactionExport {
    pub unsafe fn Export<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punktransaction: Param0) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punktransaction.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetTransactionCookie<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punktransaction: Param0, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), punktransaction.into_param().abi(), ::core::mem::transmute(cbtransactioncookie), ::core::mem::transmute(rgbtransactioncookie), ::core::mem::transmute(pcbused)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITransactionExport {
    type Vtable = ITransactionExport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0141fda5_8fc0_11ce_bd18_204c4f4f5020);
}
impl ::core::convert::From<ITransactionExport> for ::windows::core::IUnknown {
    fn from(value: ITransactionExport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionExport> for ::windows::core::IUnknown {
    fn from(value: &ITransactionExport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionExport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionExport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionExport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, punktransaction: ::windows::core::RawPtr, pcbtransactioncookie: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, punktransaction: ::windows::core::RawPtr, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionExportFactory(pub ::windows::core::IUnknown);
impl ITransactionExportFactory {
    pub unsafe fn GetRemoteClassId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn Create(&self, cbwhereabouts: u32, rgbwhereabouts: *const u8) -> ::windows::core::Result<ITransactionExport> {
        let mut result__: <ITransactionExport as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbwhereabouts), ::core::mem::transmute(rgbwhereabouts), &mut result__).from_abi::<ITransactionExport>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITransactionExportFactory {
    type Vtable = ITransactionExportFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1cf9b53_8745_11ce_a9ba_00aa006c3706);
}
impl ::core::convert::From<ITransactionExportFactory> for ::windows::core::IUnknown {
    fn from(value: ITransactionExportFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionExportFactory> for ::windows::core::IUnknown {
    fn from(value: &ITransactionExportFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionExportFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionExportFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionExportFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbwhereabouts: u32, rgbwhereabouts: *const u8, ppexport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionImport(pub ::windows::core::IUnknown);
impl ITransactionImport {
    pub unsafe fn Import<T: ::windows::core::Interface>(&self, cbtransactioncookie: u32, rgbtransactioncookie: *const u8) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbtransactioncookie), ::core::mem::transmute(rgbtransactioncookie), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for ITransactionImport {
    type Vtable = ITransactionImport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1cf9b5a_8745_11ce_a9ba_00aa006c3706);
}
impl ::core::convert::From<ITransactionImport> for ::windows::core::IUnknown {
    fn from(value: ITransactionImport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionImport> for ::windows::core::IUnknown {
    fn from(value: &ITransactionImport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionImport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionImport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionImport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbtransactioncookie: u32, rgbtransactioncookie: *const u8, piid: *const ::windows::core::GUID, ppvtransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionImportWhereabouts(pub ::windows::core::IUnknown);
impl ITransactionImportWhereabouts {
    pub unsafe fn GetWhereaboutsSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetWhereabouts(&self, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbwhereabouts), ::core::mem::transmute(rgbwhereabouts), ::core::mem::transmute(pcbused)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITransactionImportWhereabouts {
    type Vtable = ITransactionImportWhereabouts_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0141fda4_8fc0_11ce_bd18_204c4f4f5020);
}
impl ::core::convert::From<ITransactionImportWhereabouts> for ::windows::core::IUnknown {
    fn from(value: ITransactionImportWhereabouts) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionImportWhereabouts> for ::windows::core::IUnknown {
    fn from(value: &ITransactionImportWhereabouts) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionImportWhereabouts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionImportWhereabouts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionImportWhereabouts_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbwhereabouts: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionLastEnlistmentAsync(pub ::windows::core::IUnknown);
impl ITransactionLastEnlistmentAsync {
    pub unsafe fn TransactionOutcome(&self, xactstat: XACTSTAT, pboidreason: *const BOID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(xactstat), ::core::mem::transmute(pboidreason)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITransactionLastEnlistmentAsync {
    type Vtable = ITransactionLastEnlistmentAsync_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc82bd533_5b30_11d3_8a91_00c04f79eb6d);
}
impl ::core::convert::From<ITransactionLastEnlistmentAsync> for ::windows::core::IUnknown {
    fn from(value: ITransactionLastEnlistmentAsync) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionLastEnlistmentAsync> for ::windows::core::IUnknown {
    fn from(value: &ITransactionLastEnlistmentAsync) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionLastEnlistmentAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionLastEnlistmentAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionLastEnlistmentAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xactstat: XACTSTAT, pboidreason: *const BOID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionLastResourceAsync(pub ::windows::core::IUnknown);
impl ITransactionLastResourceAsync {
    pub unsafe fn DelegateCommit(&self, grfrm: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfrm)).ok()
    }
    pub unsafe fn ForgetRequest(&self, pnewuow: *const BOID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pnewuow)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITransactionLastResourceAsync {
    type Vtable = ITransactionLastResourceAsync_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc82bd532_5b30_11d3_8a91_00c04f79eb6d);
}
impl ::core::convert::From<ITransactionLastResourceAsync> for ::windows::core::IUnknown {
    fn from(value: ITransactionLastResourceAsync) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionLastResourceAsync> for ::windows::core::IUnknown {
    fn from(value: &ITransactionLastResourceAsync) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionLastResourceAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionLastResourceAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionLastResourceAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, grfrm: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnewuow: *const BOID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionOptions(pub ::windows::core::IUnknown);
impl ITransactionOptions {
    pub unsafe fn SetOptions(&self, poptions: *const XACTOPT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(poptions)).ok()
    }
    pub unsafe fn GetOptions(&self, poptions: *mut XACTOPT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(poptions)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITransactionOptions {
    type Vtable = ITransactionOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a6ad9e0_23b9_11cf_ad60_00aa00a74ccd);
}
impl ::core::convert::From<ITransactionOptions> for ::windows::core::IUnknown {
    fn from(value: ITransactionOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionOptions> for ::windows::core::IUnknown {
    fn from(value: &ITransactionOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poptions: *const XACTOPT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poptions: *mut XACTOPT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionOutcomeEvents(pub ::windows::core::IUnknown);
impl ITransactionOutcomeEvents {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Committed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fretaining.into_param().abi(), ::core::mem::transmute(pnewuow), ::core::mem::transmute(hr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Aborted<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pboidreason), fretaining.into_param().abi(), ::core::mem::transmute(pnewuow), ::core::mem::transmute(hr)).ok()
    }
    pub unsafe fn HeuristicDecision(&self, dwdecision: u32, pboidreason: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdecision), ::core::mem::transmute(pboidreason), ::core::mem::transmute(hr)).ok()
    }
    pub unsafe fn Indoubt(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITransactionOutcomeEvents {
    type Vtable = ITransactionOutcomeEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a6ad9e2_23b9_11cf_ad60_00aa00a74ccd);
}
impl ::core::convert::From<ITransactionOutcomeEvents> for ::windows::core::IUnknown {
    fn from(value: ITransactionOutcomeEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionOutcomeEvents> for ::windows::core::IUnknown {
    fn from(value: &ITransactionOutcomeEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionOutcomeEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionOutcomeEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionOutcomeEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwdecision: u32, pboidreason: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionPhase0EnlistmentAsync(pub ::windows::core::IUnknown);
impl ITransactionPhase0EnlistmentAsync {
    pub unsafe fn Enable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn WaitForEnlistment(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Phase0Done(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Unenlist(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetTransaction(&self) -> ::windows::core::Result<ITransaction> {
        let mut result__: <ITransaction as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITransaction>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITransactionPhase0EnlistmentAsync {
    type Vtable = ITransactionPhase0EnlistmentAsync_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82dc88e1_a954_11d1_8f88_00600895e7d5);
}
impl ::core::convert::From<ITransactionPhase0EnlistmentAsync> for ::windows::core::IUnknown {
    fn from(value: ITransactionPhase0EnlistmentAsync) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionPhase0EnlistmentAsync> for ::windows::core::IUnknown {
    fn from(value: &ITransactionPhase0EnlistmentAsync) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionPhase0EnlistmentAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionPhase0EnlistmentAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionPhase0EnlistmentAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppitransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionPhase0Factory(pub ::windows::core::IUnknown);
impl ITransactionPhase0Factory {
    pub unsafe fn Create<'a, Param0: ::windows::core::IntoParam<'a, ITransactionPhase0NotifyAsync>>(&self, pphase0notify: Param0) -> ::windows::core::Result<ITransactionPhase0EnlistmentAsync> {
        let mut result__: <ITransactionPhase0EnlistmentAsync as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pphase0notify.into_param().abi(), &mut result__).from_abi::<ITransactionPhase0EnlistmentAsync>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITransactionPhase0Factory {
    type Vtable = ITransactionPhase0Factory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82dc88e0_a954_11d1_8f88_00600895e7d5);
}
impl ::core::convert::From<ITransactionPhase0Factory> for ::windows::core::IUnknown {
    fn from(value: ITransactionPhase0Factory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionPhase0Factory> for ::windows::core::IUnknown {
    fn from(value: &ITransactionPhase0Factory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionPhase0Factory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionPhase0Factory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionPhase0Factory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pphase0notify: ::windows::core::RawPtr, ppphase0enlistment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionPhase0NotifyAsync(pub ::windows::core::IUnknown);
impl ITransactionPhase0NotifyAsync {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Phase0Request<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fabortinghint: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fabortinghint.into_param().abi()).ok()
    }
    pub unsafe fn EnlistCompleted(&self, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(status)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITransactionPhase0NotifyAsync {
    type Vtable = ITransactionPhase0NotifyAsync_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef081809_0c76_11d2_87a6_00c04f990f34);
}
impl ::core::convert::From<ITransactionPhase0NotifyAsync> for ::windows::core::IUnknown {
    fn from(value: ITransactionPhase0NotifyAsync) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionPhase0NotifyAsync> for ::windows::core::IUnknown {
    fn from(value: &ITransactionPhase0NotifyAsync) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionPhase0NotifyAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionPhase0NotifyAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionPhase0NotifyAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fabortinghint: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionReceiver(pub ::windows::core::IUnknown);
impl ITransactionReceiver {
    pub unsafe fn UnmarshalPropagationToken(&self, cbtoken: u32, rgbtoken: *const u8) -> ::windows::core::Result<ITransaction> {
        let mut result__: <ITransaction as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbtoken), ::core::mem::transmute(rgbtoken), &mut result__).from_abi::<ITransaction>(result__)
    }
    pub unsafe fn GetReturnTokenSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn MarshalReturnToken(&self, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbreturntoken), ::core::mem::transmute(rgbreturntoken), ::core::mem::transmute(pcbused)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITransactionReceiver {
    type Vtable = ITransactionReceiver_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59313e03_b36c_11cf_a539_00aa006887c3);
}
impl ::core::convert::From<ITransactionReceiver> for ::windows::core::IUnknown {
    fn from(value: ITransactionReceiver) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionReceiver> for ::windows::core::IUnknown {
    fn from(value: &ITransactionReceiver) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionReceiver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionReceiver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionReceiver_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbtoken: u32, rgbtoken: *const u8, pptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbreturntoken: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionReceiverFactory(pub ::windows::core::IUnknown);
impl ITransactionReceiverFactory {
    pub unsafe fn Create(&self) -> ::windows::core::Result<ITransactionReceiver> {
        let mut result__: <ITransactionReceiver as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITransactionReceiver>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITransactionReceiverFactory {
    type Vtable = ITransactionReceiverFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59313e02_b36c_11cf_a539_00aa006887c3);
}
impl ::core::convert::From<ITransactionReceiverFactory> for ::windows::core::IUnknown {
    fn from(value: ITransactionReceiverFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionReceiverFactory> for ::windows::core::IUnknown {
    fn from(value: &ITransactionReceiverFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionReceiverFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionReceiverFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionReceiverFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppreceiver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionResource(pub ::windows::core::IUnknown);
impl ITransactionResource {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PrepareRequest<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, grfrm: u32, fwantmoniker: Param2, fsinglephase: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fretaining.into_param().abi(), ::core::mem::transmute(grfrm), fwantmoniker.into_param().abi(), fsinglephase.into_param().abi()).ok()
    }
    pub unsafe fn CommitRequest(&self, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfrm), ::core::mem::transmute(pnewuow)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AbortRequest<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, pnewuow: *const BOID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pboidreason), fretaining.into_param().abi(), ::core::mem::transmute(pnewuow)).ok()
    }
    pub unsafe fn TMDown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITransactionResource {
    type Vtable = ITransactionResource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee5ff7b3_4572_11d0_9452_00a0c905416e);
}
impl ::core::convert::From<ITransactionResource> for ::windows::core::IUnknown {
    fn from(value: ITransactionResource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionResource> for ::windows::core::IUnknown {
    fn from(value: &ITransactionResource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionResource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionResourceAsync(pub ::windows::core::IUnknown);
impl ITransactionResourceAsync {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PrepareRequest<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, grfrm: u32, fwantmoniker: Param2, fsinglephase: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fretaining.into_param().abi(), ::core::mem::transmute(grfrm), fwantmoniker.into_param().abi(), fsinglephase.into_param().abi()).ok()
    }
    pub unsafe fn CommitRequest(&self, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfrm), ::core::mem::transmute(pnewuow)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AbortRequest<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, pnewuow: *const BOID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pboidreason), fretaining.into_param().abi(), ::core::mem::transmute(pnewuow)).ok()
    }
    pub unsafe fn TMDown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITransactionResourceAsync {
    type Vtable = ITransactionResourceAsync_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69e971f0_23ce_11cf_ad60_00aa00a74ccd);
}
impl ::core::convert::From<ITransactionResourceAsync> for ::windows::core::IUnknown {
    fn from(value: ITransactionResourceAsync) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionResourceAsync> for ::windows::core::IUnknown {
    fn from(value: &ITransactionResourceAsync) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionResourceAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionResourceAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionResourceAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, grfrm: u32, pnewuow: *const BOID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionTransmitter(pub ::windows::core::IUnknown);
impl ITransactionTransmitter {
    pub unsafe fn Set<'a, Param0: ::windows::core::IntoParam<'a, ITransaction>>(&self, ptransaction: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ptransaction.into_param().abi()).ok()
    }
    pub unsafe fn GetPropagationTokenSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn MarshalPropagationToken(&self, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbtoken), ::core::mem::transmute(rgbtoken), ::core::mem::transmute(pcbused)).ok()
    }
    pub unsafe fn UnmarshalReturnToken(&self, cbreturntoken: u32, rgbreturntoken: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbreturntoken), ::core::mem::transmute(rgbreturntoken)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITransactionTransmitter {
    type Vtable = ITransactionTransmitter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59313e01_b36c_11cf_a539_00aa006887c3);
}
impl ::core::convert::From<ITransactionTransmitter> for ::windows::core::IUnknown {
    fn from(value: ITransactionTransmitter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionTransmitter> for ::windows::core::IUnknown {
    fn from(value: &ITransactionTransmitter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionTransmitter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionTransmitter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionTransmitter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptransaction: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbtoken: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbreturntoken: u32, rgbreturntoken: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionTransmitterFactory(pub ::windows::core::IUnknown);
impl ITransactionTransmitterFactory {
    pub unsafe fn Create(&self) -> ::windows::core::Result<ITransactionTransmitter> {
        let mut result__: <ITransactionTransmitter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITransactionTransmitter>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITransactionTransmitterFactory {
    type Vtable = ITransactionTransmitterFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59313e00_b36c_11cf_a539_00aa006887c3);
}
impl ::core::convert::From<ITransactionTransmitterFactory> for ::windows::core::IUnknown {
    fn from(value: ITransactionTransmitterFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionTransmitterFactory> for ::windows::core::IUnknown {
    fn from(value: &ITransactionTransmitterFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionTransmitterFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionTransmitterFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionTransmitterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pptransmitter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionVoterBallotAsync2(pub ::windows::core::IUnknown);
impl ITransactionVoterBallotAsync2 {
    pub unsafe fn VoteRequestDone(&self, hr: ::windows::core::HRESULT, pboidreason: *const BOID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr), ::core::mem::transmute(pboidreason)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITransactionVoterBallotAsync2 {
    type Vtable = ITransactionVoterBallotAsync2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5433376c_414d_11d3_b206_00c04fc2f3ef);
}
impl ::core::convert::From<ITransactionVoterBallotAsync2> for ::windows::core::IUnknown {
    fn from(value: ITransactionVoterBallotAsync2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionVoterBallotAsync2> for ::windows::core::IUnknown {
    fn from(value: &ITransactionVoterBallotAsync2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionVoterBallotAsync2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionVoterBallotAsync2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionVoterBallotAsync2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hr: ::windows::core::HRESULT, pboidreason: *const BOID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionVoterFactory2(pub ::windows::core::IUnknown);
impl ITransactionVoterFactory2 {
    pub unsafe fn Create<'a, Param0: ::windows::core::IntoParam<'a, ITransaction>, Param1: ::windows::core::IntoParam<'a, ITransactionVoterNotifyAsync2>>(&self, ptransaction: Param0, pvoternotify: Param1) -> ::windows::core::Result<ITransactionVoterBallotAsync2> {
        let mut result__: <ITransactionVoterBallotAsync2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ptransaction.into_param().abi(), pvoternotify.into_param().abi(), &mut result__).from_abi::<ITransactionVoterBallotAsync2>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITransactionVoterFactory2 {
    type Vtable = ITransactionVoterFactory2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5433376a_414d_11d3_b206_00c04fc2f3ef);
}
impl ::core::convert::From<ITransactionVoterFactory2> for ::windows::core::IUnknown {
    fn from(value: ITransactionVoterFactory2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionVoterFactory2> for ::windows::core::IUnknown {
    fn from(value: &ITransactionVoterFactory2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionVoterFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionVoterFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionVoterFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptransaction: ::windows::core::RawPtr, pvoternotify: ::windows::core::RawPtr, ppvoterballot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransactionVoterNotifyAsync2(pub ::windows::core::IUnknown);
impl ITransactionVoterNotifyAsync2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Committed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fretaining.into_param().abi(), ::core::mem::transmute(pnewuow), ::core::mem::transmute(hr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Aborted<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pboidreason), fretaining.into_param().abi(), ::core::mem::transmute(pnewuow), ::core::mem::transmute(hr)).ok()
    }
    pub unsafe fn HeuristicDecision(&self, dwdecision: u32, pboidreason: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdecision), ::core::mem::transmute(pboidreason), ::core::mem::transmute(hr)).ok()
    }
    pub unsafe fn Indoubt(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn VoteRequest(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITransactionVoterNotifyAsync2 {
    type Vtable = ITransactionVoterNotifyAsync2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5433376b_414d_11d3_b206_00c04fc2f3ef);
}
impl ::core::convert::From<ITransactionVoterNotifyAsync2> for ::windows::core::IUnknown {
    fn from(value: ITransactionVoterNotifyAsync2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransactionVoterNotifyAsync2> for ::windows::core::IUnknown {
    fn from(value: &ITransactionVoterNotifyAsync2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransactionVoterNotifyAsync2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransactionVoterNotifyAsync2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITransactionVoterNotifyAsync2> for ITransactionOutcomeEvents {
    fn from(value: ITransactionVoterNotifyAsync2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionVoterNotifyAsync2> for ITransactionOutcomeEvents {
    fn from(value: &ITransactionVoterNotifyAsync2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITransactionOutcomeEvents> for ITransactionVoterNotifyAsync2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITransactionOutcomeEvents> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITransactionOutcomeEvents> for &ITransactionVoterNotifyAsync2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITransactionOutcomeEvents> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionVoterNotifyAsync2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwdecision: u32, pboidreason: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXAConfig(pub ::windows::core::IUnknown);
impl IXAConfig {
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, clsidhelperdll: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), clsidhelperdll.into_param().abi()).ok()
    }
    pub unsafe fn Terminate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IXAConfig {
    type Vtable = IXAConfig_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8a6e3a1_9a8c_11cf_a308_00a0c905416e);
}
impl ::core::convert::From<IXAConfig> for ::windows::core::IUnknown {
    fn from(value: IXAConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXAConfig> for ::windows::core::IUnknown {
    fn from(value: &IXAConfig) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXAConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXAConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clsidhelperdll: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXAObtainRMInfo(pub ::windows::core::IUnknown);
impl IXAObtainRMInfo {
    pub unsafe fn ObtainRMInfo<'a, Param0: ::windows::core::IntoParam<'a, IRMHelper>>(&self, pirmhelper: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pirmhelper.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IXAObtainRMInfo {
    type Vtable = IXAObtainRMInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe793f6d2_f53d_11cf_a60d_00a0c905416e);
}
impl ::core::convert::From<IXAObtainRMInfo> for ::windows::core::IUnknown {
    fn from(value: IXAObtainRMInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXAObtainRMInfo> for ::windows::core::IUnknown {
    fn from(value: &IXAObtainRMInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXAObtainRMInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXAObtainRMInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAObtainRMInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pirmhelper: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXATransLookup(pub ::windows::core::IUnknown);
impl IXATransLookup {
    pub unsafe fn Lookup(&self) -> ::windows::core::Result<ITransaction> {
        let mut result__: <ITransaction as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITransaction>(result__)
    }
}
unsafe impl ::windows::core::Interface for IXATransLookup {
    type Vtable = IXATransLookup_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3b1f131_eeda_11ce_aed4_00aa0051e2c4);
}
impl ::core::convert::From<IXATransLookup> for ::windows::core::IUnknown {
    fn from(value: IXATransLookup) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXATransLookup> for ::windows::core::IUnknown {
    fn from(value: &IXATransLookup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXATransLookup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXATransLookup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXATransLookup_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXATransLookup2(pub ::windows::core::IUnknown);
impl IXATransLookup2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Lookup(&self, pxid: *const xid_t) -> ::windows::core::Result<ITransaction> {
        let mut result__: <ITransaction as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pxid), &mut result__).from_abi::<ITransaction>(result__)
    }
}
unsafe impl ::windows::core::Interface for IXATransLookup2 {
    type Vtable = IXATransLookup2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf193c85_0d1a_4290_b88f_d2cb8873d1e7);
}
impl ::core::convert::From<IXATransLookup2> for ::windows::core::IUnknown {
    fn from(value: IXATransLookup2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXATransLookup2> for ::windows::core::IUnknown {
    fn from(value: &IXATransLookup2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXATransLookup2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXATransLookup2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXATransLookup2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pxid: *const xid_t, pptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub const MAXBQUALSIZE: u32 = 64u32;
pub const MAXGTRIDSIZE: u32 = 64u32;
pub const MAXINFOSIZE: u32 = 256u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct OLE_TM_CONFIG_PARAMS_V1 {
    pub dwVersion: u32,
    pub dwcConcurrencyHint: u32,
}
impl OLE_TM_CONFIG_PARAMS_V1 {}
impl ::core::default::Default for OLE_TM_CONFIG_PARAMS_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for OLE_TM_CONFIG_PARAMS_V1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("OLE_TM_CONFIG_PARAMS_V1").field("dwVersion", &self.dwVersion).field("dwcConcurrencyHint", &self.dwcConcurrencyHint).finish()
    }
}
impl ::core::cmp::PartialEq for OLE_TM_CONFIG_PARAMS_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwcConcurrencyHint == other.dwcConcurrencyHint
    }
}
impl ::core::cmp::Eq for OLE_TM_CONFIG_PARAMS_V1 {}
unsafe impl ::windows::core::Abi for OLE_TM_CONFIG_PARAMS_V1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct OLE_TM_CONFIG_PARAMS_V2 {
    pub dwVersion: u32,
    pub dwcConcurrencyHint: u32,
    pub applicationType: APPLICATIONTYPE,
    pub clusterResourceId: ::windows::core::GUID,
}
impl OLE_TM_CONFIG_PARAMS_V2 {}
impl ::core::default::Default for OLE_TM_CONFIG_PARAMS_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for OLE_TM_CONFIG_PARAMS_V2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("OLE_TM_CONFIG_PARAMS_V2").field("dwVersion", &self.dwVersion).field("dwcConcurrencyHint", &self.dwcConcurrencyHint).field("applicationType", &self.applicationType).field("clusterResourceId", &self.clusterResourceId).finish()
    }
}
impl ::core::cmp::PartialEq for OLE_TM_CONFIG_PARAMS_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwcConcurrencyHint == other.dwcConcurrencyHint && self.applicationType == other.applicationType && self.clusterResourceId == other.clusterResourceId
    }
}
impl ::core::cmp::Eq for OLE_TM_CONFIG_PARAMS_V2 {}
unsafe impl ::windows::core::Abi for OLE_TM_CONFIG_PARAMS_V2 {
    type Abi = Self;
}
pub const OLE_TM_CONFIG_VERSION_1: u32 = 1u32;
pub const OLE_TM_CONFIG_VERSION_2: u32 = 2u32;
pub const OLE_TM_FLAG_INTERNAL_TO_TM: u32 = 1073741824u32;
pub const OLE_TM_FLAG_NOAGILERECOVERY: u32 = 2u32;
pub const OLE_TM_FLAG_NODEMANDSTART: u32 = 1u32;
pub const OLE_TM_FLAG_NONE: u32 = 0u32;
pub const OLE_TM_FLAG_QUERY_SERVICE_LOCKSTATUS: u32 = 2147483648u32;
pub const RMNAMESZ: u32 = 32u32;
pub const TMASYNC: i32 = -2147483648i32;
pub const TMENDRSCAN: i32 = 8388608i32;
pub const TMER_INVAL: i32 = -2i32;
pub const TMER_PROTO: i32 = -3i32;
pub const TMER_TMERR: i32 = -1i32;
pub const TMFAIL: i32 = 536870912i32;
pub const TMJOIN: i32 = 2097152i32;
pub const TMMIGRATE: i32 = 1048576i32;
pub const TMMULTIPLE: i32 = 4194304i32;
pub const TMNOFLAGS: i32 = 0i32;
pub const TMNOMIGRATE: i32 = 2i32;
pub const TMNOWAIT: i32 = 268435456i32;
pub const TMONEPHASE: i32 = 1073741824i32;
pub const TMREGISTER: i32 = 1i32;
pub const TMRESUME: i32 = 134217728i32;
pub const TMSTARTRSCAN: i32 = 16777216i32;
pub const TMSUCCESS: i32 = 67108864i32;
pub const TMSUSPEND: i32 = 33554432i32;
pub const TMUSEASYNC: i32 = 4i32;
pub const TM_JOIN: u32 = 2u32;
pub const TM_OK: u32 = 0u32;
pub const TM_RESUME: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TX_MISC_CONSTANTS(pub i32);
pub const MAX_TRAN_DESC: TX_MISC_CONSTANTS = TX_MISC_CONSTANTS(40i32);
impl ::core::convert::From<i32> for TX_MISC_CONSTANTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TX_MISC_CONSTANTS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XACTCONST(pub i32);
pub const XACTCONST_TIMEOUTINFINITE: XACTCONST = XACTCONST(0i32);
impl ::core::convert::From<i32> for XACTCONST {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XACTCONST {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XACTHEURISTIC(pub i32);
pub const XACTHEURISTIC_ABORT: XACTHEURISTIC = XACTHEURISTIC(1i32);
pub const XACTHEURISTIC_COMMIT: XACTHEURISTIC = XACTHEURISTIC(2i32);
pub const XACTHEURISTIC_DAMAGE: XACTHEURISTIC = XACTHEURISTIC(3i32);
pub const XACTHEURISTIC_DANGER: XACTHEURISTIC = XACTHEURISTIC(4i32);
impl ::core::convert::From<i32> for XACTHEURISTIC {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XACTHEURISTIC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct XACTOPT {
    pub ulTimeout: u32,
    pub szDescription: [u8; 40],
}
impl XACTOPT {}
impl ::core::default::Default for XACTOPT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XACTOPT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XACTOPT").field("ulTimeout", &self.ulTimeout).field("szDescription", &self.szDescription).finish()
    }
}
impl ::core::cmp::PartialEq for XACTOPT {
    fn eq(&self, other: &Self) -> bool {
        self.ulTimeout == other.ulTimeout && self.szDescription == other.szDescription
    }
}
impl ::core::cmp::Eq for XACTOPT {}
unsafe impl ::windows::core::Abi for XACTOPT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XACTRM(pub i32);
pub const XACTRM_OPTIMISTICLASTWINS: XACTRM = XACTRM(1i32);
pub const XACTRM_NOREADONLYPREPARES: XACTRM = XACTRM(2i32);
impl ::core::convert::From<i32> for XACTRM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XACTRM {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XACTSTAT(pub i32);
pub const XACTSTAT_NONE: XACTSTAT = XACTSTAT(0i32);
pub const XACTSTAT_OPENNORMAL: XACTSTAT = XACTSTAT(1i32);
pub const XACTSTAT_OPENREFUSED: XACTSTAT = XACTSTAT(2i32);
pub const XACTSTAT_PREPARING: XACTSTAT = XACTSTAT(4i32);
pub const XACTSTAT_PREPARED: XACTSTAT = XACTSTAT(8i32);
pub const XACTSTAT_PREPARERETAINING: XACTSTAT = XACTSTAT(16i32);
pub const XACTSTAT_PREPARERETAINED: XACTSTAT = XACTSTAT(32i32);
pub const XACTSTAT_COMMITTING: XACTSTAT = XACTSTAT(64i32);
pub const XACTSTAT_COMMITRETAINING: XACTSTAT = XACTSTAT(128i32);
pub const XACTSTAT_ABORTING: XACTSTAT = XACTSTAT(256i32);
pub const XACTSTAT_ABORTED: XACTSTAT = XACTSTAT(512i32);
pub const XACTSTAT_COMMITTED: XACTSTAT = XACTSTAT(1024i32);
pub const XACTSTAT_HEURISTIC_ABORT: XACTSTAT = XACTSTAT(2048i32);
pub const XACTSTAT_HEURISTIC_COMMIT: XACTSTAT = XACTSTAT(4096i32);
pub const XACTSTAT_HEURISTIC_DAMAGE: XACTSTAT = XACTSTAT(8192i32);
pub const XACTSTAT_HEURISTIC_DANGER: XACTSTAT = XACTSTAT(16384i32);
pub const XACTSTAT_FORCED_ABORT: XACTSTAT = XACTSTAT(32768i32);
pub const XACTSTAT_FORCED_COMMIT: XACTSTAT = XACTSTAT(65536i32);
pub const XACTSTAT_INDOUBT: XACTSTAT = XACTSTAT(131072i32);
pub const XACTSTAT_CLOSED: XACTSTAT = XACTSTAT(262144i32);
pub const XACTSTAT_OPEN: XACTSTAT = XACTSTAT(3i32);
pub const XACTSTAT_NOTPREPARED: XACTSTAT = XACTSTAT(524227i32);
pub const XACTSTAT_ALL: XACTSTAT = XACTSTAT(524287i32);
impl ::core::convert::From<i32> for XACTSTAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XACTSTAT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
impl XACTSTATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for XACTSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for XACTSTATS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XACTSTATS")
            .field("cOpen", &self.cOpen)
            .field("cCommitting", &self.cCommitting)
            .field("cCommitted", &self.cCommitted)
            .field("cAborting", &self.cAborting)
            .field("cAborted", &self.cAborted)
            .field("cInDoubt", &self.cInDoubt)
            .field("cHeuristicDecision", &self.cHeuristicDecision)
            .field("timeTransactionsUp", &self.timeTransactionsUp)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for XACTSTATS {
    fn eq(&self, other: &Self) -> bool {
        self.cOpen == other.cOpen && self.cCommitting == other.cCommitting && self.cCommitted == other.cCommitted && self.cAborting == other.cAborting && self.cAborted == other.cAborted && self.cInDoubt == other.cInDoubt && self.cHeuristicDecision == other.cHeuristicDecision && self.timeTransactionsUp == other.timeTransactionsUp
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for XACTSTATS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for XACTSTATS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XACTTC(pub i32);
pub const XACTTC_NONE: XACTTC = XACTTC(0i32);
pub const XACTTC_SYNC_PHASEONE: XACTTC = XACTTC(1i32);
pub const XACTTC_SYNC_PHASETWO: XACTTC = XACTTC(2i32);
pub const XACTTC_SYNC: XACTTC = XACTTC(2i32);
pub const XACTTC_ASYNC_PHASEONE: XACTTC = XACTTC(4i32);
pub const XACTTC_ASYNC: XACTTC = XACTTC(4i32);
impl ::core::convert::From<i32> for XACTTC {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XACTTC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct XACTTRANSINFO {
    pub uow: BOID,
    pub isoLevel: i32,
    pub isoFlags: u32,
    pub grfTCSupported: u32,
    pub grfRMSupported: u32,
    pub grfTCSupportedRetaining: u32,
    pub grfRMSupportedRetaining: u32,
}
impl XACTTRANSINFO {}
impl ::core::default::Default for XACTTRANSINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XACTTRANSINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XACTTRANSINFO")
            .field("uow", &self.uow)
            .field("isoLevel", &self.isoLevel)
            .field("isoFlags", &self.isoFlags)
            .field("grfTCSupported", &self.grfTCSupported)
            .field("grfRMSupported", &self.grfRMSupported)
            .field("grfTCSupportedRetaining", &self.grfTCSupportedRetaining)
            .field("grfRMSupportedRetaining", &self.grfRMSupportedRetaining)
            .finish()
    }
}
impl ::core::cmp::PartialEq for XACTTRANSINFO {
    fn eq(&self, other: &Self) -> bool {
        self.uow == other.uow && self.isoLevel == other.isoLevel && self.isoFlags == other.isoFlags && self.grfTCSupported == other.grfTCSupported && self.grfRMSupported == other.grfRMSupported && self.grfTCSupportedRetaining == other.grfTCSupportedRetaining && self.grfRMSupportedRetaining == other.grfRMSupportedRetaining
    }
}
impl ::core::cmp::Eq for XACTTRANSINFO {}
unsafe impl ::windows::core::Abi for XACTTRANSINFO {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XACT_DTC_CONSTANTS(pub i32);
pub const XACT_E_CONNECTION_REQUEST_DENIED: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147168000i32);
pub const XACT_E_TOOMANY_ENLISTMENTS: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167999i32);
pub const XACT_E_DUPLICATE_GUID: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167998i32);
pub const XACT_E_NOTSINGLEPHASE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167997i32);
pub const XACT_E_RECOVERYALREADYDONE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167996i32);
pub const XACT_E_PROTOCOL: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167995i32);
pub const XACT_E_RM_FAILURE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167994i32);
pub const XACT_E_RECOVERY_FAILED: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167993i32);
pub const XACT_E_LU_NOT_FOUND: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167992i32);
pub const XACT_E_DUPLICATE_LU: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167991i32);
pub const XACT_E_LU_NOT_CONNECTED: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167990i32);
pub const XACT_E_DUPLICATE_TRANSID: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167989i32);
pub const XACT_E_LU_BUSY: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167988i32);
pub const XACT_E_LU_NO_RECOVERY_PROCESS: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167987i32);
pub const XACT_E_LU_DOWN: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167986i32);
pub const XACT_E_LU_RECOVERING: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167985i32);
pub const XACT_E_LU_RECOVERY_MISMATCH: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167984i32);
pub const XACT_E_RM_UNAVAILABLE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167983i32);
pub const XACT_E_LRMRECOVERYALREADYDONE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167982i32);
pub const XACT_E_NOLASTRESOURCEINTERFACE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167981i32);
pub const XACT_S_NONOTIFY: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(315648i32);
pub const XACT_OK_NONOTIFY: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(315649i32);
pub const dwUSER_MS_SQLSERVER: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(65535i32);
impl ::core::convert::From<i32> for XACT_DTC_CONSTANTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XACT_DTC_CONSTANTS {
    type Abi = Self;
}
pub const XAER_ASYNC: i32 = -2i32;
pub const XAER_DUPID: i32 = -8i32;
pub const XAER_INVAL: i32 = -5i32;
pub const XAER_NOTA: i32 = -4i32;
pub const XAER_OUTSIDE: i32 = -9i32;
pub const XAER_PROTO: i32 = -6i32;
pub const XAER_RMERR: i32 = -3i32;
pub const XAER_RMFAIL: i32 = -7i32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_CLOSE_EPT = unsafe extern "system" fn(param0: super::super::Foundation::PSTR, param1: i32, param2: i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_COMMIT_EPT = unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32;
pub type XA_COMPLETE_EPT = unsafe extern "system" fn(param0: *mut i32, param1: *mut i32, param2: i32, param3: i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_END_EPT = unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32;
pub const XA_FMTID_DTC: u32 = 4478019u32;
pub const XA_FMTID_DTC_VER1: u32 = 21255235u32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_FORGET_EPT = unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32;
pub const XA_HEURCOM: u32 = 7u32;
pub const XA_HEURHAZ: u32 = 8u32;
pub const XA_HEURMIX: u32 = 5u32;
pub const XA_HEURRB: u32 = 6u32;
pub const XA_NOMIGRATE: u32 = 9u32;
pub const XA_OK: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_OPEN_EPT = unsafe extern "system" fn(param0: super::super::Foundation::PSTR, param1: i32, param2: i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_PREPARE_EPT = unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32;
pub const XA_RBBASE: u32 = 100u32;
pub const XA_RBCOMMFAIL: u32 = 101u32;
pub const XA_RBDEADLOCK: u32 = 102u32;
pub const XA_RBEND: u32 = 107u32;
pub const XA_RBINTEGRITY: u32 = 103u32;
pub const XA_RBOTHER: u32 = 104u32;
pub const XA_RBPROTO: u32 = 105u32;
pub const XA_RBROLLBACK: u32 = 100u32;
pub const XA_RBTIMEOUT: u32 = 106u32;
pub const XA_RBTRANSIENT: u32 = 107u32;
pub const XA_RDONLY: u32 = 3u32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_RECOVER_EPT = unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32, param3: i32) -> i32;
pub const XA_RETRY: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_ROLLBACK_EPT = unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_START_EPT = unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32;
pub const XA_SWITCH_F_DTC: u32 = 1u32;
pub const XIDDATASIZE: u32 = 128u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DtcLu_CompareState(pub i32);
pub const DTCLUCOMPARESTATE_COMMITTED: _DtcLu_CompareState = _DtcLu_CompareState(1i32);
pub const DTCLUCOMPARESTATE_HEURISTICCOMMITTED: _DtcLu_CompareState = _DtcLu_CompareState(2i32);
pub const DTCLUCOMPARESTATE_HEURISTICMIXED: _DtcLu_CompareState = _DtcLu_CompareState(3i32);
pub const DTCLUCOMPARESTATE_HEURISTICRESET: _DtcLu_CompareState = _DtcLu_CompareState(4i32);
pub const DTCLUCOMPARESTATE_INDOUBT: _DtcLu_CompareState = _DtcLu_CompareState(5i32);
pub const DTCLUCOMPARESTATE_RESET: _DtcLu_CompareState = _DtcLu_CompareState(6i32);
impl ::core::convert::From<i32> for _DtcLu_CompareState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for _DtcLu_CompareState {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DtcLu_CompareStates_Confirmation(pub i32);
pub const DTCLUCOMPARESTATESCONFIRMATION_CONFIRM: _DtcLu_CompareStates_Confirmation = _DtcLu_CompareStates_Confirmation(1i32);
pub const DTCLUCOMPARESTATESCONFIRMATION_PROTOCOL: _DtcLu_CompareStates_Confirmation = _DtcLu_CompareStates_Confirmation(2i32);
impl ::core::convert::From<i32> for _DtcLu_CompareStates_Confirmation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for _DtcLu_CompareStates_Confirmation {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DtcLu_CompareStates_Error(pub i32);
pub const DTCLUCOMPARESTATESERROR_PROTOCOL: _DtcLu_CompareStates_Error = _DtcLu_CompareStates_Error(1i32);
impl ::core::convert::From<i32> for _DtcLu_CompareStates_Error {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for _DtcLu_CompareStates_Error {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DtcLu_CompareStates_Response(pub i32);
pub const DTCLUCOMPARESTATESRESPONSE_OK: _DtcLu_CompareStates_Response = _DtcLu_CompareStates_Response(1i32);
pub const DTCLUCOMPARESTATESRESPONSE_PROTOCOL: _DtcLu_CompareStates_Response = _DtcLu_CompareStates_Response(2i32);
impl ::core::convert::From<i32> for _DtcLu_CompareStates_Response {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for _DtcLu_CompareStates_Response {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DtcLu_LocalRecovery_Work(pub i32);
pub const DTCINITIATEDRECOVERYWORK_CHECKLUSTATUS: _DtcLu_LocalRecovery_Work = _DtcLu_LocalRecovery_Work(1i32);
pub const DTCINITIATEDRECOVERYWORK_TRANS: _DtcLu_LocalRecovery_Work = _DtcLu_LocalRecovery_Work(2i32);
pub const DTCINITIATEDRECOVERYWORK_TMDOWN: _DtcLu_LocalRecovery_Work = _DtcLu_LocalRecovery_Work(3i32);
impl ::core::convert::From<i32> for _DtcLu_LocalRecovery_Work {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for _DtcLu_LocalRecovery_Work {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DtcLu_Xln(pub i32);
pub const DTCLUXLN_COLD: _DtcLu_Xln = _DtcLu_Xln(1i32);
pub const DTCLUXLN_WARM: _DtcLu_Xln = _DtcLu_Xln(2i32);
impl ::core::convert::From<i32> for _DtcLu_Xln {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for _DtcLu_Xln {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DtcLu_Xln_Confirmation(pub i32);
pub const DTCLUXLNCONFIRMATION_CONFIRM: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(1i32);
pub const DTCLUXLNCONFIRMATION_LOGNAMEMISMATCH: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(2i32);
pub const DTCLUXLNCONFIRMATION_COLDWARMMISMATCH: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(3i32);
pub const DTCLUXLNCONFIRMATION_OBSOLETE: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(4i32);
impl ::core::convert::From<i32> for _DtcLu_Xln_Confirmation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for _DtcLu_Xln_Confirmation {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DtcLu_Xln_Error(pub i32);
pub const DTCLUXLNERROR_PROTOCOL: _DtcLu_Xln_Error = _DtcLu_Xln_Error(1i32);
pub const DTCLUXLNERROR_LOGNAMEMISMATCH: _DtcLu_Xln_Error = _DtcLu_Xln_Error(2i32);
pub const DTCLUXLNERROR_COLDWARMMISMATCH: _DtcLu_Xln_Error = _DtcLu_Xln_Error(3i32);
impl ::core::convert::From<i32> for _DtcLu_Xln_Error {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for _DtcLu_Xln_Error {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DtcLu_Xln_Response(pub i32);
pub const DTCLUXLNRESPONSE_OK_SENDOURXLNBACK: _DtcLu_Xln_Response = _DtcLu_Xln_Response(1i32);
pub const DTCLUXLNRESPONSE_OK_SENDCONFIRMATION: _DtcLu_Xln_Response = _DtcLu_Xln_Response(2i32);
pub const DTCLUXLNRESPONSE_LOGNAMEMISMATCH: _DtcLu_Xln_Response = _DtcLu_Xln_Response(3i32);
pub const DTCLUXLNRESPONSE_COLDWARMMISMATCH: _DtcLu_Xln_Response = _DtcLu_Xln_Response(4i32);
impl ::core::convert::From<i32> for _DtcLu_Xln_Response {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for _DtcLu_Xln_Response {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct _ProxyConfigParams {
    pub wcThreadsMax: u16,
}
impl _ProxyConfigParams {}
impl ::core::default::Default for _ProxyConfigParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for _ProxyConfigParams {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_ProxyConfigParams").field("wcThreadsMax", &self.wcThreadsMax).finish()
    }
}
impl ::core::cmp::PartialEq for _ProxyConfigParams {
    fn eq(&self, other: &Self) -> bool {
        self.wcThreadsMax == other.wcThreadsMax
    }
}
impl ::core::cmp::Eq for _ProxyConfigParams {}
unsafe impl ::windows::core::Abi for _ProxyConfigParams {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
impl xa_switch_t {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for xa_switch_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for xa_switch_t {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("xa_switch_t")
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
impl ::core::cmp::PartialEq for xa_switch_t {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.flags == other.flags
            && self.version == other.version
            && self.xa_open_entry == other.xa_open_entry
            && self.xa_close_entry == other.xa_close_entry
            && self.xa_start_entry == other.xa_start_entry
            && self.xa_end_entry == other.xa_end_entry
            && self.xa_rollback_entry == other.xa_rollback_entry
            && self.xa_prepare_entry == other.xa_prepare_entry
            && self.xa_commit_entry == other.xa_commit_entry
            && self.xa_recover_entry == other.xa_recover_entry
            && self.xa_forget_entry == other.xa_forget_entry
            && self.xa_complete_entry == other.xa_complete_entry
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for xa_switch_t {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for xa_switch_t {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct xid_t {
    pub formatID: i32,
    pub gtrid_length: i32,
    pub bqual_length: i32,
    pub data: [super::super::Foundation::CHAR; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl xid_t {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for xid_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for xid_t {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("xid_t").field("formatID", &self.formatID).field("gtrid_length", &self.gtrid_length).field("bqual_length", &self.bqual_length).field("data", &self.data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for xid_t {
    fn eq(&self, other: &Self) -> bool {
        self.formatID == other.formatID && self.gtrid_length == other.gtrid_length && self.bqual_length == other.bqual_length && self.data == other.data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for xid_t {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for xid_t {
    type Abi = Self;
}
