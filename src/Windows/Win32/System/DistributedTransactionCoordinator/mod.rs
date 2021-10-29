#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPLICATIONTYPE(pub i32);
pub const LOCAL_APPLICATIONTYPE: APPLICATIONTYPE = APPLICATIONTYPE(0i32);
pub const CLUSTERRESOURCE_APPLICATIONTYPE: APPLICATIONTYPE = APPLICATIONTYPE(1i32);
impl ::std::convert::From<i32> for APPLICATIONTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APPLICATIONTYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AUTHENTICATION_LEVEL(pub i32);
pub const NO_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(0i32);
pub const INCOMING_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(1i32);
pub const MUTUAL_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(2i32);
impl ::std::convert::From<i32> for AUTHENTICATION_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AUTHENTICATION_LEVEL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct BOID {
    pub rgb: [u8; 16],
}
impl BOID {}
impl ::std::default::Default for BOID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BOID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BOID").field("rgb", &self.rgb).finish()
    }
}
impl ::std::cmp::PartialEq for BOID {
    fn eq(&self, other: &Self) -> bool {
        self.rgb == other.rgb
    }
}
impl ::std::cmp::Eq for BOID {}
unsafe impl ::windows::runtime::Abi for BOID {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CLSID_MSDtcTransaction: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(972609387, 2344, 4561, [151, 223, 0, 192, 79, 185, 97, 138]);
pub const CLSID_MSDtcTransactionManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1528343393, 2333, 4561, [151, 223, 0, 192, 79, 185, 97, 138]);
pub const DTCINSTALL_E_CLIENT_ALREADY_INSTALLED: i32 = 384i32;
pub const DTCINSTALL_E_SERVER_ALREADY_INSTALLED: i32 = 385i32;
#[cfg(feature = "Win32_Foundation")]
pub type DTC_GET_TRANSACTION_MANAGER = unsafe extern "system" fn(pszhost: super::super::Foundation::PSTR, psztmname: super::super::Foundation::PSTR, rid: *const ::windows::runtime::GUID, dwreserved1: u32, wcbreserved2: u16, pvreserved2: *mut ::std::ffi::c_void, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type DTC_GET_TRANSACTION_MANAGER_EX_A = unsafe extern "system" fn(i_pszhost: super::super::Foundation::PSTR, i_psztmname: super::super::Foundation::PSTR, i_riid: *const ::windows::runtime::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::std::ffi::c_void, o_ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type DTC_GET_TRANSACTION_MANAGER_EX_W = unsafe extern "system" fn(i_pwszhost: super::super::Foundation::PWSTR, i_pwsztmname: super::super::Foundation::PWSTR, i_riid: *const ::windows::runtime::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::std::ffi::c_void, o_ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
pub type DTC_INSTALL_CLIENT = unsafe extern "system" fn(i_pszremotetmhostname: *mut i8, i_dwprotocol: u32, i_dwoverwrite: u32) -> ::windows::runtime::HRESULT;
pub const DTC_INSTALL_OVERWRITE_CLIENT: u32 = 1u32;
pub const DTC_INSTALL_OVERWRITE_SERVER: u32 = 2u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DTC_STATUS_ {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DTC_STATUS_ {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DtcGetTransactionManager<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(i_pszhost: Param0, i_psztmname: Param1, i_riid: *const ::windows::runtime::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::std::ffi::c_void, o_ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DtcGetTransactionManager(i_pszhost: super::super::Foundation::PSTR, i_psztmname: super::super::Foundation::PSTR, i_riid: *const ::windows::runtime::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::std::ffi::c_void, o_ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DtcGetTransactionManager(i_pszhost.into_param().abi(), i_psztmname.into_param().abi(), ::std::mem::transmute(i_riid), ::std::mem::transmute(i_dwreserved1), ::std::mem::transmute(i_wcbreserved2), ::std::mem::transmute(i_pvreserved2), ::std::mem::transmute(o_ppvobject)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DtcGetTransactionManagerC<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(i_pszhost: Param0, i_psztmname: Param1, i_riid: *const ::windows::runtime::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::std::ffi::c_void, o_ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DtcGetTransactionManagerC(i_pszhost: super::super::Foundation::PSTR, i_psztmname: super::super::Foundation::PSTR, i_riid: *const ::windows::runtime::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::std::ffi::c_void, o_ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DtcGetTransactionManagerC(i_pszhost.into_param().abi(), i_psztmname.into_param().abi(), ::std::mem::transmute(i_riid), ::std::mem::transmute(i_dwreserved1), ::std::mem::transmute(i_wcbreserved2), ::std::mem::transmute(i_pvreserved2), ::std::mem::transmute(o_ppvobject)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DtcGetTransactionManagerExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(i_pszhost: Param0, i_psztmname: Param1, i_riid: *const ::windows::runtime::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::std::ffi::c_void, o_ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DtcGetTransactionManagerExA(i_pszhost: super::super::Foundation::PSTR, i_psztmname: super::super::Foundation::PSTR, i_riid: *const ::windows::runtime::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::std::ffi::c_void, o_ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DtcGetTransactionManagerExA(i_pszhost.into_param().abi(), i_psztmname.into_param().abi(), ::std::mem::transmute(i_riid), ::std::mem::transmute(i_grfoptions), ::std::mem::transmute(i_pvconfigparams), ::std::mem::transmute(o_ppvobject)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DtcGetTransactionManagerExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(i_pwszhost: Param0, i_pwsztmname: Param1, i_riid: *const ::windows::runtime::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::std::ffi::c_void, o_ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DtcGetTransactionManagerExW(i_pwszhost: super::super::Foundation::PWSTR, i_pwsztmname: super::super::Foundation::PWSTR, i_riid: *const ::windows::runtime::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::std::ffi::c_void, o_ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DtcGetTransactionManagerExW(i_pwszhost.into_param().abi(), i_pwsztmname.into_param().abi(), ::std::mem::transmute(i_riid), ::std::mem::transmute(i_grfoptions), ::std::mem::transmute(i_pvconfigparams), ::std::mem::transmute(o_ppvobject)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcLuConfigure(::windows::runtime::IUnknown);
impl IDtcLuConfigure {
    pub unsafe fn Add(&self, puclupair: *const u8, cblupair: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(puclupair), ::std::mem::transmute(cblupair)).ok()
    }
    pub unsafe fn Delete(&self, puclupair: *const u8, cblupair: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(puclupair), ::std::mem::transmute(cblupair)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDtcLuConfigure {
    type Vtable = IDtcLuConfigure_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1093789536, 6890, 4560, [148, 75, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<IDtcLuConfigure> for ::windows::runtime::IUnknown {
    fn from(value: IDtcLuConfigure) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcLuConfigure> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcLuConfigure) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcLuConfigure {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcLuConfigure {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuConfigure_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puclupair: *const u8, cblupair: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puclupair: *const u8, cblupair: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcLuRecovery(::windows::runtime::IUnknown);
impl IDtcLuRecovery {}
unsafe impl ::windows::runtime::Interface for IDtcLuRecovery {
    type Vtable = IDtcLuRecovery_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2888534738, 55024, 4560, [179, 134, 0, 160, 201, 8, 51, 101]);
}
impl ::std::convert::From<IDtcLuRecovery> for ::windows::runtime::IUnknown {
    fn from(value: IDtcLuRecovery) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcLuRecovery> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcLuRecovery) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcLuRecovery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcLuRecovery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecovery_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcLuRecoveryFactory(::windows::runtime::IUnknown);
impl IDtcLuRecoveryFactory {
    pub unsafe fn Create(&self, puclupair: *const u8, cblupair: u32) -> ::windows::runtime::Result<IDtcLuRecovery> {
        let mut result__: <IDtcLuRecovery as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(puclupair), ::std::mem::transmute(cblupair), &mut result__).from_abi::<IDtcLuRecovery>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDtcLuRecoveryFactory {
    type Vtable = IDtcLuRecoveryFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1093789538, 6890, 4560, [148, 75, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<IDtcLuRecoveryFactory> for ::windows::runtime::IUnknown {
    fn from(value: IDtcLuRecoveryFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcLuRecoveryFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcLuRecoveryFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcLuRecoveryFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcLuRecoveryFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puclupair: *const u8, cblupair: u32, pprecovery: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcLuRecoveryInitiatedByDtc(::windows::runtime::IUnknown);
impl IDtcLuRecoveryInitiatedByDtc {
    pub unsafe fn GetWork(&self, pwork: *mut _DtcLu_LocalRecovery_Work, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwork), ::std::mem::transmute(ppv)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDtcLuRecoveryInitiatedByDtc {
    type Vtable = IDtcLuRecoveryInitiatedByDtc_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1093789540, 6890, 4560, [148, 75, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<IDtcLuRecoveryInitiatedByDtc> for ::windows::runtime::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByDtc) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcLuRecoveryInitiatedByDtc> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByDtc) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcLuRecoveryInitiatedByDtc {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcLuRecoveryInitiatedByDtc {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByDtc_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwork: *mut _DtcLu_LocalRecovery_Work, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcLuRecoveryInitiatedByDtcStatusWork(::windows::runtime::IUnknown);
impl IDtcLuRecoveryInitiatedByDtcStatusWork {
    pub unsafe fn HandleCheckLuStatus(&self, lrecoveryseqnum: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(lrecoveryseqnum)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDtcLuRecoveryInitiatedByDtcStatusWork {
    type Vtable = IDtcLuRecoveryInitiatedByDtcStatusWork_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1093789542, 6890, 4560, [148, 75, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<IDtcLuRecoveryInitiatedByDtcStatusWork> for ::windows::runtime::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByDtcStatusWork) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcLuRecoveryInitiatedByDtcStatusWork> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByDtcStatusWork) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcLuRecoveryInitiatedByDtcStatusWork {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcLuRecoveryInitiatedByDtcStatusWork {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByDtcStatusWork_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lrecoveryseqnum: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcLuRecoveryInitiatedByDtcTransWork(::windows::runtime::IUnknown);
impl IDtcLuRecoveryInitiatedByDtcTransWork {
    pub unsafe fn GetLogNameSizes(&self, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcbourlogname), ::std::mem::transmute(pcbremotelogname)).ok()
    }
    pub unsafe fn GetOurXln(&self, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pxln), ::std::mem::transmute(pourlogname), ::std::mem::transmute(premotelogname), ::std::mem::transmute(pdwprotocol)).ok()
    }
    pub unsafe fn HandleConfirmationFromOurXln(&self, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(confirmation)).ok()
    }
    pub unsafe fn HandleTheirXlnResponse(&self, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut _DtcLu_Xln_Confirmation) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(xln), ::std::mem::transmute(premotelogname), ::std::mem::transmute(cbremotelogname), ::std::mem::transmute(dwprotocol), ::std::mem::transmute(pconfirmation)).ok()
    }
    pub unsafe fn HandleErrorFromOurXln(&self, error: _DtcLu_Xln_Error) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(error)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckForCompareStates(&self, fcomparestates: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(fcomparestates)).ok()
    }
    pub unsafe fn GetOurTransIdSize(&self, pcbourtransid: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcbourtransid)).ok()
    }
    pub unsafe fn GetOurCompareStates(&self, pourtransid: *mut u8, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(pourtransid), ::std::mem::transmute(pcomparestate)).ok()
    }
    pub unsafe fn HandleTheirCompareStatesResponse(&self, comparestate: _DtcLu_CompareState, pconfirmation: *mut _DtcLu_CompareStates_Confirmation) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(comparestate), ::std::mem::transmute(pconfirmation)).ok()
    }
    pub unsafe fn HandleErrorFromOurCompareStates(&self, error: _DtcLu_CompareStates_Error) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(error)).ok()
    }
    pub unsafe fn ConversationLost(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetRecoverySeqNum(&self, plrecoveryseqnum: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(plrecoveryseqnum)).ok()
    }
    pub unsafe fn ObsoleteRecoverySeqNum(&self, lnewrecoveryseqnum: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(lnewrecoveryseqnum)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDtcLuRecoveryInitiatedByDtcTransWork {
    type Vtable = IDtcLuRecoveryInitiatedByDtcTransWork_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1093789541, 6890, 4560, [148, 75, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<IDtcLuRecoveryInitiatedByDtcTransWork> for ::windows::runtime::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByDtcTransWork) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcLuRecoveryInitiatedByDtcTransWork> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByDtcTransWork) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcLuRecoveryInitiatedByDtcTransWork {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcLuRecoveryInitiatedByDtcTransWork {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByDtcTransWork_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut _DtcLu_Xln_Confirmation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, error: _DtcLu_Xln_Error) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fcomparestates: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbourtransid: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pourtransid: *mut u8, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, comparestate: _DtcLu_CompareState, pconfirmation: *mut _DtcLu_CompareStates_Confirmation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, error: _DtcLu_CompareStates_Error) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plrecoveryseqnum: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lnewrecoveryseqnum: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcLuRecoveryInitiatedByLu(::windows::runtime::IUnknown);
impl IDtcLuRecoveryInitiatedByLu {
    pub unsafe fn GetObjectToHandleWorkFromLu(&self) -> ::windows::runtime::Result<IDtcLuRecoveryInitiatedByLuWork> {
        let mut result__: <IDtcLuRecoveryInitiatedByLuWork as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDtcLuRecoveryInitiatedByLuWork>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDtcLuRecoveryInitiatedByLu {
    type Vtable = IDtcLuRecoveryInitiatedByLu_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1093789544, 6890, 4560, [148, 75, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<IDtcLuRecoveryInitiatedByLu> for ::windows::runtime::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByLu) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcLuRecoveryInitiatedByLu> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByLu) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcLuRecoveryInitiatedByLu {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcLuRecoveryInitiatedByLu {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByLu_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwork: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcLuRecoveryInitiatedByLuWork(::windows::runtime::IUnknown);
impl IDtcLuRecoveryInitiatedByLuWork {
    pub unsafe fn HandleTheirXln(&self, lrecoveryseqnum: i32, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut _DtcLu_Xln_Response) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lrecoveryseqnum),
            ::std::mem::transmute(xln),
            ::std::mem::transmute(premotelogname),
            ::std::mem::transmute(cbremotelogname),
            ::std::mem::transmute(pourlogname),
            ::std::mem::transmute(cbourlogname),
            ::std::mem::transmute(dwprotocol),
            ::std::mem::transmute(presponse),
        )
        .ok()
    }
    pub unsafe fn GetOurLogNameSize(&self, pcbourlogname: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcbourlogname)).ok()
    }
    pub unsafe fn GetOurXln(&self, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pxln), ::std::mem::transmute(pourlogname), ::std::mem::transmute(pdwprotocol)).ok()
    }
    pub unsafe fn HandleConfirmationOfOurXln(&self, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(confirmation)).ok()
    }
    pub unsafe fn HandleTheirCompareStates(&self, premotetransid: *mut u8, cbremotetransid: u32, comparestate: _DtcLu_CompareState, presponse: *mut _DtcLu_CompareStates_Response, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(premotetransid), ::std::mem::transmute(cbremotetransid), ::std::mem::transmute(comparestate), ::std::mem::transmute(presponse), ::std::mem::transmute(pcomparestate)).ok()
    }
    pub unsafe fn HandleConfirmationOfOurCompareStates(&self, confirmation: _DtcLu_CompareStates_Confirmation) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(confirmation)).ok()
    }
    pub unsafe fn HandleErrorFromOurCompareStates(&self, error: _DtcLu_CompareStates_Error) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(error)).ok()
    }
    pub unsafe fn ConversationLost(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDtcLuRecoveryInitiatedByLuWork {
    type Vtable = IDtcLuRecoveryInitiatedByLuWork_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2888534737, 55024, 4560, [179, 134, 0, 160, 201, 8, 51, 101]);
}
impl ::std::convert::From<IDtcLuRecoveryInitiatedByLuWork> for ::windows::runtime::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByLuWork) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcLuRecoveryInitiatedByLuWork> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByLuWork) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcLuRecoveryInitiatedByLuWork {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcLuRecoveryInitiatedByLuWork {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByLuWork_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lrecoveryseqnum: i32, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut _DtcLu_Xln_Response) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbourlogname: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, pdwprotocol: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, confirmation: _DtcLu_Xln_Confirmation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, premotetransid: *mut u8, cbremotetransid: u32, comparestate: _DtcLu_CompareState, presponse: *mut _DtcLu_CompareStates_Response, pcomparestate: *mut _DtcLu_CompareState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, confirmation: _DtcLu_CompareStates_Confirmation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, error: _DtcLu_CompareStates_Error) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcLuRmEnlistment(::windows::runtime::IUnknown);
impl IDtcLuRmEnlistment {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unplug<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fconversationlost: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), fconversationlost.into_param().abi()).ok()
    }
    pub unsafe fn BackedOut(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Committed(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Forget(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDtcLuRmEnlistment {
    type Vtable = IDtcLuRmEnlistment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1093789545, 6890, 4560, [148, 75, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<IDtcLuRmEnlistment> for ::windows::runtime::IUnknown {
    fn from(value: IDtcLuRmEnlistment) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcLuRmEnlistment> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcLuRmEnlistment) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcLuRmEnlistment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcLuRmEnlistment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRmEnlistment_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fconversationlost: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcLuRmEnlistmentFactory(::windows::runtime::IUnknown);
impl IDtcLuRmEnlistmentFactory {
    pub unsafe fn Create<'a, Param2: ::windows::runtime::IntoParam<'a, ITransaction>, Param5: ::windows::runtime::IntoParam<'a, IDtcLuRmEnlistmentSink>>(&self, puclupair: *mut u8, cblupair: u32, pitransaction: Param2, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: Param5, pprmenlistment: *mut ::std::option::Option<IDtcLuRmEnlistment>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(puclupair), ::std::mem::transmute(cblupair), pitransaction.into_param().abi(), ::std::mem::transmute(ptransid), ::std::mem::transmute(cbtransid), prmenlistmentsink.into_param().abi(), ::std::mem::transmute(pprmenlistment)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDtcLuRmEnlistmentFactory {
    type Vtable = IDtcLuRmEnlistmentFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1093789553, 6890, 4560, [148, 75, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<IDtcLuRmEnlistmentFactory> for ::windows::runtime::IUnknown {
    fn from(value: IDtcLuRmEnlistmentFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcLuRmEnlistmentFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcLuRmEnlistmentFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcLuRmEnlistmentFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcLuRmEnlistmentFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRmEnlistmentFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puclupair: *mut u8, cblupair: u32, pitransaction: ::windows::runtime::RawPtr, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: ::windows::runtime::RawPtr, pprmenlistment: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcLuRmEnlistmentSink(::windows::runtime::IUnknown);
impl IDtcLuRmEnlistmentSink {
    pub unsafe fn AckUnplug(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn TmDown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SessionLost(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BackedOut(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Committed(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Forget(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Prepare(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDtcLuRmEnlistmentSink {
    type Vtable = IDtcLuRmEnlistmentSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1093789552, 6890, 4560, [148, 75, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<IDtcLuRmEnlistmentSink> for ::windows::runtime::IUnknown {
    fn from(value: IDtcLuRmEnlistmentSink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcLuRmEnlistmentSink> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcLuRmEnlistmentSink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcLuRmEnlistmentSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcLuRmEnlistmentSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRmEnlistmentSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcLuSubordinateDtc(::windows::runtime::IUnknown);
impl IDtcLuSubordinateDtc {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unplug<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fconversationlost: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), fconversationlost.into_param().abi()).ok()
    }
    pub unsafe fn BackedOut(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Committed(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Forget(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Prepare(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDtcLuSubordinateDtc {
    type Vtable = IDtcLuSubordinateDtc_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1093789555, 6890, 4560, [148, 75, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<IDtcLuSubordinateDtc> for ::windows::runtime::IUnknown {
    fn from(value: IDtcLuSubordinateDtc) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcLuSubordinateDtc> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcLuSubordinateDtc) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcLuSubordinateDtc {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcLuSubordinateDtc {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuSubordinateDtc_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fconversationlost: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcLuSubordinateDtcFactory(::windows::runtime::IUnknown);
impl IDtcLuSubordinateDtcFactory {
    pub unsafe fn Create<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param5: ::windows::runtime::IntoParam<'a, ITransactionOptions>, Param9: ::windows::runtime::IntoParam<'a, IDtcLuSubordinateDtcSink>>(
        &self,
        puclupair: *mut u8,
        cblupair: u32,
        punktransactionouter: Param2,
        isolevel: i32,
        isoflags: u32,
        poptions: Param5,
        pptransaction: *mut ::std::option::Option<ITransaction>,
        ptransid: *mut u8,
        cbtransid: u32,
        psubordinatedtcsink: Param9,
        ppsubordinatedtc: *mut ::std::option::Option<IDtcLuSubordinateDtc>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(puclupair),
            ::std::mem::transmute(cblupair),
            punktransactionouter.into_param().abi(),
            ::std::mem::transmute(isolevel),
            ::std::mem::transmute(isoflags),
            poptions.into_param().abi(),
            ::std::mem::transmute(pptransaction),
            ::std::mem::transmute(ptransid),
            ::std::mem::transmute(cbtransid),
            psubordinatedtcsink.into_param().abi(),
            ::std::mem::transmute(ppsubordinatedtc),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDtcLuSubordinateDtcFactory {
    type Vtable = IDtcLuSubordinateDtcFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1093789557, 6890, 4560, [148, 75, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<IDtcLuSubordinateDtcFactory> for ::windows::runtime::IUnknown {
    fn from(value: IDtcLuSubordinateDtcFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcLuSubordinateDtcFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcLuSubordinateDtcFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcLuSubordinateDtcFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcLuSubordinateDtcFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuSubordinateDtcFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puclupair: *mut u8, cblupair: u32, punktransactionouter: ::windows::runtime::RawPtr, isolevel: i32, isoflags: u32, poptions: ::windows::runtime::RawPtr, pptransaction: *mut ::windows::runtime::RawPtr, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: ::windows::runtime::RawPtr, ppsubordinatedtc: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcLuSubordinateDtcSink(::windows::runtime::IUnknown);
impl IDtcLuSubordinateDtcSink {
    pub unsafe fn AckUnplug(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn TmDown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SessionLost(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BackedOut(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Committed(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Forget(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDtcLuSubordinateDtcSink {
    type Vtable = IDtcLuSubordinateDtcSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1093789556, 6890, 4560, [148, 75, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<IDtcLuSubordinateDtcSink> for ::windows::runtime::IUnknown {
    fn from(value: IDtcLuSubordinateDtcSink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcLuSubordinateDtcSink> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcLuSubordinateDtcSink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcLuSubordinateDtcSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcLuSubordinateDtcSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuSubordinateDtcSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcNetworkAccessConfig(::windows::runtime::IUnknown);
impl IDtcNetworkAccessConfig {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnyNetworkAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAnyNetworkAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, banynetworkaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), banynetworkaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkAdministrationAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkAdministrationAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworkadministrationaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), bnetworkadministrationaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTransactionAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTransactionAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworktransactionaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bnetworktransactionaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkClientAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkClientAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworkclientaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bnetworkclientaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTIPAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTIPAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworktipaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), bnetworktipaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXAAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXAAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bxaaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), bxaaccess.into_param().abi()).ok()
    }
    pub unsafe fn RestartDtcService(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDtcNetworkAccessConfig {
    type Vtable = IDtcNetworkAccessConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2543305053, 42024, 17041, [135, 182, 9, 149, 3, 26, 103, 141]);
}
impl ::std::convert::From<IDtcNetworkAccessConfig> for ::windows::runtime::IUnknown {
    fn from(value: IDtcNetworkAccessConfig) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcNetworkAccessConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcNetworkAccessConfig) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcNetworkAccessConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcNetworkAccessConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcNetworkAccessConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbanynetworkaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, banynetworkaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbnetworkadministrationaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbnetworktransactionaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bnetworktransactionaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbnetworkclientaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bnetworkclientaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbnetworktipaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bnetworktipaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbxaaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bxaaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcNetworkAccessConfig2(::windows::runtime::IUnknown);
impl IDtcNetworkAccessConfig2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnyNetworkAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAnyNetworkAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, banynetworkaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), banynetworkaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkAdministrationAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkAdministrationAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworkadministrationaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), bnetworkadministrationaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTransactionAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTransactionAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworktransactionaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bnetworktransactionaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkClientAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkClientAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworkclientaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bnetworkclientaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTIPAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTIPAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworktipaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), bnetworktipaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXAAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXAAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bxaaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), bxaaccess.into_param().abi()).ok()
    }
    pub unsafe fn RestartDtcService(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkInboundAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkOutboundAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkInboundAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, binbound: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), binbound.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkOutboundAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, boutbound: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), boutbound.into_param().abi()).ok()
    }
    pub unsafe fn GetAuthenticationLevel(&self) -> ::windows::runtime::Result<AUTHENTICATION_LEVEL> {
        let mut result__: <AUTHENTICATION_LEVEL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<AUTHENTICATION_LEVEL>(result__)
    }
    pub unsafe fn SetAuthenticationLevel(&self, authlevel: AUTHENTICATION_LEVEL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(authlevel)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDtcNetworkAccessConfig2 {
    type Vtable = IDtcNetworkAccessConfig2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2812936507, 60285, 20290, [180, 28, 178, 222, 192, 154, 224, 52]);
}
impl ::std::convert::From<IDtcNetworkAccessConfig2> for ::windows::runtime::IUnknown {
    fn from(value: IDtcNetworkAccessConfig2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcNetworkAccessConfig2> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcNetworkAccessConfig2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcNetworkAccessConfig2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcNetworkAccessConfig2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IDtcNetworkAccessConfig2> for IDtcNetworkAccessConfig {
    fn from(value: IDtcNetworkAccessConfig2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcNetworkAccessConfig2> for IDtcNetworkAccessConfig {
    fn from(value: &IDtcNetworkAccessConfig2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDtcNetworkAccessConfig> for IDtcNetworkAccessConfig2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDtcNetworkAccessConfig> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDtcNetworkAccessConfig>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDtcNetworkAccessConfig> for &IDtcNetworkAccessConfig2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDtcNetworkAccessConfig> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDtcNetworkAccessConfig>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcNetworkAccessConfig2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbanynetworkaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, banynetworkaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbnetworkadministrationaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbnetworktransactionaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bnetworktransactionaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbnetworkclientaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bnetworkclientaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbnetworktipaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bnetworktipaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbxaaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bxaaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbinbound: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pboutbound: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, binbound: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, boutbound: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pauthlevel: *mut AUTHENTICATION_LEVEL) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, authlevel: AUTHENTICATION_LEVEL) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcNetworkAccessConfig3(::windows::runtime::IUnknown);
impl IDtcNetworkAccessConfig3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnyNetworkAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAnyNetworkAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, banynetworkaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), banynetworkaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkAdministrationAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkAdministrationAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworkadministrationaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), bnetworkadministrationaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTransactionAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTransactionAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworktransactionaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bnetworktransactionaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkClientAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkClientAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworkclientaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bnetworkclientaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTIPAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTIPAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworktipaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), bnetworktipaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXAAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXAAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bxaaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), bxaaccess.into_param().abi()).ok()
    }
    pub unsafe fn RestartDtcService(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkInboundAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkOutboundAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkInboundAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, binbound: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), binbound.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkOutboundAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, boutbound: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), boutbound.into_param().abi()).ok()
    }
    pub unsafe fn GetAuthenticationLevel(&self) -> ::windows::runtime::Result<AUTHENTICATION_LEVEL> {
        let mut result__: <AUTHENTICATION_LEVEL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<AUTHENTICATION_LEVEL>(result__)
    }
    pub unsafe fn SetAuthenticationLevel(&self, authlevel: AUTHENTICATION_LEVEL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(authlevel)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLUAccess(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLUAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bluaccess: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), bluaccess.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDtcNetworkAccessConfig3 {
    type Vtable = IDtcNetworkAccessConfig3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1994700019, 11429, 18027, [137, 213, 253, 33, 142, 231, 91, 73]);
}
impl ::std::convert::From<IDtcNetworkAccessConfig3> for ::windows::runtime::IUnknown {
    fn from(value: IDtcNetworkAccessConfig3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcNetworkAccessConfig3> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcNetworkAccessConfig3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcNetworkAccessConfig3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcNetworkAccessConfig3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IDtcNetworkAccessConfig3> for IDtcNetworkAccessConfig2 {
    fn from(value: IDtcNetworkAccessConfig3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcNetworkAccessConfig3> for IDtcNetworkAccessConfig2 {
    fn from(value: &IDtcNetworkAccessConfig3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDtcNetworkAccessConfig2> for IDtcNetworkAccessConfig3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDtcNetworkAccessConfig2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDtcNetworkAccessConfig2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDtcNetworkAccessConfig2> for &IDtcNetworkAccessConfig3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDtcNetworkAccessConfig2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDtcNetworkAccessConfig2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IDtcNetworkAccessConfig3> for IDtcNetworkAccessConfig {
    fn from(value: IDtcNetworkAccessConfig3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcNetworkAccessConfig3> for IDtcNetworkAccessConfig {
    fn from(value: &IDtcNetworkAccessConfig3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDtcNetworkAccessConfig> for IDtcNetworkAccessConfig3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDtcNetworkAccessConfig> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDtcNetworkAccessConfig>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDtcNetworkAccessConfig> for &IDtcNetworkAccessConfig3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDtcNetworkAccessConfig> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDtcNetworkAccessConfig>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcNetworkAccessConfig3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbanynetworkaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, banynetworkaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbnetworkadministrationaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbnetworktransactionaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bnetworktransactionaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbnetworkclientaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bnetworkclientaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbnetworktipaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bnetworktipaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbxaaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bxaaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbinbound: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pboutbound: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, binbound: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, boutbound: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pauthlevel: *mut AUTHENTICATION_LEVEL) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, authlevel: AUTHENTICATION_LEVEL) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbluaccess: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bluaccess: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcToXaHelper(::windows::runtime::IUnknown);
impl IDtcToXaHelper {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Close<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, i_fdorecovery: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), i_fdorecovery.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateTridToXid<'a, Param0: ::windows::runtime::IntoParam<'a, ITransaction>>(&self, pitransaction: Param0, pguidbqual: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<xid_t> {
        let mut result__: <xid_t as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pitransaction.into_param().abi(), ::std::mem::transmute(pguidbqual), &mut result__).from_abi::<xid_t>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDtcToXaHelper {
    type Vtable = IDtcToXaHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2844136977, 12362, 4561, [152, 19, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<IDtcToXaHelper> for ::windows::runtime::IUnknown {
    fn from(value: IDtcToXaHelper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcToXaHelper> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcToXaHelper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcToXaHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcToXaHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i_fdorecovery: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitransaction: ::windows::runtime::RawPtr, pguidbqual: *const ::windows::runtime::GUID, pxid: *mut xid_t) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcToXaHelperFactory(::windows::runtime::IUnknown);
impl IDtcToXaHelperFactory {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(&self, pszdsn: Param0, pszclientdllname: Param1, pguidrm: *mut ::windows::runtime::GUID, ppxahelper: *mut ::std::option::Option<IDtcToXaHelper>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszdsn.into_param().abi(), pszclientdllname.into_param().abi(), ::std::mem::transmute(pguidrm), ::std::mem::transmute(ppxahelper)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDtcToXaHelperFactory {
    type Vtable = IDtcToXaHelperFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2844136976, 12362, 4561, [152, 19, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<IDtcToXaHelperFactory> for ::windows::runtime::IUnknown {
    fn from(value: IDtcToXaHelperFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcToXaHelperFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcToXaHelperFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcToXaHelperFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcToXaHelperFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaHelperFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszdsn: super::super::Foundation::PSTR, pszclientdllname: super::super::Foundation::PSTR, pguidrm: *mut ::windows::runtime::GUID, ppxahelper: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcToXaHelperSinglePipe(::windows::runtime::IUnknown);
impl IDtcToXaHelperSinglePipe {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn XARMCreate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(&self, pszdsn: Param0, pszclientdll: Param1, pdwrmcookie: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszdsn.into_param().abi(), pszclientdll.into_param().abi(), ::std::mem::transmute(pdwrmcookie)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConvertTridToXID(&self, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwitrans), ::std::mem::transmute(dwrmcookie), ::std::mem::transmute(pxid)).ok()
    }
    pub unsafe fn EnlistWithRM<'a, Param1: ::windows::runtime::IntoParam<'a, ITransaction>, Param2: ::windows::runtime::IntoParam<'a, ITransactionResourceAsync>>(&self, dwrmcookie: u32, i_pitransaction: Param1, i_pitransres: Param2) -> ::windows::runtime::Result<ITransactionEnlistmentAsync> {
        let mut result__: <ITransactionEnlistmentAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwrmcookie), i_pitransaction.into_param().abi(), i_pitransres.into_param().abi(), &mut result__).from_abi::<ITransactionEnlistmentAsync>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseRMCookie<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, i_dwrmcookie: u32, i_fnormal: Param1) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(i_dwrmcookie), i_fnormal.into_param().abi()))
    }
}
unsafe impl ::windows::runtime::Interface for IDtcToXaHelperSinglePipe {
    type Vtable = IDtcToXaHelperSinglePipe_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1206733169, 21427, 4561, [187, 185, 0, 192, 79, 214, 88, 246]);
}
impl ::std::convert::From<IDtcToXaHelperSinglePipe> for ::windows::runtime::IUnknown {
    fn from(value: IDtcToXaHelperSinglePipe) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcToXaHelperSinglePipe> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcToXaHelperSinglePipe) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcToXaHelperSinglePipe {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcToXaHelperSinglePipe {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaHelperSinglePipe_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszdsn: super::super::Foundation::PSTR, pszclientdll: super::super::Foundation::PSTR, pdwrmcookie: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwrmcookie: u32, i_pitransaction: ::windows::runtime::RawPtr, i_pitransres: ::windows::runtime::RawPtr, o_ppitransenslitment: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i_dwrmcookie: u32, i_fnormal: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDtcToXaMapper(::windows::runtime::IUnknown);
impl IDtcToXaMapper {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestNewResourceManager<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(&self, pszdsn: Param0, pszclientdllname: Param1, pdwrmcookie: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszdsn.into_param().abi(), pszclientdllname.into_param().abi(), ::std::mem::transmute(pdwrmcookie)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateTridToXid(&self, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwitransaction), ::std::mem::transmute(dwrmcookie), ::std::mem::transmute(pxid)).ok()
    }
    pub unsafe fn EnlistResourceManager(&self, dwrmcookie: u32, pdwitransaction: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwrmcookie), ::std::mem::transmute(pdwitransaction)).ok()
    }
    pub unsafe fn ReleaseResourceManager(&self, dwrmcookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwrmcookie)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDtcToXaMapper {
    type Vtable = IDtcToXaMapper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1694477280, 31977, 4560, [140, 230, 0, 192, 79, 220, 135, 126]);
}
impl ::std::convert::From<IDtcToXaMapper> for ::windows::runtime::IUnknown {
    fn from(value: IDtcToXaMapper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDtcToXaMapper> for ::windows::runtime::IUnknown {
    fn from(value: &IDtcToXaMapper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDtcToXaMapper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDtcToXaMapper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaMapper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszdsn: super::super::Foundation::PSTR, pszclientdllname: super::super::Foundation::PSTR, pdwrmcookie: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwrmcookie: u32, pdwitransaction: *const u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwrmcookie: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IGetDispenser(::windows::runtime::IUnknown);
impl IGetDispenser {
    pub unsafe fn GetDispenser(&self, iid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(iid), ::std::mem::transmute(ppvobject)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGetDispenser {
    type Vtable = IGetDispenser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3258762096, 34799, 4558, [128, 129, 0, 128, 199, 88, 82, 126]);
}
impl ::std::convert::From<IGetDispenser> for ::windows::runtime::IUnknown {
    fn from(value: IGetDispenser) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IGetDispenser> for ::windows::runtime::IUnknown {
    fn from(value: &IGetDispenser) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGetDispenser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IGetDispenser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetDispenser_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IKernelTransaction(::windows::runtime::IUnknown);
impl IKernelTransaction {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandle(&self) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IKernelTransaction {
    type Vtable = IKernelTransaction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2034399787, 63637, 16608, [190, 121, 181, 125, 200, 46, 210, 49]);
}
impl ::std::convert::From<IKernelTransaction> for ::windows::runtime::IUnknown {
    fn from(value: IKernelTransaction) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IKernelTransaction> for ::windows::runtime::IUnknown {
    fn from(value: &IKernelTransaction) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IKernelTransaction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IKernelTransaction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKernelTransaction_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ILastResourceManager(::windows::runtime::IUnknown);
impl ILastResourceManager {
    pub unsafe fn TransactionCommitted(&self, pprepinfo: *const u8, cbprepinfo: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pprepinfo), ::std::mem::transmute(cbprepinfo)).ok()
    }
    pub unsafe fn RecoveryDone(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ILastResourceManager {
    type Vtable = ILastResourceManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1301695188, 23347, 4563, [138, 145, 0, 192, 79, 121, 235, 109]);
}
impl ::std::convert::From<ILastResourceManager> for ::windows::runtime::IUnknown {
    fn from(value: ILastResourceManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ILastResourceManager> for ::windows::runtime::IUnknown {
    fn from(value: &ILastResourceManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILastResourceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ILastResourceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILastResourceManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprepinfo: *const u8, cbprepinfo: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPrepareInfo(::windows::runtime::IUnknown);
impl IPrepareInfo {
    pub unsafe fn GetPrepareInfoSize(&self, pcbprepinfo: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcbprepinfo)).ok()
    }
    pub unsafe fn GetPrepareInfo(&self, pprepinfo: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pprepinfo)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPrepareInfo {
    type Vtable = IPrepareInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2160574416, 34798, 4558, [128, 129, 0, 128, 199, 88, 82, 126]);
}
impl ::std::convert::From<IPrepareInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPrepareInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPrepareInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPrepareInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrepareInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPrepareInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrepareInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbprepinfo: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprepinfo: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPrepareInfo2(::windows::runtime::IUnknown);
impl IPrepareInfo2 {
    pub unsafe fn GetPrepareInfoSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPrepareInfo(&self, cbprepareinfo: u32, pprepinfo: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbprepareinfo), ::std::mem::transmute(pprepinfo)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPrepareInfo2 {
    type Vtable = IPrepareInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1605051719, 38777, 4561, [184, 134, 0, 192, 79, 185, 97, 138]);
}
impl ::std::convert::From<IPrepareInfo2> for ::windows::runtime::IUnknown {
    fn from(value: IPrepareInfo2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPrepareInfo2> for ::windows::runtime::IUnknown {
    fn from(value: &IPrepareInfo2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrepareInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPrepareInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrepareInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbprepinfo: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbprepareinfo: u32, pprepinfo: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRMHelper(::windows::runtime::IUnknown);
impl IRMHelper {
    pub unsafe fn RMCount(&self, dwctotalnumberofrms: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwctotalnumberofrms)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RMInfo<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: Param1, pszopenstring: Param2, pszclosestring: Param3, guidrmrecovery: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pxa_switch), fcdeclcallingconv.into_param().abi(), pszopenstring.into_param().abi(), pszclosestring.into_param().abi(), guidrmrecovery.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRMHelper {
    type Vtable = IRMHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3885233873, 62781, 4559, [166, 13, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<IRMHelper> for ::windows::runtime::IUnknown {
    fn from(value: IRMHelper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRMHelper> for ::windows::runtime::IUnknown {
    fn from(value: &IRMHelper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRMHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRMHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRMHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwctotalnumberofrms: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: super::super::Foundation::BOOL, pszopenstring: super::super::Foundation::PSTR, pszclosestring: super::super::Foundation::PSTR, guidrmrecovery: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IResourceManager(::windows::runtime::IUnknown);
impl IResourceManager {
    pub unsafe fn Enlist<'a, Param0: ::windows::runtime::IntoParam<'a, ITransaction>, Param1: ::windows::runtime::IntoParam<'a, ITransactionResourceAsync>>(&self, ptransaction: Param0, pres: Param1, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::std::option::Option<ITransactionEnlistmentAsync>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ptransaction.into_param().abi(), pres.into_param().abi(), ::std::mem::transmute(puow), ::std::mem::transmute(pisolevel), ::std::mem::transmute(ppenlist)).ok()
    }
    pub unsafe fn Reenlist(&self, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32) -> ::windows::runtime::Result<XACTSTAT> {
        let mut result__: <XACTSTAT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pprepinfo), ::std::mem::transmute(cbprepinfo), ::std::mem::transmute(ltimeout), &mut result__).from_abi::<XACTSTAT>(result__)
    }
    pub unsafe fn ReenlistmentComplete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetDistributedTransactionManager(&self, iid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(iid), ::std::mem::transmute(ppvobject)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IResourceManager {
    type Vtable = IResourceManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(326376737, 34795, 4558, [128, 129, 0, 128, 199, 88, 82, 126]);
}
impl ::std::convert::From<IResourceManager> for ::windows::runtime::IUnknown {
    fn from(value: IResourceManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IResourceManager> for ::windows::runtime::IUnknown {
    fn from(value: &IResourceManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IResourceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IResourceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptransaction: ::windows::runtime::RawPtr, pres: ::windows::runtime::RawPtr, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IResourceManager2(::windows::runtime::IUnknown);
impl IResourceManager2 {
    pub unsafe fn Enlist<'a, Param0: ::windows::runtime::IntoParam<'a, ITransaction>, Param1: ::windows::runtime::IntoParam<'a, ITransactionResourceAsync>>(&self, ptransaction: Param0, pres: Param1, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::std::option::Option<ITransactionEnlistmentAsync>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ptransaction.into_param().abi(), pres.into_param().abi(), ::std::mem::transmute(puow), ::std::mem::transmute(pisolevel), ::std::mem::transmute(ppenlist)).ok()
    }
    pub unsafe fn Reenlist(&self, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32) -> ::windows::runtime::Result<XACTSTAT> {
        let mut result__: <XACTSTAT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pprepinfo), ::std::mem::transmute(cbprepinfo), ::std::mem::transmute(ltimeout), &mut result__).from_abi::<XACTSTAT>(result__)
    }
    pub unsafe fn ReenlistmentComplete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetDistributedTransactionManager(&self, iid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(iid), ::std::mem::transmute(ppvobject)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enlist2<'a, Param0: ::windows::runtime::IntoParam<'a, ITransaction>, Param1: ::windows::runtime::IntoParam<'a, ITransactionResourceAsync>>(&self, ptransaction: Param0, presasync: Param1, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut ::std::option::Option<ITransactionEnlistmentAsync>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ptransaction.into_param().abi(), presasync.into_param().abi(), ::std::mem::transmute(puow), ::std::mem::transmute(pisolevel), ::std::mem::transmute(pxid), ::std::mem::transmute(ppenlist)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Reenlist2(&self, pxid: *const xid_t, dwtimeout: u32) -> ::windows::runtime::Result<XACTSTAT> {
        let mut result__: <XACTSTAT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pxid), ::std::mem::transmute(dwtimeout), &mut result__).from_abi::<XACTSTAT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IResourceManager2 {
    type Vtable = IResourceManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3510027930, 63305, 4561, [143, 71, 0, 192, 79, 142, 229, 125]);
}
impl ::std::convert::From<IResourceManager2> for ::windows::runtime::IUnknown {
    fn from(value: IResourceManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IResourceManager2> for ::windows::runtime::IUnknown {
    fn from(value: &IResourceManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IResourceManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IResourceManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IResourceManager2> for IResourceManager {
    fn from(value: IResourceManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IResourceManager2> for IResourceManager {
    fn from(value: &IResourceManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IResourceManager> for IResourceManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IResourceManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IResourceManager>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IResourceManager> for &IResourceManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IResourceManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IResourceManager>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptransaction: ::windows::runtime::RawPtr, pres: ::windows::runtime::RawPtr, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptransaction: ::windows::runtime::RawPtr, presasync: ::windows::runtime::RawPtr, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pxid: *const xid_t, dwtimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IResourceManagerFactory(::windows::runtime::IUnknown);
impl IResourceManagerFactory {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, IResourceManagerSink>>(&self, pguidrm: *const ::windows::runtime::GUID, pszrmname: Param1, piresmgrsink: Param2) -> ::windows::runtime::Result<IResourceManager> {
        let mut result__: <IResourceManager as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidrm), pszrmname.into_param().abi(), piresmgrsink.into_param().abi(), &mut result__).from_abi::<IResourceManager>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IResourceManagerFactory {
    type Vtable = IResourceManagerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(326376736, 34795, 4558, [128, 129, 0, 128, 199, 88, 82, 126]);
}
impl ::std::convert::From<IResourceManagerFactory> for ::windows::runtime::IUnknown {
    fn from(value: IResourceManagerFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IResourceManagerFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IResourceManagerFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IResourceManagerFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IResourceManagerFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidrm: *const ::windows::runtime::GUID, pszrmname: super::super::Foundation::PSTR, piresmgrsink: ::windows::runtime::RawPtr, ppresmgr: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IResourceManagerFactory2(::windows::runtime::IUnknown);
impl IResourceManagerFactory2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, IResourceManagerSink>>(&self, pguidrm: *const ::windows::runtime::GUID, pszrmname: Param1, piresmgrsink: Param2) -> ::windows::runtime::Result<IResourceManager> {
        let mut result__: <IResourceManager as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidrm), pszrmname.into_param().abi(), piresmgrsink.into_param().abi(), &mut result__).from_abi::<IResourceManager>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateEx<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, IResourceManagerSink>>(&self, pguidrm: *const ::windows::runtime::GUID, pszrmname: Param1, piresmgrsink: Param2, riidrequested: *const ::windows::runtime::GUID, ppvresmgr: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidrm), pszrmname.into_param().abi(), piresmgrsink.into_param().abi(), ::std::mem::transmute(riidrequested), ::std::mem::transmute(ppvresmgr)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IResourceManagerFactory2 {
    type Vtable = IResourceManagerFactory2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1798741025, 64466, 4561, [143, 71, 0, 192, 79, 142, 229, 125]);
}
impl ::std::convert::From<IResourceManagerFactory2> for ::windows::runtime::IUnknown {
    fn from(value: IResourceManagerFactory2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IResourceManagerFactory2> for ::windows::runtime::IUnknown {
    fn from(value: &IResourceManagerFactory2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IResourceManagerFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IResourceManagerFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IResourceManagerFactory2> for IResourceManagerFactory {
    fn from(value: IResourceManagerFactory2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IResourceManagerFactory2> for IResourceManagerFactory {
    fn from(value: &IResourceManagerFactory2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IResourceManagerFactory> for IResourceManagerFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IResourceManagerFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IResourceManagerFactory>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IResourceManagerFactory> for &IResourceManagerFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IResourceManagerFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IResourceManagerFactory>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidrm: *const ::windows::runtime::GUID, pszrmname: super::super::Foundation::PSTR, piresmgrsink: ::windows::runtime::RawPtr, ppresmgr: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidrm: *const ::windows::runtime::GUID, pszrmname: super::super::Foundation::PSTR, piresmgrsink: ::windows::runtime::RawPtr, riidrequested: *const ::windows::runtime::GUID, ppvresmgr: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IResourceManagerRejoinable(::windows::runtime::IUnknown);
impl IResourceManagerRejoinable {
    pub unsafe fn Enlist<'a, Param0: ::windows::runtime::IntoParam<'a, ITransaction>, Param1: ::windows::runtime::IntoParam<'a, ITransactionResourceAsync>>(&self, ptransaction: Param0, pres: Param1, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::std::option::Option<ITransactionEnlistmentAsync>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ptransaction.into_param().abi(), pres.into_param().abi(), ::std::mem::transmute(puow), ::std::mem::transmute(pisolevel), ::std::mem::transmute(ppenlist)).ok()
    }
    pub unsafe fn Reenlist(&self, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32) -> ::windows::runtime::Result<XACTSTAT> {
        let mut result__: <XACTSTAT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pprepinfo), ::std::mem::transmute(cbprepinfo), ::std::mem::transmute(ltimeout), &mut result__).from_abi::<XACTSTAT>(result__)
    }
    pub unsafe fn ReenlistmentComplete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetDistributedTransactionManager(&self, iid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(iid), ::std::mem::transmute(ppvobject)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enlist2<'a, Param0: ::windows::runtime::IntoParam<'a, ITransaction>, Param1: ::windows::runtime::IntoParam<'a, ITransactionResourceAsync>>(&self, ptransaction: Param0, presasync: Param1, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut ::std::option::Option<ITransactionEnlistmentAsync>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ptransaction.into_param().abi(), presasync.into_param().abi(), ::std::mem::transmute(puow), ::std::mem::transmute(pisolevel), ::std::mem::transmute(pxid), ::std::mem::transmute(ppenlist)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Reenlist2(&self, pxid: *const xid_t, dwtimeout: u32) -> ::windows::runtime::Result<XACTSTAT> {
        let mut result__: <XACTSTAT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pxid), ::std::mem::transmute(dwtimeout), &mut result__).from_abi::<XACTSTAT>(result__)
    }
    pub unsafe fn Rejoin(&self, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32) -> ::windows::runtime::Result<XACTSTAT> {
        let mut result__: <XACTSTAT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pprepinfo), ::std::mem::transmute(cbprepinfo), ::std::mem::transmute(ltimeout), &mut result__).from_abi::<XACTSTAT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IResourceManagerRejoinable {
    type Vtable = IResourceManagerRejoinable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1869473312, 46559, 20286, [156, 250, 200, 174, 189, 5, 23, 43]);
}
impl ::std::convert::From<IResourceManagerRejoinable> for ::windows::runtime::IUnknown {
    fn from(value: IResourceManagerRejoinable) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IResourceManagerRejoinable> for ::windows::runtime::IUnknown {
    fn from(value: &IResourceManagerRejoinable) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IResourceManagerRejoinable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IResourceManagerRejoinable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IResourceManagerRejoinable> for IResourceManager2 {
    fn from(value: IResourceManagerRejoinable) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IResourceManagerRejoinable> for IResourceManager2 {
    fn from(value: &IResourceManagerRejoinable) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IResourceManager2> for IResourceManagerRejoinable {
    fn into_param(self) -> ::windows::runtime::Param<'a, IResourceManager2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IResourceManager2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IResourceManager2> for &IResourceManagerRejoinable {
    fn into_param(self) -> ::windows::runtime::Param<'a, IResourceManager2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IResourceManager2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IResourceManagerRejoinable> for IResourceManager {
    fn from(value: IResourceManagerRejoinable) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IResourceManagerRejoinable> for IResourceManager {
    fn from(value: &IResourceManagerRejoinable) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IResourceManager> for IResourceManagerRejoinable {
    fn into_param(self) -> ::windows::runtime::Param<'a, IResourceManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IResourceManager>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IResourceManager> for &IResourceManagerRejoinable {
    fn into_param(self) -> ::windows::runtime::Param<'a, IResourceManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IResourceManager>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerRejoinable_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptransaction: ::windows::runtime::RawPtr, pres: ::windows::runtime::RawPtr, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptransaction: ::windows::runtime::RawPtr, presasync: ::windows::runtime::RawPtr, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pxid: *const xid_t, dwtimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IResourceManagerSink(::windows::runtime::IUnknown);
impl IResourceManagerSink {
    pub unsafe fn TMDown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IResourceManagerSink {
    type Vtable = IResourceManagerSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(223752577, 57083, 4558, [174, 209, 0, 170, 0, 81, 226, 196]);
}
impl ::std::convert::From<IResourceManagerSink> for ::windows::runtime::IUnknown {
    fn from(value: IResourceManagerSink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IResourceManagerSink> for ::windows::runtime::IUnknown {
    fn from(value: &IResourceManagerSink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IResourceManagerSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IResourceManagerSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for ISOFLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ISOFLAG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for ISOLATIONLEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ISOLATIONLEVEL {
    type Abi = Self;
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITipHelper(::windows::runtime::IUnknown);
impl ITipHelper {
    pub unsafe fn Pull(&self, i_psztxurl: *const u8) -> ::windows::runtime::Result<ITransaction> {
        let mut result__: <ITransaction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(i_psztxurl), &mut result__).from_abi::<ITransaction>(result__)
    }
    pub unsafe fn PullAsync<'a, Param1: ::windows::runtime::IntoParam<'a, ITipPullSink>>(&self, i_psztxurl: *const u8, i_ptippullsink: Param1) -> ::windows::runtime::Result<ITransaction> {
        let mut result__: <ITransaction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(i_psztxurl), i_ptippullsink.into_param().abi(), &mut result__).from_abi::<ITransaction>(result__)
    }
    pub unsafe fn GetLocalTmUrl(&self) -> ::windows::runtime::Result<*mut u8> {
        let mut result__: <*mut u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut u8>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITipHelper {
    type Vtable = ITipHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(399471313, 47813, 4561, [177, 191, 0, 192, 79, 194, 243, 239]);
}
impl ::std::convert::From<ITipHelper> for ::windows::runtime::IUnknown {
    fn from(value: ITipHelper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITipHelper> for ::windows::runtime::IUnknown {
    fn from(value: &ITipHelper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITipHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITipHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i_psztxurl: *const u8, o_ppitransaction: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i_psztxurl: *const u8, i_ptippullsink: ::windows::runtime::RawPtr, o_ppitransaction: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, o_ppszlocaltmurl: *mut *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITipPullSink(::windows::runtime::IUnknown);
impl ITipPullSink {
    pub unsafe fn PullComplete(&self, i_hrpull: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(i_hrpull)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITipPullSink {
    type Vtable = ITipPullSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(399471314, 47813, 4561, [177, 191, 0, 192, 79, 194, 243, 239]);
}
impl ::std::convert::From<ITipPullSink> for ::windows::runtime::IUnknown {
    fn from(value: ITipPullSink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITipPullSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITipPullSink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITipPullSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITipPullSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipPullSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i_hrpull: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITipTransaction(::windows::runtime::IUnknown);
impl ITipTransaction {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Push(&self, i_pszremotetmurl: *const u8) -> ::windows::runtime::Result<super::super::Foundation::PSTR> {
        let mut result__: <super::super::Foundation::PSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(i_pszremotetmurl), &mut result__).from_abi::<super::super::Foundation::PSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransactionUrl(&self) -> ::windows::runtime::Result<super::super::Foundation::PSTR> {
        let mut result__: <super::super::Foundation::PSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITipTransaction {
    type Vtable = ITipTransaction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(399471312, 47813, 4561, [177, 191, 0, 192, 79, 194, 243, 239]);
}
impl ::std::convert::From<ITipTransaction> for ::windows::runtime::IUnknown {
    fn from(value: ITipTransaction) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITipTransaction> for ::windows::runtime::IUnknown {
    fn from(value: &ITipTransaction) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITipTransaction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITipTransaction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipTransaction_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i_pszremotetmurl: *const u8, o_ppszremotetxurl: *mut super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, o_ppszlocaltxurl: *mut super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITmNodeName(::windows::runtime::IUnknown);
impl ITmNodeName {
    pub unsafe fn GetNodeNameSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNodeName<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cbnodenamebuffersize: u32, pnodenamebuffer: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbnodenamebuffersize), pnodenamebuffer.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITmNodeName {
    type Vtable = ITmNodeName_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(807882632, 28388, 18254, [155, 149, 120, 7, 188, 158, 248, 207]);
}
impl ::std::convert::From<ITmNodeName> for ::windows::runtime::IUnknown {
    fn from(value: ITmNodeName) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITmNodeName> for ::windows::runtime::IUnknown {
    fn from(value: &ITmNodeName) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITmNodeName {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITmNodeName {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITmNodeName_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbnodenamesize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbnodenamebuffersize: u32, pnodenamebuffer: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransaction(::windows::runtime::IUnknown);
impl ITransaction {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Commit<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, grftc: u32, grfrm: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), fretaining.into_param().abi(), ::std::mem::transmute(grftc), ::std::mem::transmute(grfrm)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Abort<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, fasync: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pboidreason), fretaining.into_param().abi(), fasync.into_param().abi()).ok()
    }
    pub unsafe fn GetTransactionInfo(&self) -> ::windows::runtime::Result<XACTTRANSINFO> {
        let mut result__: <XACTTRANSINFO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<XACTTRANSINFO>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITransaction {
    type Vtable = ITransaction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(263278724, 44865, 4558, [189, 43, 32, 76, 79, 79, 80, 32]);
}
impl ::std::convert::From<ITransaction> for ::windows::runtime::IUnknown {
    fn from(value: ITransaction) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransaction> for ::windows::runtime::IUnknown {
    fn from(value: &ITransaction) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransaction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransaction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransaction_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *mut XACTTRANSINFO) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransaction2(::windows::runtime::IUnknown);
impl ITransaction2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Commit<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, grftc: u32, grfrm: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), fretaining.into_param().abi(), ::std::mem::transmute(grftc), ::std::mem::transmute(grfrm)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Abort<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, fasync: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pboidreason), fretaining.into_param().abi(), fasync.into_param().abi()).ok()
    }
    pub unsafe fn GetTransactionInfo(&self) -> ::windows::runtime::Result<XACTTRANSINFO> {
        let mut result__: <XACTTRANSINFO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<XACTTRANSINFO>(result__)
    }
    pub unsafe fn CloneWithCommitDisabled(&self) -> ::windows::runtime::Result<ITransaction> {
        let mut result__: <ITransaction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ITransaction>(result__)
    }
    pub unsafe fn GetTransactionInfo2(&self) -> ::windows::runtime::Result<XACTTRANSINFO> {
        let mut result__: <XACTTRANSINFO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<XACTTRANSINFO>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITransaction2 {
    type Vtable = ITransaction2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(872551752, 101, 4563, [186, 193, 0, 192, 79, 121, 123, 226]);
}
impl ::std::convert::From<ITransaction2> for ::windows::runtime::IUnknown {
    fn from(value: ITransaction2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransaction2> for ::windows::runtime::IUnknown {
    fn from(value: &ITransaction2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransaction2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransaction2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ITransaction2> for ITransactionCloner {
    fn from(value: ITransaction2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransaction2> for ITransactionCloner {
    fn from(value: &ITransaction2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITransactionCloner> for ITransaction2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITransactionCloner> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITransactionCloner>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITransactionCloner> for &ITransaction2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITransactionCloner> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITransactionCloner>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ITransaction2> for ITransaction {
    fn from(value: ITransaction2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransaction2> for ITransaction {
    fn from(value: &ITransaction2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITransaction> for ITransaction2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITransaction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITransaction>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITransaction> for &ITransaction2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITransaction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITransaction>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransaction2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *mut XACTTRANSINFO) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppitransaction: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *mut XACTTRANSINFO) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionCloner(::windows::runtime::IUnknown);
impl ITransactionCloner {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Commit<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, grftc: u32, grfrm: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), fretaining.into_param().abi(), ::std::mem::transmute(grftc), ::std::mem::transmute(grfrm)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Abort<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, fasync: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pboidreason), fretaining.into_param().abi(), fasync.into_param().abi()).ok()
    }
    pub unsafe fn GetTransactionInfo(&self) -> ::windows::runtime::Result<XACTTRANSINFO> {
        let mut result__: <XACTTRANSINFO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<XACTTRANSINFO>(result__)
    }
    pub unsafe fn CloneWithCommitDisabled(&self) -> ::windows::runtime::Result<ITransaction> {
        let mut result__: <ITransaction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ITransaction>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionCloner {
    type Vtable = ITransactionCloner_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(40200528, 8530, 4560, [148, 76, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<ITransactionCloner> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionCloner) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionCloner> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionCloner) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionCloner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionCloner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ITransactionCloner> for ITransaction {
    fn from(value: ITransactionCloner) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionCloner> for ITransaction {
    fn from(value: &ITransactionCloner) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITransaction> for ITransactionCloner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITransaction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITransaction>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITransaction> for &ITransactionCloner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITransaction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITransaction>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionCloner_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *mut XACTTRANSINFO) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppitransaction: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionDispenser(::windows::runtime::IUnknown);
impl ITransactionDispenser {
    pub unsafe fn GetOptionsObject(&self) -> ::windows::runtime::Result<ITransactionOptions> {
        let mut result__: <ITransactionOptions as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ITransactionOptions>(result__)
    }
    pub unsafe fn BeginTransaction<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, ITransactionOptions>>(&self, punkouter: Param0, isolevel: i32, isoflags: u32, poptions: Param3) -> ::windows::runtime::Result<ITransaction> {
        let mut result__: <ITransaction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), punkouter.into_param().abi(), ::std::mem::transmute(isolevel), ::std::mem::transmute(isoflags), poptions.into_param().abi(), &mut result__).from_abi::<ITransaction>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionDispenser {
    type Vtable = ITransactionDispenser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(980081121, 9145, 4559, [173, 96, 0, 170, 0, 167, 76, 205]);
}
impl ::std::convert::From<ITransactionDispenser> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionDispenser) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionDispenser> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionDispenser) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionDispenser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionDispenser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionDispenser_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppoptions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr, isolevel: i32, isoflags: u32, poptions: ::windows::runtime::RawPtr, pptransaction: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionEnlistmentAsync(::windows::runtime::IUnknown);
impl ITransactionEnlistmentAsync {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrepareRequestDone<'a, Param1: ::windows::runtime::IntoParam<'a, super::Com::IMoniker>>(&self, hr: ::windows::runtime::HRESULT, pmk: Param1, pboidreason: *const BOID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(hr), pmk.into_param().abi(), ::std::mem::transmute(pboidreason)).ok()
    }
    pub unsafe fn CommitRequestDone(&self, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(hr)).ok()
    }
    pub unsafe fn AbortRequestDone(&self, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(hr)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionEnlistmentAsync {
    type Vtable = ITransactionEnlistmentAsync_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(263278721, 44865, 4558, [189, 43, 32, 76, 79, 79, 80, 32]);
}
impl ::std::convert::From<ITransactionEnlistmentAsync> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionEnlistmentAsync) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionEnlistmentAsync> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionEnlistmentAsync) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionEnlistmentAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionEnlistmentAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionEnlistmentAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT, pmk: ::windows::runtime::RawPtr, pboidreason: *const BOID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionExport(::windows::runtime::IUnknown);
impl ITransactionExport {
    pub unsafe fn Export<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punktransaction: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), punktransaction.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetTransactionCookie<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punktransaction: Param0, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), punktransaction.into_param().abi(), ::std::mem::transmute(cbtransactioncookie), ::std::mem::transmute(rgbtransactioncookie), ::std::mem::transmute(pcbused)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionExport {
    type Vtable = ITransactionExport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(21101989, 36800, 4558, [189, 24, 32, 76, 79, 79, 80, 32]);
}
impl ::std::convert::From<ITransactionExport> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionExport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionExport> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionExport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionExport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionExport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionExport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punktransaction: ::windows::runtime::RawPtr, pcbtransactioncookie: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punktransaction: ::windows::runtime::RawPtr, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionExportFactory(::windows::runtime::IUnknown);
impl ITransactionExportFactory {
    pub unsafe fn GetRemoteClassId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn Create(&self, cbwhereabouts: u32, rgbwhereabouts: *const u8) -> ::windows::runtime::Result<ITransactionExport> {
        let mut result__: <ITransactionExport as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbwhereabouts), ::std::mem::transmute(rgbwhereabouts), &mut result__).from_abi::<ITransactionExport>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionExportFactory {
    type Vtable = ITransactionExportFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3788479315, 34629, 4558, [169, 186, 0, 170, 0, 108, 55, 6]);
}
impl ::std::convert::From<ITransactionExportFactory> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionExportFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionExportFactory> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionExportFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionExportFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionExportFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionExportFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbwhereabouts: u32, rgbwhereabouts: *const u8, ppexport: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionImport(::windows::runtime::IUnknown);
impl ITransactionImport {
    pub unsafe fn Import<T: ::windows::runtime::Interface>(&self, cbtransactioncookie: u32, rgbtransactioncookie: *const u8) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbtransactioncookie), ::std::mem::transmute(rgbtransactioncookie), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionImport {
    type Vtable = ITransactionImport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3788479322, 34629, 4558, [169, 186, 0, 170, 0, 108, 55, 6]);
}
impl ::std::convert::From<ITransactionImport> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionImport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionImport> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionImport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionImport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionImport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionImport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbtransactioncookie: u32, rgbtransactioncookie: *const u8, piid: *const ::windows::runtime::GUID, ppvtransaction: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionImportWhereabouts(::windows::runtime::IUnknown);
impl ITransactionImportWhereabouts {
    pub unsafe fn GetWhereaboutsSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetWhereabouts(&self, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbwhereabouts), ::std::mem::transmute(rgbwhereabouts), ::std::mem::transmute(pcbused)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionImportWhereabouts {
    type Vtable = ITransactionImportWhereabouts_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(21101988, 36800, 4558, [189, 24, 32, 76, 79, 79, 80, 32]);
}
impl ::std::convert::From<ITransactionImportWhereabouts> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionImportWhereabouts) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionImportWhereabouts> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionImportWhereabouts) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionImportWhereabouts {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionImportWhereabouts {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionImportWhereabouts_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbwhereabouts: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionLastEnlistmentAsync(::windows::runtime::IUnknown);
impl ITransactionLastEnlistmentAsync {
    pub unsafe fn TransactionOutcome(&self, xactstat: XACTSTAT, pboidreason: *const BOID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(xactstat), ::std::mem::transmute(pboidreason)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionLastEnlistmentAsync {
    type Vtable = ITransactionLastEnlistmentAsync_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3358315827, 23344, 4563, [138, 145, 0, 192, 79, 121, 235, 109]);
}
impl ::std::convert::From<ITransactionLastEnlistmentAsync> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionLastEnlistmentAsync) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionLastEnlistmentAsync> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionLastEnlistmentAsync) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionLastEnlistmentAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionLastEnlistmentAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionLastEnlistmentAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xactstat: XACTSTAT, pboidreason: *const BOID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionLastResourceAsync(::windows::runtime::IUnknown);
impl ITransactionLastResourceAsync {
    pub unsafe fn DelegateCommit(&self, grfrm: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfrm)).ok()
    }
    pub unsafe fn ForgetRequest(&self, pnewuow: *const BOID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pnewuow)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionLastResourceAsync {
    type Vtable = ITransactionLastResourceAsync_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3358315826, 23344, 4563, [138, 145, 0, 192, 79, 121, 235, 109]);
}
impl ::std::convert::From<ITransactionLastResourceAsync> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionLastResourceAsync) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionLastResourceAsync> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionLastResourceAsync) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionLastResourceAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionLastResourceAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionLastResourceAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfrm: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnewuow: *const BOID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionOptions(::windows::runtime::IUnknown);
impl ITransactionOptions {
    pub unsafe fn SetOptions(&self, poptions: *const XACTOPT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(poptions)).ok()
    }
    pub unsafe fn GetOptions(&self, poptions: *mut XACTOPT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(poptions)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionOptions {
    type Vtable = ITransactionOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(980081120, 9145, 4559, [173, 96, 0, 170, 0, 167, 76, 205]);
}
impl ::std::convert::From<ITransactionOptions> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionOptions> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poptions: *const XACTOPT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poptions: *mut XACTOPT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionOutcomeEvents(::windows::runtime::IUnknown);
impl ITransactionOutcomeEvents {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Committed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, pnewuow: *const BOID, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), fretaining.into_param().abi(), ::std::mem::transmute(pnewuow), ::std::mem::transmute(hr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Aborted<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, pnewuow: *const BOID, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pboidreason), fretaining.into_param().abi(), ::std::mem::transmute(pnewuow), ::std::mem::transmute(hr)).ok()
    }
    pub unsafe fn HeuristicDecision(&self, dwdecision: u32, pboidreason: *const BOID, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwdecision), ::std::mem::transmute(pboidreason), ::std::mem::transmute(hr)).ok()
    }
    pub unsafe fn Indoubt(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionOutcomeEvents {
    type Vtable = ITransactionOutcomeEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(980081122, 9145, 4559, [173, 96, 0, 170, 0, 167, 76, 205]);
}
impl ::std::convert::From<ITransactionOutcomeEvents> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionOutcomeEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionOutcomeEvents> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionOutcomeEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionOutcomeEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionOutcomeEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionOutcomeEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdecision: u32, pboidreason: *const BOID, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionPhase0EnlistmentAsync(::windows::runtime::IUnknown);
impl ITransactionPhase0EnlistmentAsync {
    pub unsafe fn Enable(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn WaitForEnlistment(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Phase0Done(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Unenlist(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetTransaction(&self) -> ::windows::runtime::Result<ITransaction> {
        let mut result__: <ITransaction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ITransaction>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionPhase0EnlistmentAsync {
    type Vtable = ITransactionPhase0EnlistmentAsync_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2195491041, 43348, 4561, [143, 136, 0, 96, 8, 149, 231, 213]);
}
impl ::std::convert::From<ITransactionPhase0EnlistmentAsync> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionPhase0EnlistmentAsync) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionPhase0EnlistmentAsync> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionPhase0EnlistmentAsync) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionPhase0EnlistmentAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionPhase0EnlistmentAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionPhase0EnlistmentAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppitransaction: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionPhase0Factory(::windows::runtime::IUnknown);
impl ITransactionPhase0Factory {
    pub unsafe fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ITransactionPhase0NotifyAsync>>(&self, pphase0notify: Param0) -> ::windows::runtime::Result<ITransactionPhase0EnlistmentAsync> {
        let mut result__: <ITransactionPhase0EnlistmentAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pphase0notify.into_param().abi(), &mut result__).from_abi::<ITransactionPhase0EnlistmentAsync>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionPhase0Factory {
    type Vtable = ITransactionPhase0Factory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2195491040, 43348, 4561, [143, 136, 0, 96, 8, 149, 231, 213]);
}
impl ::std::convert::From<ITransactionPhase0Factory> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionPhase0Factory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionPhase0Factory> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionPhase0Factory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionPhase0Factory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionPhase0Factory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionPhase0Factory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pphase0notify: ::windows::runtime::RawPtr, ppphase0enlistment: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionPhase0NotifyAsync(::windows::runtime::IUnknown);
impl ITransactionPhase0NotifyAsync {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Phase0Request<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fabortinghint: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), fabortinghint.into_param().abi()).ok()
    }
    pub unsafe fn EnlistCompleted(&self, status: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(status)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionPhase0NotifyAsync {
    type Vtable = ITransactionPhase0NotifyAsync_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4010285065, 3190, 4562, [135, 166, 0, 192, 79, 153, 15, 52]);
}
impl ::std::convert::From<ITransactionPhase0NotifyAsync> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionPhase0NotifyAsync) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionPhase0NotifyAsync> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionPhase0NotifyAsync) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionPhase0NotifyAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionPhase0NotifyAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionPhase0NotifyAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fabortinghint: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionReceiver(::windows::runtime::IUnknown);
impl ITransactionReceiver {
    pub unsafe fn UnmarshalPropagationToken(&self, cbtoken: u32, rgbtoken: *const u8) -> ::windows::runtime::Result<ITransaction> {
        let mut result__: <ITransaction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbtoken), ::std::mem::transmute(rgbtoken), &mut result__).from_abi::<ITransaction>(result__)
    }
    pub unsafe fn GetReturnTokenSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn MarshalReturnToken(&self, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbreturntoken), ::std::mem::transmute(rgbreturntoken), ::std::mem::transmute(pcbused)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionReceiver {
    type Vtable = ITransactionReceiver_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1496399363, 45932, 4559, [165, 57, 0, 170, 0, 104, 135, 195]);
}
impl ::std::convert::From<ITransactionReceiver> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionReceiver) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionReceiver> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionReceiver) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionReceiver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionReceiver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionReceiver_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbtoken: u32, rgbtoken: *const u8, pptransaction: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbreturntoken: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionReceiverFactory(::windows::runtime::IUnknown);
impl ITransactionReceiverFactory {
    pub unsafe fn Create(&self) -> ::windows::runtime::Result<ITransactionReceiver> {
        let mut result__: <ITransactionReceiver as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ITransactionReceiver>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionReceiverFactory {
    type Vtable = ITransactionReceiverFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1496399362, 45932, 4559, [165, 57, 0, 170, 0, 104, 135, 195]);
}
impl ::std::convert::From<ITransactionReceiverFactory> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionReceiverFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionReceiverFactory> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionReceiverFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionReceiverFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionReceiverFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionReceiverFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppreceiver: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionResource(::windows::runtime::IUnknown);
impl ITransactionResource {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PrepareRequest<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, grfrm: u32, fwantmoniker: Param2, fsinglephase: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), fretaining.into_param().abi(), ::std::mem::transmute(grfrm), fwantmoniker.into_param().abi(), fsinglephase.into_param().abi()).ok()
    }
    pub unsafe fn CommitRequest(&self, grfrm: u32, pnewuow: *const BOID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfrm), ::std::mem::transmute(pnewuow)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AbortRequest<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, pnewuow: *const BOID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pboidreason), fretaining.into_param().abi(), ::std::mem::transmute(pnewuow)).ok()
    }
    pub unsafe fn TMDown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionResource {
    type Vtable = ITransactionResource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3999266739, 17778, 4560, [148, 82, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<ITransactionResource> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionResource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionResource> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionResource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionResource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfrm: u32, pnewuow: *const BOID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionResourceAsync(::windows::runtime::IUnknown);
impl ITransactionResourceAsync {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PrepareRequest<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, grfrm: u32, fwantmoniker: Param2, fsinglephase: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), fretaining.into_param().abi(), ::std::mem::transmute(grfrm), fwantmoniker.into_param().abi(), fsinglephase.into_param().abi()).ok()
    }
    pub unsafe fn CommitRequest(&self, grfrm: u32, pnewuow: *const BOID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfrm), ::std::mem::transmute(pnewuow)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AbortRequest<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, pnewuow: *const BOID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pboidreason), fretaining.into_param().abi(), ::std::mem::transmute(pnewuow)).ok()
    }
    pub unsafe fn TMDown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionResourceAsync {
    type Vtable = ITransactionResourceAsync_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1776906736, 9166, 4559, [173, 96, 0, 170, 0, 167, 76, 205]);
}
impl ::std::convert::From<ITransactionResourceAsync> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionResourceAsync) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionResourceAsync> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionResourceAsync) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionResourceAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionResourceAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionResourceAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfrm: u32, pnewuow: *const BOID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionTransmitter(::windows::runtime::IUnknown);
impl ITransactionTransmitter {
    pub unsafe fn Set<'a, Param0: ::windows::runtime::IntoParam<'a, ITransaction>>(&self, ptransaction: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ptransaction.into_param().abi()).ok()
    }
    pub unsafe fn GetPropagationTokenSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn MarshalPropagationToken(&self, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbtoken), ::std::mem::transmute(rgbtoken), ::std::mem::transmute(pcbused)).ok()
    }
    pub unsafe fn UnmarshalReturnToken(&self, cbreturntoken: u32, rgbreturntoken: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbreturntoken), ::std::mem::transmute(rgbreturntoken)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionTransmitter {
    type Vtable = ITransactionTransmitter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1496399361, 45932, 4559, [165, 57, 0, 170, 0, 104, 135, 195]);
}
impl ::std::convert::From<ITransactionTransmitter> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionTransmitter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionTransmitter> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionTransmitter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionTransmitter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionTransmitter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionTransmitter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptransaction: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbtoken: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbreturntoken: u32, rgbreturntoken: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionTransmitterFactory(::windows::runtime::IUnknown);
impl ITransactionTransmitterFactory {
    pub unsafe fn Create(&self) -> ::windows::runtime::Result<ITransactionTransmitter> {
        let mut result__: <ITransactionTransmitter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ITransactionTransmitter>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionTransmitterFactory {
    type Vtable = ITransactionTransmitterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1496399360, 45932, 4559, [165, 57, 0, 170, 0, 104, 135, 195]);
}
impl ::std::convert::From<ITransactionTransmitterFactory> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionTransmitterFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionTransmitterFactory> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionTransmitterFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionTransmitterFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionTransmitterFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionTransmitterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptransmitter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionVoterBallotAsync2(::windows::runtime::IUnknown);
impl ITransactionVoterBallotAsync2 {
    pub unsafe fn VoteRequestDone(&self, hr: ::windows::runtime::HRESULT, pboidreason: *const BOID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(hr), ::std::mem::transmute(pboidreason)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionVoterBallotAsync2 {
    type Vtable = ITransactionVoterBallotAsync2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1412642668, 16717, 4563, [178, 6, 0, 192, 79, 194, 243, 239]);
}
impl ::std::convert::From<ITransactionVoterBallotAsync2> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionVoterBallotAsync2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionVoterBallotAsync2> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionVoterBallotAsync2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionVoterBallotAsync2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionVoterBallotAsync2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionVoterBallotAsync2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT, pboidreason: *const BOID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionVoterFactory2(::windows::runtime::IUnknown);
impl ITransactionVoterFactory2 {
    pub unsafe fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ITransaction>, Param1: ::windows::runtime::IntoParam<'a, ITransactionVoterNotifyAsync2>>(&self, ptransaction: Param0, pvoternotify: Param1) -> ::windows::runtime::Result<ITransactionVoterBallotAsync2> {
        let mut result__: <ITransactionVoterBallotAsync2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ptransaction.into_param().abi(), pvoternotify.into_param().abi(), &mut result__).from_abi::<ITransactionVoterBallotAsync2>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionVoterFactory2 {
    type Vtable = ITransactionVoterFactory2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1412642666, 16717, 4563, [178, 6, 0, 192, 79, 194, 243, 239]);
}
impl ::std::convert::From<ITransactionVoterFactory2> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionVoterFactory2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionVoterFactory2> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionVoterFactory2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionVoterFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionVoterFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionVoterFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptransaction: ::windows::runtime::RawPtr, pvoternotify: ::windows::runtime::RawPtr, ppvoterballot: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITransactionVoterNotifyAsync2(::windows::runtime::IUnknown);
impl ITransactionVoterNotifyAsync2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Committed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, pnewuow: *const BOID, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), fretaining.into_param().abi(), ::std::mem::transmute(pnewuow), ::std::mem::transmute(hr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Aborted<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, pnewuow: *const BOID, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pboidreason), fretaining.into_param().abi(), ::std::mem::transmute(pnewuow), ::std::mem::transmute(hr)).ok()
    }
    pub unsafe fn HeuristicDecision(&self, dwdecision: u32, pboidreason: *const BOID, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwdecision), ::std::mem::transmute(pboidreason), ::std::mem::transmute(hr)).ok()
    }
    pub unsafe fn Indoubt(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn VoteRequest(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITransactionVoterNotifyAsync2 {
    type Vtable = ITransactionVoterNotifyAsync2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1412642667, 16717, 4563, [178, 6, 0, 192, 79, 194, 243, 239]);
}
impl ::std::convert::From<ITransactionVoterNotifyAsync2> for ::windows::runtime::IUnknown {
    fn from(value: ITransactionVoterNotifyAsync2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionVoterNotifyAsync2> for ::windows::runtime::IUnknown {
    fn from(value: &ITransactionVoterNotifyAsync2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransactionVoterNotifyAsync2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransactionVoterNotifyAsync2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ITransactionVoterNotifyAsync2> for ITransactionOutcomeEvents {
    fn from(value: ITransactionVoterNotifyAsync2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransactionVoterNotifyAsync2> for ITransactionOutcomeEvents {
    fn from(value: &ITransactionVoterNotifyAsync2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITransactionOutcomeEvents> for ITransactionVoterNotifyAsync2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITransactionOutcomeEvents> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITransactionOutcomeEvents>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITransactionOutcomeEvents> for &ITransactionVoterNotifyAsync2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITransactionOutcomeEvents> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITransactionOutcomeEvents>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionVoterNotifyAsync2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdecision: u32, pboidreason: *const BOID, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IXAConfig(::windows::runtime::IUnknown);
impl IXAConfig {
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, clsidhelperdll: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), clsidhelperdll.into_param().abi()).ok()
    }
    pub unsafe fn Terminate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXAConfig {
    type Vtable = IXAConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3366380449, 39564, 4559, [163, 8, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<IXAConfig> for ::windows::runtime::IUnknown {
    fn from(value: IXAConfig) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXAConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IXAConfig) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXAConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXAConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clsidhelperdll: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IXAObtainRMInfo(::windows::runtime::IUnknown);
impl IXAObtainRMInfo {
    pub unsafe fn ObtainRMInfo<'a, Param0: ::windows::runtime::IntoParam<'a, IRMHelper>>(&self, pirmhelper: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pirmhelper.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXAObtainRMInfo {
    type Vtable = IXAObtainRMInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3885233874, 62781, 4559, [166, 13, 0, 160, 201, 5, 65, 110]);
}
impl ::std::convert::From<IXAObtainRMInfo> for ::windows::runtime::IUnknown {
    fn from(value: IXAObtainRMInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXAObtainRMInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IXAObtainRMInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXAObtainRMInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXAObtainRMInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAObtainRMInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirmhelper: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IXATransLookup(::windows::runtime::IUnknown);
impl IXATransLookup {
    pub unsafe fn Lookup(&self) -> ::windows::runtime::Result<ITransaction> {
        let mut result__: <ITransaction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ITransaction>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXATransLookup {
    type Vtable = IXATransLookup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4088525105, 61146, 4558, [174, 212, 0, 170, 0, 81, 226, 196]);
}
impl ::std::convert::From<IXATransLookup> for ::windows::runtime::IUnknown {
    fn from(value: IXATransLookup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXATransLookup> for ::windows::runtime::IUnknown {
    fn from(value: &IXATransLookup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXATransLookup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXATransLookup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXATransLookup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptransaction: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IXATransLookup2(::windows::runtime::IUnknown);
impl IXATransLookup2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Lookup(&self, pxid: *const xid_t) -> ::windows::runtime::Result<ITransaction> {
        let mut result__: <ITransaction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pxid), &mut result__).from_abi::<ITransaction>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXATransLookup2 {
    type Vtable = IXATransLookup2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3206102149, 3354, 17040, [184, 143, 210, 203, 136, 115, 209, 231]);
}
impl ::std::convert::From<IXATransLookup2> for ::windows::runtime::IUnknown {
    fn from(value: IXATransLookup2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXATransLookup2> for ::windows::runtime::IUnknown {
    fn from(value: &IXATransLookup2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXATransLookup2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXATransLookup2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXATransLookup2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pxid: *const xid_t, pptransaction: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub const MAXBQUALSIZE: u32 = 64u32;
pub const MAXGTRIDSIZE: u32 = 64u32;
pub const MAXINFOSIZE: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct OLE_TM_CONFIG_PARAMS_V1 {
    pub dwVersion: u32,
    pub dwcConcurrencyHint: u32,
}
impl OLE_TM_CONFIG_PARAMS_V1 {}
impl ::std::default::Default for OLE_TM_CONFIG_PARAMS_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for OLE_TM_CONFIG_PARAMS_V1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLE_TM_CONFIG_PARAMS_V1").field("dwVersion", &self.dwVersion).field("dwcConcurrencyHint", &self.dwcConcurrencyHint).finish()
    }
}
impl ::std::cmp::PartialEq for OLE_TM_CONFIG_PARAMS_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwcConcurrencyHint == other.dwcConcurrencyHint
    }
}
impl ::std::cmp::Eq for OLE_TM_CONFIG_PARAMS_V1 {}
unsafe impl ::windows::runtime::Abi for OLE_TM_CONFIG_PARAMS_V1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct OLE_TM_CONFIG_PARAMS_V2 {
    pub dwVersion: u32,
    pub dwcConcurrencyHint: u32,
    pub applicationType: APPLICATIONTYPE,
    pub clusterResourceId: ::windows::runtime::GUID,
}
impl OLE_TM_CONFIG_PARAMS_V2 {}
impl ::std::default::Default for OLE_TM_CONFIG_PARAMS_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for OLE_TM_CONFIG_PARAMS_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLE_TM_CONFIG_PARAMS_V2").field("dwVersion", &self.dwVersion).field("dwcConcurrencyHint", &self.dwcConcurrencyHint).field("applicationType", &self.applicationType).field("clusterResourceId", &self.clusterResourceId).finish()
    }
}
impl ::std::cmp::PartialEq for OLE_TM_CONFIG_PARAMS_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwcConcurrencyHint == other.dwcConcurrencyHint && self.applicationType == other.applicationType && self.clusterResourceId == other.clusterResourceId
    }
}
impl ::std::cmp::Eq for OLE_TM_CONFIG_PARAMS_V2 {}
unsafe impl ::windows::runtime::Abi for OLE_TM_CONFIG_PARAMS_V2 {
    type Abi = Self;
    type DefaultType = Self;
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TX_MISC_CONSTANTS(pub i32);
pub const MAX_TRAN_DESC: TX_MISC_CONSTANTS = TX_MISC_CONSTANTS(40i32);
impl ::std::convert::From<i32> for TX_MISC_CONSTANTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TX_MISC_CONSTANTS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XACTCONST(pub i32);
pub const XACTCONST_TIMEOUTINFINITE: XACTCONST = XACTCONST(0i32);
impl ::std::convert::From<i32> for XACTCONST {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XACTCONST {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XACTHEURISTIC(pub i32);
pub const XACTHEURISTIC_ABORT: XACTHEURISTIC = XACTHEURISTIC(1i32);
pub const XACTHEURISTIC_COMMIT: XACTHEURISTIC = XACTHEURISTIC(2i32);
pub const XACTHEURISTIC_DAMAGE: XACTHEURISTIC = XACTHEURISTIC(3i32);
pub const XACTHEURISTIC_DANGER: XACTHEURISTIC = XACTHEURISTIC(4i32);
impl ::std::convert::From<i32> for XACTHEURISTIC {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XACTHEURISTIC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct XACTOPT {
    pub ulTimeout: u32,
    pub szDescription: [u8; 40],
}
impl XACTOPT {}
impl ::std::default::Default for XACTOPT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for XACTOPT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("XACTOPT").field("ulTimeout", &self.ulTimeout).field("szDescription", &self.szDescription).finish()
    }
}
impl ::std::cmp::PartialEq for XACTOPT {
    fn eq(&self, other: &Self) -> bool {
        self.ulTimeout == other.ulTimeout && self.szDescription == other.szDescription
    }
}
impl ::std::cmp::Eq for XACTOPT {}
unsafe impl ::windows::runtime::Abi for XACTOPT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XACTRM(pub i32);
pub const XACTRM_OPTIMISTICLASTWINS: XACTRM = XACTRM(1i32);
pub const XACTRM_NOREADONLYPREPARES: XACTRM = XACTRM(2i32);
impl ::std::convert::From<i32> for XACTRM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XACTRM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for XACTSTAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XACTSTAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for XACTSTATS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for XACTSTATS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for XACTSTATS {
    fn eq(&self, other: &Self) -> bool {
        self.cOpen == other.cOpen && self.cCommitting == other.cCommitting && self.cCommitted == other.cCommitted && self.cAborting == other.cAborting && self.cAborted == other.cAborted && self.cInDoubt == other.cInDoubt && self.cHeuristicDecision == other.cHeuristicDecision && self.timeTransactionsUp == other.timeTransactionsUp
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for XACTSTATS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for XACTSTATS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XACTTC(pub i32);
pub const XACTTC_NONE: XACTTC = XACTTC(0i32);
pub const XACTTC_SYNC_PHASEONE: XACTTC = XACTTC(1i32);
pub const XACTTC_SYNC_PHASETWO: XACTTC = XACTTC(2i32);
pub const XACTTC_SYNC: XACTTC = XACTTC(2i32);
pub const XACTTC_ASYNC_PHASEONE: XACTTC = XACTTC(4i32);
pub const XACTTC_ASYNC: XACTTC = XACTTC(4i32);
impl ::std::convert::From<i32> for XACTTC {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XACTTC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for XACTTRANSINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for XACTTRANSINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for XACTTRANSINFO {
    fn eq(&self, other: &Self) -> bool {
        self.uow == other.uow && self.isoLevel == other.isoLevel && self.isoFlags == other.isoFlags && self.grfTCSupported == other.grfTCSupported && self.grfRMSupported == other.grfRMSupported && self.grfTCSupportedRetaining == other.grfTCSupportedRetaining && self.grfRMSupportedRetaining == other.grfRMSupportedRetaining
    }
}
impl ::std::cmp::Eq for XACTTRANSINFO {}
unsafe impl ::windows::runtime::Abi for XACTTRANSINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for XACT_DTC_CONSTANTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XACT_DTC_CONSTANTS {
    type Abi = Self;
    type DefaultType = Self;
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DtcLu_CompareState(pub i32);
pub const DTCLUCOMPARESTATE_COMMITTED: _DtcLu_CompareState = _DtcLu_CompareState(1i32);
pub const DTCLUCOMPARESTATE_HEURISTICCOMMITTED: _DtcLu_CompareState = _DtcLu_CompareState(2i32);
pub const DTCLUCOMPARESTATE_HEURISTICMIXED: _DtcLu_CompareState = _DtcLu_CompareState(3i32);
pub const DTCLUCOMPARESTATE_HEURISTICRESET: _DtcLu_CompareState = _DtcLu_CompareState(4i32);
pub const DTCLUCOMPARESTATE_INDOUBT: _DtcLu_CompareState = _DtcLu_CompareState(5i32);
pub const DTCLUCOMPARESTATE_RESET: _DtcLu_CompareState = _DtcLu_CompareState(6i32);
impl ::std::convert::From<i32> for _DtcLu_CompareState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DtcLu_CompareState {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DtcLu_CompareStates_Confirmation(pub i32);
pub const DTCLUCOMPARESTATESCONFIRMATION_CONFIRM: _DtcLu_CompareStates_Confirmation = _DtcLu_CompareStates_Confirmation(1i32);
pub const DTCLUCOMPARESTATESCONFIRMATION_PROTOCOL: _DtcLu_CompareStates_Confirmation = _DtcLu_CompareStates_Confirmation(2i32);
impl ::std::convert::From<i32> for _DtcLu_CompareStates_Confirmation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DtcLu_CompareStates_Confirmation {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DtcLu_CompareStates_Error(pub i32);
pub const DTCLUCOMPARESTATESERROR_PROTOCOL: _DtcLu_CompareStates_Error = _DtcLu_CompareStates_Error(1i32);
impl ::std::convert::From<i32> for _DtcLu_CompareStates_Error {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DtcLu_CompareStates_Error {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DtcLu_CompareStates_Response(pub i32);
pub const DTCLUCOMPARESTATESRESPONSE_OK: _DtcLu_CompareStates_Response = _DtcLu_CompareStates_Response(1i32);
pub const DTCLUCOMPARESTATESRESPONSE_PROTOCOL: _DtcLu_CompareStates_Response = _DtcLu_CompareStates_Response(2i32);
impl ::std::convert::From<i32> for _DtcLu_CompareStates_Response {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DtcLu_CompareStates_Response {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DtcLu_LocalRecovery_Work(pub i32);
pub const DTCINITIATEDRECOVERYWORK_CHECKLUSTATUS: _DtcLu_LocalRecovery_Work = _DtcLu_LocalRecovery_Work(1i32);
pub const DTCINITIATEDRECOVERYWORK_TRANS: _DtcLu_LocalRecovery_Work = _DtcLu_LocalRecovery_Work(2i32);
pub const DTCINITIATEDRECOVERYWORK_TMDOWN: _DtcLu_LocalRecovery_Work = _DtcLu_LocalRecovery_Work(3i32);
impl ::std::convert::From<i32> for _DtcLu_LocalRecovery_Work {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DtcLu_LocalRecovery_Work {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DtcLu_Xln(pub i32);
pub const DTCLUXLN_COLD: _DtcLu_Xln = _DtcLu_Xln(1i32);
pub const DTCLUXLN_WARM: _DtcLu_Xln = _DtcLu_Xln(2i32);
impl ::std::convert::From<i32> for _DtcLu_Xln {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DtcLu_Xln {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DtcLu_Xln_Confirmation(pub i32);
pub const DTCLUXLNCONFIRMATION_CONFIRM: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(1i32);
pub const DTCLUXLNCONFIRMATION_LOGNAMEMISMATCH: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(2i32);
pub const DTCLUXLNCONFIRMATION_COLDWARMMISMATCH: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(3i32);
pub const DTCLUXLNCONFIRMATION_OBSOLETE: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(4i32);
impl ::std::convert::From<i32> for _DtcLu_Xln_Confirmation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DtcLu_Xln_Confirmation {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DtcLu_Xln_Error(pub i32);
pub const DTCLUXLNERROR_PROTOCOL: _DtcLu_Xln_Error = _DtcLu_Xln_Error(1i32);
pub const DTCLUXLNERROR_LOGNAMEMISMATCH: _DtcLu_Xln_Error = _DtcLu_Xln_Error(2i32);
pub const DTCLUXLNERROR_COLDWARMMISMATCH: _DtcLu_Xln_Error = _DtcLu_Xln_Error(3i32);
impl ::std::convert::From<i32> for _DtcLu_Xln_Error {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DtcLu_Xln_Error {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DtcLu_Xln_Response(pub i32);
pub const DTCLUXLNRESPONSE_OK_SENDOURXLNBACK: _DtcLu_Xln_Response = _DtcLu_Xln_Response(1i32);
pub const DTCLUXLNRESPONSE_OK_SENDCONFIRMATION: _DtcLu_Xln_Response = _DtcLu_Xln_Response(2i32);
pub const DTCLUXLNRESPONSE_LOGNAMEMISMATCH: _DtcLu_Xln_Response = _DtcLu_Xln_Response(3i32);
pub const DTCLUXLNRESPONSE_COLDWARMMISMATCH: _DtcLu_Xln_Response = _DtcLu_Xln_Response(4i32);
impl ::std::convert::From<i32> for _DtcLu_Xln_Response {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DtcLu_Xln_Response {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct _ProxyConfigParams {
    pub wcThreadsMax: u16,
}
impl _ProxyConfigParams {}
impl ::std::default::Default for _ProxyConfigParams {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for _ProxyConfigParams {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ProxyConfigParams").field("wcThreadsMax", &self.wcThreadsMax).finish()
    }
}
impl ::std::cmp::PartialEq for _ProxyConfigParams {
    fn eq(&self, other: &Self) -> bool {
        self.wcThreadsMax == other.wcThreadsMax
    }
}
impl ::std::cmp::Eq for _ProxyConfigParams {}
unsafe impl ::windows::runtime::Abi for _ProxyConfigParams {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for xa_switch_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for xa_switch_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for xa_switch_t {
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
impl ::std::cmp::Eq for xa_switch_t {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for xa_switch_t {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for xid_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for xid_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xid_t").field("formatID", &self.formatID).field("gtrid_length", &self.gtrid_length).field("bqual_length", &self.bqual_length).field("data", &self.data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for xid_t {
    fn eq(&self, other: &Self) -> bool {
        self.formatID == other.formatID && self.gtrid_length == other.gtrid_length && self.bqual_length == other.bqual_length && self.data == other.data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for xid_t {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for xid_t {
    type Abi = Self;
    type DefaultType = Self;
}
