#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_System_Performance_HardwareCounterProfiling")]
pub mod HardwareCounterProfiling;
pub const AppearPropPage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe49741e9_93a8_4ab1_8e96_bf4482282e9c);
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type AutoPathFormat = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaNone: AutoPathFormat = 0i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaPattern: AutoPathFormat = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaComputer: AutoPathFormat = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaMonthDayHour: AutoPathFormat = 256i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaSerialNumber: AutoPathFormat = 512i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaYearDayOfYear: AutoPathFormat = 1024i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaYearMonth: AutoPathFormat = 2048i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaYearMonthDay: AutoPathFormat = 4096i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaYearMonthDayHour: AutoPathFormat = 8192i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaMonthDayHourMinute: AutoPathFormat = 16384i32;
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BackupPerfRegistryToFileW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szfilename: Param0, szcommentstring: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BackupPerfRegistryToFileW(szfilename: super::super::Foundation::PWSTR, szcommentstring: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(BackupPerfRegistryToFileW(szfilename.into_param().abi(), szcommentstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const BootTraceSession: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837538_098b_11d8_9414_505054503030);
pub const BootTraceSessionCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837539_098b_11d8_9414_505054503030);
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type ClockType = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaTimeStamp: ClockType = 0i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaPerformance: ClockType = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaSystem: ClockType = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaCycle: ClockType = 3i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type CommitMode = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaCreateNew: CommitMode = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaModify: CommitMode = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaCreateOrModify: CommitMode = 3i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaUpdateRunningInstance: CommitMode = 16i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaFlushTrace: CommitMode = 32i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaValidateOnly: CommitMode = 4096i32;
pub const CounterItem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4d2d8e0_d1dd_11ce_940f_008029004348);
pub const CounterItem2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43196c62_c31f_4ce3_a02e_79efe0f6a525);
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type CounterPathCallBack = ::core::option::Option<unsafe extern "system" fn(param0: usize) -> i32>;
pub const CounterPropPage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf948561_ede8_11ce_941e_008029004347);
pub const Counters: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2b066d2_2aac_11cf_942f_008029004347);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct DICounterItem(::windows::core::IUnknown);
impl DICounterItem {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DICounterItem> for super::Com::IDispatch {
    fn from(value: DICounterItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DICounterItem> for super::Com::IDispatch {
    fn from(value: &DICounterItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for DICounterItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &DICounterItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DICounterItem> for ::windows::core::IUnknown {
    fn from(value: DICounterItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DICounterItem> for ::windows::core::IUnknown {
    fn from(value: &DICounterItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DICounterItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DICounterItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for DICounterItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DICounterItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DICounterItem {}
impl ::core::fmt::Debug for DICounterItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DICounterItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for DICounterItem {
    type Vtable = DICounterItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08c4ff2_0e2e_11cf_942c_008029004347);
}
#[repr(C)]
#[doc(hidden)]
pub struct DICounterItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
pub const DIID_DICounterItem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08c4ff2_0e2e_11cf_942c_008029004347);
pub const DIID_DILogFileItem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d093ffc_f777_4917_82d1_833fbc54c58f);
pub const DIID_DISystemMonitor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13d73d81_c32e_11cf_9398_00aa00a3ddea);
pub const DIID_DISystemMonitorEvents: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84979930_4ab3_11cf_943a_008029004347);
pub const DIID_DISystemMonitorInternal: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x194eb242_c32c_11cf_9398_00aa00a3ddea);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct DILogFileItem(::windows::core::IUnknown);
impl DILogFileItem {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DILogFileItem> for super::Com::IDispatch {
    fn from(value: DILogFileItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DILogFileItem> for super::Com::IDispatch {
    fn from(value: &DILogFileItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for DILogFileItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &DILogFileItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DILogFileItem> for ::windows::core::IUnknown {
    fn from(value: DILogFileItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DILogFileItem> for ::windows::core::IUnknown {
    fn from(value: &DILogFileItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DILogFileItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DILogFileItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for DILogFileItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DILogFileItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DILogFileItem {}
impl ::core::fmt::Debug for DILogFileItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DILogFileItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for DILogFileItem {
    type Vtable = DILogFileItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d093ffc_f777_4917_82d1_833fbc54c58f);
}
#[repr(C)]
#[doc(hidden)]
pub struct DILogFileItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct DISystemMonitor(::windows::core::IUnknown);
impl DISystemMonitor {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DISystemMonitor> for super::Com::IDispatch {
    fn from(value: DISystemMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DISystemMonitor> for super::Com::IDispatch {
    fn from(value: &DISystemMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for DISystemMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &DISystemMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DISystemMonitor> for ::windows::core::IUnknown {
    fn from(value: DISystemMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DISystemMonitor> for ::windows::core::IUnknown {
    fn from(value: &DISystemMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DISystemMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DISystemMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for DISystemMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DISystemMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DISystemMonitor {}
impl ::core::fmt::Debug for DISystemMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISystemMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for DISystemMonitor {
    type Vtable = DISystemMonitorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13d73d81_c32e_11cf_9398_00aa00a3ddea);
}
#[repr(C)]
#[doc(hidden)]
pub struct DISystemMonitorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct DISystemMonitorEvents(::windows::core::IUnknown);
impl DISystemMonitorEvents {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DISystemMonitorEvents> for super::Com::IDispatch {
    fn from(value: DISystemMonitorEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DISystemMonitorEvents> for super::Com::IDispatch {
    fn from(value: &DISystemMonitorEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for DISystemMonitorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &DISystemMonitorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DISystemMonitorEvents> for ::windows::core::IUnknown {
    fn from(value: DISystemMonitorEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DISystemMonitorEvents> for ::windows::core::IUnknown {
    fn from(value: &DISystemMonitorEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DISystemMonitorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DISystemMonitorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for DISystemMonitorEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DISystemMonitorEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DISystemMonitorEvents {}
impl ::core::fmt::Debug for DISystemMonitorEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISystemMonitorEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for DISystemMonitorEvents {
    type Vtable = DISystemMonitorEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84979930_4ab3_11cf_943a_008029004347);
}
#[repr(C)]
#[doc(hidden)]
pub struct DISystemMonitorEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct DISystemMonitorInternal(::windows::core::IUnknown);
impl DISystemMonitorInternal {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DISystemMonitorInternal> for super::Com::IDispatch {
    fn from(value: DISystemMonitorInternal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DISystemMonitorInternal> for super::Com::IDispatch {
    fn from(value: &DISystemMonitorInternal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for DISystemMonitorInternal {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &DISystemMonitorInternal {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DISystemMonitorInternal> for ::windows::core::IUnknown {
    fn from(value: DISystemMonitorInternal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DISystemMonitorInternal> for ::windows::core::IUnknown {
    fn from(value: &DISystemMonitorInternal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DISystemMonitorInternal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DISystemMonitorInternal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for DISystemMonitorInternal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DISystemMonitorInternal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DISystemMonitorInternal {}
impl ::core::fmt::Debug for DISystemMonitorInternal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISystemMonitorInternal").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for DISystemMonitorInternal {
    type Vtable = DISystemMonitorInternalVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x194eb242_c32c_11cf_9398_00aa00a3ddea);
}
#[repr(C)]
#[doc(hidden)]
pub struct DISystemMonitorInternalVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
pub const DataCollectorSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837521_098b_11d8_9414_505054503030);
pub const DataCollectorSetCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837525_098b_11d8_9414_505054503030);
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type DataCollectorSetStatus = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaStopped: DataCollectorSetStatus = 0i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaRunning: DataCollectorSetStatus = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaCompiling: DataCollectorSetStatus = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaPending: DataCollectorSetStatus = 3i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaUndefined: DataCollectorSetStatus = 4i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type DataCollectorType = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaPerformanceCounter: DataCollectorType = 0i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaTrace: DataCollectorType = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaConfiguration: DataCollectorType = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaAlert: DataCollectorType = 3i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaApiTrace: DataCollectorType = 4i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type DataManagerSteps = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaCreateReport: DataManagerSteps = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaRunRules: DataManagerSteps = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaCreateHtml: DataManagerSteps = 4i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaFolderActions: DataManagerSteps = 8i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaResourceFreeing: DataManagerSteps = 16i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type DataSourceTypeConstants = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonNullDataSource: DataSourceTypeConstants = -1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonCurrentActivity: DataSourceTypeConstants = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonLogFiles: DataSourceTypeConstants = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonSqlLog: DataSourceTypeConstants = 3i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type DisplayTypeConstants = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonLineGraph: DisplayTypeConstants = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonHistogram: DisplayTypeConstants = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonReport: DisplayTypeConstants = 3i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonChartArea: DisplayTypeConstants = 4i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonChartStackedArea: DisplayTypeConstants = 5i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type FileFormat = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaCommaSeparated: FileFormat = 0i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaTabSeparated: FileFormat = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaSql: FileFormat = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaBinary: FileFormat = 3i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type FolderActionSteps = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaCreateCab: FolderActionSteps = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaDeleteData: FolderActionSteps = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaSendCab: FolderActionSteps = 4i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaDeleteCab: FolderActionSteps = 8i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaDeleteReport: FolderActionSteps = 16i32;
pub const GeneralPropPage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3e5d3d2_1a03_11cf_942d_008029004347);
pub const GraphPropPage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3e5d3d3_1a03_11cf_942d_008029004347);
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const H_WBEM_DATASOURCE: i32 = -1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct IAlertDataCollector(::windows::core::IUnknown);
impl IAlertDataCollector {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataCollectorSet(&self) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollectorSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, group: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), group.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataCollectorType(&self) -> ::windows::core::Result<DataCollectorType> {
        let mut result__: DataCollectorType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataCollectorType>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn FileNameFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: AutoPathFormat = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AutoPathFormat>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNameFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNameFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogAppend(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogAppend(&self, append: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(append)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogCircular(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogCircular(&self, circular: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(circular)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogOverwrite(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetIndex(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), xml.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOutputLocation(&self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(latest), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AlertThresholds(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAlertThresholds(&self, alerts: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(alerts)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn EventLog(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetEventLog(&self, log: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(log)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SampleInterval(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetSampleInterval(&self, interval: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(interval)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Task(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, task: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), task.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn TaskRunAsSelf(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetTaskRunAsSelf(&self, runasself: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(runasself)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskArguments(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTaskArguments<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, task: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), task.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskUserTextArguments(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTaskUserTextArguments<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, task: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), task.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TriggerDataCollectorSet(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTriggerDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAlertDataCollector> for IDataCollector {
    fn from(value: IAlertDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAlertDataCollector> for IDataCollector {
    fn from(value: &IAlertDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for IAlertDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for &IAlertDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAlertDataCollector> for super::Com::IDispatch {
    fn from(value: IAlertDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAlertDataCollector> for super::Com::IDispatch {
    fn from(value: &IAlertDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IAlertDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IAlertDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAlertDataCollector> for ::windows::core::IUnknown {
    fn from(value: IAlertDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAlertDataCollector> for ::windows::core::IUnknown {
    fn from(value: &IAlertDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAlertDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAlertDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAlertDataCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAlertDataCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAlertDataCollector {}
impl ::core::fmt::Debug for IAlertDataCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAlertDataCollector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAlertDataCollector {
    type Vtable = IAlertDataCollectorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837516_098b_11d8_9414_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlertDataCollectorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut DataCollectorType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *mut AutoPathFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: AutoPathFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, append: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, append: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, circular: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, circular: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, latest: i16, location: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alerts: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alerts: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, log: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, log: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interval: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interval: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runasself: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runasself: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct IApiTracingDataCollector(::windows::core::IUnknown);
impl IApiTracingDataCollector {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataCollectorSet(&self) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollectorSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, group: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), group.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataCollectorType(&self) -> ::windows::core::Result<DataCollectorType> {
        let mut result__: DataCollectorType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataCollectorType>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn FileNameFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: AutoPathFormat = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AutoPathFormat>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNameFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNameFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogAppend(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogAppend(&self, append: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(append)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogCircular(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogCircular(&self, circular: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(circular)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogOverwrite(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetIndex(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), xml.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOutputLocation(&self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(latest), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogApiNamesOnly(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogApiNamesOnly(&self, logapinames: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(logapinames)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogApisRecursively(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogApisRecursively(&self, logrecursively: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(logrecursively)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExePath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExePath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, exepath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), exepath.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogFilePath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogFilePath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, logfilepath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), logfilepath.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IncludeModules(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetIncludeModules(&self, includemodules: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(includemodules)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IncludeApis(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetIncludeApis(&self, includeapis: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(includeapis)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExcludeApis(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetExcludeApis(&self, excludeapis: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(excludeapis)).ok()
    }
}
impl ::core::convert::From<IApiTracingDataCollector> for IDataCollector {
    fn from(value: IApiTracingDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IApiTracingDataCollector> for IDataCollector {
    fn from(value: &IApiTracingDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for IApiTracingDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for &IApiTracingDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IApiTracingDataCollector> for super::Com::IDispatch {
    fn from(value: IApiTracingDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IApiTracingDataCollector> for super::Com::IDispatch {
    fn from(value: &IApiTracingDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IApiTracingDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IApiTracingDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IApiTracingDataCollector> for ::windows::core::IUnknown {
    fn from(value: IApiTracingDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IApiTracingDataCollector> for ::windows::core::IUnknown {
    fn from(value: &IApiTracingDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IApiTracingDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IApiTracingDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IApiTracingDataCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IApiTracingDataCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApiTracingDataCollector {}
impl ::core::fmt::Debug for IApiTracingDataCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApiTracingDataCollector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IApiTracingDataCollector {
    type Vtable = IApiTracingDataCollectorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0383751a_098b_11d8_9414_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApiTracingDataCollectorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut DataCollectorType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *mut AutoPathFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: AutoPathFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, append: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, append: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, circular: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, circular: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, latest: i16, location: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logapinames: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logapinames: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logrecursively: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logrecursively: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, exepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, exepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logfilepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includemodules: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includemodules: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includeapis: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includeapis: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, excludeapis: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, excludeapis: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct IConfigurationDataCollector(::windows::core::IUnknown);
impl IConfigurationDataCollector {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataCollectorSet(&self) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollectorSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, group: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), group.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataCollectorType(&self) -> ::windows::core::Result<DataCollectorType> {
        let mut result__: DataCollectorType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataCollectorType>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn FileNameFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: AutoPathFormat = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AutoPathFormat>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNameFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNameFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogAppend(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogAppend(&self, append: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(append)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogCircular(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogCircular(&self, circular: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(circular)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogOverwrite(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetIndex(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), xml.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOutputLocation(&self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(latest), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn FileMaxCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetFileMaxCount(&self, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(count)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn FileMaxRecursiveDepth(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetFileMaxRecursiveDepth(&self, depth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(depth)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn FileMaxTotalSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetFileMaxTotalSize(&self, size: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(size)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Files(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFiles(&self, files: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(files)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ManagementQueries(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetManagementQueries(&self, queries: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(queries)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn QueryNetworkAdapters(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetQueryNetworkAdapters(&self, network: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(network)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegistryKeys(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRegistryKeys(&self, query: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(query)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn RegistryMaxRecursiveDepth(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetRegistryMaxRecursiveDepth(&self, depth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(depth)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SystemStateFile(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSystemStateFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), filename.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IConfigurationDataCollector> for IDataCollector {
    fn from(value: IConfigurationDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConfigurationDataCollector> for IDataCollector {
    fn from(value: &IConfigurationDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for IConfigurationDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for &IConfigurationDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IConfigurationDataCollector> for super::Com::IDispatch {
    fn from(value: IConfigurationDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IConfigurationDataCollector> for super::Com::IDispatch {
    fn from(value: &IConfigurationDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IConfigurationDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IConfigurationDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IConfigurationDataCollector> for ::windows::core::IUnknown {
    fn from(value: IConfigurationDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConfigurationDataCollector> for ::windows::core::IUnknown {
    fn from(value: &IConfigurationDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConfigurationDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IConfigurationDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConfigurationDataCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConfigurationDataCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConfigurationDataCollector {}
impl ::core::fmt::Debug for IConfigurationDataCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConfigurationDataCollector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConfigurationDataCollector {
    type Vtable = IConfigurationDataCollectorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837514_098b_11d8_9414_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConfigurationDataCollectorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut DataCollectorType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *mut AutoPathFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: AutoPathFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, append: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, append: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, circular: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, circular: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, latest: i16, location: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, depth: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, depth: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, files: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, files: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queries: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queries: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, network: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, network: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, depth: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, depth: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct ICounterItem(::windows::core::IUnknown);
impl ICounterItem {
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Value(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Color(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetWidth(&self, iwidth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(iwidth)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Width(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLineStyle(&self, ilinestyle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ilinestyle)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LineStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetScaleFactor(&self, iscale: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(iscale)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ScaleFactor(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn GetValue(&self, value: *mut f64, status: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn GetStatistics(&self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(max), ::core::mem::transmute(min), ::core::mem::transmute(avg), ::core::mem::transmute(status)).ok()
    }
}
impl ::core::convert::From<ICounterItem> for ::windows::core::IUnknown {
    fn from(value: ICounterItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICounterItem> for ::windows::core::IUnknown {
    fn from(value: &ICounterItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICounterItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICounterItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICounterItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICounterItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICounterItem {}
impl ::core::fmt::Debug for ICounterItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICounterItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICounterItem {
    type Vtable = ICounterItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x771a9520_ee28_11ce_941e_008029004347);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICounterItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdblvalue: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iwidth: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ilinestyle: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iscale: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64, status: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct ICounterItem2(::windows::core::IUnknown);
impl ICounterItem2 {
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Value(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Color(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetWidth(&self, iwidth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(iwidth)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Width(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLineStyle(&self, ilinestyle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ilinestyle)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LineStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetScaleFactor(&self, iscale: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(iscale)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ScaleFactor(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn GetValue(&self, value: *mut f64, status: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn GetStatistics(&self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(max), ::core::mem::transmute(min), ::core::mem::transmute(avg), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetSelected(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Selected(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetVisible(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Visible(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetDataAt(&self, iindex: i32, iwhich: SysmonDataType) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), ::core::mem::transmute(iwhich), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<ICounterItem2> for ICounterItem {
    fn from(value: ICounterItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICounterItem2> for ICounterItem {
    fn from(value: &ICounterItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICounterItem> for ICounterItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ICounterItem> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICounterItem> for &ICounterItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ICounterItem> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICounterItem2> for ::windows::core::IUnknown {
    fn from(value: ICounterItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICounterItem2> for ::windows::core::IUnknown {
    fn from(value: &ICounterItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICounterItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICounterItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICounterItem2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICounterItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICounterItem2 {}
impl ::core::fmt::Debug for ICounterItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICounterItem2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICounterItem2 {
    type Vtable = ICounterItem2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeefcd4e1_ea1c_4435_b7f4_e341ba03b4f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICounterItem2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdblvalue: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iwidth: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ilinestyle: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iscale: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64, status: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: i32, iwhich: SysmonDataType, pvariant: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct ICounters(::windows::core::IUnknown);
impl ICounters {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<DICounterItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), index.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<DICounterItem>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pathname: Param0) -> ::windows::core::Result<DICounterItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pathname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<DICounterItem>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), index.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICounters> for super::Com::IDispatch {
    fn from(value: ICounters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICounters> for super::Com::IDispatch {
    fn from(value: &ICounters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ICounters {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ICounters {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICounters> for ::windows::core::IUnknown {
    fn from(value: ICounters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICounters> for ::windows::core::IUnknown {
    fn from(value: &ICounters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICounters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICounters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICounters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICounters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICounters {}
impl ::core::fmt::Debug for ICounters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICounters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICounters {
    type Vtable = ICountersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79167962_28fc_11cf_942f_008029004347);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICountersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plong: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct IDataCollector(::windows::core::IUnknown);
impl IDataCollector {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataCollectorSet(&self) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollectorSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, group: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), group.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataCollectorType(&self) -> ::windows::core::Result<DataCollectorType> {
        let mut result__: DataCollectorType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataCollectorType>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn FileNameFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: AutoPathFormat = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AutoPathFormat>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNameFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNameFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogAppend(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogAppend(&self, append: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(append)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogCircular(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogCircular(&self, circular: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(circular)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogOverwrite(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetIndex(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), xml.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOutputLocation(&self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(latest), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataCollector> for super::Com::IDispatch {
    fn from(value: IDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataCollector> for super::Com::IDispatch {
    fn from(value: &IDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDataCollector> for ::windows::core::IUnknown {
    fn from(value: IDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDataCollector> for ::windows::core::IUnknown {
    fn from(value: &IDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDataCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataCollector {}
impl ::core::fmt::Debug for IDataCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataCollector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDataCollector {
    type Vtable = IDataCollectorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x038374ff_098b_11d8_9414_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollectorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut DataCollectorType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *mut AutoPathFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: AutoPathFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, append: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, append: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, circular: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, circular: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, latest: i16, location: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct IDataCollectorCollection(::windows::core::IUnknown);
impl IDataCollectorCollection {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<IDataCollector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), index.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollector>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, IDataCollector>>(&self, collector: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), collector.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, collector: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), collector.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn AddRange<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorCollection>>(&self, collectors: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), collectors.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDataCollectorFromXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrxml: Param0, pvalidation: *mut ::core::option::Option<IValueMap>, pcollector: *mut ::core::option::Option<IDataCollector>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), bstrxml.into_param().abi(), ::core::mem::transmute(pvalidation), ::core::mem::transmute(pcollector)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn CreateDataCollector(&self, r#type: DataCollectorType) -> ::windows::core::Result<IDataCollector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollector>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataCollectorCollection> for super::Com::IDispatch {
    fn from(value: IDataCollectorCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataCollectorCollection> for super::Com::IDispatch {
    fn from(value: &IDataCollectorCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IDataCollectorCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IDataCollectorCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDataCollectorCollection> for ::windows::core::IUnknown {
    fn from(value: IDataCollectorCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDataCollectorCollection> for ::windows::core::IUnknown {
    fn from(value: &IDataCollectorCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataCollectorCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDataCollectorCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDataCollectorCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataCollectorCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataCollectorCollection {}
impl ::core::fmt::Debug for IDataCollectorCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataCollectorCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDataCollectorCollection {
    type Vtable = IDataCollectorCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837502_098b_11d8_9414_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollectorCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, collector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collector: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collector: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collectors: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvalidation: *mut ::windows::core::RawPtr, pcollector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: DataCollectorType, collector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct IDataCollectorSet(::windows::core::IUnknown);
impl IDataCollectorSet {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataCollectors(&self) -> ::windows::core::Result<IDataCollectorCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollectorCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Duration(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDuration(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DescriptionUnresolved(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, displayname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), displayname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayNameUnresolved(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Keywords(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetKeywords(&self, keywords: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(keywords)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RootPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRootPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, folder: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), folder.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Segment(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetSegment(&self, segment: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(segment)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SegmentMaxDuration(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetSegmentMaxDuration(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SegmentMaxSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetSegmentMaxSize(&self, size: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(size)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SerialNumber(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetSerialNumber(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Server(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<DataCollectorSetStatus> {
        let mut result__: DataCollectorSetStatus = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataCollectorSetStatus>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subdirectory(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSubdirectory<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, folder: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), folder.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SubdirectoryFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: AutoPathFormat = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AutoPathFormat>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetSubdirectoryFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubdirectoryFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSubdirectoryFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Task(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, task: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), task.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn TaskRunAsSelf(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetTaskRunAsSelf(&self, runasself: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(runasself)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskArguments(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTaskArguments<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, task: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), task.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskUserTextArguments(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTaskUserTextArguments<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, usertext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), usertext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Schedules(&self) -> ::windows::core::Result<IScheduleCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IScheduleCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SchedulesEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetSchedulesEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Security(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSecurity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsecurity: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), bstrsecurity.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn StopOnCompletion(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetStopOnCompletion(&self, stop: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(stop)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataManager(&self) -> ::windows::core::Result<IDataManager> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDataManager>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentials<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, user: Param0, password: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), user.into_param().abi(), password.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Query<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, server: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), name.into_param().abi(), server.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Commit<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, server: Param1, mode: CommitMode) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), name.into_param().abi(), server.into_param().abi(), ::core::mem::transmute(mode), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Start(&self, synchronous: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self), ::core::mem::transmute(synchronous)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Stop(&self, synchronous: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self), ::core::mem::transmute(synchronous)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), xml.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, key: Param0, value: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), key.into_param().abi(), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, key: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), key.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataCollectorSet> for super::Com::IDispatch {
    fn from(value: IDataCollectorSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataCollectorSet> for super::Com::IDispatch {
    fn from(value: &IDataCollectorSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IDataCollectorSet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IDataCollectorSet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDataCollectorSet> for ::windows::core::IUnknown {
    fn from(value: IDataCollectorSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDataCollectorSet> for ::windows::core::IUnknown {
    fn from(value: &IDataCollectorSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataCollectorSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDataCollectorSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDataCollectorSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataCollectorSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataCollectorSet {}
impl ::core::fmt::Debug for IDataCollectorSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataCollectorSet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDataCollectorSet {
    type Vtable = IDataCollectorSetVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837520_098b_11d8_9414_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollectorSetVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collectors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segment: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segment: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, server: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut DataCollectorSetStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *mut AutoPathFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: AutoPathFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runasself: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runasself: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usertext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usertext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppschedules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsecurity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsecurity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stop: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stop: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datamanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mode: CommitMode, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, synchronous: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, synchronous: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct IDataCollectorSetCollection(::windows::core::IUnknown);
impl IDataCollectorSetCollection {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), index.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollectorSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, set: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), set.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, set: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), set.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn AddRange<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSetCollection>>(&self, sets: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), sets.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDataCollectorSets<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, server: Param0, filter: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), server.into_param().abi(), filter.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataCollectorSetCollection> for super::Com::IDispatch {
    fn from(value: IDataCollectorSetCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataCollectorSetCollection> for super::Com::IDispatch {
    fn from(value: &IDataCollectorSetCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IDataCollectorSetCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IDataCollectorSetCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDataCollectorSetCollection> for ::windows::core::IUnknown {
    fn from(value: IDataCollectorSetCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDataCollectorSetCollection> for ::windows::core::IUnknown {
    fn from(value: &IDataCollectorSetCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataCollectorSetCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDataCollectorSetCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDataCollectorSetCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataCollectorSetCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataCollectorSetCollection {}
impl ::core::fmt::Debug for IDataCollectorSetCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataCollectorSetCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDataCollectorSetCollection {
    type Vtable = IDataCollectorSetCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837524_098b_11d8_9414_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollectorSetCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, set: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, set: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, set: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sets: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct IDataManager(::windows::core::IUnknown);
impl IDataManager {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetEnabled(&self, fenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(fenabled)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn CheckBeforeRunning(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetCheckBeforeRunning(&self, fcheck: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(fcheck)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn MinFreeDisk(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetMinFreeDisk(&self, minfreedisk: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(minfreedisk)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn MaxSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetMaxSize(&self, ulmaxsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulmaxsize)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn MaxFolderCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetMaxFolderCount(&self, ulmaxfoldercount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulmaxfoldercount)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ResourcePolicy(&self) -> ::windows::core::Result<ResourcePolicy> {
        let mut result__: ResourcePolicy = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ResourcePolicy>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetResourcePolicy(&self, policy: ResourcePolicy) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(policy)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn FolderActions(&self) -> ::windows::core::Result<IFolderActionCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFolderActionCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportSchema(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReportSchema<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, reportschema: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), reportschema.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReportFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pbstrfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), pbstrfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RuleTargetFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRuleTargetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), filename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EventsFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventsFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pbstrfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), pbstrfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Rules(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRules<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrxml: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), bstrxml.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Run<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, steps: DataManagerSteps, bstrfolder: Param1) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(steps), bstrfolder.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Extract<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, cabfilename: Param0, destinationpath: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), cabfilename.into_param().abi(), destinationpath.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataManager> for super::Com::IDispatch {
    fn from(value: IDataManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataManager> for super::Com::IDispatch {
    fn from(value: &IDataManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IDataManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IDataManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDataManager> for ::windows::core::IUnknown {
    fn from(value: IDataManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDataManager> for ::windows::core::IUnknown {
    fn from(value: &IDataManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDataManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDataManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataManager {}
impl ::core::fmt::Debug for IDataManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDataManager {
    type Vtable = IDataManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837541_098b_11d8_9414_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcheck: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fcheck: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minfreedisk: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minfreedisk: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulmaxsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulmaxsize: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulmaxfoldercount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulmaxfoldercount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppolicy: *mut ResourcePolicy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: ResourcePolicy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportschema: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportschema: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, steps: DataManagerSteps, bstrfolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, errors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cabfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, destinationpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct IFolderAction(::windows::core::IUnknown);
impl IFolderAction {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Age(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetAge(&self, ulage: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulage)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Size(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetSize(&self, ulage: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulage)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Actions(&self) -> ::windows::core::Result<FolderActionSteps> {
        let mut result__: FolderActionSteps = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FolderActionSteps>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetActions(&self, steps: FolderActionSteps) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(steps)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendCabTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSendCabTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdestination: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), bstrdestination.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFolderAction> for super::Com::IDispatch {
    fn from(value: IFolderAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFolderAction> for super::Com::IDispatch {
    fn from(value: &IFolderAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IFolderAction {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IFolderAction {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFolderAction> for ::windows::core::IUnknown {
    fn from(value: IFolderAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFolderAction> for ::windows::core::IUnknown {
    fn from(value: &IFolderAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFolderAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFolderAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFolderAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFolderAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFolderAction {}
impl ::core::fmt::Debug for IFolderAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFolderAction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFolderAction {
    type Vtable = IFolderActionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837543_098b_11d8_9414_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderActionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulage: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulage: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulage: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulage: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, steps: *mut FolderActionSteps) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, steps: FolderActionSteps) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdestination: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdestination: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct IFolderActionCollection(::windows::core::IUnknown);
impl IFolderActionCollection {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<IFolderAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), index.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFolderAction>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, IFolderAction>>(&self, action: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), action.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), index.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn AddRange<'a, Param0: ::windows::core::IntoParam<'a, IFolderActionCollection>>(&self, actions: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), actions.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn CreateFolderAction(&self) -> ::windows::core::Result<IFolderAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFolderAction>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFolderActionCollection> for super::Com::IDispatch {
    fn from(value: IFolderActionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFolderActionCollection> for super::Com::IDispatch {
    fn from(value: &IFolderActionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IFolderActionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IFolderActionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFolderActionCollection> for ::windows::core::IUnknown {
    fn from(value: IFolderActionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFolderActionCollection> for ::windows::core::IUnknown {
    fn from(value: &IFolderActionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFolderActionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFolderActionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFolderActionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFolderActionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFolderActionCollection {}
impl ::core::fmt::Debug for IFolderActionCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFolderActionCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFolderActionCollection {
    type Vtable = IFolderActionCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837544_098b_11d8_9414_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderActionCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#enum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actions: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct ILogFileItem(::windows::core::IUnknown);
impl ILogFileItem {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ILogFileItem> for ::windows::core::IUnknown {
    fn from(value: ILogFileItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILogFileItem> for ::windows::core::IUnknown {
    fn from(value: &ILogFileItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILogFileItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ILogFileItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILogFileItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILogFileItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILogFileItem {}
impl ::core::fmt::Debug for ILogFileItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILogFileItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILogFileItem {
    type Vtable = ILogFileItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6b518dd_05c7_418a_89e6_4f9ce8c6841e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILogFileItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct ILogFiles(::windows::core::IUnknown);
impl ILogFiles {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<DILogFileItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), index.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<DILogFileItem>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pathname: Param0) -> ::windows::core::Result<DILogFileItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pathname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<DILogFileItem>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), index.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ILogFiles> for super::Com::IDispatch {
    fn from(value: ILogFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ILogFiles> for super::Com::IDispatch {
    fn from(value: &ILogFiles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ILogFiles {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ILogFiles {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILogFiles> for ::windows::core::IUnknown {
    fn from(value: ILogFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILogFiles> for ::windows::core::IUnknown {
    fn from(value: &ILogFiles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILogFiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ILogFiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILogFiles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILogFiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILogFiles {}
impl ::core::fmt::Debug for ILogFiles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILogFiles").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILogFiles {
    type Vtable = ILogFilesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a2a97e6_6851_41ea_87ad_2a8225335865);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILogFilesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plong: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct IPerformanceCounterDataCollector(::windows::core::IUnknown);
impl IPerformanceCounterDataCollector {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataCollectorSet(&self) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollectorSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, group: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), group.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataCollectorType(&self) -> ::windows::core::Result<DataCollectorType> {
        let mut result__: DataCollectorType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataCollectorType>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn FileNameFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: AutoPathFormat = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AutoPathFormat>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNameFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNameFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogAppend(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogAppend(&self, append: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(append)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogCircular(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogCircular(&self, circular: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(circular)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogOverwrite(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetIndex(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), xml.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOutputLocation(&self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(latest), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DataSourceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDataSourceName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, dsn: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), dsn.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PerformanceCounters(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPerformanceCounters(&self, counters: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(counters)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogFileFormat(&self) -> ::windows::core::Result<FileFormat> {
        let mut result__: FileFormat = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FileFormat>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogFileFormat(&self, format: FileFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SampleInterval(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetSampleInterval(&self, interval: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(interval)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SegmentMaxRecords(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetSegmentMaxRecords(&self, records: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(records)).ok()
    }
}
impl ::core::convert::From<IPerformanceCounterDataCollector> for IDataCollector {
    fn from(value: IPerformanceCounterDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPerformanceCounterDataCollector> for IDataCollector {
    fn from(value: &IPerformanceCounterDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for IPerformanceCounterDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for &IPerformanceCounterDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPerformanceCounterDataCollector> for super::Com::IDispatch {
    fn from(value: IPerformanceCounterDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPerformanceCounterDataCollector> for super::Com::IDispatch {
    fn from(value: &IPerformanceCounterDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IPerformanceCounterDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IPerformanceCounterDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPerformanceCounterDataCollector> for ::windows::core::IUnknown {
    fn from(value: IPerformanceCounterDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPerformanceCounterDataCollector> for ::windows::core::IUnknown {
    fn from(value: &IPerformanceCounterDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPerformanceCounterDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPerformanceCounterDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPerformanceCounterDataCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPerformanceCounterDataCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPerformanceCounterDataCollector {}
impl ::core::fmt::Debug for IPerformanceCounterDataCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPerformanceCounterDataCollector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPerformanceCounterDataCollector {
    type Vtable = IPerformanceCounterDataCollectorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837506_098b_11d8_9414_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerformanceCounterDataCollectorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut DataCollectorType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *mut AutoPathFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: AutoPathFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, append: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, append: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, circular: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, circular: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, latest: i16, location: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dsn: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, counters: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, counters: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *mut FileFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: FileFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interval: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interval: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, records: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, records: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct ISchedule(::windows::core::IUnknown);
impl ISchedule {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn StartDate(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetStartDate<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, start: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), start.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EndDate(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetEndDate<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, end: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), end.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn StartTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, start: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), start.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Days(&self) -> ::windows::core::Result<WeekDays> {
        let mut result__: WeekDays = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WeekDays>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDays(&self, days: WeekDays) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(days)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISchedule> for super::Com::IDispatch {
    fn from(value: ISchedule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISchedule> for super::Com::IDispatch {
    fn from(value: &ISchedule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISchedule {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISchedule {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISchedule> for ::windows::core::IUnknown {
    fn from(value: ISchedule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISchedule> for ::windows::core::IUnknown {
    fn from(value: &ISchedule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISchedule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISchedule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISchedule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISchedule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISchedule {}
impl ::core::fmt::Debug for ISchedule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchedule").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISchedule {
    type Vtable = IScheduleVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0383753a_098b_11d8_9414_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduleVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, end: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, end: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: *mut WeekDays) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: WeekDays) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct IScheduleCollection(::windows::core::IUnknown);
impl IScheduleCollection {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<ISchedule> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), index.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISchedule>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, ISchedule>>(&self, pschedule: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pschedule.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, vschedule: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), vschedule.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn AddRange<'a, Param0: ::windows::core::IntoParam<'a, IScheduleCollection>>(&self, pschedules: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pschedules.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn CreateSchedule(&self) -> ::windows::core::Result<ISchedule> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISchedule>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IScheduleCollection> for super::Com::IDispatch {
    fn from(value: IScheduleCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IScheduleCollection> for super::Com::IDispatch {
    fn from(value: &IScheduleCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IScheduleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IScheduleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IScheduleCollection> for ::windows::core::IUnknown {
    fn from(value: IScheduleCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IScheduleCollection> for ::windows::core::IUnknown {
    fn from(value: &IScheduleCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IScheduleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IScheduleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IScheduleCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IScheduleCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScheduleCollection {}
impl ::core::fmt::Debug for IScheduleCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScheduleCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IScheduleCollection {
    type Vtable = IScheduleCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0383753d_098b_11d8_9414_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduleCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppschedule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ienum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pschedule: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vschedule: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pschedules: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, schedule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct ISystemMonitor(::windows::core::IUnknown);
impl ISystemMonitor {
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Appearance(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetAppearance(&self, iappearance: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(iappearance)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn BackColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetBackColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn BorderStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetBorderStyle(&self, iborderstyle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(iborderstyle)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ForeColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetForeColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Ole'*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn Font(&self) -> ::windows::core::Result<super::Ole::IFontDisp> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Ole::IFontDisp>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Ole'*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn putref_Font<'a, Param0: ::windows::core::IntoParam<'a, super::Ole::IFontDisp>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Counters(&self) -> ::windows::core::Result<ICounters> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ICounters>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowVerticalGrid(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowVerticalGrid(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowHorizontalGrid(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowHorizontalGrid(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowLegend(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowLegend(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowScaleLabels(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowScaleLabels(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowValueBar(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowValueBar(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetMaximumScale(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn MaximumScale(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetMinimumScale(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn MinimumScale(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetUpdateInterval(&self, fvalue: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(fvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn UpdateInterval(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDisplayType(&self, edisplaytype: DisplayTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(edisplaytype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DisplayType(&self) -> ::windows::core::Result<DisplayTypeConstants> {
        let mut result__: DisplayTypeConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DisplayTypeConstants>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetManualUpdate(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ManualUpdate(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGraphTitle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), bstitle.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GraphTitle(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetYAxisLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), bstitle.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn YAxisLabel(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn CollectSample(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn UpdateGraph(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn BrowseCounters(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DisplayProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Counter(&self, iindex: i32) -> ::windows::core::Result<ICounterItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), ::core::mem::transmute(&mut result__)).from_abi::<ICounterItem>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddCounter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bspath: Param0) -> ::windows::core::Result<ICounterItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), bspath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ICounterItem>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DeleteCounter<'a, Param0: ::windows::core::IntoParam<'a, ICounterItem>>(&self, pctr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), pctr.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn BackColorCtl(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetBackColorCtl(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bsfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), bsfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogViewStart(&self, starttime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogViewStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogViewStop(&self, stoptime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(stoptime)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogViewStop(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn GridColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetGridColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn TimeBarColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetTimeBarColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Highlight(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetHighlight(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowToolbar(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowToolbar(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Paste(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Copy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetReadOnly(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ReadOnly(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetReportValueType(&self, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(ereportvaluetype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ReportValueType(&self) -> ::windows::core::Result<ReportValueTypeConstants> {
        let mut result__: ReportValueTypeConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ReportValueTypeConstants>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetMonitorDuplicateInstances(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn MonitorDuplicateInstances(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDisplayFilter(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DisplayFilter(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).71)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogFiles(&self) -> ::windows::core::Result<ILogFiles> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).72)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ILogFiles>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDataSourceType(&self, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).73)(::core::mem::transmute_copy(self), ::core::mem::transmute(edatasourcetype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataSourceType(&self) -> ::windows::core::Result<DataSourceTypeConstants> {
        let mut result__: DataSourceTypeConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).74)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataSourceTypeConstants>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSqlDsnName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bssqldsnname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).75)(::core::mem::transmute_copy(self), bssqldsnname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqlDsnName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).76)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSqlLogSetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bssqllogsetname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).77)(::core::mem::transmute_copy(self), bssqllogsetname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqlLogSetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).78)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ISystemMonitor> for ::windows::core::IUnknown {
    fn from(value: ISystemMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemMonitor> for ::windows::core::IUnknown {
    fn from(value: &ISystemMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISystemMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISystemMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISystemMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISystemMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISystemMonitor {}
impl ::core::fmt::Debug for ISystemMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISystemMonitor {
    type Vtable = ISystemMonitorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x194eb241_c32c_11cf_9398_00aa00a3ddea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMonitorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iappearance: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iappearance: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iborderstyle: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iborderstyle: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppicounters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fvalue: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, edisplaytype: DisplayTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pedisplaytype: *mut DisplayTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: i32, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctr: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stoptime: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stoptime: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pereportvaluetype: *mut ReportValueTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppilogfiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pedatasourcetype: *mut DataSourceTypeConstants) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqldsnname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqldsnname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqllogsetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqllogsetname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct ISystemMonitor2(::windows::core::IUnknown);
impl ISystemMonitor2 {
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Appearance(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetAppearance(&self, iappearance: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(iappearance)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn BackColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetBackColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn BorderStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetBorderStyle(&self, iborderstyle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(iborderstyle)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ForeColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetForeColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Ole'*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn Font(&self) -> ::windows::core::Result<super::Ole::IFontDisp> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Ole::IFontDisp>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Ole'*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn putref_Font<'a, Param0: ::windows::core::IntoParam<'a, super::Ole::IFontDisp>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Counters(&self) -> ::windows::core::Result<ICounters> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ICounters>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowVerticalGrid(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowVerticalGrid(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowHorizontalGrid(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowHorizontalGrid(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowLegend(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowLegend(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowScaleLabels(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowScaleLabels(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowValueBar(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowValueBar(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetMaximumScale(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn MaximumScale(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetMinimumScale(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn MinimumScale(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetUpdateInterval(&self, fvalue: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(fvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn UpdateInterval(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDisplayType(&self, edisplaytype: DisplayTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(edisplaytype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DisplayType(&self) -> ::windows::core::Result<DisplayTypeConstants> {
        let mut result__: DisplayTypeConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DisplayTypeConstants>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetManualUpdate(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ManualUpdate(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGraphTitle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), bstitle.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GraphTitle(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetYAxisLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), bstitle.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn YAxisLabel(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn CollectSample(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn UpdateGraph(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn BrowseCounters(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DisplayProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Counter(&self, iindex: i32) -> ::windows::core::Result<ICounterItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), ::core::mem::transmute(&mut result__)).from_abi::<ICounterItem>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddCounter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bspath: Param0) -> ::windows::core::Result<ICounterItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), bspath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ICounterItem>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DeleteCounter<'a, Param0: ::windows::core::IntoParam<'a, ICounterItem>>(&self, pctr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), pctr.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn BackColorCtl(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetBackColorCtl(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bsfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), bsfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogViewStart(&self, starttime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogViewStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogViewStop(&self, stoptime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(stoptime)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogViewStop(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn GridColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetGridColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn TimeBarColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetTimeBarColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Highlight(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetHighlight(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowToolbar(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowToolbar(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Paste(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Copy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetReadOnly(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ReadOnly(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetReportValueType(&self, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(ereportvaluetype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ReportValueType(&self) -> ::windows::core::Result<ReportValueTypeConstants> {
        let mut result__: ReportValueTypeConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ReportValueTypeConstants>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetMonitorDuplicateInstances(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn MonitorDuplicateInstances(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDisplayFilter(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DisplayFilter(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).71)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogFiles(&self) -> ::windows::core::Result<ILogFiles> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).72)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ILogFiles>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDataSourceType(&self, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).73)(::core::mem::transmute_copy(self), ::core::mem::transmute(edatasourcetype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataSourceType(&self) -> ::windows::core::Result<DataSourceTypeConstants> {
        let mut result__: DataSourceTypeConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).74)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataSourceTypeConstants>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSqlDsnName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bssqldsnname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).75)(::core::mem::transmute_copy(self), bssqldsnname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqlDsnName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).76)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSqlLogSetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bssqllogsetname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).77)(::core::mem::transmute_copy(self), bssqllogsetname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqlLogSetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).78)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetEnableDigitGrouping(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).79)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn EnableDigitGrouping(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).80)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetEnableToolTips(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).81)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn EnableToolTips(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).82)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowTimeAxisLabels(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).83)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowTimeAxisLabels(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).84)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetChartScroll(&self, bscroll: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).85)(::core::mem::transmute_copy(self), ::core::mem::transmute(bscroll)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ChartScroll(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).86)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDataPointCount(&self, inewcount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).87)(::core::mem::transmute_copy(self), ::core::mem::transmute(inewcount)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataPointCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).88)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ScaleToFit(&self, bselectedcountersonly: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).89)(::core::mem::transmute_copy(self), ::core::mem::transmute(bselectedcountersonly)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveAs<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfilename: Param0, esysmonfiletype: SysmonFileType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).90)(::core::mem::transmute_copy(self), bstrfilename.into_param().abi(), ::core::mem::transmute(esysmonfiletype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Relog<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfilename: Param0, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).91)(::core::mem::transmute_copy(self), bstrfilename.into_param().abi(), ::core::mem::transmute(esysmonfiletype), ::core::mem::transmute(ifilter)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ClearData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).92)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogSourceStartTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).93)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogSourceStopTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).94)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogViewRange(&self, starttime: f64, stoptime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).95)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime), ::core::mem::transmute(stoptime)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn GetLogViewRange(&self, starttime: *mut f64, stoptime: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).96)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime), ::core::mem::transmute(stoptime)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn BatchingLock(&self, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).97)(::core::mem::transmute_copy(self), ::core::mem::transmute(flock), ::core::mem::transmute(ebatchreason)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadSettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsettingfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).98)(::core::mem::transmute_copy(self), bstrsettingfilename.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISystemMonitor2> for ISystemMonitor {
    fn from(value: ISystemMonitor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemMonitor2> for ISystemMonitor {
    fn from(value: &ISystemMonitor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISystemMonitor> for ISystemMonitor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISystemMonitor> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISystemMonitor> for &ISystemMonitor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISystemMonitor> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISystemMonitor2> for ::windows::core::IUnknown {
    fn from(value: ISystemMonitor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemMonitor2> for ::windows::core::IUnknown {
    fn from(value: &ISystemMonitor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISystemMonitor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISystemMonitor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISystemMonitor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISystemMonitor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISystemMonitor2 {}
impl ::core::fmt::Debug for ISystemMonitor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemMonitor2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISystemMonitor2 {
    type Vtable = ISystemMonitor2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08e3206a_5fd2_4fde_a8a5_8cb3b63d2677);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMonitor2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iappearance: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iappearance: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iborderstyle: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iborderstyle: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppicounters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fvalue: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, edisplaytype: DisplayTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pedisplaytype: *mut DisplayTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: i32, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctr: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stoptime: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stoptime: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pereportvaluetype: *mut ReportValueTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppilogfiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pedatasourcetype: *mut DataSourceTypeConstants) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqldsnname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqldsnname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqllogsetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqllogsetname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bscroll: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbscroll: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inewcount: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidatapointcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bselectedcountersonly: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: f64, stoptime: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: *mut f64, stoptime: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsettingfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct ISystemMonitorEvents(::windows::core::IUnknown);
impl ISystemMonitorEvents {
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn OnCounterSelected(&self, index: i32) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index))
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn OnCounterAdded(&self, index: i32) {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index))
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn OnCounterDeleted(&self, index: i32) {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index))
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn OnSampleCollected(&self) {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn OnDblClick(&self, index: i32) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index))
    }
}
impl ::core::convert::From<ISystemMonitorEvents> for ::windows::core::IUnknown {
    fn from(value: ISystemMonitorEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemMonitorEvents> for ::windows::core::IUnknown {
    fn from(value: &ISystemMonitorEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISystemMonitorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISystemMonitorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISystemMonitorEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISystemMonitorEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISystemMonitorEvents {}
impl ::core::fmt::Debug for ISystemMonitorEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemMonitorEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISystemMonitorEvents {
    type Vtable = ISystemMonitorEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee660ea0_4abd_11cf_943a_008029004347);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMonitorEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32),
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct ITraceDataCollector(::windows::core::IUnknown);
impl ITraceDataCollector {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataCollectorSet(&self) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollectorSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, group: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), group.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataCollectorType(&self) -> ::windows::core::Result<DataCollectorType> {
        let mut result__: DataCollectorType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataCollectorType>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn FileNameFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: AutoPathFormat = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AutoPathFormat>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNameFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNameFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogAppend(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogAppend(&self, append: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(append)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogCircular(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogCircular(&self, circular: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(circular)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogOverwrite(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetIndex(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), xml.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOutputLocation(&self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(latest), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn BufferSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetBufferSize(&self, size: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(size)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn BuffersLost(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetBuffersLost(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn BuffersWritten(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetBuffersWritten(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ClockType(&self) -> ::windows::core::Result<ClockType> {
        let mut result__: ClockType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ClockType>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetClockType(&self, clock: ClockType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(clock)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn EventsLost(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetEventsLost(&self, events: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(events)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ExtendedModes(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetExtendedModes(&self, mode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn FlushTimer(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetFlushTimer(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn FreeBuffers(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetFreeBuffers(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Guid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetGuid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), guid.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn IsKernelTrace(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn MaximumBuffers(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetMaximumBuffers(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn MinimumBuffers(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetMinimumBuffers(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn NumberOfBuffers(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetNumberOfBuffers(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn PreallocateFile(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetPreallocateFile(&self, allocate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(allocate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ProcessMode(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetProcessMode(&self, process: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(process)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn RealTimeBuffersLost(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetRealTimeBuffersLost(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SessionId(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetSessionId(&self, id: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SessionName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSessionName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SessionThreadId(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetSessionThreadId(&self, tid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn StreamMode(&self) -> ::windows::core::Result<StreamMode> {
        let mut result__: StreamMode = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<StreamMode>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetStreamMode(&self, mode: StreamMode) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn TraceDataProviders(&self) -> ::windows::core::Result<ITraceDataProviderCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).71)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITraceDataProviderCollection>(result__)
    }
}
impl ::core::convert::From<ITraceDataCollector> for IDataCollector {
    fn from(value: ITraceDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITraceDataCollector> for IDataCollector {
    fn from(value: &ITraceDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for ITraceDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for &ITraceDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITraceDataCollector> for super::Com::IDispatch {
    fn from(value: ITraceDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITraceDataCollector> for super::Com::IDispatch {
    fn from(value: &ITraceDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ITraceDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ITraceDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITraceDataCollector> for ::windows::core::IUnknown {
    fn from(value: ITraceDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITraceDataCollector> for ::windows::core::IUnknown {
    fn from(value: &ITraceDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITraceDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITraceDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITraceDataCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITraceDataCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITraceDataCollector {}
impl ::core::fmt::Debug for ITraceDataCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITraceDataCollector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITraceDataCollector {
    type Vtable = ITraceDataCollectorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0383750b_098b_11d8_9414_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITraceDataCollectorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut DataCollectorType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *mut AutoPathFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: AutoPathFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, append: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, append: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, circular: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, circular: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, latest: i16, location: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clock: *mut ClockType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clock: ClockType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, events: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, events: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kernel: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allocate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allocate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, process: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, process: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut StreamMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: StreamMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct ITraceDataProvider(::windows::core::IUnknown);
impl ITraceDataProvider {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Guid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetGuid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), guid.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Level(&self) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn KeywordsAny(&self) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn KeywordsAll(&self) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn FilterEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetFilterEnabled(&self, filterenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(filterenabled)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn FilterType(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetFilterType(&self, ultype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(ultype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FilterData(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFilterData(&self, pdata: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Query<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, bstrserver: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), bstrname.into_param().abi(), bstrserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Resolve<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, pfrom: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), pfrom.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSecurity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, sddl: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), sddl.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSecurity(&self, securityinfo: u32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(securityinfo), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn GetRegisteredProcesses(&self) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITraceDataProvider> for super::Com::IDispatch {
    fn from(value: ITraceDataProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITraceDataProvider> for super::Com::IDispatch {
    fn from(value: &ITraceDataProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ITraceDataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ITraceDataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITraceDataProvider> for ::windows::core::IUnknown {
    fn from(value: ITraceDataProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITraceDataProvider> for ::windows::core::IUnknown {
    fn from(value: &ITraceDataProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITraceDataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITraceDataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITraceDataProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITraceDataProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITraceDataProvider {}
impl ::core::fmt::Debug for ITraceDataProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITraceDataProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITraceDataProvider {
    type Vtable = ITraceDataProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837512_098b_11d8_9414_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITraceDataProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplevel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppkeywords: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppkeywords: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filterenabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filterenabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pultype: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ultype: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdata: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrom: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, securityinfo: u32, sddl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct ITraceDataProviderCollection(::windows::core::IUnknown);
impl ITraceDataProviderCollection {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<ITraceDataProvider> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), index.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITraceDataProvider>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, ITraceDataProvider>>(&self, pprovider: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pprovider.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, vprovider: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), vprovider.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn AddRange<'a, Param0: ::windows::core::IntoParam<'a, ITraceDataProviderCollection>>(&self, providers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), providers.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn CreateTraceDataProvider(&self) -> ::windows::core::Result<ITraceDataProvider> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITraceDataProvider>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTraceDataProviders<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, server: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), server.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTraceDataProvidersByProcess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, server: Param0, pid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), server.into_param().abi(), ::core::mem::transmute(pid)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITraceDataProviderCollection> for super::Com::IDispatch {
    fn from(value: ITraceDataProviderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITraceDataProviderCollection> for super::Com::IDispatch {
    fn from(value: &ITraceDataProviderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ITraceDataProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ITraceDataProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITraceDataProviderCollection> for ::windows::core::IUnknown {
    fn from(value: ITraceDataProviderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITraceDataProviderCollection> for ::windows::core::IUnknown {
    fn from(value: &ITraceDataProviderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITraceDataProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITraceDataProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITraceDataProviderCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITraceDataProviderCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITraceDataProviderCollection {}
impl ::core::fmt::Debug for ITraceDataProviderCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITraceDataProviderCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITraceDataProviderCollection {
    type Vtable = ITraceDataProviderCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837510_098b_11d8_9414_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITraceDataProviderCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vprovider: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providers: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct IValueMap(::windows::core::IUnknown);
impl IValueMap {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<IValueMapItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), index.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IValueMapItem>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ValueMapType(&self) -> ::windows::core::Result<ValueMapType> {
        let mut result__: ValueMapType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ValueMapType>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetValueMapType(&self, r#type: ValueMapType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn AddRange<'a, Param0: ::windows::core::IntoParam<'a, IValueMap>>(&self, map: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), map.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn CreateValueMapItem(&self) -> ::windows::core::Result<IValueMapItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IValueMapItem>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IValueMap> for super::Com::IDispatch {
    fn from(value: IValueMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IValueMap> for super::Com::IDispatch {
    fn from(value: &IValueMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IValueMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IValueMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IValueMap> for ::windows::core::IUnknown {
    fn from(value: IValueMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IValueMap> for ::windows::core::IUnknown {
    fn from(value: &IValueMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IValueMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IValueMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IValueMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IValueMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IValueMap {}
impl ::core::fmt::Debug for IValueMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IValueMap").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IValueMap {
    type Vtable = IValueMapVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837534_098b_11d8_9414_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValueMapVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut ValueMapType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ValueMapType) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, map: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct IValueMapItem(::windows::core::IUnknown);
impl IValueMapItem {
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Key(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ValueMapType(&self) -> ::windows::core::Result<ValueMapType> {
        let mut result__: ValueMapType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ValueMapType>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetValueMapType(&self, r#type: ValueMapType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IValueMapItem> for super::Com::IDispatch {
    fn from(value: IValueMapItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IValueMapItem> for super::Com::IDispatch {
    fn from(value: &IValueMapItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IValueMapItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IValueMapItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IValueMapItem> for ::windows::core::IUnknown {
    fn from(value: IValueMapItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IValueMapItem> for ::windows::core::IUnknown {
    fn from(value: &IValueMapItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IValueMapItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IValueMapItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IValueMapItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IValueMapItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IValueMapItem {}
impl ::core::fmt::Debug for IValueMapItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IValueMapItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IValueMapItem {
    type Vtable = IValueMapItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837533_098b_11d8_9414_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValueMapItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut ValueMapType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ValueMapType) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InstallPerfDllA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szcomputername: Param0, lpinifile: Param1, dwflags: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InstallPerfDllA(szcomputername: super::super::Foundation::PSTR, lpinifile: super::super::Foundation::PSTR, dwflags: usize) -> u32;
        }
        ::core::mem::transmute(InstallPerfDllA(szcomputername.into_param().abi(), lpinifile.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InstallPerfDllW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szcomputername: Param0, lpinifile: Param1, dwflags: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InstallPerfDllW(szcomputername: super::super::Foundation::PWSTR, lpinifile: super::super::Foundation::PWSTR, dwflags: usize) -> u32;
        }
        ::core::mem::transmute(InstallPerfDllW(szcomputername.into_param().abi(), lpinifile.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const LIBID_SystemMonitor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b773e42_2509_11cf_942f_008029004347);
pub const LegacyDataCollectorSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837526_098b_11d8_9414_505054503030);
pub const LegacyDataCollectorSetCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837527_098b_11d8_9414_505054503030);
pub const LegacyTraceSession: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837528_098b_11d8_9414_505054503030);
pub const LegacyTraceSessionCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837529_098b_11d8_9414_505054503030);
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadPerfCounterTextStringsA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpcommandline: Param0, bquietmodearg: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadPerfCounterTextStringsA(lpcommandline: super::super::Foundation::PSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(LoadPerfCounterTextStringsA(lpcommandline.into_param().abi(), bquietmodearg.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadPerfCounterTextStringsW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpcommandline: Param0, bquietmodearg: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadPerfCounterTextStringsW(lpcommandline: super::super::Foundation::PWSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(LoadPerfCounterTextStringsW(lpcommandline.into_param().abi(), bquietmodearg.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const LogFileItem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16ec5be8_df93_4237_94e4_9ee918111d71);
pub const LogFiles: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2735d9fd_f6b9_4f19_a5d9_e2d068584bc5);
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const MAX_COUNTER_PATH: u32 = 256u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const MAX_PERF_OBJECTS_IN_QUERY_FUNCTION: i32 = 64i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_ACCESS_DENIED: i32 = -1073738789i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_ASYNC_QUERY_TIMEOUT: i32 = -2147481637i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_BINARY_LOG_CORRUPT: i32 = -1073738761i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_A {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub szDataSource: super::super::Foundation::PSTR,
    pub szReturnPathBuffer: super::super::Foundation::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_BROWSE_DLG_CONFIG_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_BROWSE_DLG_CONFIG_A")
            .field("_bitfield", &self._bitfield)
            .field("hWndOwner", &self.hWndOwner)
            .field("szDataSource", &self.szDataSource)
            .field("szReturnPathBuffer", &self.szReturnPathBuffer)
            .field("cchReturnPathLength", &self.cchReturnPathLength)
            .field("pCallBack", &self.pCallBack.map(|f| f as usize))
            .field("dwCallBackArg", &self.dwCallBackArg)
            .field("CallBackStatus", &self.CallBackStatus)
            .field("dwDefaultDetailLevel", &self.dwDefaultDetailLevel)
            .field("szDialogBoxCaption", &self.szDialogBoxCaption)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_BROWSE_DLG_CONFIG_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_BROWSE_DLG_CONFIG_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_BROWSE_DLG_CONFIG_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_BROWSE_DLG_CONFIG_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_BROWSE_DLG_CONFIG_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_HA {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub hDataSource: isize,
    pub szReturnPathBuffer: super::super::Foundation::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_HA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_HA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_BROWSE_DLG_CONFIG_HA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_BROWSE_DLG_CONFIG_HA")
            .field("_bitfield", &self._bitfield)
            .field("hWndOwner", &self.hWndOwner)
            .field("hDataSource", &self.hDataSource)
            .field("szReturnPathBuffer", &self.szReturnPathBuffer)
            .field("cchReturnPathLength", &self.cchReturnPathLength)
            .field("pCallBack", &self.pCallBack.map(|f| f as usize))
            .field("dwCallBackArg", &self.dwCallBackArg)
            .field("CallBackStatus", &self.CallBackStatus)
            .field("dwDefaultDetailLevel", &self.dwDefaultDetailLevel)
            .field("szDialogBoxCaption", &self.szDialogBoxCaption)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_BROWSE_DLG_CONFIG_HA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_BROWSE_DLG_CONFIG_HA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_BROWSE_DLG_CONFIG_HA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_BROWSE_DLG_CONFIG_HA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_BROWSE_DLG_CONFIG_HA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_HW {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub hDataSource: isize,
    pub szReturnPathBuffer: super::super::Foundation::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_HW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_HW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_BROWSE_DLG_CONFIG_HW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_BROWSE_DLG_CONFIG_HW")
            .field("_bitfield", &self._bitfield)
            .field("hWndOwner", &self.hWndOwner)
            .field("hDataSource", &self.hDataSource)
            .field("szReturnPathBuffer", &self.szReturnPathBuffer)
            .field("cchReturnPathLength", &self.cchReturnPathLength)
            .field("pCallBack", &self.pCallBack.map(|f| f as usize))
            .field("dwCallBackArg", &self.dwCallBackArg)
            .field("CallBackStatus", &self.CallBackStatus)
            .field("dwDefaultDetailLevel", &self.dwDefaultDetailLevel)
            .field("szDialogBoxCaption", &self.szDialogBoxCaption)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_BROWSE_DLG_CONFIG_HW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_BROWSE_DLG_CONFIG_HW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_BROWSE_DLG_CONFIG_HW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_BROWSE_DLG_CONFIG_HW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_BROWSE_DLG_CONFIG_HW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_W {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub szDataSource: super::super::Foundation::PWSTR,
    pub szReturnPathBuffer: super::super::Foundation::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_BROWSE_DLG_CONFIG_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_BROWSE_DLG_CONFIG_W")
            .field("_bitfield", &self._bitfield)
            .field("hWndOwner", &self.hWndOwner)
            .field("szDataSource", &self.szDataSource)
            .field("szReturnPathBuffer", &self.szReturnPathBuffer)
            .field("cchReturnPathLength", &self.cchReturnPathLength)
            .field("pCallBack", &self.pCallBack.map(|f| f as usize))
            .field("dwCallBackArg", &self.dwCallBackArg)
            .field("CallBackStatus", &self.CallBackStatus)
            .field("dwDefaultDetailLevel", &self.dwDefaultDetailLevel)
            .field("szDialogBoxCaption", &self.szDialogBoxCaption)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_BROWSE_DLG_CONFIG_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_BROWSE_DLG_CONFIG_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_BROWSE_DLG_CONFIG_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_BROWSE_DLG_CONFIG_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_BROWSE_DLG_CONFIG_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_CALC_NEGATIVE_DENOMINATOR: i32 = -2147481642i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_CALC_NEGATIVE_TIMEBASE: i32 = -2147481641i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_CALC_NEGATIVE_VALUE: i32 = -2147481640i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_CANNOT_CONNECT_MACHINE: i32 = -1073738813i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_CANNOT_CONNECT_WMI_SERVER: i32 = -1073738776i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_CANNOT_READ_NAME_STRINGS: i32 = -1073738808i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_CANNOT_SET_DEFAULT_REALTIME_DATASOURCE: i32 = -2147481636i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_COUNTER_ALREADY_IN_QUERY: i32 = -1073738762i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_COUNTER_INFO_A {
    pub dwLength: u32,
    pub dwType: u32,
    pub CVersion: u32,
    pub CStatus: u32,
    pub lScale: i32,
    pub lDefaultScale: i32,
    pub dwUserData: usize,
    pub dwQueryUserData: usize,
    pub szFullPath: super::super::Foundation::PSTR,
    pub Anonymous: PDH_COUNTER_INFO_A_0,
    pub szExplainText: super::super::Foundation::PSTR,
    pub DataBuffer: [u32; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_COUNTER_INFO_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_COUNTER_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_COUNTER_INFO_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_COUNTER_INFO_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_COUNTER_INFO_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_COUNTER_INFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union PDH_COUNTER_INFO_A_0 {
    pub DataItemPath: PDH_DATA_ITEM_PATH_ELEMENTS_A,
    pub CounterPath: PDH_COUNTER_PATH_ELEMENTS_A,
    pub Anonymous: PDH_COUNTER_INFO_A_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_COUNTER_INFO_A_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_COUNTER_INFO_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_COUNTER_INFO_A_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_A_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_COUNTER_INFO_A_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_COUNTER_INFO_A_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_COUNTER_INFO_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_COUNTER_INFO_A_0_0 {
    pub szMachineName: super::super::Foundation::PSTR,
    pub szObjectName: super::super::Foundation::PSTR,
    pub szInstanceName: super::super::Foundation::PSTR,
    pub szParentInstance: super::super::Foundation::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_COUNTER_INFO_A_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_COUNTER_INFO_A_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_COUNTER_INFO_A_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_COUNTER_INFO_A_0_0").field("szMachineName", &self.szMachineName).field("szObjectName", &self.szObjectName).field("szInstanceName", &self.szInstanceName).field("szParentInstance", &self.szParentInstance).field("dwInstanceIndex", &self.dwInstanceIndex).field("szCounterName", &self.szCounterName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_COUNTER_INFO_A_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_A_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_COUNTER_INFO_A_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_COUNTER_INFO_A_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_COUNTER_INFO_A_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_COUNTER_INFO_W {
    pub dwLength: u32,
    pub dwType: u32,
    pub CVersion: u32,
    pub CStatus: u32,
    pub lScale: i32,
    pub lDefaultScale: i32,
    pub dwUserData: usize,
    pub dwQueryUserData: usize,
    pub szFullPath: super::super::Foundation::PWSTR,
    pub Anonymous: PDH_COUNTER_INFO_W_0,
    pub szExplainText: super::super::Foundation::PWSTR,
    pub DataBuffer: [u32; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_COUNTER_INFO_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_COUNTER_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_COUNTER_INFO_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_COUNTER_INFO_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_COUNTER_INFO_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_COUNTER_INFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union PDH_COUNTER_INFO_W_0 {
    pub DataItemPath: PDH_DATA_ITEM_PATH_ELEMENTS_W,
    pub CounterPath: PDH_COUNTER_PATH_ELEMENTS_W,
    pub Anonymous: PDH_COUNTER_INFO_W_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_COUNTER_INFO_W_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_COUNTER_INFO_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_COUNTER_INFO_W_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_W_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_COUNTER_INFO_W_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_COUNTER_INFO_W_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_COUNTER_INFO_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_COUNTER_INFO_W_0_0 {
    pub szMachineName: super::super::Foundation::PWSTR,
    pub szObjectName: super::super::Foundation::PWSTR,
    pub szInstanceName: super::super::Foundation::PWSTR,
    pub szParentInstance: super::super::Foundation::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_COUNTER_INFO_W_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_COUNTER_INFO_W_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_COUNTER_INFO_W_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_COUNTER_INFO_W_0_0").field("szMachineName", &self.szMachineName).field("szObjectName", &self.szObjectName).field("szInstanceName", &self.szInstanceName).field("szParentInstance", &self.szParentInstance).field("dwInstanceIndex", &self.dwInstanceIndex).field("szCounterName", &self.szCounterName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_COUNTER_INFO_W_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_W_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_COUNTER_INFO_W_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_COUNTER_INFO_W_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_COUNTER_INFO_W_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_COUNTER_PATH_ELEMENTS_A {
    pub szMachineName: super::super::Foundation::PSTR,
    pub szObjectName: super::super::Foundation::PSTR,
    pub szInstanceName: super::super::Foundation::PSTR,
    pub szParentInstance: super::super::Foundation::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_COUNTER_PATH_ELEMENTS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_COUNTER_PATH_ELEMENTS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_COUNTER_PATH_ELEMENTS_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_COUNTER_PATH_ELEMENTS_A").field("szMachineName", &self.szMachineName).field("szObjectName", &self.szObjectName).field("szInstanceName", &self.szInstanceName).field("szParentInstance", &self.szParentInstance).field("dwInstanceIndex", &self.dwInstanceIndex).field("szCounterName", &self.szCounterName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_COUNTER_PATH_ELEMENTS_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_COUNTER_PATH_ELEMENTS_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_COUNTER_PATH_ELEMENTS_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_COUNTER_PATH_ELEMENTS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_COUNTER_PATH_ELEMENTS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_COUNTER_PATH_ELEMENTS_W {
    pub szMachineName: super::super::Foundation::PWSTR,
    pub szObjectName: super::super::Foundation::PWSTR,
    pub szInstanceName: super::super::Foundation::PWSTR,
    pub szParentInstance: super::super::Foundation::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_COUNTER_PATH_ELEMENTS_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_COUNTER_PATH_ELEMENTS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_COUNTER_PATH_ELEMENTS_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_COUNTER_PATH_ELEMENTS_W").field("szMachineName", &self.szMachineName).field("szObjectName", &self.szObjectName).field("szInstanceName", &self.szInstanceName).field("szParentInstance", &self.szParentInstance).field("dwInstanceIndex", &self.dwInstanceIndex).field("szCounterName", &self.szCounterName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_COUNTER_PATH_ELEMENTS_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_COUNTER_PATH_ELEMENTS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_COUNTER_PATH_ELEMENTS_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_COUNTER_PATH_ELEMENTS_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_COUNTER_PATH_ELEMENTS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_CSTATUS_BAD_COUNTERNAME: i32 = -1073738816i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_CSTATUS_INVALID_DATA: i32 = -1073738822i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_CSTATUS_ITEM_NOT_VALIDATED: i32 = -2147481645i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_CSTATUS_NEW_DATA: i32 = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_CSTATUS_NO_COUNTER: i32 = -1073738823i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_CSTATUS_NO_COUNTERNAME: i32 = -1073738817i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_CSTATUS_NO_INSTANCE: i32 = -2147481647i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_CSTATUS_NO_MACHINE: i32 = -2147481648i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_CSTATUS_NO_OBJECT: i32 = -1073738824i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_CSTATUS_VALID_DATA: i32 = 0i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_A {
    pub szMachineName: super::super::Foundation::PSTR,
    pub ObjectGUID: ::windows::core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_DATA_ITEM_PATH_ELEMENTS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_DATA_ITEM_PATH_ELEMENTS_A").field("szMachineName", &self.szMachineName).field("ObjectGUID", &self.ObjectGUID).field("dwItemId", &self.dwItemId).field("szInstanceName", &self.szInstanceName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_DATA_ITEM_PATH_ELEMENTS_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_DATA_ITEM_PATH_ELEMENTS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_W {
    pub szMachineName: super::super::Foundation::PWSTR,
    pub ObjectGUID: ::windows::core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_DATA_ITEM_PATH_ELEMENTS_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_DATA_ITEM_PATH_ELEMENTS_W").field("szMachineName", &self.szMachineName).field("ObjectGUID", &self.ObjectGUID).field("dwItemId", &self.dwItemId).field("szInstanceName", &self.szInstanceName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_DATA_ITEM_PATH_ELEMENTS_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_DATA_ITEM_PATH_ELEMENTS_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_DATA_SOURCE_IS_LOG_FILE: i32 = -1073738802i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_DATA_SOURCE_IS_REAL_TIME: i32 = -1073738801i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_DIALOG_CANCELLED: i32 = -2147481639i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type PDH_DLL_VERSION = u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_CVERSION_WIN50: PDH_DLL_VERSION = 1280u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_VERSION: PDH_DLL_VERSION = 1283u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_END_OF_LOG_FILE: i32 = -2147481638i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_ENTRY_NOT_IN_LOG_FILE: i32 = -1073738803i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_FILE_ALREADY_EXISTS: i32 = -1073738798i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_FILE_NOT_FOUND: i32 = -1073738799i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type PDH_FMT = u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_FMT_DOUBLE: PDH_FMT = 512u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_FMT_LARGE: PDH_FMT = 1024u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_FMT_LONG: PDH_FMT = 256u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_FMT_COUNTERVALUE {
    pub CStatus: u32,
    pub Anonymous: PDH_FMT_COUNTERVALUE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_FMT_COUNTERVALUE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_FMT_COUNTERVALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_FMT_COUNTERVALUE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_FMT_COUNTERVALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_FMT_COUNTERVALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union PDH_FMT_COUNTERVALUE_0 {
    pub longValue: i32,
    pub doubleValue: f64,
    pub largeValue: i64,
    pub AnsiStringValue: super::super::Foundation::PSTR,
    pub WideStringValue: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_FMT_COUNTERVALUE_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_FMT_COUNTERVALUE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_FMT_COUNTERVALUE_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_FMT_COUNTERVALUE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_FMT_COUNTERVALUE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_FMT_COUNTERVALUE_ITEM_A {
    pub szName: super::super::Foundation::PSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE_ITEM_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE_ITEM_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_FMT_COUNTERVALUE_ITEM_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_FMT_COUNTERVALUE_ITEM_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_FMT_COUNTERVALUE_ITEM_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_FMT_COUNTERVALUE_ITEM_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_FMT_COUNTERVALUE_ITEM_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_FMT_COUNTERVALUE_ITEM_W {
    pub szName: super::super::Foundation::PWSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE_ITEM_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_FMT_COUNTERVALUE_ITEM_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_FMT_COUNTERVALUE_ITEM_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_FMT_COUNTERVALUE_ITEM_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_FUNCTION_NOT_FOUND: i32 = -1073738818i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_INCORRECT_APPEND_TIME: i32 = -1073738757i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_INSUFFICIENT_BUFFER: i32 = -1073738814i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_INVALID_ARGUMENT: i32 = -1073738819i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_INVALID_BUFFER: i32 = -1073738815i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_INVALID_DATA: i32 = -1073738810i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_INVALID_DATASOURCE: i32 = -1073738787i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_INVALID_HANDLE: i32 = -1073738820i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_INVALID_INSTANCE: i32 = -1073738811i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_INVALID_PATH: i32 = -1073738812i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_INVALID_SQLDB: i32 = -1073738786i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_INVALID_SQL_LOG_FORMAT: i32 = -1073738763i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type PDH_LOG = u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOG_READ_ACCESS: PDH_LOG = 65536u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOG_WRITE_ACCESS: PDH_LOG = 131072u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOG_UPDATE_ACCESS: PDH_LOG = 262144u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOGSVC_NOT_OPENED: i32 = -1073738791i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOGSVC_QUERY_NOT_FOUND: i32 = -1073738792i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOG_FILE_CREATE_ERROR: i32 = -1073738807i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOG_FILE_OPEN_ERROR: i32 = -1073738806i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOG_FILE_TOO_SMALL: i32 = -1073738788i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOG_SAMPLE_TOO_SMALL: i32 = -1073738760i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwLogQuota: u32,
    pub szLogFileCaption: super::super::Foundation::PSTR,
    pub szDefaultDir: super::super::Foundation::PSTR,
    pub szBaseFileName: super::super::Foundation::PSTR,
    pub dwFileType: u32,
    pub dwReserved: u32,
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_A_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_LOG_SERVICE_QUERY_INFO_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    pub Anonymous1: PDH_LOG_SERVICE_QUERY_INFO_A_0_0,
    pub Anonymous2: PDH_LOG_SERVICE_QUERY_INFO_A_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_LOG_SERVICE_QUERY_INFO_A_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_A_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: super::super::Foundation::PSTR,
    pub PdlCounterList: super::super::Foundation::PSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: super::super::Foundation::FILETIME,
    pub PdlLogEndTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_LOG_SERVICE_QUERY_INFO_A_0_0").field("PdlAutoNameInterval", &self.PdlAutoNameInterval).field("PdlAutoNameUnits", &self.PdlAutoNameUnits).field("PdlCommandFilename", &self.PdlCommandFilename).field("PdlCounterList", &self.PdlCounterList).field("PdlAutoNameFormat", &self.PdlAutoNameFormat).field("PdlSampleInterval", &self.PdlSampleInterval).field("PdlLogStartTime", &self.PdlLogStartTime).field("PdlLogEndTime", &self.PdlLogEndTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_LOG_SERVICE_QUERY_INFO_A_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    pub TlNumberOfBuffers: u32,
    pub TlMinimumBuffers: u32,
    pub TlMaximumBuffers: u32,
    pub TlFreeBuffers: u32,
    pub TlBufferSize: u32,
    pub TlEventsLost: u32,
    pub TlLoggerThreadId: u32,
    pub TlBuffersWritten: u32,
    pub TlLogHandle: u32,
    pub TlLogFileName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_LOG_SERVICE_QUERY_INFO_A_0_1")
            .field("TlNumberOfBuffers", &self.TlNumberOfBuffers)
            .field("TlMinimumBuffers", &self.TlMinimumBuffers)
            .field("TlMaximumBuffers", &self.TlMaximumBuffers)
            .field("TlFreeBuffers", &self.TlFreeBuffers)
            .field("TlBufferSize", &self.TlBufferSize)
            .field("TlEventsLost", &self.TlEventsLost)
            .field("TlLoggerThreadId", &self.TlLoggerThreadId)
            .field("TlBuffersWritten", &self.TlBuffersWritten)
            .field("TlLogHandle", &self.TlLogHandle)
            .field("TlLogFileName", &self.TlLogFileName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_LOG_SERVICE_QUERY_INFO_A_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwLogQuota: u32,
    pub szLogFileCaption: super::super::Foundation::PWSTR,
    pub szDefaultDir: super::super::Foundation::PWSTR,
    pub szBaseFileName: super::super::Foundation::PWSTR,
    pub dwFileType: u32,
    pub dwReserved: u32,
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_W_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_LOG_SERVICE_QUERY_INFO_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    pub Anonymous1: PDH_LOG_SERVICE_QUERY_INFO_W_0_0,
    pub Anonymous2: PDH_LOG_SERVICE_QUERY_INFO_W_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_LOG_SERVICE_QUERY_INFO_W_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_W_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: super::super::Foundation::PWSTR,
    pub PdlCounterList: super::super::Foundation::PWSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: super::super::Foundation::FILETIME,
    pub PdlLogEndTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_LOG_SERVICE_QUERY_INFO_W_0_0").field("PdlAutoNameInterval", &self.PdlAutoNameInterval).field("PdlAutoNameUnits", &self.PdlAutoNameUnits).field("PdlCommandFilename", &self.PdlCommandFilename).field("PdlCounterList", &self.PdlCounterList).field("PdlAutoNameFormat", &self.PdlAutoNameFormat).field("PdlSampleInterval", &self.PdlSampleInterval).field("PdlLogStartTime", &self.PdlLogStartTime).field("PdlLogEndTime", &self.PdlLogEndTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_LOG_SERVICE_QUERY_INFO_W_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    pub TlNumberOfBuffers: u32,
    pub TlMinimumBuffers: u32,
    pub TlMaximumBuffers: u32,
    pub TlFreeBuffers: u32,
    pub TlBufferSize: u32,
    pub TlEventsLost: u32,
    pub TlLoggerThreadId: u32,
    pub TlBuffersWritten: u32,
    pub TlLogHandle: u32,
    pub TlLogFileName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_LOG_SERVICE_QUERY_INFO_W_0_1")
            .field("TlNumberOfBuffers", &self.TlNumberOfBuffers)
            .field("TlMinimumBuffers", &self.TlMinimumBuffers)
            .field("TlMaximumBuffers", &self.TlMaximumBuffers)
            .field("TlFreeBuffers", &self.TlFreeBuffers)
            .field("TlBufferSize", &self.TlBufferSize)
            .field("TlEventsLost", &self.TlEventsLost)
            .field("TlLoggerThreadId", &self.TlLoggerThreadId)
            .field("TlBuffersWritten", &self.TlBuffersWritten)
            .field("TlLogHandle", &self.TlLogHandle)
            .field("TlLogFileName", &self.TlLogFileName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_LOG_SERVICE_QUERY_INFO_W_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type PDH_LOG_TYPE = u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOG_TYPE_UNDEFINED: PDH_LOG_TYPE = 0u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOG_TYPE_CSV: PDH_LOG_TYPE = 1u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOG_TYPE_SQL: PDH_LOG_TYPE = 7u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOG_TYPE_TSV: PDH_LOG_TYPE = 2u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOG_TYPE_BINARY: PDH_LOG_TYPE = 8u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOG_TYPE_PERFMON: PDH_LOG_TYPE = 6u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOG_TYPE_NOT_FOUND: i32 = -1073738805i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOG_TYPE_RETIRED_BIN: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOG_TYPE_TRACE_GENERIC: u32 = 5u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_LOG_TYPE_TRACE_KERNEL: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_MAX_COUNTER_NAME: u32 = 1024u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_MAX_COUNTER_PATH: u32 = 2048u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_MAX_DATASOURCE_PATH: u32 = 1024u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_MAX_INSTANCE_NAME: u32 = 1024u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_MAX_SCALE: i32 = 7i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_MEMORY_ALLOCATION_FAILURE: i32 = -1073738821i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_MIN_SCALE: i32 = -7i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_MORE_DATA: i32 = -2147481646i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_NOEXPANDCOUNTERS: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_NOEXPANDINSTANCES: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_NOT_IMPLEMENTED: i32 = -1073738797i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_NO_COUNTERS: i32 = -1073738785i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_NO_DATA: i32 = -2147481643i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_NO_DIALOG_DATA: i32 = -1073738809i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_NO_MORE_DATA: i32 = -1073738804i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_OS_EARLIER_VERSION: i32 = -1073738758i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_OS_LATER_VERSION: i32 = -1073738759i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type PDH_PATH_FLAGS = u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_PATH_WBEM_RESULT: PDH_PATH_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_PATH_WBEM_INPUT: PDH_PATH_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_PATH_WBEM_NONE: PDH_PATH_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_PLA_COLLECTION_ALREADY_RUNNING: i32 = -1073738775i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_PLA_COLLECTION_NOT_FOUND: i32 = -1073738773i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_PLA_ERROR_ALREADY_EXISTS: i32 = -1073738770i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_PLA_ERROR_FILEPATH: i32 = -1073738768i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_PLA_ERROR_NAME_TOO_LONG: i32 = -1073738764i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_PLA_ERROR_NOSTART: i32 = -1073738771i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_PLA_ERROR_SCHEDULE_ELAPSED: i32 = -1073738772i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_PLA_ERROR_SCHEDULE_OVERLAP: i32 = -1073738774i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_PLA_ERROR_TYPE_MISMATCH: i32 = -1073738769i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_PLA_SERVICE_ERROR: i32 = -1073738767i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_PLA_VALIDATION_ERROR: i32 = -1073738766i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_PLA_VALIDATION_WARNING: i32 = -2147480589i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_QUERY_PERF_DATA_TIMEOUT: i32 = -1073738754i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_RAW_COUNTER {
    pub CStatus: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub FirstValue: i64,
    pub SecondValue: i64,
    pub MultiCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_RAW_COUNTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_RAW_COUNTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_RAW_COUNTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_RAW_COUNTER").field("CStatus", &self.CStatus).field("TimeStamp", &self.TimeStamp).field("FirstValue", &self.FirstValue).field("SecondValue", &self.SecondValue).field("MultiCount", &self.MultiCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_RAW_COUNTER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_RAW_COUNTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_RAW_COUNTER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_RAW_COUNTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_RAW_COUNTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_RAW_COUNTER_ITEM_A {
    pub szName: super::super::Foundation::PSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_RAW_COUNTER_ITEM_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_RAW_COUNTER_ITEM_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_RAW_COUNTER_ITEM_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_RAW_COUNTER_ITEM_A").field("szName", &self.szName).field("RawValue", &self.RawValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_RAW_COUNTER_ITEM_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_RAW_COUNTER_ITEM_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_RAW_COUNTER_ITEM_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_RAW_COUNTER_ITEM_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_RAW_COUNTER_ITEM_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_RAW_COUNTER_ITEM_W {
    pub szName: super::super::Foundation::PWSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_RAW_COUNTER_ITEM_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_RAW_COUNTER_ITEM_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_RAW_COUNTER_ITEM_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_RAW_COUNTER_ITEM_W").field("szName", &self.szName).field("RawValue", &self.RawValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_RAW_COUNTER_ITEM_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_RAW_COUNTER_ITEM_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_RAW_COUNTER_ITEM_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_RAW_COUNTER_ITEM_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_RAW_COUNTER_ITEM_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PDH_RAW_LOG_RECORD {
    pub dwStructureSize: u32,
    pub dwRecordType: PDH_LOG_TYPE,
    pub dwItems: u32,
    pub RawBytes: [u8; 1],
}
impl ::core::marker::Copy for PDH_RAW_LOG_RECORD {}
impl ::core::clone::Clone for PDH_RAW_LOG_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_RAW_LOG_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_RAW_LOG_RECORD").field("dwStructureSize", &self.dwStructureSize).field("dwRecordType", &self.dwRecordType).field("dwItems", &self.dwItems).field("RawBytes", &self.RawBytes).finish()
    }
}
unsafe impl ::windows::core::Abi for PDH_RAW_LOG_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_RAW_LOG_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_RAW_LOG_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_RAW_LOG_RECORD {}
impl ::core::default::Default for PDH_RAW_LOG_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_REFRESHCOUNTERS: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_RETRY: i32 = -2147481644i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type PDH_SELECT_DATA_SOURCE_FLAGS = u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_FLAGS_FILE_BROWSER_ONLY: PDH_SELECT_DATA_SOURCE_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_FLAGS_NONE: PDH_SELECT_DATA_SOURCE_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_SQL_ALLOCCON_FAILED: i32 = -1073738783i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_SQL_ALLOC_FAILED: i32 = -1073738784i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_SQL_ALTER_DETAIL_FAILED: i32 = -1073738755i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_SQL_BIND_FAILED: i32 = -1073738777i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_SQL_CONNECT_FAILED: i32 = -1073738778i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_SQL_EXEC_DIRECT_FAILED: i32 = -1073738782i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_SQL_FETCH_FAILED: i32 = -1073738781i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_SQL_MORE_RESULTS_FAILED: i32 = -1073738779i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_SQL_ROWCOUNT_FAILED: i32 = -1073738780i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_STATISTICS {
    pub dwFormat: u32,
    pub count: u32,
    pub min: PDH_FMT_COUNTERVALUE,
    pub max: PDH_FMT_COUNTERVALUE,
    pub mean: PDH_FMT_COUNTERVALUE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_STATISTICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_STATISTICS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_STATISTICS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_STATISTICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_STRING_NOT_FOUND: i32 = -1073738796i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PDH_TIME_INFO {
    pub StartTime: i64,
    pub EndTime: i64,
    pub SampleCount: u32,
}
impl ::core::marker::Copy for PDH_TIME_INFO {}
impl ::core::clone::Clone for PDH_TIME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_TIME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_TIME_INFO").field("StartTime", &self.StartTime).field("EndTime", &self.EndTime).field("SampleCount", &self.SampleCount).finish()
    }
}
unsafe impl ::windows::core::Abi for PDH_TIME_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_TIME_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_TIME_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_TIME_INFO {}
impl ::core::default::Default for PDH_TIME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_UNABLE_MAP_NAME_FILES: i32 = -2147480619i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_UNABLE_READ_LOG_HEADER: i32 = -1073738800i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_UNKNOWN_LOGSVC_COMMAND: i32 = -1073738793i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_UNKNOWN_LOG_FORMAT: i32 = -1073738794i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_UNMATCHED_APPEND_COUNTER: i32 = -1073738756i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PDH_WBEM_ERROR: i32 = -1073738790i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type PERFLIBREQUEST = ::core::option::Option<unsafe extern "system" fn(requestcode: u32, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> u32>;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_ADD_COUNTER: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_AGGREGATE_MAX: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_ATTRIB_BY_REFERENCE: u64 = 1u64;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_ATTRIB_DISPLAY_AS_HEX: u64 = 16u64;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_ATTRIB_DISPLAY_AS_REAL: u64 = 8u64;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_ATTRIB_NO_DISPLAYABLE: u64 = 2u64;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_ATTRIB_NO_GROUP_SEPARATOR: u64 = 4u64;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COLLECT_END: u32 = 6u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COLLECT_START: u32 = 5u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COUNTERSET_FLAG_AGGREGATE: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COUNTERSET_FLAG_HISTORY: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COUNTERSET_FLAG_INSTANCE: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COUNTERSET_FLAG_MULTIPLE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PERF_COUNTERSET_INFO {
    pub CounterSetGuid: ::windows::core::GUID,
    pub ProviderGuid: ::windows::core::GUID,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
impl ::core::marker::Copy for PERF_COUNTERSET_INFO {}
impl ::core::clone::Clone for PERF_COUNTERSET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTERSET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTERSET_INFO").field("CounterSetGuid", &self.CounterSetGuid).field("ProviderGuid", &self.ProviderGuid).field("NumCounters", &self.NumCounters).field("InstanceType", &self.InstanceType).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTERSET_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTERSET_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTERSET_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTERSET_INFO {}
impl ::core::default::Default for PERF_COUNTERSET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PERF_COUNTERSET_INSTANCE {
    pub CounterSetGuid: ::windows::core::GUID,
    pub dwSize: u32,
    pub InstanceId: u32,
    pub InstanceNameOffset: u32,
    pub InstanceNameSize: u32,
}
impl ::core::marker::Copy for PERF_COUNTERSET_INSTANCE {}
impl ::core::clone::Clone for PERF_COUNTERSET_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTERSET_INSTANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTERSET_INSTANCE").field("CounterSetGuid", &self.CounterSetGuid).field("dwSize", &self.dwSize).field("InstanceId", &self.InstanceId).field("InstanceNameOffset", &self.InstanceNameOffset).field("InstanceNameSize", &self.InstanceNameSize).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTERSET_INSTANCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTERSET_INSTANCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTERSET_INSTANCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTERSET_INSTANCE {}
impl ::core::default::Default for PERF_COUNTERSET_INSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COUNTERSET_MULTI_INSTANCES: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PERF_COUNTERSET_REG_INFO {
    pub CounterSetGuid: ::windows::core::GUID,
    pub CounterSetType: u32,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
impl ::core::marker::Copy for PERF_COUNTERSET_REG_INFO {}
impl ::core::clone::Clone for PERF_COUNTERSET_REG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTERSET_REG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTERSET_REG_INFO").field("CounterSetGuid", &self.CounterSetGuid).field("CounterSetType", &self.CounterSetType).field("DetailLevel", &self.DetailLevel).field("NumCounters", &self.NumCounters).field("InstanceType", &self.InstanceType).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTERSET_REG_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTERSET_REG_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTERSET_REG_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTERSET_REG_INFO {}
impl ::core::default::Default for PERF_COUNTERSET_REG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COUNTERSET_SINGLE_AGGREGATE: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COUNTERSET_SINGLE_INSTANCE: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type PERF_COUNTER_AGGREGATE_FUNC = u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_AGGREGATE_UNDEFINED: PERF_COUNTER_AGGREGATE_FUNC = 0u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_AGGREGATE_TOTAL: PERF_COUNTER_AGGREGATE_FUNC = 1u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_AGGREGATE_AVG: PERF_COUNTER_AGGREGATE_FUNC = 2u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_AGGREGATE_MIN: PERF_COUNTER_AGGREGATE_FUNC = 3u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COUNTER_BASE: u32 = 196608u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PERF_COUNTER_BLOCK {
    pub ByteLength: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_BLOCK {}
impl ::core::clone::Clone for PERF_COUNTER_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_BLOCK").field("ByteLength", &self.ByteLength).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTER_BLOCK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTER_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTER_BLOCK>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_BLOCK {}
impl ::core::default::Default for PERF_COUNTER_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PERF_COUNTER_DATA {
    pub dwDataSize: u32,
    pub dwSize: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_DATA {}
impl ::core::clone::Clone for PERF_COUNTER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_DATA").field("dwDataSize", &self.dwDataSize).field("dwSize", &self.dwSize).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTER_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTER_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTER_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_DATA {}
impl ::core::default::Default for PERF_COUNTER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: u32,
    pub CounterNameTitleIndex: u32,
    pub CounterNameTitle: u32,
    pub CounterHelpTitleIndex: u32,
    pub CounterHelpTitle: u32,
    pub DefaultScale: i32,
    pub DetailLevel: u32,
    pub CounterType: u32,
    pub CounterSize: u32,
    pub CounterOffset: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERF_COUNTER_DEFINITION {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERF_COUNTER_DEFINITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERF_COUNTER_DEFINITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_DEFINITION")
            .field("ByteLength", &self.ByteLength)
            .field("CounterNameTitleIndex", &self.CounterNameTitleIndex)
            .field("CounterNameTitle", &self.CounterNameTitle)
            .field("CounterHelpTitleIndex", &self.CounterHelpTitleIndex)
            .field("CounterHelpTitle", &self.CounterHelpTitle)
            .field("DefaultScale", &self.DefaultScale)
            .field("DetailLevel", &self.DetailLevel)
            .field("CounterType", &self.CounterType)
            .field("CounterSize", &self.CounterSize)
            .field("CounterOffset", &self.CounterOffset)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PERF_COUNTER_DEFINITION {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERF_COUNTER_DEFINITION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTER_DEFINITION>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERF_COUNTER_DEFINITION {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERF_COUNTER_DEFINITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: u32,
    pub CounterNameTitleIndex: u32,
    pub CounterNameTitle: super::super::Foundation::PWSTR,
    pub CounterHelpTitleIndex: u32,
    pub CounterHelpTitle: super::super::Foundation::PWSTR,
    pub DefaultScale: i32,
    pub DetailLevel: u32,
    pub CounterType: u32,
    pub CounterSize: u32,
    pub CounterOffset: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERF_COUNTER_DEFINITION {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERF_COUNTER_DEFINITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERF_COUNTER_DEFINITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_DEFINITION")
            .field("ByteLength", &self.ByteLength)
            .field("CounterNameTitleIndex", &self.CounterNameTitleIndex)
            .field("CounterNameTitle", &self.CounterNameTitle)
            .field("CounterHelpTitleIndex", &self.CounterHelpTitleIndex)
            .field("CounterHelpTitle", &self.CounterHelpTitle)
            .field("DefaultScale", &self.DefaultScale)
            .field("DetailLevel", &self.DetailLevel)
            .field("CounterType", &self.CounterType)
            .field("CounterSize", &self.CounterSize)
            .field("CounterOffset", &self.CounterOffset)
            .finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PERF_COUNTER_DEFINITION {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERF_COUNTER_DEFINITION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTER_DEFINITION>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERF_COUNTER_DEFINITION {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERF_COUNTER_DEFINITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COUNTER_ELAPSED: u32 = 262144u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COUNTER_FRACTION: u32 = 131072u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PERF_COUNTER_HEADER {
    pub dwStatus: u32,
    pub dwType: PerfCounterDataType,
    pub dwSize: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_HEADER {}
impl ::core::clone::Clone for PERF_COUNTER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_HEADER").field("dwStatus", &self.dwStatus).field("dwType", &self.dwType).field("dwSize", &self.dwSize).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTER_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTER_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTER_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_HEADER {}
impl ::core::default::Default for PERF_COUNTER_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COUNTER_HISTOGRAM: u32 = 393216u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COUNTER_HISTOGRAM_TYPE: u32 = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PERF_COUNTER_IDENTIFIER {
    pub CounterSetGuid: ::windows::core::GUID,
    pub Status: u32,
    pub Size: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub Index: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_IDENTIFIER {}
impl ::core::clone::Clone for PERF_COUNTER_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_IDENTIFIER").field("CounterSetGuid", &self.CounterSetGuid).field("Status", &self.Status).field("Size", &self.Size).field("CounterId", &self.CounterId).field("InstanceId", &self.InstanceId).field("Index", &self.Index).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTER_IDENTIFIER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTER_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTER_IDENTIFIER>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_IDENTIFIER {}
impl ::core::default::Default for PERF_COUNTER_IDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PERF_COUNTER_IDENTITY {
    pub CounterSetGuid: ::windows::core::GUID,
    pub BufferSize: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub MachineOffset: u32,
    pub NameOffset: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_IDENTITY {}
impl ::core::clone::Clone for PERF_COUNTER_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_IDENTITY").field("CounterSetGuid", &self.CounterSetGuid).field("BufferSize", &self.BufferSize).field("CounterId", &self.CounterId).field("InstanceId", &self.InstanceId).field("MachineOffset", &self.MachineOffset).field("NameOffset", &self.NameOffset).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTER_IDENTITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTER_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTER_IDENTITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_IDENTITY {}
impl ::core::default::Default for PERF_COUNTER_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PERF_COUNTER_INFO {
    pub CounterId: u32,
    pub Type: u32,
    pub Attrib: u64,
    pub Size: u32,
    pub DetailLevel: u32,
    pub Scale: i32,
    pub Offset: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_INFO {}
impl ::core::clone::Clone for PERF_COUNTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_INFO").field("CounterId", &self.CounterId).field("Type", &self.Type).field("Attrib", &self.Attrib).field("Size", &self.Size).field("DetailLevel", &self.DetailLevel).field("Scale", &self.Scale).field("Offset", &self.Offset).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_INFO {}
impl ::core::default::Default for PERF_COUNTER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COUNTER_PRECISION: u32 = 458752u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COUNTER_QUEUELEN: u32 = 327680u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COUNTER_RATE: u32 = 65536u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PERF_COUNTER_REG_INFO {
    pub CounterId: u32,
    pub Type: u32,
    pub Attrib: u64,
    pub DetailLevel: u32,
    pub DefaultScale: i32,
    pub BaseCounterId: u32,
    pub PerfTimeId: u32,
    pub PerfFreqId: u32,
    pub MultiId: u32,
    pub AggregateFunc: PERF_COUNTER_AGGREGATE_FUNC,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_REG_INFO {}
impl ::core::clone::Clone for PERF_COUNTER_REG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_REG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_REG_INFO").field("CounterId", &self.CounterId).field("Type", &self.Type).field("Attrib", &self.Attrib).field("DetailLevel", &self.DetailLevel).field("DefaultScale", &self.DefaultScale).field("BaseCounterId", &self.BaseCounterId).field("PerfTimeId", &self.PerfTimeId).field("PerfFreqId", &self.PerfFreqId).field("MultiId", &self.MultiId).field("AggregateFunc", &self.AggregateFunc).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTER_REG_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTER_REG_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTER_REG_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_REG_INFO {}
impl ::core::default::Default for PERF_COUNTER_REG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COUNTER_VALUE: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_DATA_BLOCK {
    pub Signature: [u16; 4],
    pub LittleEndian: u32,
    pub Version: u32,
    pub Revision: u32,
    pub TotalByteLength: u32,
    pub HeaderLength: u32,
    pub NumObjectTypes: u32,
    pub DefaultObject: i32,
    pub SystemTime: super::super::Foundation::SYSTEMTIME,
    pub PerfTime: i64,
    pub PerfFreq: i64,
    pub PerfTime100nSec: i64,
    pub SystemNameLength: u32,
    pub SystemNameOffset: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERF_DATA_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERF_DATA_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERF_DATA_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_DATA_BLOCK")
            .field("Signature", &self.Signature)
            .field("LittleEndian", &self.LittleEndian)
            .field("Version", &self.Version)
            .field("Revision", &self.Revision)
            .field("TotalByteLength", &self.TotalByteLength)
            .field("HeaderLength", &self.HeaderLength)
            .field("NumObjectTypes", &self.NumObjectTypes)
            .field("DefaultObject", &self.DefaultObject)
            .field("SystemTime", &self.SystemTime)
            .field("PerfTime", &self.PerfTime)
            .field("PerfFreq", &self.PerfFreq)
            .field("PerfTime100nSec", &self.PerfTime100nSec)
            .field("SystemNameLength", &self.SystemNameLength)
            .field("SystemNameOffset", &self.SystemNameOffset)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PERF_DATA_BLOCK {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERF_DATA_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_DATA_BLOCK>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERF_DATA_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERF_DATA_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_DATA_HEADER {
    pub dwTotalSize: u32,
    pub dwNumCounters: u32,
    pub PerfTimeStamp: i64,
    pub PerfTime100NSec: i64,
    pub PerfFreq: i64,
    pub SystemTime: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERF_DATA_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERF_DATA_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERF_DATA_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_DATA_HEADER").field("dwTotalSize", &self.dwTotalSize).field("dwNumCounters", &self.dwNumCounters).field("PerfTimeStamp", &self.PerfTimeStamp).field("PerfTime100NSec", &self.PerfTime100NSec).field("PerfFreq", &self.PerfFreq).field("SystemTime", &self.SystemTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PERF_DATA_HEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERF_DATA_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_DATA_HEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERF_DATA_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERF_DATA_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_DATA_REVISION: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_DATA_VERSION: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_DELTA_BASE: u32 = 8388608u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_DELTA_COUNTER: u32 = 4194304u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type PERF_DETAIL = u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_DETAIL_NOVICE: PERF_DETAIL = 100u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_DETAIL_ADVANCED: PERF_DETAIL = 200u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_DETAIL_EXPERT: PERF_DETAIL = 300u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_DETAIL_WIZARD: PERF_DETAIL = 400u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_DISPLAY_NOSHOW: u32 = 1073741824u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_DISPLAY_NO_SUFFIX: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_DISPLAY_PERCENT: u32 = 536870912u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_DISPLAY_PER_SEC: u32 = 268435456u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_DISPLAY_SECONDS: u32 = 805306368u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_ENUM_INSTANCES: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_FILTER: u32 = 9u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PERF_INSTANCE_DEFINITION {
    pub ByteLength: u32,
    pub ParentObjectTitleIndex: u32,
    pub ParentObjectInstance: u32,
    pub UniqueID: i32,
    pub NameOffset: u32,
    pub NameLength: u32,
}
impl ::core::marker::Copy for PERF_INSTANCE_DEFINITION {}
impl ::core::clone::Clone for PERF_INSTANCE_DEFINITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_INSTANCE_DEFINITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_INSTANCE_DEFINITION").field("ByteLength", &self.ByteLength).field("ParentObjectTitleIndex", &self.ParentObjectTitleIndex).field("ParentObjectInstance", &self.ParentObjectInstance).field("UniqueID", &self.UniqueID).field("NameOffset", &self.NameOffset).field("NameLength", &self.NameLength).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_INSTANCE_DEFINITION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_INSTANCE_DEFINITION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_INSTANCE_DEFINITION>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_INSTANCE_DEFINITION {}
impl ::core::default::Default for PERF_INSTANCE_DEFINITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PERF_INSTANCE_HEADER {
    pub Size: u32,
    pub InstanceId: u32,
}
impl ::core::marker::Copy for PERF_INSTANCE_HEADER {}
impl ::core::clone::Clone for PERF_INSTANCE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_INSTANCE_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_INSTANCE_HEADER").field("Size", &self.Size).field("InstanceId", &self.InstanceId).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_INSTANCE_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_INSTANCE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_INSTANCE_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_INSTANCE_HEADER {}
impl ::core::default::Default for PERF_INSTANCE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_INVERSE_COUNTER: u32 = 16777216u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_MAX_INSTANCE_NAME: u32 = 1024u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type PERF_MEM_ALLOC = ::core::option::Option<unsafe extern "system" fn(allocsize: usize, pcontext: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type PERF_MEM_FREE = ::core::option::Option<unsafe extern "system" fn(pbuffer: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_METADATA_MULTIPLE_INSTANCES: i32 = -2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_METADATA_NO_INSTANCES: i32 = -3i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_MULTI_COUNTER: u32 = 33554432u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PERF_MULTI_COUNTERS {
    pub dwSize: u32,
    pub dwCounters: u32,
}
impl ::core::marker::Copy for PERF_MULTI_COUNTERS {}
impl ::core::clone::Clone for PERF_MULTI_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_MULTI_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_MULTI_COUNTERS").field("dwSize", &self.dwSize).field("dwCounters", &self.dwCounters).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_MULTI_COUNTERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_MULTI_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_MULTI_COUNTERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_MULTI_COUNTERS {}
impl ::core::default::Default for PERF_MULTI_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PERF_MULTI_INSTANCES {
    pub dwTotalSize: u32,
    pub dwInstances: u32,
}
impl ::core::marker::Copy for PERF_MULTI_INSTANCES {}
impl ::core::clone::Clone for PERF_MULTI_INSTANCES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_MULTI_INSTANCES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_MULTI_INSTANCES").field("dwTotalSize", &self.dwTotalSize).field("dwInstances", &self.dwInstances).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_MULTI_INSTANCES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_MULTI_INSTANCES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_MULTI_INSTANCES>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_MULTI_INSTANCES {}
impl ::core::default::Default for PERF_MULTI_INSTANCES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_NO_INSTANCES: i32 = -1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_NO_UNIQUE_ID: i32 = -1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_NUMBER_DECIMAL: u32 = 65536u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_NUMBER_DEC_1000: u32 = 131072u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_NUMBER_HEX: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_OBJECT_TIMER: u32 = 2097152u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: u32,
    pub DefinitionLength: u32,
    pub HeaderLength: u32,
    pub ObjectNameTitleIndex: u32,
    pub ObjectNameTitle: u32,
    pub ObjectHelpTitleIndex: u32,
    pub ObjectHelpTitle: u32,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub DefaultCounter: i32,
    pub NumInstances: i32,
    pub CodePage: u32,
    pub PerfTime: i64,
    pub PerfFreq: i64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERF_OBJECT_TYPE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERF_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERF_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_OBJECT_TYPE")
            .field("TotalByteLength", &self.TotalByteLength)
            .field("DefinitionLength", &self.DefinitionLength)
            .field("HeaderLength", &self.HeaderLength)
            .field("ObjectNameTitleIndex", &self.ObjectNameTitleIndex)
            .field("ObjectNameTitle", &self.ObjectNameTitle)
            .field("ObjectHelpTitleIndex", &self.ObjectHelpTitleIndex)
            .field("ObjectHelpTitle", &self.ObjectHelpTitle)
            .field("DetailLevel", &self.DetailLevel)
            .field("NumCounters", &self.NumCounters)
            .field("DefaultCounter", &self.DefaultCounter)
            .field("NumInstances", &self.NumInstances)
            .field("CodePage", &self.CodePage)
            .field("PerfTime", &self.PerfTime)
            .field("PerfFreq", &self.PerfFreq)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PERF_OBJECT_TYPE {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERF_OBJECT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_OBJECT_TYPE>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERF_OBJECT_TYPE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERF_OBJECT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: u32,
    pub DefinitionLength: u32,
    pub HeaderLength: u32,
    pub ObjectNameTitleIndex: u32,
    pub ObjectNameTitle: super::super::Foundation::PWSTR,
    pub ObjectHelpTitleIndex: u32,
    pub ObjectHelpTitle: super::super::Foundation::PWSTR,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub DefaultCounter: i32,
    pub NumInstances: i32,
    pub CodePage: u32,
    pub PerfTime: i64,
    pub PerfFreq: i64,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERF_OBJECT_TYPE {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERF_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERF_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_OBJECT_TYPE")
            .field("TotalByteLength", &self.TotalByteLength)
            .field("DefinitionLength", &self.DefinitionLength)
            .field("HeaderLength", &self.HeaderLength)
            .field("ObjectNameTitleIndex", &self.ObjectNameTitleIndex)
            .field("ObjectNameTitle", &self.ObjectNameTitle)
            .field("ObjectHelpTitleIndex", &self.ObjectHelpTitleIndex)
            .field("ObjectHelpTitle", &self.ObjectHelpTitle)
            .field("DetailLevel", &self.DetailLevel)
            .field("NumCounters", &self.NumCounters)
            .field("DefaultCounter", &self.DefaultCounter)
            .field("NumInstances", &self.NumInstances)
            .field("CodePage", &self.CodePage)
            .field("PerfTime", &self.PerfTime)
            .field("PerfFreq", &self.PerfFreq)
            .finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PERF_OBJECT_TYPE {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERF_OBJECT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_OBJECT_TYPE>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERF_OBJECT_TYPE {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERF_OBJECT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PERF_PROVIDER_CONTEXT {
    pub ContextSize: u32,
    pub Reserved: u32,
    pub ControlCallback: PERFLIBREQUEST,
    pub MemAllocRoutine: PERF_MEM_ALLOC,
    pub MemFreeRoutine: PERF_MEM_FREE,
    pub pMemContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for PERF_PROVIDER_CONTEXT {}
impl ::core::clone::Clone for PERF_PROVIDER_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_PROVIDER_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_PROVIDER_CONTEXT").field("ContextSize", &self.ContextSize).field("Reserved", &self.Reserved).field("ControlCallback", &self.ControlCallback.map(|f| f as usize)).field("MemAllocRoutine", &self.MemAllocRoutine.map(|f| f as usize)).field("MemFreeRoutine", &self.MemFreeRoutine.map(|f| f as usize)).field("pMemContext", &self.pMemContext).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_PROVIDER_CONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_PROVIDER_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_PROVIDER_CONTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_PROVIDER_CONTEXT {}
impl ::core::default::Default for PERF_PROVIDER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_PROVIDER_DRIVER: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_PROVIDER_KERNEL_MODE: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_PROVIDER_USER_MODE: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_REMOVE_COUNTER: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_SIZE_DWORD: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_SIZE_LARGE: u32 = 256u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_SIZE_VARIABLE_LEN: u32 = 768u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_SIZE_ZERO: u32 = 512u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PERF_STRING_BUFFER_HEADER {
    pub dwSize: u32,
    pub dwCounters: u32,
}
impl ::core::marker::Copy for PERF_STRING_BUFFER_HEADER {}
impl ::core::clone::Clone for PERF_STRING_BUFFER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_STRING_BUFFER_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_STRING_BUFFER_HEADER").field("dwSize", &self.dwSize).field("dwCounters", &self.dwCounters).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_STRING_BUFFER_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_STRING_BUFFER_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_STRING_BUFFER_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_STRING_BUFFER_HEADER {}
impl ::core::default::Default for PERF_STRING_BUFFER_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub struct PERF_STRING_COUNTER_HEADER {
    pub dwCounterId: u32,
    pub dwOffset: u32,
}
impl ::core::marker::Copy for PERF_STRING_COUNTER_HEADER {}
impl ::core::clone::Clone for PERF_STRING_COUNTER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_STRING_COUNTER_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_STRING_COUNTER_HEADER").field("dwCounterId", &self.dwCounterId).field("dwOffset", &self.dwOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_STRING_COUNTER_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_STRING_COUNTER_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_STRING_COUNTER_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_STRING_COUNTER_HEADER {}
impl ::core::default::Default for PERF_STRING_COUNTER_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_TEXT_ASCII: u32 = 65536u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_TEXT_UNICODE: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_TIMER_100NS: u32 = 1048576u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_TIMER_TICK: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_TYPE_COUNTER: u32 = 1024u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_TYPE_NUMBER: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_TYPE_TEXT: u32 = 2048u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_TYPE_ZERO: u32 = 3072u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_WILDCARD_COUNTER: u32 = 4294967295u32;
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLA_CABEXTRACT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(filename: super::super::Foundation::PWSTR, context: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PLA_CAPABILITY_AUTOLOGGER: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PLA_CAPABILITY_LEGACY_SESSION: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PLA_CAPABILITY_LEGACY_SVC: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PLA_CAPABILITY_LOCAL: u32 = 268435456u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PLA_CAPABILITY_V1_SESSION: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PLA_CAPABILITY_V1_SVC: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PLA_CAPABILITY_V1_SYSTEM: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type PM_CLOSE_PROC = ::core::option::Option<unsafe extern "system" fn() -> u32>;
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PM_COLLECT_PROC = ::core::option::Option<unsafe extern "system" fn(pvaluename: super::super::Foundation::PWSTR, ppdata: *mut *mut ::core::ffi::c_void, pcbtotalbytes: *mut u32, pnumobjecttypes: *mut u32) -> u32>;
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PM_OPEN_PROC = ::core::option::Option<unsafe extern "system" fn(pcontext: super::super::Foundation::PWSTR) -> u32>;
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhAddCounterA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hquery: isize, szfullcounterpath: Param1, dwuserdata: usize, phcounter: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhAddCounterA(hquery: isize, szfullcounterpath: super::super::Foundation::PSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhAddCounterA(::core::mem::transmute(hquery), szfullcounterpath.into_param().abi(), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phcounter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhAddCounterW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hquery: isize, szfullcounterpath: Param1, dwuserdata: usize, phcounter: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhAddCounterW(hquery: isize, szfullcounterpath: super::super::Foundation::PWSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhAddCounterW(::core::mem::transmute(hquery), szfullcounterpath.into_param().abi(), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phcounter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhAddEnglishCounterA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hquery: isize, szfullcounterpath: Param1, dwuserdata: usize, phcounter: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhAddEnglishCounterA(hquery: isize, szfullcounterpath: super::super::Foundation::PSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhAddEnglishCounterA(::core::mem::transmute(hquery), szfullcounterpath.into_param().abi(), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phcounter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhAddEnglishCounterW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hquery: isize, szfullcounterpath: Param1, dwuserdata: usize, phcounter: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhAddEnglishCounterW(hquery: isize, szfullcounterpath: super::super::Foundation::PWSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhAddEnglishCounterW(::core::mem::transmute(hquery), szfullcounterpath.into_param().abi(), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phcounter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhBindInputDataSourceA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(phdatasource: *mut isize, logfilenamelist: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhBindInputDataSourceA(phdatasource: *mut isize, logfilenamelist: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(PdhBindInputDataSourceA(::core::mem::transmute(phdatasource), logfilenamelist.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhBindInputDataSourceW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(phdatasource: *mut isize, logfilenamelist: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhBindInputDataSourceW(phdatasource: *mut isize, logfilenamelist: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(PdhBindInputDataSourceW(::core::mem::transmute(phdatasource), logfilenamelist.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhBrowseCountersA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_A) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhBrowseCountersA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_A) -> i32;
        }
        ::core::mem::transmute(PdhBrowseCountersA(::core::mem::transmute(pbrowsedlgdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhBrowseCountersHA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhBrowseCountersHA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HA) -> i32;
        }
        ::core::mem::transmute(PdhBrowseCountersHA(::core::mem::transmute(pbrowsedlgdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhBrowseCountersHW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhBrowseCountersHW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HW) -> i32;
        }
        ::core::mem::transmute(PdhBrowseCountersHW(::core::mem::transmute(pbrowsedlgdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhBrowseCountersW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_W) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhBrowseCountersW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_W) -> i32;
        }
        ::core::mem::transmute(PdhBrowseCountersW(::core::mem::transmute(pbrowsedlgdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhCalculateCounterFromRawValue(hcounter: isize, dwformat: PDH_FMT, rawvalue1: *const PDH_RAW_COUNTER, rawvalue2: *const PDH_RAW_COUNTER, fmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCalculateCounterFromRawValue(hcounter: isize, dwformat: PDH_FMT, rawvalue1: *const PDH_RAW_COUNTER, rawvalue2: *const PDH_RAW_COUNTER, fmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
        }
        ::core::mem::transmute(PdhCalculateCounterFromRawValue(::core::mem::transmute(hcounter), ::core::mem::transmute(dwformat), ::core::mem::transmute(rawvalue1), ::core::mem::transmute(rawvalue2), ::core::mem::transmute(fmtvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PdhCloseLog(hlog: isize, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCloseLog(hlog: isize, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhCloseLog(::core::mem::transmute(hlog), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PdhCloseQuery(hquery: isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCloseQuery(hquery: isize) -> i32;
        }
        ::core::mem::transmute(PdhCloseQuery(::core::mem::transmute(hquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PdhCollectQueryData(hquery: isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCollectQueryData(hquery: isize) -> i32;
        }
        ::core::mem::transmute(PdhCollectQueryData(::core::mem::transmute(hquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhCollectQueryDataEx<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hquery: isize, dwintervaltime: u32, hnewdataevent: Param2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCollectQueryDataEx(hquery: isize, dwintervaltime: u32, hnewdataevent: super::super::Foundation::HANDLE) -> i32;
        }
        ::core::mem::transmute(PdhCollectQueryDataEx(::core::mem::transmute(hquery), ::core::mem::transmute(dwintervaltime), hnewdataevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PdhCollectQueryDataWithTime(hquery: isize, plltimestamp: *mut i64) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCollectQueryDataWithTime(hquery: isize, plltimestamp: *mut i64) -> i32;
        }
        ::core::mem::transmute(PdhCollectQueryDataWithTime(::core::mem::transmute(hquery), ::core::mem::transmute(plltimestamp)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhComputeCounterStatistics(hcounter: isize, dwformat: PDH_FMT, dwfirstentry: u32, dwnumentries: u32, lprawvaluearray: *const PDH_RAW_COUNTER, data: *mut PDH_STATISTICS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhComputeCounterStatistics(hcounter: isize, dwformat: PDH_FMT, dwfirstentry: u32, dwnumentries: u32, lprawvaluearray: *const PDH_RAW_COUNTER, data: *mut PDH_STATISTICS) -> i32;
        }
        ::core::mem::transmute(PdhComputeCounterStatistics(::core::mem::transmute(hcounter), ::core::mem::transmute(dwformat), ::core::mem::transmute(dwfirstentry), ::core::mem::transmute(dwnumentries), ::core::mem::transmute(lprawvaluearray), ::core::mem::transmute(data)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhConnectMachineA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szmachinename: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhConnectMachineA(szmachinename: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(PdhConnectMachineA(szmachinename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhConnectMachineW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szmachinename: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhConnectMachineW(szmachinename: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(PdhConnectMachineW(szmachinename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhCreateSQLTablesA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCreateSQLTablesA(szdatasource: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(PdhCreateSQLTablesA(szdatasource.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhCreateSQLTablesW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCreateSQLTablesW(szdatasource: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(PdhCreateSQLTablesW(szdatasource.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumLogSetNamesA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0, mszdatasetnamelist: super::super::Foundation::PSTR, pcchbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumLogSetNamesA(szdatasource: super::super::Foundation::PSTR, mszdatasetnamelist: super::super::Foundation::PSTR, pcchbufferlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumLogSetNamesA(szdatasource.into_param().abi(), ::core::mem::transmute(mszdatasetnamelist), ::core::mem::transmute(pcchbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumLogSetNamesW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0, mszdatasetnamelist: super::super::Foundation::PWSTR, pcchbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumLogSetNamesW(szdatasource: super::super::Foundation::PWSTR, mszdatasetnamelist: super::super::Foundation::PWSTR, pcchbufferlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumLogSetNamesW(szdatasource.into_param().abi(), ::core::mem::transmute(mszdatasetnamelist), ::core::mem::transmute(pcchbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumMachinesA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0, mszmachinelist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumMachinesA(szdatasource: super::super::Foundation::PSTR, mszmachinelist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumMachinesA(szdatasource.into_param().abi(), ::core::mem::transmute(mszmachinelist), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumMachinesHA(hdatasource: isize, mszmachinelist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumMachinesHA(hdatasource: isize, mszmachinelist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumMachinesHA(::core::mem::transmute(hdatasource), ::core::mem::transmute(mszmachinelist), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumMachinesHW(hdatasource: isize, mszmachinelist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumMachinesHW(hdatasource: isize, mszmachinelist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumMachinesHW(::core::mem::transmute(hdatasource), ::core::mem::transmute(mszmachinelist), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumMachinesW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0, mszmachinelist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumMachinesW(szdatasource: super::super::Foundation::PWSTR, mszmachinelist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumMachinesW(szdatasource.into_param().abi(), ::core::mem::transmute(mszmachinelist), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectItemsA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0, szmachinename: Param1, szobjectname: Param2, mszcounterlist: super::super::Foundation::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectItemsA(szdatasource: super::super::Foundation::PSTR, szmachinename: super::super::Foundation::PSTR, szobjectname: super::super::Foundation::PSTR, mszcounterlist: super::super::Foundation::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectItemsA(szdatasource.into_param().abi(), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(mszcounterlist), ::core::mem::transmute(pcchcounterlistlength), ::core::mem::transmute(mszinstancelist), ::core::mem::transmute(pcchinstancelistlength), ::core::mem::transmute(dwdetaillevel), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectItemsHA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hdatasource: isize, szmachinename: Param1, szobjectname: Param2, mszcounterlist: super::super::Foundation::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectItemsHA(hdatasource: isize, szmachinename: super::super::Foundation::PSTR, szobjectname: super::super::Foundation::PSTR, mszcounterlist: super::super::Foundation::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectItemsHA(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(mszcounterlist), ::core::mem::transmute(pcchcounterlistlength), ::core::mem::transmute(mszinstancelist), ::core::mem::transmute(pcchinstancelistlength), ::core::mem::transmute(dwdetaillevel), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectItemsHW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hdatasource: isize, szmachinename: Param1, szobjectname: Param2, mszcounterlist: super::super::Foundation::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectItemsHW(hdatasource: isize, szmachinename: super::super::Foundation::PWSTR, szobjectname: super::super::Foundation::PWSTR, mszcounterlist: super::super::Foundation::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectItemsHW(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(mszcounterlist), ::core::mem::transmute(pcchcounterlistlength), ::core::mem::transmute(mszinstancelist), ::core::mem::transmute(pcchinstancelistlength), ::core::mem::transmute(dwdetaillevel), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectItemsW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0, szmachinename: Param1, szobjectname: Param2, mszcounterlist: super::super::Foundation::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectItemsW(szdatasource: super::super::Foundation::PWSTR, szmachinename: super::super::Foundation::PWSTR, szobjectname: super::super::Foundation::PWSTR, mszcounterlist: super::super::Foundation::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectItemsW(szdatasource.into_param().abi(), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(mszcounterlist), ::core::mem::transmute(pcchcounterlistlength), ::core::mem::transmute(mszinstancelist), ::core::mem::transmute(pcchinstancelistlength), ::core::mem::transmute(dwdetaillevel), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectsA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(szdatasource: Param0, szmachinename: Param1, mszobjectlist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: Param5) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectsA(szdatasource: super::super::Foundation::PSTR, szmachinename: super::super::Foundation::PSTR, mszobjectlist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectsA(szdatasource.into_param().abi(), szmachinename.into_param().abi(), ::core::mem::transmute(mszobjectlist), ::core::mem::transmute(pcchbuffersize), ::core::mem::transmute(dwdetaillevel), brefresh.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectsHA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hdatasource: isize, szmachinename: Param1, mszobjectlist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: Param5) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectsHA(hdatasource: isize, szmachinename: super::super::Foundation::PSTR, mszobjectlist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectsHA(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), ::core::mem::transmute(mszobjectlist), ::core::mem::transmute(pcchbuffersize), ::core::mem::transmute(dwdetaillevel), brefresh.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectsHW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hdatasource: isize, szmachinename: Param1, mszobjectlist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: Param5) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectsHW(hdatasource: isize, szmachinename: super::super::Foundation::PWSTR, mszobjectlist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectsHW(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), ::core::mem::transmute(mszobjectlist), ::core::mem::transmute(pcchbuffersize), ::core::mem::transmute(dwdetaillevel), brefresh.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectsW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(szdatasource: Param0, szmachinename: Param1, mszobjectlist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: Param5) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectsW(szdatasource: super::super::Foundation::PWSTR, szmachinename: super::super::Foundation::PWSTR, mszobjectlist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectsW(szdatasource.into_param().abi(), szmachinename.into_param().abi(), ::core::mem::transmute(mszobjectlist), ::core::mem::transmute(pcchbuffersize), ::core::mem::transmute(dwdetaillevel), brefresh.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhExpandCounterPathA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szwildcardpath: Param0, mszexpandedpathlist: super::super::Foundation::PSTR, pcchpathlistlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhExpandCounterPathA(szwildcardpath: super::super::Foundation::PSTR, mszexpandedpathlist: super::super::Foundation::PSTR, pcchpathlistlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhExpandCounterPathA(szwildcardpath.into_param().abi(), ::core::mem::transmute(mszexpandedpathlist), ::core::mem::transmute(pcchpathlistlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhExpandCounterPathW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szwildcardpath: Param0, mszexpandedpathlist: super::super::Foundation::PWSTR, pcchpathlistlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhExpandCounterPathW(szwildcardpath: super::super::Foundation::PWSTR, mszexpandedpathlist: super::super::Foundation::PWSTR, pcchpathlistlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhExpandCounterPathW(szwildcardpath.into_param().abi(), ::core::mem::transmute(mszexpandedpathlist), ::core::mem::transmute(pcchpathlistlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhExpandWildCardPathA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0, szwildcardpath: Param1, mszexpandedpathlist: super::super::Foundation::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhExpandWildCardPathA(szdatasource: super::super::Foundation::PSTR, szwildcardpath: super::super::Foundation::PSTR, mszexpandedpathlist: super::super::Foundation::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhExpandWildCardPathA(szdatasource.into_param().abi(), szwildcardpath.into_param().abi(), ::core::mem::transmute(mszexpandedpathlist), ::core::mem::transmute(pcchpathlistlength), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhExpandWildCardPathHA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hdatasource: isize, szwildcardpath: Param1, mszexpandedpathlist: super::super::Foundation::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhExpandWildCardPathHA(hdatasource: isize, szwildcardpath: super::super::Foundation::PSTR, mszexpandedpathlist: super::super::Foundation::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhExpandWildCardPathHA(::core::mem::transmute(hdatasource), szwildcardpath.into_param().abi(), ::core::mem::transmute(mszexpandedpathlist), ::core::mem::transmute(pcchpathlistlength), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhExpandWildCardPathHW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hdatasource: isize, szwildcardpath: Param1, mszexpandedpathlist: super::super::Foundation::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhExpandWildCardPathHW(hdatasource: isize, szwildcardpath: super::super::Foundation::PWSTR, mszexpandedpathlist: super::super::Foundation::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhExpandWildCardPathHW(::core::mem::transmute(hdatasource), szwildcardpath.into_param().abi(), ::core::mem::transmute(mszexpandedpathlist), ::core::mem::transmute(pcchpathlistlength), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhExpandWildCardPathW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0, szwildcardpath: Param1, mszexpandedpathlist: super::super::Foundation::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhExpandWildCardPathW(szdatasource: super::super::Foundation::PWSTR, szwildcardpath: super::super::Foundation::PWSTR, mszexpandedpathlist: super::super::Foundation::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhExpandWildCardPathW(szdatasource.into_param().abi(), szwildcardpath.into_param().abi(), ::core::mem::transmute(mszexpandedpathlist), ::core::mem::transmute(pcchpathlistlength), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhFormatFromRawValue(dwcountertype: u32, dwformat: PDH_FMT, ptimebase: *const i64, prawvalue1: *const PDH_RAW_COUNTER, prawvalue2: *const PDH_RAW_COUNTER, pfmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhFormatFromRawValue(dwcountertype: u32, dwformat: PDH_FMT, ptimebase: *const i64, prawvalue1: *const PDH_RAW_COUNTER, prawvalue2: *const PDH_RAW_COUNTER, pfmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
        }
        ::core::mem::transmute(PdhFormatFromRawValue(::core::mem::transmute(dwcountertype), ::core::mem::transmute(dwformat), ::core::mem::transmute(ptimebase), ::core::mem::transmute(prawvalue1), ::core::mem::transmute(prawvalue2), ::core::mem::transmute(pfmtvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetCounterInfoA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(hcounter: isize, bretrieveexplaintext: Param1, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_A) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetCounterInfoA(hcounter: isize, bretrieveexplaintext: super::super::Foundation::BOOLEAN, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_A) -> i32;
        }
        ::core::mem::transmute(PdhGetCounterInfoA(::core::mem::transmute(hcounter), bretrieveexplaintext.into_param().abi(), ::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(lpbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetCounterInfoW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(hcounter: isize, bretrieveexplaintext: Param1, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_W) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetCounterInfoW(hcounter: isize, bretrieveexplaintext: super::super::Foundation::BOOLEAN, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_W) -> i32;
        }
        ::core::mem::transmute(PdhGetCounterInfoW(::core::mem::transmute(hcounter), bretrieveexplaintext.into_param().abi(), ::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(lpbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PdhGetCounterTimeBase(hcounter: isize, ptimebase: *mut i64) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetCounterTimeBase(hcounter: isize, ptimebase: *mut i64) -> i32;
        }
        ::core::mem::transmute(PdhGetCounterTimeBase(::core::mem::transmute(hcounter), ::core::mem::transmute(ptimebase)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDataSourceTimeRangeA(szdatasource: super::super::Foundation::PSTR, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDataSourceTimeRangeA(szdatasource.into_param().abi(), ::core::mem::transmute(pdwnumentries), ::core::mem::transmute(pinfo), ::core::mem::transmute(pdwbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeH(hdatasource: isize, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDataSourceTimeRangeH(hdatasource: isize, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDataSourceTimeRangeH(::core::mem::transmute(hdatasource), ::core::mem::transmute(pdwnumentries), ::core::mem::transmute(pinfo), ::core::mem::transmute(pdwbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDataSourceTimeRangeW(szdatasource: super::super::Foundation::PWSTR, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDataSourceTimeRangeW(szdatasource.into_param().abi(), ::core::mem::transmute(pdwnumentries), ::core::mem::transmute(pinfo), ::core::mem::transmute(pdwbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0, szmachinename: Param1, szobjectname: Param2, szdefaultcountername: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfCounterA(szdatasource: super::super::Foundation::PSTR, szmachinename: super::super::Foundation::PSTR, szobjectname: super::super::Foundation::PSTR, szdefaultcountername: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfCounterA(szdatasource.into_param().abi(), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(szdefaultcountername), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterHA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hdatasource: isize, szmachinename: Param1, szobjectname: Param2, szdefaultcountername: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfCounterHA(hdatasource: isize, szmachinename: super::super::Foundation::PSTR, szobjectname: super::super::Foundation::PSTR, szdefaultcountername: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfCounterHA(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(szdefaultcountername), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterHW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hdatasource: isize, szmachinename: Param1, szobjectname: Param2, szdefaultcountername: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfCounterHW(hdatasource: isize, szmachinename: super::super::Foundation::PWSTR, szobjectname: super::super::Foundation::PWSTR, szdefaultcountername: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfCounterHW(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(szdefaultcountername), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0, szmachinename: Param1, szobjectname: Param2, szdefaultcountername: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfCounterW(szdatasource: super::super::Foundation::PWSTR, szmachinename: super::super::Foundation::PWSTR, szobjectname: super::super::Foundation::PWSTR, szdefaultcountername: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfCounterW(szdatasource.into_param().abi(), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(szdefaultcountername), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0, szmachinename: Param1, szdefaultobjectname: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfObjectA(szdatasource: super::super::Foundation::PSTR, szmachinename: super::super::Foundation::PSTR, szdefaultobjectname: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfObjectA(szdatasource.into_param().abi(), szmachinename.into_param().abi(), ::core::mem::transmute(szdefaultobjectname), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectHA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hdatasource: isize, szmachinename: Param1, szdefaultobjectname: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfObjectHA(hdatasource: isize, szmachinename: super::super::Foundation::PSTR, szdefaultobjectname: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfObjectHA(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), ::core::mem::transmute(szdefaultobjectname), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectHW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hdatasource: isize, szmachinename: Param1, szdefaultobjectname: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfObjectHW(hdatasource: isize, szmachinename: super::super::Foundation::PWSTR, szdefaultobjectname: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfObjectHW(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), ::core::mem::transmute(szdefaultobjectname), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0, szmachinename: Param1, szdefaultobjectname: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfObjectW(szdatasource: super::super::Foundation::PWSTR, szmachinename: super::super::Foundation::PWSTR, szdefaultobjectname: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfObjectW(szdatasource.into_param().abi(), szmachinename.into_param().abi(), ::core::mem::transmute(szdefaultobjectname), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PdhGetDllVersion(lpdwversion: *mut PDH_DLL_VERSION) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDllVersion(lpdwversion: *mut PDH_DLL_VERSION) -> i32;
        }
        ::core::mem::transmute(PdhGetDllVersion(::core::mem::transmute(lpdwversion)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetFormattedCounterArrayA(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_A) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetFormattedCounterArrayA(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_A) -> i32;
        }
        ::core::mem::transmute(PdhGetFormattedCounterArrayA(::core::mem::transmute(hcounter), ::core::mem::transmute(dwformat), ::core::mem::transmute(lpdwbuffersize), ::core::mem::transmute(lpdwitemcount), ::core::mem::transmute(itembuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetFormattedCounterArrayW(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_W) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetFormattedCounterArrayW(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_W) -> i32;
        }
        ::core::mem::transmute(PdhGetFormattedCounterArrayW(::core::mem::transmute(hcounter), ::core::mem::transmute(dwformat), ::core::mem::transmute(lpdwbuffersize), ::core::mem::transmute(lpdwitemcount), ::core::mem::transmute(itembuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetFormattedCounterValue(hcounter: isize, dwformat: PDH_FMT, lpdwtype: *mut u32, pvalue: *mut PDH_FMT_COUNTERVALUE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetFormattedCounterValue(hcounter: isize, dwformat: PDH_FMT, lpdwtype: *mut u32, pvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
        }
        ::core::mem::transmute(PdhGetFormattedCounterValue(::core::mem::transmute(hcounter), ::core::mem::transmute(dwformat), ::core::mem::transmute(lpdwtype), ::core::mem::transmute(pvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PdhGetLogFileSize(hlog: isize, llsize: *mut i64) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetLogFileSize(hlog: isize, llsize: *mut i64) -> i32;
        }
        ::core::mem::transmute(PdhGetLogFileSize(::core::mem::transmute(hlog), ::core::mem::transmute(llsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PdhGetLogSetGUID(hlog: isize, pguid: *mut ::windows::core::GUID, prunid: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetLogSetGUID(hlog: isize, pguid: *mut ::windows::core::GUID, prunid: *mut i32) -> i32;
        }
        ::core::mem::transmute(PdhGetLogSetGUID(::core::mem::transmute(hlog), ::core::mem::transmute(pguid), ::core::mem::transmute(prunid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetRawCounterArrayA(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_A) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetRawCounterArrayA(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_A) -> i32;
        }
        ::core::mem::transmute(PdhGetRawCounterArrayA(::core::mem::transmute(hcounter), ::core::mem::transmute(lpdwbuffersize), ::core::mem::transmute(lpdwitemcount), ::core::mem::transmute(itembuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetRawCounterArrayW(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_W) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetRawCounterArrayW(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_W) -> i32;
        }
        ::core::mem::transmute(PdhGetRawCounterArrayW(::core::mem::transmute(hcounter), ::core::mem::transmute(lpdwbuffersize), ::core::mem::transmute(lpdwitemcount), ::core::mem::transmute(itembuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetRawCounterValue(hcounter: isize, lpdwtype: *mut u32, pvalue: *mut PDH_RAW_COUNTER) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetRawCounterValue(hcounter: isize, lpdwtype: *mut u32, pvalue: *mut PDH_RAW_COUNTER) -> i32;
        }
        ::core::mem::transmute(PdhGetRawCounterValue(::core::mem::transmute(hcounter), ::core::mem::transmute(lpdwtype), ::core::mem::transmute(pvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhIsRealTimeQuery(hquery: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhIsRealTimeQuery(hquery: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PdhIsRealTimeQuery(::core::mem::transmute(hquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhLookupPerfIndexByNameA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szmachinename: Param0, sznamebuffer: Param1, pdwindex: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhLookupPerfIndexByNameA(szmachinename: super::super::Foundation::PSTR, sznamebuffer: super::super::Foundation::PSTR, pdwindex: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhLookupPerfIndexByNameA(szmachinename.into_param().abi(), sznamebuffer.into_param().abi(), ::core::mem::transmute(pdwindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhLookupPerfIndexByNameW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szmachinename: Param0, sznamebuffer: Param1, pdwindex: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhLookupPerfIndexByNameW(szmachinename: super::super::Foundation::PWSTR, sznamebuffer: super::super::Foundation::PWSTR, pdwindex: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhLookupPerfIndexByNameW(szmachinename.into_param().abi(), sznamebuffer.into_param().abi(), ::core::mem::transmute(pdwindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhLookupPerfNameByIndexA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szmachinename: Param0, dwnameindex: u32, sznamebuffer: super::super::Foundation::PSTR, pcchnamebuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhLookupPerfNameByIndexA(szmachinename: super::super::Foundation::PSTR, dwnameindex: u32, sznamebuffer: super::super::Foundation::PSTR, pcchnamebuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhLookupPerfNameByIndexA(szmachinename.into_param().abi(), ::core::mem::transmute(dwnameindex), ::core::mem::transmute(sznamebuffer), ::core::mem::transmute(pcchnamebuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhLookupPerfNameByIndexW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szmachinename: Param0, dwnameindex: u32, sznamebuffer: super::super::Foundation::PWSTR, pcchnamebuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhLookupPerfNameByIndexW(szmachinename: super::super::Foundation::PWSTR, dwnameindex: u32, sznamebuffer: super::super::Foundation::PWSTR, pcchnamebuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhLookupPerfNameByIndexW(szmachinename.into_param().abi(), ::core::mem::transmute(dwnameindex), ::core::mem::transmute(sznamebuffer), ::core::mem::transmute(pcchnamebuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhMakeCounterPathA(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer: super::super::Foundation::PSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhMakeCounterPathA(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer: super::super::Foundation::PSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32;
        }
        ::core::mem::transmute(PdhMakeCounterPathA(::core::mem::transmute(pcounterpathelements), ::core::mem::transmute(szfullpathbuffer), ::core::mem::transmute(pcchbuffersize), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhMakeCounterPathW(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhMakeCounterPathW(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32;
        }
        ::core::mem::transmute(PdhMakeCounterPathW(::core::mem::transmute(pcounterpathelements), ::core::mem::transmute(szfullpathbuffer), ::core::mem::transmute(pcchbuffersize), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhOpenLogA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szlogfilename: Param0, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: Param5, phlog: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhOpenLogA(szlogfilename: super::super::Foundation::PSTR, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: super::super::Foundation::PSTR, phlog: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhOpenLogA(szlogfilename.into_param().abi(), ::core::mem::transmute(dwaccessflags), ::core::mem::transmute(lpdwlogtype), ::core::mem::transmute(hquery), ::core::mem::transmute(dwmaxsize), szusercaption.into_param().abi(), ::core::mem::transmute(phlog)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhOpenLogW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szlogfilename: Param0, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: Param5, phlog: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhOpenLogW(szlogfilename: super::super::Foundation::PWSTR, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: super::super::Foundation::PWSTR, phlog: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhOpenLogW(szlogfilename.into_param().abi(), ::core::mem::transmute(dwaccessflags), ::core::mem::transmute(lpdwlogtype), ::core::mem::transmute(hquery), ::core::mem::transmute(dwmaxsize), szusercaption.into_param().abi(), ::core::mem::transmute(phlog)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhOpenQueryA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0, dwuserdata: usize, phquery: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhOpenQueryA(szdatasource: super::super::Foundation::PSTR, dwuserdata: usize, phquery: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhOpenQueryA(szdatasource.into_param().abi(), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PdhOpenQueryH(hdatasource: isize, dwuserdata: usize, phquery: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhOpenQueryH(hdatasource: isize, dwuserdata: usize, phquery: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhOpenQueryH(::core::mem::transmute(hdatasource), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhOpenQueryW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0, dwuserdata: usize, phquery: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhOpenQueryW(szdatasource: super::super::Foundation::PWSTR, dwuserdata: usize, phquery: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhOpenQueryW(szdatasource.into_param().abi(), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhParseCounterPathA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szfullpathbuffer: Param0, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_A, pdwbuffersize: *mut u32, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhParseCounterPathA(szfullpathbuffer: super::super::Foundation::PSTR, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_A, pdwbuffersize: *mut u32, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhParseCounterPathA(szfullpathbuffer.into_param().abi(), ::core::mem::transmute(pcounterpathelements), ::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhParseCounterPathW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szfullpathbuffer: Param0, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_W, pdwbuffersize: *mut u32, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhParseCounterPathW(szfullpathbuffer: super::super::Foundation::PWSTR, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_W, pdwbuffersize: *mut u32, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhParseCounterPathW(szfullpathbuffer.into_param().abi(), ::core::mem::transmute(pcounterpathelements), ::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhParseInstanceNameA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szinstancestring: Param0, szinstancename: super::super::Foundation::PSTR, pcchinstancenamelength: *mut u32, szparentname: super::super::Foundation::PSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhParseInstanceNameA(szinstancestring: super::super::Foundation::PSTR, szinstancename: super::super::Foundation::PSTR, pcchinstancenamelength: *mut u32, szparentname: super::super::Foundation::PSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhParseInstanceNameA(szinstancestring.into_param().abi(), ::core::mem::transmute(szinstancename), ::core::mem::transmute(pcchinstancenamelength), ::core::mem::transmute(szparentname), ::core::mem::transmute(pcchparentnamelength), ::core::mem::transmute(lpindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhParseInstanceNameW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szinstancestring: Param0, szinstancename: super::super::Foundation::PWSTR, pcchinstancenamelength: *mut u32, szparentname: super::super::Foundation::PWSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhParseInstanceNameW(szinstancestring: super::super::Foundation::PWSTR, szinstancename: super::super::Foundation::PWSTR, pcchinstancenamelength: *mut u32, szparentname: super::super::Foundation::PWSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhParseInstanceNameW(szinstancestring.into_param().abi(), ::core::mem::transmute(szinstancename), ::core::mem::transmute(pcchinstancenamelength), ::core::mem::transmute(szparentname), ::core::mem::transmute(pcchparentnamelength), ::core::mem::transmute(lpindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhReadRawLogRecord<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>>(hlog: isize, ftrecord: Param1, prawlogrecord: *mut PDH_RAW_LOG_RECORD, pdwbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhReadRawLogRecord(hlog: isize, ftrecord: super::super::Foundation::FILETIME, prawlogrecord: *mut PDH_RAW_LOG_RECORD, pdwbufferlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhReadRawLogRecord(::core::mem::transmute(hlog), ftrecord.into_param().abi(), ::core::mem::transmute(prawlogrecord), ::core::mem::transmute(pdwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PdhRemoveCounter(hcounter: isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhRemoveCounter(hcounter: isize) -> i32;
        }
        ::core::mem::transmute(PdhRemoveCounter(::core::mem::transmute(hcounter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhSelectDataSourceA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hwndowner: Param0, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: Param2, pcchbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhSelectDataSourceA(hwndowner: super::super::Foundation::HWND, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: super::super::Foundation::PSTR, pcchbufferlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhSelectDataSourceA(hwndowner.into_param().abi(), ::core::mem::transmute(dwflags), szdatasource.into_param().abi(), ::core::mem::transmute(pcchbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhSelectDataSourceW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hwndowner: Param0, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: Param2, pcchbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhSelectDataSourceW(hwndowner: super::super::Foundation::HWND, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: super::super::Foundation::PWSTR, pcchbufferlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhSelectDataSourceW(hwndowner.into_param().abi(), ::core::mem::transmute(dwflags), szdatasource.into_param().abi(), ::core::mem::transmute(pcchbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PdhSetCounterScaleFactor(hcounter: isize, lfactor: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhSetCounterScaleFactor(hcounter: isize, lfactor: i32) -> i32;
        }
        ::core::mem::transmute(PdhSetCounterScaleFactor(::core::mem::transmute(hcounter), ::core::mem::transmute(lfactor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PdhSetDefaultRealTimeDataSource(dwdatasourceid: REAL_TIME_DATA_SOURCE_ID_FLAGS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhSetDefaultRealTimeDataSource(dwdatasourceid: REAL_TIME_DATA_SOURCE_ID_FLAGS) -> i32;
        }
        ::core::mem::transmute(PdhSetDefaultRealTimeDataSource(::core::mem::transmute(dwdatasourceid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PdhSetLogSetRunID(hlog: isize, runid: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhSetLogSetRunID(hlog: isize, runid: i32) -> i32;
        }
        ::core::mem::transmute(PdhSetLogSetRunID(::core::mem::transmute(hlog), ::core::mem::transmute(runid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PdhSetQueryTimeRange(hquery: isize, pinfo: *const PDH_TIME_INFO) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhSetQueryTimeRange(hquery: isize, pinfo: *const PDH_TIME_INFO) -> i32;
        }
        ::core::mem::transmute(PdhSetQueryTimeRange(::core::mem::transmute(hquery), ::core::mem::transmute(pinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhUpdateLogA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hlog: isize, szuserstring: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhUpdateLogA(hlog: isize, szuserstring: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(PdhUpdateLogA(::core::mem::transmute(hlog), szuserstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PdhUpdateLogFileCatalog(hlog: isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhUpdateLogFileCatalog(hlog: isize) -> i32;
        }
        ::core::mem::transmute(PdhUpdateLogFileCatalog(::core::mem::transmute(hlog)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhUpdateLogW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hlog: isize, szuserstring: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhUpdateLogW(hlog: isize, szuserstring: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(PdhUpdateLogW(::core::mem::transmute(hlog), szuserstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhValidatePathA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szfullpathbuffer: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhValidatePathA(szfullpathbuffer: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(PdhValidatePathA(szfullpathbuffer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhValidatePathExA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hdatasource: isize, szfullpathbuffer: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhValidatePathExA(hdatasource: isize, szfullpathbuffer: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(PdhValidatePathExA(::core::mem::transmute(hdatasource), szfullpathbuffer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhValidatePathExW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hdatasource: isize, szfullpathbuffer: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhValidatePathExW(hdatasource: isize, szfullpathbuffer: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(PdhValidatePathExW(::core::mem::transmute(hdatasource), szfullpathbuffer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhValidatePathW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szfullpathbuffer: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhValidatePathW(szfullpathbuffer: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(PdhValidatePathW(szfullpathbuffer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhVerifySQLDBA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhVerifySQLDBA(szdatasource: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(PdhVerifySQLDBA(szdatasource.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhVerifySQLDBW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhVerifySQLDBW(szdatasource: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(PdhVerifySQLDBW(szdatasource.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PerfAddCounters<'a, Param0: ::windows::core::IntoParam<'a, PerfQueryHandle>>(hquery: Param0, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfAddCounters(hquery: PerfQueryHandle, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32;
        }
        ::core::mem::transmute(PerfAddCounters(hquery.into_param().abi(), ::core::mem::transmute(pcounters), ::core::mem::transmute(cbcounters)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfCloseQueryHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hquery: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfCloseQueryHandle(hquery: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(PerfCloseQueryHandle(hquery.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type PerfCounterDataType = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_ERROR_RETURN: PerfCounterDataType = 0i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_SINGLE_COUNTER: PerfCounterDataType = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_MULTIPLE_COUNTERS: PerfCounterDataType = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_MULTIPLE_INSTANCES: PerfCounterDataType = 4i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_COUNTERSET: PerfCounterDataType = 6i32;
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfCreateInstance<'a, Param0: ::windows::core::IntoParam<'a, PerfProviderHandle>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(providerhandle: Param0, countersetguid: *const ::windows::core::GUID, name: Param2, id: u32) -> *mut PERF_COUNTERSET_INSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfCreateInstance(providerhandle: PerfProviderHandle, countersetguid: *const ::windows::core::GUID, name: super::super::Foundation::PWSTR, id: u32) -> *mut PERF_COUNTERSET_INSTANCE;
        }
        ::core::mem::transmute(PerfCreateInstance(providerhandle.into_param().abi(), ::core::mem::transmute(countersetguid), name.into_param().abi(), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfDecrementULongCounterValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfDecrementULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
        }
        ::core::mem::transmute(PerfDecrementULongCounterValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfDecrementULongLongCounterValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfDecrementULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
        }
        ::core::mem::transmute(PerfDecrementULongLongCounterValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PerfDeleteCounters<'a, Param0: ::windows::core::IntoParam<'a, PerfQueryHandle>>(hquery: Param0, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfDeleteCounters(hquery: PerfQueryHandle, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32;
        }
        ::core::mem::transmute(PerfDeleteCounters(hquery.into_param().abi(), ::core::mem::transmute(pcounters), ::core::mem::transmute(cbcounters)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PerfDeleteInstance<'a, Param0: ::windows::core::IntoParam<'a, PerfProviderHandle>>(provider: Param0, instanceblock: *const PERF_COUNTERSET_INSTANCE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfDeleteInstance(provider: PerfProviderHandle, instanceblock: *const PERF_COUNTERSET_INSTANCE) -> u32;
        }
        ::core::mem::transmute(PerfDeleteInstance(provider.into_param().abi(), ::core::mem::transmute(instanceblock)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfEnumerateCounterSet<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szmachine: Param0, pcountersetids: *mut ::windows::core::GUID, ccountersetids: u32, pccountersetidsactual: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfEnumerateCounterSet(szmachine: super::super::Foundation::PWSTR, pcountersetids: *mut ::windows::core::GUID, ccountersetids: u32, pccountersetidsactual: *mut u32) -> u32;
        }
        ::core::mem::transmute(PerfEnumerateCounterSet(szmachine.into_param().abi(), ::core::mem::transmute(pcountersetids), ::core::mem::transmute(ccountersetids), ::core::mem::transmute(pccountersetidsactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfEnumerateCounterSetInstances<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szmachine: Param0, pcountersetid: *const ::windows::core::GUID, pinstances: *mut PERF_INSTANCE_HEADER, cbinstances: u32, pcbinstancesactual: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfEnumerateCounterSetInstances(szmachine: super::super::Foundation::PWSTR, pcountersetid: *const ::windows::core::GUID, pinstances: *mut PERF_INSTANCE_HEADER, cbinstances: u32, pcbinstancesactual: *mut u32) -> u32;
        }
        ::core::mem::transmute(PerfEnumerateCounterSetInstances(szmachine.into_param().abi(), ::core::mem::transmute(pcountersetid), ::core::mem::transmute(pinstances), ::core::mem::transmute(cbinstances), ::core::mem::transmute(pcbinstancesactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfIncrementULongCounterValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfIncrementULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
        }
        ::core::mem::transmute(PerfIncrementULongCounterValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfIncrementULongLongCounterValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfIncrementULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
        }
        ::core::mem::transmute(PerfIncrementULongLongCounterValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfOpenQueryHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szmachine: Param0, phquery: *mut PerfQueryHandle) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfOpenQueryHandle(szmachine: super::super::Foundation::PWSTR, phquery: *mut PerfQueryHandle) -> u32;
        }
        ::core::mem::transmute(PerfOpenQueryHandle(szmachine.into_param().abi(), ::core::mem::transmute(phquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PerfProviderHandle(pub isize);
impl PerfProviderHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for PerfProviderHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PerfProviderHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PerfProviderHandle {}
impl ::core::fmt::Debug for PerfProviderHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerfProviderHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for PerfProviderHandle {
    type Abi = Self;
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfQueryCounterData<'a, Param0: ::windows::core::IntoParam<'a, PerfQueryHandle>>(hquery: Param0, pcounterblock: *mut PERF_DATA_HEADER, cbcounterblock: u32, pcbcounterblockactual: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfQueryCounterData(hquery: PerfQueryHandle, pcounterblock: *mut PERF_DATA_HEADER, cbcounterblock: u32, pcbcounterblockactual: *mut u32) -> u32;
        }
        ::core::mem::transmute(PerfQueryCounterData(hquery.into_param().abi(), ::core::mem::transmute(pcounterblock), ::core::mem::transmute(cbcounterblock), ::core::mem::transmute(pcbcounterblockactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PerfQueryCounterInfo<'a, Param0: ::windows::core::IntoParam<'a, PerfQueryHandle>>(hquery: Param0, pcounters: *mut PERF_COUNTER_IDENTIFIER, cbcounters: u32, pcbcountersactual: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfQueryCounterInfo(hquery: PerfQueryHandle, pcounters: *mut PERF_COUNTER_IDENTIFIER, cbcounters: u32, pcbcountersactual: *mut u32) -> u32;
        }
        ::core::mem::transmute(PerfQueryCounterInfo(hquery.into_param().abi(), ::core::mem::transmute(pcounters), ::core::mem::transmute(cbcounters), ::core::mem::transmute(pcbcountersactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfQueryCounterSetRegistrationInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szmachine: Param0, pcountersetid: *const ::windows::core::GUID, requestcode: PerfRegInfoType, requestlangid: u32, pbreginfo: *mut u8, cbreginfo: u32, pcbreginfoactual: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfQueryCounterSetRegistrationInfo(szmachine: super::super::Foundation::PWSTR, pcountersetid: *const ::windows::core::GUID, requestcode: PerfRegInfoType, requestlangid: u32, pbreginfo: *mut u8, cbreginfo: u32, pcbreginfoactual: *mut u32) -> u32;
        }
        ::core::mem::transmute(PerfQueryCounterSetRegistrationInfo(szmachine.into_param().abi(), ::core::mem::transmute(pcountersetid), ::core::mem::transmute(requestcode), ::core::mem::transmute(requestlangid), ::core::mem::transmute(pbreginfo), ::core::mem::transmute(cbreginfo), ::core::mem::transmute(pcbreginfoactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PerfQueryHandle(pub isize);
impl PerfQueryHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for PerfQueryHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PerfQueryHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PerfQueryHandle {}
impl ::core::fmt::Debug for PerfQueryHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerfQueryHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for PerfQueryHandle {
    type Abi = Self;
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfQueryInstance<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(providerhandle: Param0, countersetguid: *const ::windows::core::GUID, name: Param2, id: u32) -> *mut PERF_COUNTERSET_INSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfQueryInstance(providerhandle: super::super::Foundation::HANDLE, countersetguid: *const ::windows::core::GUID, name: super::super::Foundation::PWSTR, id: u32) -> *mut PERF_COUNTERSET_INSTANCE;
        }
        ::core::mem::transmute(PerfQueryInstance(providerhandle.into_param().abi(), ::core::mem::transmute(countersetguid), name.into_param().abi(), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type PerfRegInfoType = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_REG_COUNTERSET_STRUCT: PerfRegInfoType = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_REG_COUNTER_STRUCT: PerfRegInfoType = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_REG_COUNTERSET_NAME_STRING: PerfRegInfoType = 3i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_REG_COUNTERSET_HELP_STRING: PerfRegInfoType = 4i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_REG_COUNTER_NAME_STRINGS: PerfRegInfoType = 5i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_REG_COUNTER_HELP_STRINGS: PerfRegInfoType = 6i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_REG_PROVIDER_NAME: PerfRegInfoType = 7i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_REG_PROVIDER_GUID: PerfRegInfoType = 8i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_REG_COUNTERSET_ENGLISH_NAME: PerfRegInfoType = 9i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const PERF_REG_COUNTER_ENGLISH_NAMES: PerfRegInfoType = 10i32;
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfSetCounterRefValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, address: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfSetCounterRefValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, address: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(PerfSetCounterRefValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(address)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfSetCounterSetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(providerhandle: Param0, template: *mut PERF_COUNTERSET_INFO, templatesize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfSetCounterSetInfo(providerhandle: super::super::Foundation::HANDLE, template: *mut PERF_COUNTERSET_INFO, templatesize: u32) -> u32;
        }
        ::core::mem::transmute(PerfSetCounterSetInfo(providerhandle.into_param().abi(), ::core::mem::transmute(template), ::core::mem::transmute(templatesize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfSetULongCounterValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfSetULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
        }
        ::core::mem::transmute(PerfSetULongCounterValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfSetULongLongCounterValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfSetULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
        }
        ::core::mem::transmute(PerfSetULongLongCounterValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PerfStartProvider(providerguid: *const ::windows::core::GUID, controlcallback: PERFLIBREQUEST, phprovider: *mut PerfProviderHandle) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfStartProvider(providerguid: *const ::windows::core::GUID, controlcallback: ::windows::core::RawPtr, phprovider: *mut PerfProviderHandle) -> u32;
        }
        ::core::mem::transmute(PerfStartProvider(::core::mem::transmute(providerguid), ::core::mem::transmute(controlcallback), ::core::mem::transmute(phprovider)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PerfStartProviderEx(providerguid: *const ::windows::core::GUID, providercontext: *const PERF_PROVIDER_CONTEXT, provider: *mut PerfProviderHandle) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfStartProviderEx(providerguid: *const ::windows::core::GUID, providercontext: *const PERF_PROVIDER_CONTEXT, provider: *mut PerfProviderHandle) -> u32;
        }
        ::core::mem::transmute(PerfStartProviderEx(::core::mem::transmute(providerguid), ::core::mem::transmute(providercontext), ::core::mem::transmute(provider)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[inline]
pub unsafe fn PerfStopProvider<'a, Param0: ::windows::core::IntoParam<'a, PerfProviderHandle>>(providerhandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfStopProvider(providerhandle: PerfProviderHandle) -> u32;
        }
        ::core::mem::transmute(PerfStopProvider(providerhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryPerformanceCounter(lpperformancecount: *mut i64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryPerformanceCounter(lpperformancecount: *mut i64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryPerformanceCounter(::core::mem::transmute(lpperformancecount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryPerformanceFrequency(lpfrequency: *mut i64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryPerformanceFrequency(lpfrequency: *mut i64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryPerformanceFrequency(::core::mem::transmute(lpfrequency)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type REAL_TIME_DATA_SOURCE_ID_FLAGS = u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const DATA_SOURCE_REGISTRY: REAL_TIME_DATA_SOURCE_ID_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const DATA_SOURCE_WBEM: REAL_TIME_DATA_SOURCE_ID_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type ReportValueTypeConstants = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonDefaultValue: ReportValueTypeConstants = 0i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonCurrentValue: ReportValueTypeConstants = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonAverage: ReportValueTypeConstants = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonMinimum: ReportValueTypeConstants = 3i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonMaximum: ReportValueTypeConstants = 4i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type ResourcePolicy = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaDeleteLargest: ResourcePolicy = 0i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaDeleteOldest: ResourcePolicy = 1i32;
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RestorePerfRegistryFromFileW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szfilename: Param0, szlangid: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RestorePerfRegistryFromFileW(szfilename: super::super::Foundation::PWSTR, szlangid: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(RestorePerfRegistryFromFileW(szfilename.into_param().abi(), szlangid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const S_PDH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04d66358_c4a1_419b_8023_23b73902de2c);
pub const ServerDataCollectorSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837531_098b_11d8_9414_505054503030);
pub const ServerDataCollectorSetCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837532_098b_11d8_9414_505054503030);
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetServiceAsTrustedA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szreserved: Param0, szservicename: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetServiceAsTrustedA(szreserved: super::super::Foundation::PSTR, szservicename: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(SetServiceAsTrustedA(szreserved.into_param().abi(), szservicename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetServiceAsTrustedW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szreserved: Param0, szservicename: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetServiceAsTrustedW(szreserved: super::super::Foundation::PWSTR, szservicename: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(SetServiceAsTrustedW(szreserved.into_param().abi(), szservicename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SourcePropPage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cf32aa1_7571_11d0_93c4_00aa00a3ddea);
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type StreamMode = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaFile: StreamMode = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaRealTime: StreamMode = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaBoth: StreamMode = 3i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaBuffering: StreamMode = 4i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type SysmonBatchReason = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonBatchNone: SysmonBatchReason = 0i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonBatchAddFiles: SysmonBatchReason = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonBatchAddCounters: SysmonBatchReason = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonBatchAddFilesAutoCounters: SysmonBatchReason = 3i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type SysmonDataType = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonDataAvg: SysmonDataType = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonDataMin: SysmonDataType = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonDataMax: SysmonDataType = 3i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonDataTime: SysmonDataType = 4i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonDataCount: SysmonDataType = 5i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type SysmonFileType = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonFileHtml: SysmonFileType = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonFileReport: SysmonFileType = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonFileCsv: SysmonFileType = 3i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonFileTsv: SysmonFileType = 4i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonFileBlg: SysmonFileType = 5i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonFileRetiredBlg: SysmonFileType = 6i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const sysmonFileGif: SysmonFileType = 7i32;
pub const SystemDataCollectorSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837546_098b_11d8_9414_505054503030);
pub const SystemDataCollectorSetCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837547_098b_11d8_9414_505054503030);
pub const SystemMonitor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4d2d8e0_d1dd_11ce_940f_008029004347);
pub const SystemMonitor2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f30578c_5f38_4612_acfe_6ed04c7b7af8);
pub const TraceDataProvider: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837513_098b_11d8_9414_505054503030);
pub const TraceDataProviderCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837511_098b_11d8_9414_505054503030);
pub const TraceSession: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0383751c_098b_11d8_9414_505054503030);
pub const TraceSessionCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837530_098b_11d8_9414_505054503030);
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnloadPerfCounterTextStringsA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpcommandline: Param0, bquietmodearg: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnloadPerfCounterTextStringsA(lpcommandline: super::super::Foundation::PSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(UnloadPerfCounterTextStringsA(lpcommandline.into_param().abi(), bquietmodearg.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnloadPerfCounterTextStringsW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpcommandline: Param0, bquietmodearg: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnloadPerfCounterTextStringsW(lpcommandline: super::super::Foundation::PWSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(UnloadPerfCounterTextStringsW(lpcommandline.into_param().abi(), bquietmodearg.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdatePerfNameFilesA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(sznewctrfilepath: Param0, sznewhlpfilepath: Param1, szlanguageid: Param2, dwflags: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdatePerfNameFilesA(sznewctrfilepath: super::super::Foundation::PSTR, sznewhlpfilepath: super::super::Foundation::PSTR, szlanguageid: super::super::Foundation::PSTR, dwflags: usize) -> u32;
        }
        ::core::mem::transmute(UpdatePerfNameFilesA(sznewctrfilepath.into_param().abi(), sznewhlpfilepath.into_param().abi(), szlanguageid.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdatePerfNameFilesW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(sznewctrfilepath: Param0, sznewhlpfilepath: Param1, szlanguageid: Param2, dwflags: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdatePerfNameFilesW(sznewctrfilepath: super::super::Foundation::PWSTR, sznewhlpfilepath: super::super::Foundation::PWSTR, szlanguageid: super::super::Foundation::PWSTR, dwflags: usize) -> u32;
        }
        ::core::mem::transmute(UpdatePerfNameFilesW(sznewctrfilepath.into_param().abi(), sznewhlpfilepath.into_param().abi(), szlanguageid.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type ValueMapType = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaIndex: ValueMapType = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaFlag: ValueMapType = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaFlagArray: ValueMapType = 3i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaValidation: ValueMapType = 4i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const WINPERF_LOG_DEBUG: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const WINPERF_LOG_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const WINPERF_LOG_USER: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const WINPERF_LOG_VERBOSE: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub type WeekDays = i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaRunOnce: WeekDays = 0i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaSunday: WeekDays = 1i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaMonday: WeekDays = 2i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaTuesday: WeekDays = 4i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaWednesday: WeekDays = 8i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaThursday: WeekDays = 16i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaFriday: WeekDays = 32i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaSaturday: WeekDays = 64i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
pub const plaEveryday: WeekDays = 127i32;
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct _ICounterItemUnion(::windows::core::IUnknown);
impl _ICounterItemUnion {
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Value(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Color(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetWidth(&self, iwidth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(iwidth)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Width(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLineStyle(&self, ilinestyle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ilinestyle)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LineStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetScaleFactor(&self, iscale: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(iscale)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ScaleFactor(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn GetValue(&self, value: *mut f64, status: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn GetStatistics(&self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(max), ::core::mem::transmute(min), ::core::mem::transmute(avg), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetSelected(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Selected(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetVisible(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Visible(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetDataAt(&self, iindex: i32, iwhich: SysmonDataType) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), ::core::mem::transmute(iwhich), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<_ICounterItemUnion> for ::windows::core::IUnknown {
    fn from(value: _ICounterItemUnion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&_ICounterItemUnion> for ::windows::core::IUnknown {
    fn from(value: &_ICounterItemUnion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for _ICounterItemUnion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &_ICounterItemUnion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for _ICounterItemUnion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for _ICounterItemUnion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for _ICounterItemUnion {}
impl ::core::fmt::Debug for _ICounterItemUnion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_ICounterItemUnion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for _ICounterItemUnion {
    type Vtable = _ICounterItemUnionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde1a6b74_9182_4c41_8e2c_24c2cd30ee83);
}
#[repr(C)]
#[doc(hidden)]
pub struct _ICounterItemUnionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdblvalue: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iwidth: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ilinestyle: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iscale: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64, status: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: i32, iwhich: SysmonDataType, pvariant: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_System_Performance'*"]
#[repr(transparent)]
pub struct _ISystemMonitorUnion(::windows::core::IUnknown);
impl _ISystemMonitorUnion {
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Appearance(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetAppearance(&self, iappearance: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(iappearance)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn BackColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetBackColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn BorderStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetBorderStyle(&self, iborderstyle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(iborderstyle)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ForeColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetForeColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Ole'*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn Font(&self) -> ::windows::core::Result<super::Ole::IFontDisp> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Ole::IFontDisp>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_System_Ole'*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn putref_Font<'a, Param0: ::windows::core::IntoParam<'a, super::Ole::IFontDisp>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Counters(&self) -> ::windows::core::Result<ICounters> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ICounters>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowVerticalGrid(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowVerticalGrid(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowHorizontalGrid(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowHorizontalGrid(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowLegend(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowLegend(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowScaleLabels(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowScaleLabels(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowValueBar(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowValueBar(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetMaximumScale(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn MaximumScale(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetMinimumScale(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn MinimumScale(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetUpdateInterval(&self, fvalue: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(fvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn UpdateInterval(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDisplayType(&self, edisplaytype: DisplayTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(edisplaytype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DisplayType(&self) -> ::windows::core::Result<DisplayTypeConstants> {
        let mut result__: DisplayTypeConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DisplayTypeConstants>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetManualUpdate(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ManualUpdate(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGraphTitle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), bstitle.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GraphTitle(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetYAxisLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), bstitle.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn YAxisLabel(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn CollectSample(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn UpdateGraph(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn BrowseCounters(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DisplayProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Counter(&self, iindex: i32) -> ::windows::core::Result<ICounterItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), ::core::mem::transmute(&mut result__)).from_abi::<ICounterItem>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddCounter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bspath: Param0) -> ::windows::core::Result<ICounterItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), bspath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ICounterItem>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DeleteCounter<'a, Param0: ::windows::core::IntoParam<'a, ICounterItem>>(&self, pctr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), pctr.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn BackColorCtl(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetBackColorCtl(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bsfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), bsfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogViewStart(&self, starttime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogViewStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogViewStop(&self, stoptime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(stoptime)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogViewStop(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn GridColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetGridColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn TimeBarColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetTimeBarColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Highlight(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetHighlight(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowToolbar(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowToolbar(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Paste(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Copy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetReadOnly(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ReadOnly(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetReportValueType(&self, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(ereportvaluetype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ReportValueType(&self) -> ::windows::core::Result<ReportValueTypeConstants> {
        let mut result__: ReportValueTypeConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ReportValueTypeConstants>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetMonitorDuplicateInstances(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn MonitorDuplicateInstances(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDisplayFilter(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DisplayFilter(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).71)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogFiles(&self) -> ::windows::core::Result<ILogFiles> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).72)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ILogFiles>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDataSourceType(&self, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).73)(::core::mem::transmute_copy(self), ::core::mem::transmute(edatasourcetype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataSourceType(&self) -> ::windows::core::Result<DataSourceTypeConstants> {
        let mut result__: DataSourceTypeConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).74)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataSourceTypeConstants>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSqlDsnName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bssqldsnname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).75)(::core::mem::transmute_copy(self), bssqldsnname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqlDsnName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).76)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSqlLogSetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bssqllogsetname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).77)(::core::mem::transmute_copy(self), bssqllogsetname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqlLogSetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).78)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetEnableDigitGrouping(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).79)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn EnableDigitGrouping(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).80)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetEnableToolTips(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).81)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn EnableToolTips(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).82)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetShowTimeAxisLabels(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).83)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ShowTimeAxisLabels(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).84)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetChartScroll(&self, bscroll: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).85)(::core::mem::transmute_copy(self), ::core::mem::transmute(bscroll)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ChartScroll(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).86)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetDataPointCount(&self, inewcount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).87)(::core::mem::transmute_copy(self), ::core::mem::transmute(inewcount)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn DataPointCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).88)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ScaleToFit(&self, bselectedcountersonly: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).89)(::core::mem::transmute_copy(self), ::core::mem::transmute(bselectedcountersonly)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveAs<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfilename: Param0, esysmonfiletype: SysmonFileType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).90)(::core::mem::transmute_copy(self), bstrfilename.into_param().abi(), ::core::mem::transmute(esysmonfiletype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Relog<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfilename: Param0, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).91)(::core::mem::transmute_copy(self), bstrfilename.into_param().abi(), ::core::mem::transmute(esysmonfiletype), ::core::mem::transmute(ifilter)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn ClearData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).92)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogSourceStartTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).93)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn LogSourceStopTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).94)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn SetLogViewRange(&self, starttime: f64, stoptime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).95)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime), ::core::mem::transmute(stoptime)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn GetLogViewRange(&self, starttime: *mut f64, stoptime: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).96)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime), ::core::mem::transmute(stoptime)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance'*"]
    pub unsafe fn BatchingLock(&self, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).97)(::core::mem::transmute_copy(self), ::core::mem::transmute(flock), ::core::mem::transmute(ebatchreason)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Performance', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadSettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsettingfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).98)(::core::mem::transmute_copy(self), bstrsettingfilename.into_param().abi()).ok()
    }
}
impl ::core::convert::From<_ISystemMonitorUnion> for ::windows::core::IUnknown {
    fn from(value: _ISystemMonitorUnion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&_ISystemMonitorUnion> for ::windows::core::IUnknown {
    fn from(value: &_ISystemMonitorUnion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for _ISystemMonitorUnion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &_ISystemMonitorUnion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for _ISystemMonitorUnion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for _ISystemMonitorUnion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for _ISystemMonitorUnion {}
impl ::core::fmt::Debug for _ISystemMonitorUnion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_ISystemMonitorUnion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for _ISystemMonitorUnion {
    type Vtable = _ISystemMonitorUnionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8a77338_265f_4de5_aa25_c7da1ce5a8f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct _ISystemMonitorUnionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iappearance: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iappearance: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iborderstyle: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iborderstyle: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppicounters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fvalue: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, edisplaytype: DisplayTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pedisplaytype: *mut DisplayTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: i32, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctr: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stoptime: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stoptime: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pereportvaluetype: *mut ReportValueTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppilogfiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pedatasourcetype: *mut DataSourceTypeConstants) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqldsnname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqldsnname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqllogsetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqllogsetname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bscroll: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbscroll: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inewcount: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidatapointcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bselectedcountersonly: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: f64, stoptime: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: *mut f64, stoptime: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsettingfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
