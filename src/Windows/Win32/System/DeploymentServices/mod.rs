#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CPU_ARCHITECTURE(pub u32);
pub const CPU_ARCHITECTURE_AMD64: CPU_ARCHITECTURE = CPU_ARCHITECTURE(9u32);
pub const CPU_ARCHITECTURE_IA64: CPU_ARCHITECTURE = CPU_ARCHITECTURE(6u32);
pub const CPU_ARCHITECTURE_INTEL: CPU_ARCHITECTURE = CPU_ARCHITECTURE(0u32);
impl ::std::convert::From<u32> for CPU_ARCHITECTURE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CPU_ARCHITECTURE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CPU_ARCHITECTURE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CPU_ARCHITECTURE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CPU_ARCHITECTURE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CPU_ARCHITECTURE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CPU_ARCHITECTURE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const EVT_WDSMCS_E_CP_CALLBACKS_NOT_REG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801324i32 as _);
pub const EVT_WDSMCS_E_CP_CLOSE_INSTANCE_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801320i32 as _);
pub const EVT_WDSMCS_E_CP_DLL_LOAD_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801328i32 as _);
pub const EVT_WDSMCS_E_CP_DLL_LOAD_FAILED_CRITICAL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801317i32 as _);
pub const EVT_WDSMCS_E_CP_INCOMPATIBLE_SERVER_VERSION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801325i32 as _);
pub const EVT_WDSMCS_E_CP_INIT_FUNC_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801326i32 as _);
pub const EVT_WDSMCS_E_CP_INIT_FUNC_MISSING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801327i32 as _);
pub const EVT_WDSMCS_E_CP_MEMORY_LEAK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801322i32 as _);
pub const EVT_WDSMCS_E_CP_OPEN_CONTENT_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801319i32 as _);
pub const EVT_WDSMCS_E_CP_OPEN_INSTANCE_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801321i32 as _);
pub const EVT_WDSMCS_E_CP_SHUTDOWN_FUNC_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801323i32 as _);
pub const EVT_WDSMCS_E_DUPLICATE_MULTICAST_ADDR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801406i32 as _);
pub const EVT_WDSMCS_E_NON_WDS_DUPLICATE_MULTICAST_ADDR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801405i32 as _);
pub const EVT_WDSMCS_E_NSREG_CONTENT_PROVIDER_NOT_REG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801151i32 as _);
pub const EVT_WDSMCS_E_NSREG_FAILURE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801149i32 as _);
pub const EVT_WDSMCS_E_NSREG_NAMESPACE_EXISTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801150i32 as _);
pub const EVT_WDSMCS_E_NSREG_START_TIME_IN_PAST: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801152i32 as _);
pub const EVT_WDSMCS_E_PARAMETERS_READ_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801407i32 as _);
pub const EVT_WDSMCS_S_PARAMETERS_READ: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(1092682240i32 as _);
pub const EVT_WDSMCS_W_CP_DLL_LOAD_FAILED_NOT_CRITICAL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2128543142i32 as _);
pub const FACILITY_WDSMCCLIENT: u32 = 290u32;
pub const FACILITY_WDSMCSERVER: u32 = 289u32;
pub const FACILITY_WDSTPTMGMT: u32 = 272u32;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportCacheable(::windows::runtime::IUnknown);
impl IWdsTransportCacheable {
    pub unsafe fn Dirty(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Discard(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportCacheable {
    type Vtable = IWdsTransportCacheable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1185777995, 2987, 18396, [132, 178, 123, 85, 63, 29, 143, 128]);
}
impl ::std::convert::From<IWdsTransportCacheable> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportCacheable) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportCacheable> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportCacheable) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportCacheable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportCacheable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportCacheable> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportCacheable) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportCacheable> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportCacheable) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportCacheable {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportCacheable {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportCacheable_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbdirty: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportClient(::windows::runtime::IUnknown);
impl IWdsTransportClient {
    pub unsafe fn Session(&self) -> ::windows::runtime::Result<IWdsTransportSession> {
        let mut result__: <IWdsTransportSession as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportSession>(result__)
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MacAddress(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IpAddress(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn PercentCompletion(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn JoinDuration(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn CpuUtilization(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn MemoryUtilization(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn NetworkUtilization(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserIdentity(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Disconnect(&self, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(disconnectiontype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportClient {
    type Vtable = IWdsTransportClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3051079994, 51902, 18122, [131, 127, 62, 68, 233, 60, 101, 69]);
}
impl ::std::convert::From<IWdsTransportClient> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportClient) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportClient> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportClient) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportClient> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportClient) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportClient> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportClient) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportClient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportsession: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszmacaddress: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszipaddress: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulpercentcompletion: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puljoinduration: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulcpuutilization: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulmemoryutilization: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulnetworkutilization: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszuseridentity: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportCollection(::windows::runtime::IUnknown);
impl IWdsTransportCollection {
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Item(&self, ulindex: u32) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulindex), &mut result__).from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportCollection {
    type Vtable = IWdsTransportCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3099216666, 12276, 17323, [153, 108, 178, 177, 10, 145, 166, 235]);
}
impl ::std::convert::From<IWdsTransportCollection> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportCollection> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportCollection> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulindex: u32, ppval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportConfigurationManager(::windows::runtime::IUnknown);
impl IWdsTransportConfigurationManager {
    pub unsafe fn ServicePolicy(&self) -> ::windows::runtime::Result<IWdsTransportServicePolicy> {
        let mut result__: <IWdsTransportServicePolicy as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportServicePolicy>(result__)
    }
    pub unsafe fn DiagnosticsPolicy(&self) -> ::windows::runtime::Result<IWdsTransportDiagnosticsPolicy> {
        let mut result__: <IWdsTransportDiagnosticsPolicy as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportDiagnosticsPolicy>(result__)
    }
    pub unsafe fn WdsTransportServicesRunning(&self, brealtimestatus: i16) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(brealtimestatus), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn EnableWdsTransportServices(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DisableWdsTransportServices(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn StartWdsTransportServices(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn StopWdsTransportServices(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RestartWdsTransportServices(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn NotifyWdsTransportServices(&self, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(servicenotification)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportConfigurationManager {
    type Vtable = IWdsTransportConfigurationManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2227980153, 17117, 18322, [137, 30, 19, 33, 214, 215, 75, 68]);
}
impl ::std::convert::From<IWdsTransportConfigurationManager> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportConfigurationManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportConfigurationManager> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportConfigurationManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportConfigurationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportConfigurationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportConfigurationManager> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportConfigurationManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportConfigurationManager> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportConfigurationManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportConfigurationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportConfigurationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportConfigurationManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportservicepolicy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportdiagnosticspolicy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, brealtimestatus: i16, pbservicesrunning: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportConfigurationManager2(::windows::runtime::IUnknown);
impl IWdsTransportConfigurationManager2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn ServicePolicy(&self) -> ::windows::runtime::Result<IWdsTransportServicePolicy> {
        let mut result__: <IWdsTransportServicePolicy as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportServicePolicy>(result__)
    }
    pub unsafe fn DiagnosticsPolicy(&self) -> ::windows::runtime::Result<IWdsTransportDiagnosticsPolicy> {
        let mut result__: <IWdsTransportDiagnosticsPolicy as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportDiagnosticsPolicy>(result__)
    }
    pub unsafe fn WdsTransportServicesRunning(&self, brealtimestatus: i16) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(brealtimestatus), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn EnableWdsTransportServices(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DisableWdsTransportServices(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn StartWdsTransportServices(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn StopWdsTransportServices(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RestartWdsTransportServices(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn NotifyWdsTransportServices(&self, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(servicenotification)).ok()
    }
    pub unsafe fn MulticastSessionPolicy(&self) -> ::windows::runtime::Result<IWdsTransportMulticastSessionPolicy> {
        let mut result__: <IWdsTransportMulticastSessionPolicy as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportMulticastSessionPolicy>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportConfigurationManager2 {
    type Vtable = IWdsTransportConfigurationManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3503840431, 41299, 20253, [169, 221, 150, 244, 49, 197, 7, 23]);
}
impl ::std::convert::From<IWdsTransportConfigurationManager2> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportConfigurationManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportConfigurationManager2> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportConfigurationManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportConfigurationManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportConfigurationManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWdsTransportConfigurationManager2> for IWdsTransportConfigurationManager {
    fn from(value: IWdsTransportConfigurationManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportConfigurationManager2> for IWdsTransportConfigurationManager {
    fn from(value: &IWdsTransportConfigurationManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportConfigurationManager> for IWdsTransportConfigurationManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportConfigurationManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportConfigurationManager>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportConfigurationManager> for &IWdsTransportConfigurationManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportConfigurationManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportConfigurationManager>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportConfigurationManager2> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportConfigurationManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportConfigurationManager2> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportConfigurationManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportConfigurationManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportConfigurationManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportConfigurationManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportservicepolicy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportdiagnosticspolicy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, brealtimestatus: i16, pbservicesrunning: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportmulticastsessionpolicy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportContent(::windows::runtime::IUnknown);
impl IWdsTransportContent {
    pub unsafe fn Namespace(&self) -> ::windows::runtime::Result<IWdsTransportNamespace> {
        let mut result__: <IWdsTransportNamespace as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportNamespace>(result__)
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn RetrieveSessions(&self) -> ::windows::runtime::Result<IWdsTransportCollection> {
        let mut result__: <IWdsTransportCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportCollection>(result__)
    }
    pub unsafe fn Terminate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportContent {
    type Vtable = IWdsTransportContent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3557152529, 662, 19124, [168, 96, 172, 125, 50, 230, 87, 152]);
}
impl ::std::convert::From<IWdsTransportContent> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportContent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportContent> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportContent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportContent> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportContent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportContent> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportContent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportContent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportnamespace: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportsessions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportContentProvider(::windows::runtime::IUnknown);
impl IWdsTransportContentProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FilePath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializationRoutine(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportContentProvider {
    type Vtable = IWdsTransportContentProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3108544292, 61977, 19151, [170, 215, 38, 92, 124, 8, 166, 174]);
}
impl ::std::convert::From<IWdsTransportContentProvider> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportContentProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportContentProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportContentProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportContentProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportContentProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportContentProvider> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportContentProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportContentProvider> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportContentProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportContentProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportContentProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportContentProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszdescription: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszfilepath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszinitializationroutine: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportDiagnosticsPolicy(::windows::runtime::IUnknown);
impl IWdsTransportDiagnosticsPolicy {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Dirty(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Discard(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, benabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(benabled)).ok()
    }
    pub unsafe fn Components(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetComponents(&self, ulcomponents: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulcomponents)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportDiagnosticsPolicy {
    type Vtable = IWdsTransportDiagnosticsPolicy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(330514172, 30806, 20321, [154, 89, 141, 230, 123, 107, 135, 182]);
}
impl ::std::convert::From<IWdsTransportDiagnosticsPolicy> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportDiagnosticsPolicy) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportDiagnosticsPolicy> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportDiagnosticsPolicy) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportDiagnosticsPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportDiagnosticsPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWdsTransportDiagnosticsPolicy> for IWdsTransportCacheable {
    fn from(value: IWdsTransportDiagnosticsPolicy) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportDiagnosticsPolicy> for IWdsTransportCacheable {
    fn from(value: &IWdsTransportDiagnosticsPolicy) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportCacheable> for IWdsTransportDiagnosticsPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportCacheable> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportCacheable>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportCacheable> for &IWdsTransportDiagnosticsPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportCacheable> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportCacheable>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportDiagnosticsPolicy> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportDiagnosticsPolicy) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportDiagnosticsPolicy> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportDiagnosticsPolicy) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportDiagnosticsPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportDiagnosticsPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportDiagnosticsPolicy_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbdirty: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, benabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulcomponents: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcomponents: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportManager(::windows::runtime::IUnknown);
impl IWdsTransportManager {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWdsTransportServer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszservername: Param0) -> ::windows::runtime::Result<IWdsTransportServer> {
        let mut result__: <IWdsTransportServer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), bszservername.into_param().abi(), &mut result__).from_abi::<IWdsTransportServer>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportManager {
    type Vtable = IWdsTransportManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1527592437, 6931, 19197, [184, 120, 101, 38, 220, 52, 11, 93]);
}
impl ::std::convert::From<IWdsTransportManager> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportManager> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportManager> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportManager> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszservername: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwdstransportserver: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportMulticastSessionPolicy(::windows::runtime::IUnknown);
impl IWdsTransportMulticastSessionPolicy {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Dirty(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Discard(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SlowClientHandling(&self) -> ::windows::runtime::Result<WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE> {
        let mut result__: <WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE>(result__)
    }
    pub unsafe fn SetSlowClientHandling(&self, slowclienthandling: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(slowclienthandling)).ok()
    }
    pub unsafe fn AutoDisconnectThreshold(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetAutoDisconnectThreshold(&self, ulthreshold: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulthreshold)).ok()
    }
    pub unsafe fn MultistreamStreamCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMultistreamStreamCount(&self, ulstreamcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulstreamcount)).ok()
    }
    pub unsafe fn SlowClientFallback(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetSlowClientFallback(&self, bclientfallback: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(bclientfallback)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportMulticastSessionPolicy {
    type Vtable = IWdsTransportMulticastSessionPolicy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1314345935, 26860, 17668, [169, 81, 74, 0, 50, 102, 96, 107]);
}
impl ::std::convert::From<IWdsTransportMulticastSessionPolicy> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportMulticastSessionPolicy) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportMulticastSessionPolicy> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportMulticastSessionPolicy) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportMulticastSessionPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportMulticastSessionPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWdsTransportMulticastSessionPolicy> for IWdsTransportCacheable {
    fn from(value: IWdsTransportMulticastSessionPolicy) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportMulticastSessionPolicy> for IWdsTransportCacheable {
    fn from(value: &IWdsTransportMulticastSessionPolicy) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportCacheable> for IWdsTransportMulticastSessionPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportCacheable> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportCacheable>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportCacheable> for &IWdsTransportMulticastSessionPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportCacheable> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportCacheable>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportMulticastSessionPolicy> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportMulticastSessionPolicy) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportMulticastSessionPolicy> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportMulticastSessionPolicy) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportMulticastSessionPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportMulticastSessionPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportMulticastSessionPolicy_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbdirty: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pslowclienthandling: *mut WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, slowclienthandling: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulthreshold: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulthreshold: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulstreamcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulstreamcount: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbclientfallback: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bclientfallback: i16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportNamespace(::windows::runtime::IUnknown);
impl IWdsTransportNamespace {
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<WDSTRANSPORT_NAMESPACE_TYPE> {
        let mut result__: <WDSTRANSPORT_NAMESPACE_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WDSTRANSPORT_NAMESPACE_TYPE>(result__)
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bszname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FriendlyName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFriendlyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszfriendlyname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), bszfriendlyname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszdescription: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), bszdescription.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContentProvider(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContentProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszcontentprovider: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), bszcontentprovider.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Configuration(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszconfiguration: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), bszconfiguration.into_param().abi()).ok()
    }
    pub unsafe fn Registered(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Tombstoned(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn TombstoneTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn TransmissionStarted(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Register(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Deregister(&self, bterminatesessions: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(bterminatesessions)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IWdsTransportNamespace> {
        let mut result__: <IWdsTransportNamespace as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportNamespace>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RetrieveContents(&self) -> ::windows::runtime::Result<IWdsTransportCollection> {
        let mut result__: <IWdsTransportCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportNamespace {
    type Vtable = IWdsTransportNamespace_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4199948119, 64495, 20179, [176, 86, 18, 124, 177, 179, 59, 132]);
}
impl ::std::convert::From<IWdsTransportNamespace> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportNamespace) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportNamespace> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportNamespace) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportNamespace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportNamespace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportNamespace> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportNamespace) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportNamespace> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportNamespace) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportNamespace {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportNamespace {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportNamespace_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut WDSTRANSPORT_NAMESPACE_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszfriendlyname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszfriendlyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszdescription: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszdescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszcontentprovider: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszcontentprovider: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszconfiguration: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszconfiguration: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbregistered: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbtombstoned: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptombstonetime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbtransmissionstarted: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bterminatesessions: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportnamespaceclone: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportcontents: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportNamespaceAutoCast(::windows::runtime::IUnknown);
impl IWdsTransportNamespaceAutoCast {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<WDSTRANSPORT_NAMESPACE_TYPE> {
        let mut result__: <WDSTRANSPORT_NAMESPACE_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WDSTRANSPORT_NAMESPACE_TYPE>(result__)
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bszname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FriendlyName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFriendlyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszfriendlyname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), bszfriendlyname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszdescription: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), bszdescription.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContentProvider(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContentProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszcontentprovider: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), bszcontentprovider.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Configuration(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszconfiguration: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), bszconfiguration.into_param().abi()).ok()
    }
    pub unsafe fn Registered(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Tombstoned(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn TombstoneTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn TransmissionStarted(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Register(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Deregister(&self, bterminatesessions: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(bterminatesessions)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IWdsTransportNamespace> {
        let mut result__: <IWdsTransportNamespace as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportNamespace>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RetrieveContents(&self) -> ::windows::runtime::Result<IWdsTransportCollection> {
        let mut result__: <IWdsTransportCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportNamespaceAutoCast {
    type Vtable = IWdsTransportNamespaceAutoCast_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2912098930, 50365, 19521, [143, 188, 89, 201, 199, 72, 223, 158]);
}
impl ::std::convert::From<IWdsTransportNamespaceAutoCast> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportNamespaceAutoCast) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportNamespaceAutoCast> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportNamespaceAutoCast) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportNamespaceAutoCast {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportNamespaceAutoCast {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWdsTransportNamespaceAutoCast> for IWdsTransportNamespace {
    fn from(value: IWdsTransportNamespaceAutoCast) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportNamespaceAutoCast> for IWdsTransportNamespace {
    fn from(value: &IWdsTransportNamespaceAutoCast) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportNamespace> for IWdsTransportNamespaceAutoCast {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportNamespace> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportNamespace>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportNamespace> for &IWdsTransportNamespaceAutoCast {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportNamespace> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportNamespace>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportNamespaceAutoCast> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportNamespaceAutoCast) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportNamespaceAutoCast> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportNamespaceAutoCast) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportNamespaceAutoCast {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportNamespaceAutoCast {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportNamespaceAutoCast_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut WDSTRANSPORT_NAMESPACE_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszfriendlyname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszfriendlyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszdescription: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszdescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszcontentprovider: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszcontentprovider: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszconfiguration: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszconfiguration: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbregistered: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbtombstoned: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptombstonetime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbtransmissionstarted: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bterminatesessions: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportnamespaceclone: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportcontents: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportNamespaceManager(::windows::runtime::IUnknown);
impl IWdsTransportNamespaceManager {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateNamespace<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, namespacetype: WDSTRANSPORT_NAMESPACE_TYPE, bsznamespacename: Param1, bszcontentprovider: Param2, bszconfiguration: Param3) -> ::windows::runtime::Result<IWdsTransportNamespace> {
        let mut result__: <IWdsTransportNamespace as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(namespacetype), bsznamespacename.into_param().abi(), bszcontentprovider.into_param().abi(), bszconfiguration.into_param().abi(), &mut result__).from_abi::<IWdsTransportNamespace>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RetrieveNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bsznamespacename: Param0) -> ::windows::runtime::Result<IWdsTransportNamespace> {
        let mut result__: <IWdsTransportNamespace as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bsznamespacename.into_param().abi(), &mut result__).from_abi::<IWdsTransportNamespace>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RetrieveNamespaces<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszcontentprovider: Param0, bsznamespacename: Param1, bincludetombstones: i16) -> ::windows::runtime::Result<IWdsTransportCollection> {
        let mut result__: <IWdsTransportCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), bszcontentprovider.into_param().abi(), bsznamespacename.into_param().abi(), ::std::mem::transmute(bincludetombstones), &mut result__).from_abi::<IWdsTransportCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportNamespaceManager {
    type Vtable = IWdsTransportNamespaceManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1042471414, 14199, 19864, [131, 225, 249, 134, 150, 113, 123, 163]);
}
impl ::std::convert::From<IWdsTransportNamespaceManager> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportNamespaceManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportNamespaceManager> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportNamespaceManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportNamespaceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportNamespaceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportNamespaceManager> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportNamespaceManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportNamespaceManager> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportNamespaceManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportNamespaceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportNamespaceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportNamespaceManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namespacetype: WDSTRANSPORT_NAMESPACE_TYPE, bsznamespacename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszcontentprovider: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszconfiguration: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwdstransportnamespace: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bsznamespacename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwdstransportnamespace: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszcontentprovider: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bsznamespacename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bincludetombstones: i16, ppwdstransportnamespaces: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportNamespaceScheduledCast(::windows::runtime::IUnknown);
impl IWdsTransportNamespaceScheduledCast {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<WDSTRANSPORT_NAMESPACE_TYPE> {
        let mut result__: <WDSTRANSPORT_NAMESPACE_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WDSTRANSPORT_NAMESPACE_TYPE>(result__)
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bszname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FriendlyName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFriendlyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszfriendlyname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), bszfriendlyname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszdescription: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), bszdescription.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContentProvider(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContentProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszcontentprovider: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), bszcontentprovider.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Configuration(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszconfiguration: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), bszconfiguration.into_param().abi()).ok()
    }
    pub unsafe fn Registered(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Tombstoned(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn TombstoneTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn TransmissionStarted(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Register(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Deregister(&self, bterminatesessions: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(bterminatesessions)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IWdsTransportNamespace> {
        let mut result__: <IWdsTransportNamespace as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportNamespace>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RetrieveContents(&self) -> ::windows::runtime::Result<IWdsTransportCollection> {
        let mut result__: <IWdsTransportCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportCollection>(result__)
    }
    pub unsafe fn StartTransmission(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportNamespaceScheduledCast {
    type Vtable = IWdsTransportNamespaceScheduledCast_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(943771343, 55148, 16750, [164, 204, 49, 199, 65, 210, 135, 75]);
}
impl ::std::convert::From<IWdsTransportNamespaceScheduledCast> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportNamespaceScheduledCast) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportNamespaceScheduledCast> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportNamespaceScheduledCast) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportNamespaceScheduledCast {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportNamespaceScheduledCast {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWdsTransportNamespaceScheduledCast> for IWdsTransportNamespace {
    fn from(value: IWdsTransportNamespaceScheduledCast) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportNamespaceScheduledCast> for IWdsTransportNamespace {
    fn from(value: &IWdsTransportNamespaceScheduledCast) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportNamespace> for IWdsTransportNamespaceScheduledCast {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportNamespace> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportNamespace>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportNamespace> for &IWdsTransportNamespaceScheduledCast {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportNamespace> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportNamespace>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportNamespaceScheduledCast> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportNamespaceScheduledCast) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportNamespaceScheduledCast> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportNamespaceScheduledCast) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportNamespaceScheduledCast {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportNamespaceScheduledCast {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportNamespaceScheduledCast_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut WDSTRANSPORT_NAMESPACE_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszfriendlyname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszfriendlyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszdescription: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszdescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszcontentprovider: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszcontentprovider: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszconfiguration: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszconfiguration: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbregistered: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbtombstoned: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptombstonetime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbtransmissionstarted: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bterminatesessions: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportnamespaceclone: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportcontents: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportNamespaceScheduledCastAutoStart(::windows::runtime::IUnknown);
impl IWdsTransportNamespaceScheduledCastAutoStart {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<WDSTRANSPORT_NAMESPACE_TYPE> {
        let mut result__: <WDSTRANSPORT_NAMESPACE_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WDSTRANSPORT_NAMESPACE_TYPE>(result__)
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bszname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FriendlyName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFriendlyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszfriendlyname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), bszfriendlyname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszdescription: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), bszdescription.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContentProvider(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContentProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszcontentprovider: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), bszcontentprovider.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Configuration(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszconfiguration: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), bszconfiguration.into_param().abi()).ok()
    }
    pub unsafe fn Registered(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Tombstoned(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn TombstoneTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn TransmissionStarted(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Register(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Deregister(&self, bterminatesessions: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(bterminatesessions)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IWdsTransportNamespace> {
        let mut result__: <IWdsTransportNamespace as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportNamespace>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RetrieveContents(&self) -> ::windows::runtime::Result<IWdsTransportCollection> {
        let mut result__: <IWdsTransportCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportCollection>(result__)
    }
    pub unsafe fn StartTransmission(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn MinimumClients(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMinimumClients(&self, ulminimumclients: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulminimumclients)).ok()
    }
    pub unsafe fn StartTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SetStartTime(&self, starttime: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(starttime)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportNamespaceScheduledCastAutoStart {
    type Vtable = IWdsTransportNamespaceScheduledCastAutoStart_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3590762301, 60060, 16921, [150, 30, 116, 145, 214, 24, 217, 185]);
}
impl ::std::convert::From<IWdsTransportNamespaceScheduledCastAutoStart> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportNamespaceScheduledCastAutoStart) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportNamespaceScheduledCastAutoStart> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportNamespaceScheduledCastAutoStart) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportNamespaceScheduledCastAutoStart {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportNamespaceScheduledCastAutoStart {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWdsTransportNamespaceScheduledCastAutoStart> for IWdsTransportNamespaceScheduledCast {
    fn from(value: IWdsTransportNamespaceScheduledCastAutoStart) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportNamespaceScheduledCastAutoStart> for IWdsTransportNamespaceScheduledCast {
    fn from(value: &IWdsTransportNamespaceScheduledCastAutoStart) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportNamespaceScheduledCast> for IWdsTransportNamespaceScheduledCastAutoStart {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportNamespaceScheduledCast> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportNamespaceScheduledCast>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportNamespaceScheduledCast> for &IWdsTransportNamespaceScheduledCastAutoStart {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportNamespaceScheduledCast> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportNamespaceScheduledCast>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWdsTransportNamespaceScheduledCastAutoStart> for IWdsTransportNamespace {
    fn from(value: IWdsTransportNamespaceScheduledCastAutoStart) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportNamespaceScheduledCastAutoStart> for IWdsTransportNamespace {
    fn from(value: &IWdsTransportNamespaceScheduledCastAutoStart) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportNamespace> for IWdsTransportNamespaceScheduledCastAutoStart {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportNamespace> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportNamespace>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportNamespace> for &IWdsTransportNamespaceScheduledCastAutoStart {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportNamespace> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportNamespace>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportNamespaceScheduledCastAutoStart> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportNamespaceScheduledCastAutoStart) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportNamespaceScheduledCastAutoStart> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportNamespaceScheduledCastAutoStart) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportNamespaceScheduledCastAutoStart {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportNamespaceScheduledCastAutoStart {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportNamespaceScheduledCastAutoStart_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut WDSTRANSPORT_NAMESPACE_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszfriendlyname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszfriendlyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszdescription: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszdescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszcontentprovider: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszcontentprovider: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszconfiguration: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszconfiguration: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbregistered: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbtombstoned: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptombstonetime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbtransmissionstarted: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bterminatesessions: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportnamespaceclone: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportcontents: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulminimumclients: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulminimumclients: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstarttime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starttime: f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportNamespaceScheduledCastManualStart(::windows::runtime::IUnknown);
impl IWdsTransportNamespaceScheduledCastManualStart {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<WDSTRANSPORT_NAMESPACE_TYPE> {
        let mut result__: <WDSTRANSPORT_NAMESPACE_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WDSTRANSPORT_NAMESPACE_TYPE>(result__)
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bszname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FriendlyName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFriendlyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszfriendlyname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), bszfriendlyname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszdescription: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), bszdescription.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContentProvider(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContentProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszcontentprovider: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), bszcontentprovider.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Configuration(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszconfiguration: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), bszconfiguration.into_param().abi()).ok()
    }
    pub unsafe fn Registered(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Tombstoned(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn TombstoneTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn TransmissionStarted(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Register(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Deregister(&self, bterminatesessions: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(bterminatesessions)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IWdsTransportNamespace> {
        let mut result__: <IWdsTransportNamespace as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportNamespace>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RetrieveContents(&self) -> ::windows::runtime::Result<IWdsTransportCollection> {
        let mut result__: <IWdsTransportCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportCollection>(result__)
    }
    pub unsafe fn StartTransmission(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportNamespaceScheduledCastManualStart {
    type Vtable = IWdsTransportNamespaceScheduledCastManualStart_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(20868684, 59047, 20405, [183, 255, 217, 245, 218, 128, 92, 49]);
}
impl ::std::convert::From<IWdsTransportNamespaceScheduledCastManualStart> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportNamespaceScheduledCastManualStart) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportNamespaceScheduledCastManualStart> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportNamespaceScheduledCastManualStart) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportNamespaceScheduledCastManualStart {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportNamespaceScheduledCastManualStart {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWdsTransportNamespaceScheduledCastManualStart> for IWdsTransportNamespaceScheduledCast {
    fn from(value: IWdsTransportNamespaceScheduledCastManualStart) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportNamespaceScheduledCastManualStart> for IWdsTransportNamespaceScheduledCast {
    fn from(value: &IWdsTransportNamespaceScheduledCastManualStart) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportNamespaceScheduledCast> for IWdsTransportNamespaceScheduledCastManualStart {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportNamespaceScheduledCast> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportNamespaceScheduledCast>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportNamespaceScheduledCast> for &IWdsTransportNamespaceScheduledCastManualStart {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportNamespaceScheduledCast> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportNamespaceScheduledCast>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWdsTransportNamespaceScheduledCastManualStart> for IWdsTransportNamespace {
    fn from(value: IWdsTransportNamespaceScheduledCastManualStart) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportNamespaceScheduledCastManualStart> for IWdsTransportNamespace {
    fn from(value: &IWdsTransportNamespaceScheduledCastManualStart) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportNamespace> for IWdsTransportNamespaceScheduledCastManualStart {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportNamespace> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportNamespace>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportNamespace> for &IWdsTransportNamespaceScheduledCastManualStart {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportNamespace> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportNamespace>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportNamespaceScheduledCastManualStart> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportNamespaceScheduledCastManualStart) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportNamespaceScheduledCastManualStart> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportNamespaceScheduledCastManualStart) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportNamespaceScheduledCastManualStart {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportNamespaceScheduledCastManualStart {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportNamespaceScheduledCastManualStart_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut WDSTRANSPORT_NAMESPACE_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszfriendlyname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszfriendlyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszdescription: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszdescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszcontentprovider: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszcontentprovider: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszconfiguration: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszconfiguration: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbregistered: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbtombstoned: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptombstonetime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbtransmissionstarted: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bterminatesessions: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportnamespaceclone: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportcontents: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportServer(::windows::runtime::IUnknown);
impl IWdsTransportServer {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetupManager(&self) -> ::windows::runtime::Result<IWdsTransportSetupManager> {
        let mut result__: <IWdsTransportSetupManager as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportSetupManager>(result__)
    }
    pub unsafe fn ConfigurationManager(&self) -> ::windows::runtime::Result<IWdsTransportConfigurationManager> {
        let mut result__: <IWdsTransportConfigurationManager as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportConfigurationManager>(result__)
    }
    pub unsafe fn NamespaceManager(&self) -> ::windows::runtime::Result<IWdsTransportNamespaceManager> {
        let mut result__: <IWdsTransportNamespaceManager as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportNamespaceManager>(result__)
    }
    pub unsafe fn DisconnectClient(&self, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulclientid), ::std::mem::transmute(disconnectiontype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportServer {
    type Vtable = IWdsTransportServer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(164417683, 33549, 17220, [163, 10, 115, 174, 142, 143, 202, 144]);
}
impl ::std::convert::From<IWdsTransportServer> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportServer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportServer> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportServer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportServer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportServer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportServer> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportServer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportServer> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportServer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportServer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportServer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportServer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportsetupmanager: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportconfigurationmanager: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportnamespacemanager: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportServer2(::windows::runtime::IUnknown);
impl IWdsTransportServer2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetupManager(&self) -> ::windows::runtime::Result<IWdsTransportSetupManager> {
        let mut result__: <IWdsTransportSetupManager as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportSetupManager>(result__)
    }
    pub unsafe fn ConfigurationManager(&self) -> ::windows::runtime::Result<IWdsTransportConfigurationManager> {
        let mut result__: <IWdsTransportConfigurationManager as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportConfigurationManager>(result__)
    }
    pub unsafe fn NamespaceManager(&self) -> ::windows::runtime::Result<IWdsTransportNamespaceManager> {
        let mut result__: <IWdsTransportNamespaceManager as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportNamespaceManager>(result__)
    }
    pub unsafe fn DisconnectClient(&self, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulclientid), ::std::mem::transmute(disconnectiontype)).ok()
    }
    pub unsafe fn TftpManager(&self) -> ::windows::runtime::Result<IWdsTransportTftpManager> {
        let mut result__: <IWdsTransportTftpManager as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportTftpManager>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportServer2 {
    type Vtable = IWdsTransportServer2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(628005279, 28148, 17720, [129, 185, 133, 123, 154, 184, 251, 71]);
}
impl ::std::convert::From<IWdsTransportServer2> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportServer2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportServer2> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportServer2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportServer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportServer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWdsTransportServer2> for IWdsTransportServer {
    fn from(value: IWdsTransportServer2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportServer2> for IWdsTransportServer {
    fn from(value: &IWdsTransportServer2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportServer> for IWdsTransportServer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportServer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportServer>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportServer> for &IWdsTransportServer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportServer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportServer>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportServer2> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportServer2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportServer2> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportServer2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportServer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportServer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportServer2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportsetupmanager: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportconfigurationmanager: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportnamespacemanager: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransporttftpmanager: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportServicePolicy(::windows::runtime::IUnknown);
impl IWdsTransportServicePolicy {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Dirty(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Discard(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IpAddressSource(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows::runtime::Result<WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE> {
        let mut result__: <WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(addresstype), &mut result__).from_abi::<WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE>(result__)
    }
    pub unsafe fn SetIpAddressSource(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(addresstype), ::std::mem::transmute(sourcetype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(addresstype), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStartIpAddress<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(addresstype), bszstartipaddress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(addresstype), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEndIpAddress<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(addresstype), bszendipaddress.into_param().abi()).ok()
    }
    pub unsafe fn StartPort(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetStartPort(&self, ulstartport: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulstartport)).ok()
    }
    pub unsafe fn EndPort(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetEndPort(&self, ulendport: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulendport)).ok()
    }
    pub unsafe fn NetworkProfile(&self) -> ::windows::runtime::Result<WDSTRANSPORT_NETWORK_PROFILE_TYPE> {
        let mut result__: <WDSTRANSPORT_NETWORK_PROFILE_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WDSTRANSPORT_NETWORK_PROFILE_TYPE>(result__)
    }
    pub unsafe fn SetNetworkProfile(&self, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportServicePolicy {
    type Vtable = IWdsTransportServicePolicy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3108406648, 40747, 18636, [178, 122, 166, 7, 153, 194, 117, 12]);
}
impl ::std::convert::From<IWdsTransportServicePolicy> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportServicePolicy) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportServicePolicy> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportServicePolicy) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportServicePolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportServicePolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWdsTransportServicePolicy> for IWdsTransportCacheable {
    fn from(value: IWdsTransportServicePolicy) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportServicePolicy> for IWdsTransportCacheable {
    fn from(value: &IWdsTransportServicePolicy) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportCacheable> for IWdsTransportServicePolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportCacheable> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportCacheable>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportCacheable> for &IWdsTransportServicePolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportCacheable> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportCacheable>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportServicePolicy> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportServicePolicy) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportServicePolicy> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportServicePolicy) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportServicePolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportServicePolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportServicePolicy_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbdirty: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, psourcetype: *mut WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszstartipaddress: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszendipaddress: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulstartport: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulstartport: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulendport: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulendport: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprofiletype: *mut WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportServicePolicy2(::windows::runtime::IUnknown);
impl IWdsTransportServicePolicy2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Dirty(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Discard(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IpAddressSource(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows::runtime::Result<WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE> {
        let mut result__: <WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(addresstype), &mut result__).from_abi::<WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE>(result__)
    }
    pub unsafe fn SetIpAddressSource(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(addresstype), ::std::mem::transmute(sourcetype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(addresstype), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStartIpAddress<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(addresstype), bszstartipaddress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(addresstype), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEndIpAddress<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(addresstype), bszendipaddress.into_param().abi()).ok()
    }
    pub unsafe fn StartPort(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetStartPort(&self, ulstartport: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulstartport)).ok()
    }
    pub unsafe fn EndPort(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetEndPort(&self, ulendport: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulendport)).ok()
    }
    pub unsafe fn NetworkProfile(&self) -> ::windows::runtime::Result<WDSTRANSPORT_NETWORK_PROFILE_TYPE> {
        let mut result__: <WDSTRANSPORT_NETWORK_PROFILE_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WDSTRANSPORT_NETWORK_PROFILE_TYPE>(result__)
    }
    pub unsafe fn SetNetworkProfile(&self, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletype)).ok()
    }
    pub unsafe fn UdpPortPolicy(&self) -> ::windows::runtime::Result<WDSTRANSPORT_UDP_PORT_POLICY> {
        let mut result__: <WDSTRANSPORT_UDP_PORT_POLICY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WDSTRANSPORT_UDP_PORT_POLICY>(result__)
    }
    pub unsafe fn SetUdpPortPolicy(&self, udpportpolicy: WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(udpportpolicy)).ok()
    }
    pub unsafe fn TftpMaximumBlockSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetTftpMaximumBlockSize(&self, ultftpmaximumblocksize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(ultftpmaximumblocksize)).ok()
    }
    pub unsafe fn EnableTftpVariableWindowExtension(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnableTftpVariableWindowExtension(&self, benabletftpvariablewindowextension: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(benabletftpvariablewindowextension)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportServicePolicy2 {
    type Vtable = IWdsTransportServicePolicy2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1707187804, 43646, 19345, [137, 68, 145, 224, 229, 87, 39, 151]);
}
impl ::std::convert::From<IWdsTransportServicePolicy2> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportServicePolicy2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportServicePolicy2> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportServicePolicy2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportServicePolicy2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportServicePolicy2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWdsTransportServicePolicy2> for IWdsTransportServicePolicy {
    fn from(value: IWdsTransportServicePolicy2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportServicePolicy2> for IWdsTransportServicePolicy {
    fn from(value: &IWdsTransportServicePolicy2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportServicePolicy> for IWdsTransportServicePolicy2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportServicePolicy> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportServicePolicy>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportServicePolicy> for &IWdsTransportServicePolicy2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportServicePolicy> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportServicePolicy>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWdsTransportServicePolicy2> for IWdsTransportCacheable {
    fn from(value: IWdsTransportServicePolicy2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportServicePolicy2> for IWdsTransportCacheable {
    fn from(value: &IWdsTransportServicePolicy2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportCacheable> for IWdsTransportServicePolicy2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportCacheable> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportCacheable>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportCacheable> for &IWdsTransportServicePolicy2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportCacheable> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportCacheable>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportServicePolicy2> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportServicePolicy2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportServicePolicy2> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportServicePolicy2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportServicePolicy2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportServicePolicy2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportServicePolicy2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbdirty: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, psourcetype: *mut WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszstartipaddress: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszendipaddress: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulstartport: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulstartport: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulendport: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulendport: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprofiletype: *mut WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pudpportpolicy: *mut WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, udpportpolicy: WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pultftpmaximumblocksize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ultftpmaximumblocksize: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbenabletftpvariablewindowextension: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, benabletftpvariablewindowextension: i16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportSession(::windows::runtime::IUnknown);
impl IWdsTransportSession {
    pub unsafe fn Content(&self) -> ::windows::runtime::Result<IWdsTransportContent> {
        let mut result__: <IWdsTransportContent as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportContent>(result__)
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NetworkInterfaceName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NetworkInterfaceAddress(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn TransferRate(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn MasterClientId(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn RetrieveClients(&self) -> ::windows::runtime::Result<IWdsTransportCollection> {
        let mut result__: <IWdsTransportCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportCollection>(result__)
    }
    pub unsafe fn Terminate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportSession {
    type Vtable = IWdsTransportSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4109363848, 26033, 20272, [164, 185, 39, 147, 152, 119, 150, 251]);
}
impl ::std::convert::From<IWdsTransportSession> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportSession) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportSession> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportSession) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportSession> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportSession) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportSession> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportSession) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportcontent: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbsznetworkinterfacename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbsznetworkinterfaceaddress: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pultransferrate: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulmasterclientid: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransportclients: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportSetupManager(::windows::runtime::IUnknown);
impl IWdsTransportSetupManager {
    pub unsafe fn Version(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn InstalledFeatures(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Protocols(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterContentProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszname: Param0, bszdescription: Param1, bszfilepath: Param2, bszinitializationroutine: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bszname.into_param().abi(), bszdescription.into_param().abi(), bszfilepath.into_param().abi(), bszinitializationroutine.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeregisterContentProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), bszname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportSetupManager {
    type Vtable = IWdsTransportSetupManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4146299941, 61352, 16548, [174, 249, 201, 141, 150, 156, 11, 117]);
}
impl ::std::convert::From<IWdsTransportSetupManager> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportSetupManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportSetupManager> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportSetupManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportSetupManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportSetupManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportSetupManager> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportSetupManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportSetupManager> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportSetupManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportSetupManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportSetupManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportSetupManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pullversion: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulinstalledfeatures: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulprotocols: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszdescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszfilepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszinitializationroutine: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportSetupManager2(::windows::runtime::IUnknown);
impl IWdsTransportSetupManager2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Version(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn InstalledFeatures(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Protocols(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterContentProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszname: Param0, bszdescription: Param1, bszfilepath: Param2, bszinitializationroutine: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bszname.into_param().abi(), bszdescription.into_param().abi(), bszfilepath.into_param().abi(), bszinitializationroutine.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeregisterContentProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bszname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), bszname.into_param().abi()).ok()
    }
    pub unsafe fn TftpCapabilities(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn ContentProviders(&self) -> ::windows::runtime::Result<IWdsTransportCollection> {
        let mut result__: <IWdsTransportCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportSetupManager2 {
    type Vtable = IWdsTransportSetupManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(46037466, 32414, 17254, [139, 110, 42, 169, 169, 27, 228, 127]);
}
impl ::std::convert::From<IWdsTransportSetupManager2> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportSetupManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportSetupManager2> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportSetupManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportSetupManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportSetupManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWdsTransportSetupManager2> for IWdsTransportSetupManager {
    fn from(value: IWdsTransportSetupManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportSetupManager2> for IWdsTransportSetupManager {
    fn from(value: &IWdsTransportSetupManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportSetupManager> for IWdsTransportSetupManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportSetupManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportSetupManager>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWdsTransportSetupManager> for &IWdsTransportSetupManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWdsTransportSetupManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWdsTransportSetupManager>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportSetupManager2> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportSetupManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportSetupManager2> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportSetupManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportSetupManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportSetupManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportSetupManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pullversion: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulinstalledfeatures: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulprotocols: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszdescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszfilepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszinitializationroutine: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bszname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pultftpcapabilities: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppprovidercollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportTftpClient(::windows::runtime::IUnknown);
impl IWdsTransportTftpClient {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IpAddress(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Timeout(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn CurrentFileOffset(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn FileSize(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn BlockSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn WindowSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportTftpClient {
    type Vtable = IWdsTransportTftpClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2955072430, 34893, 19845, [177, 70, 83, 50, 14, 118, 239, 98]);
}
impl ::std::convert::From<IWdsTransportTftpClient> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportTftpClient) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportTftpClient> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportTftpClient) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportTftpClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportTftpClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportTftpClient> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportTftpClient) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportTftpClient> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportTftpClient) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportTftpClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportTftpClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportTftpClient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszfilename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbszipaddress: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pultimeout: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pul64currentoffset: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pul64filesize: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulblocksize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulwindowsize: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWdsTransportTftpManager(::windows::runtime::IUnknown);
impl IWdsTransportTftpManager {
    pub unsafe fn RetrieveTftpClients(&self) -> ::windows::runtime::Result<IWdsTransportCollection> {
        let mut result__: <IWdsTransportCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWdsTransportCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWdsTransportTftpManager {
    type Vtable = IWdsTransportTftpManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(321365960, 44682, 20403, [129, 80, 19, 98, 39, 195, 126, 154]);
}
impl ::std::convert::From<IWdsTransportTftpManager> for ::windows::runtime::IUnknown {
    fn from(value: IWdsTransportTftpManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWdsTransportTftpManager> for ::windows::runtime::IUnknown {
    fn from(value: &IWdsTransportTftpManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWdsTransportTftpManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWdsTransportTftpManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWdsTransportTftpManager> for super::Ole::Automation::IDispatch {
    fn from(value: IWdsTransportTftpManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWdsTransportTftpManager> for super::Ole::Automation::IDispatch {
    fn from(value: &IWdsTransportTftpManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWdsTransportTftpManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWdsTransportTftpManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportTftpManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwdstransporttftpclients: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
pub const MC_SERVER_CURRENT_VERSION: u32 = 1u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PFN_WDS_CLI_CALLBACK_MESSAGE_ID(pub u32);
pub const WDS_CLI_MSG_START: PFN_WDS_CLI_CALLBACK_MESSAGE_ID = PFN_WDS_CLI_CALLBACK_MESSAGE_ID(0u32);
pub const WDS_CLI_MSG_COMPLETE: PFN_WDS_CLI_CALLBACK_MESSAGE_ID = PFN_WDS_CLI_CALLBACK_MESSAGE_ID(1u32);
pub const WDS_CLI_MSG_PROGRESS: PFN_WDS_CLI_CALLBACK_MESSAGE_ID = PFN_WDS_CLI_CALLBACK_MESSAGE_ID(2u32);
pub const WDS_CLI_MSG_TEXT: PFN_WDS_CLI_CALLBACK_MESSAGE_ID = PFN_WDS_CLI_CALLBACK_MESSAGE_ID(3u32);
impl ::std::convert::From<u32> for PFN_WDS_CLI_CALLBACK_MESSAGE_ID {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PFN_WDS_CLI_CALLBACK_MESSAGE_ID {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PFN_WDS_CLI_CALLBACK_MESSAGE_ID {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PFN_WDS_CLI_CALLBACK_MESSAGE_ID {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PFN_WDS_CLI_CALLBACK_MESSAGE_ID {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PFN_WDS_CLI_CALLBACK_MESSAGE_ID {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PFN_WDS_CLI_CALLBACK_MESSAGE_ID {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsCliCallback = unsafe extern "system" fn(dwmessageid: PFN_WDS_CLI_CALLBACK_MESSAGE_ID, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pvuserdata: *const ::std::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsCliTraceFunction = unsafe extern "system" fn(pwszformat: super::super::Foundation::PWSTR, params: *const i8);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsTransportClientReceiveContents = unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const ::std::ffi::c_void, pcontents: *const ::std::ffi::c_void, ulsize: u32, pullcontentoffset: *const u64);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsTransportClientReceiveMetadata = unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const ::std::ffi::c_void, pmetadata: *const ::std::ffi::c_void, ulsize: u32);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsTransportClientSessionComplete = unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const ::std::ffi::c_void, dwerror: u32);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsTransportClientSessionNegotiate = unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const ::std::ffi::c_void, pinfo: *const TRANSPORTCLIENT_SESSION_INFO, hnegotiatekey: super::super::Foundation::HANDLE);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsTransportClientSessionStart = unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const ::std::ffi::c_void, ullfilesize: *const u64);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsTransportClientSessionStartEx = unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const ::std::ffi::c_void, info: *const TRANSPORTCLIENT_SESSION_INFO);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PXE_ADDRESS {
    pub uFlags: u32,
    pub Anonymous: PXE_ADDRESS_0,
    pub uAddrLen: u32,
    pub uPort: u16,
}
impl PXE_ADDRESS {}
impl ::std::default::Default for PXE_ADDRESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PXE_ADDRESS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PXE_ADDRESS {}
unsafe impl ::windows::runtime::Abi for PXE_ADDRESS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union PXE_ADDRESS_0 {
    pub bAddress: [u8; 16],
    pub uIpAddress: u32,
}
impl PXE_ADDRESS_0 {}
impl ::std::default::Default for PXE_ADDRESS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PXE_ADDRESS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PXE_ADDRESS_0 {}
unsafe impl ::windows::runtime::Abi for PXE_ADDRESS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PXE_ADDR_BROADCAST: u32 = 1u32;
pub const PXE_ADDR_USE_ADDR: u32 = 4u32;
pub const PXE_ADDR_USE_DHCP_RULES: u32 = 8u32;
pub const PXE_ADDR_USE_PORT: u32 = 2u32;
pub const PXE_BA_CUSTOM: u32 = 2u32;
pub const PXE_BA_IGNORE: u32 = 3u32;
pub const PXE_BA_NBP: u32 = 1u32;
pub const PXE_BA_REJECTED: u32 = 4u32;
pub const PXE_CALLBACK_MAX: u32 = 3u32;
pub const PXE_CALLBACK_RECV_REQUEST: u32 = 0u32;
pub const PXE_CALLBACK_SERVICE_CONTROL: u32 = 2u32;
pub const PXE_CALLBACK_SHUTDOWN: u32 = 1u32;
pub const PXE_DHCPV6_CLIENT_PORT: u32 = 546u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PXE_DHCPV6_MESSAGE {
    pub MessageType: u8,
    pub TransactionIDByte1: u8,
    pub TransactionIDByte2: u8,
    pub TransactionIDByte3: u8,
    pub Options: [PXE_DHCPV6_OPTION; 1],
}
impl PXE_DHCPV6_MESSAGE {}
impl ::std::default::Default for PXE_DHCPV6_MESSAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PXE_DHCPV6_MESSAGE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PXE_DHCPV6_MESSAGE {}
unsafe impl ::windows::runtime::Abi for PXE_DHCPV6_MESSAGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PXE_DHCPV6_MESSAGE_HEADER {
    pub MessageType: u8,
    pub Message: [u8; 1],
}
impl PXE_DHCPV6_MESSAGE_HEADER {}
impl ::std::default::Default for PXE_DHCPV6_MESSAGE_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PXE_DHCPV6_MESSAGE_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PXE_DHCPV6_MESSAGE_HEADER").field("MessageType", &self.MessageType).field("Message", &self.Message).finish()
    }
}
impl ::std::cmp::PartialEq for PXE_DHCPV6_MESSAGE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Message == other.Message
    }
}
impl ::std::cmp::Eq for PXE_DHCPV6_MESSAGE_HEADER {}
unsafe impl ::windows::runtime::Abi for PXE_DHCPV6_MESSAGE_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PXE_DHCPV6_NESTED_RELAY_MESSAGE {
    pub pRelayMessage: *mut PXE_DHCPV6_RELAY_MESSAGE,
    pub cbRelayMessage: u32,
    pub pInterfaceIdOption: *mut ::std::ffi::c_void,
    pub cbInterfaceIdOption: u16,
}
impl PXE_DHCPV6_NESTED_RELAY_MESSAGE {}
impl ::std::default::Default for PXE_DHCPV6_NESTED_RELAY_MESSAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PXE_DHCPV6_NESTED_RELAY_MESSAGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PXE_DHCPV6_NESTED_RELAY_MESSAGE").field("pRelayMessage", &self.pRelayMessage).field("cbRelayMessage", &self.cbRelayMessage).field("pInterfaceIdOption", &self.pInterfaceIdOption).field("cbInterfaceIdOption", &self.cbInterfaceIdOption).finish()
    }
}
impl ::std::cmp::PartialEq for PXE_DHCPV6_NESTED_RELAY_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.pRelayMessage == other.pRelayMessage && self.cbRelayMessage == other.cbRelayMessage && self.pInterfaceIdOption == other.pInterfaceIdOption && self.cbInterfaceIdOption == other.cbInterfaceIdOption
    }
}
impl ::std::cmp::Eq for PXE_DHCPV6_NESTED_RELAY_MESSAGE {}
unsafe impl ::windows::runtime::Abi for PXE_DHCPV6_NESTED_RELAY_MESSAGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct PXE_DHCPV6_OPTION {
    pub OptionCode: u16,
    pub DataLength: u16,
    pub Data: [u8; 1],
}
impl PXE_DHCPV6_OPTION {}
impl ::std::default::Default for PXE_DHCPV6_OPTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PXE_DHCPV6_OPTION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PXE_DHCPV6_OPTION {}
unsafe impl ::windows::runtime::Abi for PXE_DHCPV6_OPTION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PXE_DHCPV6_RELAY_HOP_COUNT_LIMIT: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PXE_DHCPV6_RELAY_MESSAGE {
    pub MessageType: u8,
    pub HopCount: u8,
    pub LinkAddress: [u8; 16],
    pub PeerAddress: [u8; 16],
    pub Options: [PXE_DHCPV6_OPTION; 1],
}
impl PXE_DHCPV6_RELAY_MESSAGE {}
impl ::std::default::Default for PXE_DHCPV6_RELAY_MESSAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PXE_DHCPV6_RELAY_MESSAGE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PXE_DHCPV6_RELAY_MESSAGE {}
unsafe impl ::windows::runtime::Abi for PXE_DHCPV6_RELAY_MESSAGE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PXE_DHCPV6_SERVER_PORT: u32 = 547u32;
pub const PXE_DHCP_CLIENT_PORT: u32 = 68u32;
pub const PXE_DHCP_FILE_SIZE: u32 = 128u32;
pub const PXE_DHCP_HWAADR_SIZE: u32 = 16u32;
pub const PXE_DHCP_MAGIC_COOKIE_SIZE: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct PXE_DHCP_MESSAGE {
    pub Operation: u8,
    pub HardwareAddressType: u8,
    pub HardwareAddressLength: u8,
    pub HopCount: u8,
    pub TransactionID: u32,
    pub SecondsSinceBoot: u16,
    pub Reserved: u16,
    pub ClientIpAddress: u32,
    pub YourIpAddress: u32,
    pub BootstrapServerAddress: u32,
    pub RelayAgentIpAddress: u32,
    pub HardwareAddress: [u8; 16],
    pub HostName: [u8; 64],
    pub BootFileName: [u8; 128],
    pub Anonymous: PXE_DHCP_MESSAGE_0,
    pub Option: PXE_DHCP_OPTION,
}
impl PXE_DHCP_MESSAGE {}
impl ::std::default::Default for PXE_DHCP_MESSAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PXE_DHCP_MESSAGE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PXE_DHCP_MESSAGE {}
unsafe impl ::windows::runtime::Abi for PXE_DHCP_MESSAGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub union PXE_DHCP_MESSAGE_0 {
    pub bMagicCookie: [u8; 4],
    pub uMagicCookie: u32,
}
impl PXE_DHCP_MESSAGE_0 {}
impl ::std::default::Default for PXE_DHCP_MESSAGE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PXE_DHCP_MESSAGE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PXE_DHCP_MESSAGE_0 {}
unsafe impl ::windows::runtime::Abi for PXE_DHCP_MESSAGE_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PXE_DHCP_OPTION {
    pub OptionType: u8,
    pub OptionLength: u8,
    pub OptionValue: [u8; 1],
}
impl PXE_DHCP_OPTION {}
impl ::std::default::Default for PXE_DHCP_OPTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PXE_DHCP_OPTION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PXE_DHCP_OPTION").field("OptionType", &self.OptionType).field("OptionLength", &self.OptionLength).field("OptionValue", &self.OptionValue).finish()
    }
}
impl ::std::cmp::PartialEq for PXE_DHCP_OPTION {
    fn eq(&self, other: &Self) -> bool {
        self.OptionType == other.OptionType && self.OptionLength == other.OptionLength && self.OptionValue == other.OptionValue
    }
}
impl ::std::cmp::Eq for PXE_DHCP_OPTION {}
unsafe impl ::windows::runtime::Abi for PXE_DHCP_OPTION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PXE_DHCP_SERVER_PORT: u32 = 67u32;
pub const PXE_DHCP_SERVER_SIZE: u32 = 64u32;
pub const PXE_GSI_SERVER_DUID: u32 = 2u32;
pub const PXE_GSI_TRACE_ENABLED: u32 = 1u32;
pub const PXE_MAX_ADDRESS: u32 = 16u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PXE_PROVIDER {
    pub uSizeOfStruct: u32,
    pub pwszName: super::super::Foundation::PWSTR,
    pub pwszFilePath: super::super::Foundation::PWSTR,
    pub bIsCritical: super::super::Foundation::BOOL,
    pub uIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl PXE_PROVIDER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PXE_PROVIDER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PXE_PROVIDER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PXE_PROVIDER").field("uSizeOfStruct", &self.uSizeOfStruct).field("pwszName", &self.pwszName).field("pwszFilePath", &self.pwszFilePath).field("bIsCritical", &self.bIsCritical).field("uIndex", &self.uIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PXE_PROVIDER {
    fn eq(&self, other: &Self) -> bool {
        self.uSizeOfStruct == other.uSizeOfStruct && self.pwszName == other.pwszName && self.pwszFilePath == other.pwszFilePath && self.bIsCritical == other.bIsCritical && self.uIndex == other.uIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PXE_PROVIDER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PXE_PROVIDER {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PXE_PROV_ATTR_FILTER: u32 = 0u32;
pub const PXE_PROV_ATTR_FILTER_IPV6: u32 = 1u32;
pub const PXE_PROV_ATTR_IPV6_CAPABLE: u32 = 2u32;
pub const PXE_PROV_FILTER_ALL: u32 = 0u32;
pub const PXE_PROV_FILTER_DHCP_ONLY: u32 = 1u32;
pub const PXE_PROV_FILTER_PXE_ONLY: u32 = 2u32;
pub const PXE_REG_INDEX_BOTTOM: u32 = 4294967295u32;
pub const PXE_REG_INDEX_TOP: u32 = 0u32;
pub const PXE_SERVER_PORT: u32 = 4011u32;
pub const PXE_TRACE_ERROR: u32 = 524288u32;
pub const PXE_TRACE_FATAL: u32 = 1048576u32;
pub const PXE_TRACE_INFO: u32 = 131072u32;
pub const PXE_TRACE_VERBOSE: u32 = 65536u32;
pub const PXE_TRACE_WARNING: u32 = 262144u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeAsyncRecvDone<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hclientrequest: Param0, action: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeAsyncRecvDone(hclientrequest: super::super::Foundation::HANDLE, action: u32) -> u32;
        }
        ::std::mem::transmute(PxeAsyncRecvDone(hclientrequest.into_param().abi(), ::std::mem::transmute(action)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PxeDhcpAppendOption(preplypacket: *mut ::std::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32, boption: u8, boptionlen: u8, pvalue: *const ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeDhcpAppendOption(preplypacket: *mut ::std::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32, boption: u8, boptionlen: u8, pvalue: *const ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(PxeDhcpAppendOption(::std::mem::transmute(preplypacket), ::std::mem::transmute(umaxreplypacketlen), ::std::mem::transmute(pureplypacketlen), ::std::mem::transmute(boption), ::std::mem::transmute(boptionlen), ::std::mem::transmute(pvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PxeDhcpAppendOptionRaw(preplypacket: *mut ::std::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32, ubufferlen: u16, pbuffer: *const ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeDhcpAppendOptionRaw(preplypacket: *mut ::std::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32, ubufferlen: u16, pbuffer: *const ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(PxeDhcpAppendOptionRaw(::std::mem::transmute(preplypacket), ::std::mem::transmute(umaxreplypacketlen), ::std::mem::transmute(pureplypacketlen), ::std::mem::transmute(ubufferlen), ::std::mem::transmute(pbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PxeDhcpGetOptionValue(ppacket: *const ::std::ffi::c_void, upacketlen: u32, uinstance: u32, boption: u8, pboptionlen: *mut u8, ppoptionvalue: *mut *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeDhcpGetOptionValue(ppacket: *const ::std::ffi::c_void, upacketlen: u32, uinstance: u32, boption: u8, pboptionlen: *mut u8, ppoptionvalue: *mut *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(PxeDhcpGetOptionValue(::std::mem::transmute(ppacket), ::std::mem::transmute(upacketlen), ::std::mem::transmute(uinstance), ::std::mem::transmute(boption), ::std::mem::transmute(pboptionlen), ::std::mem::transmute(ppoptionvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PxeDhcpGetVendorOptionValue(ppacket: *const ::std::ffi::c_void, upacketlen: u32, boption: u8, uinstance: u32, pboptionlen: *mut u8, ppoptionvalue: *mut *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeDhcpGetVendorOptionValue(ppacket: *const ::std::ffi::c_void, upacketlen: u32, boption: u8, uinstance: u32, pboptionlen: *mut u8, ppoptionvalue: *mut *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(PxeDhcpGetVendorOptionValue(::std::mem::transmute(ppacket), ::std::mem::transmute(upacketlen), ::std::mem::transmute(boption), ::std::mem::transmute(uinstance), ::std::mem::transmute(pboptionlen), ::std::mem::transmute(ppoptionvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PxeDhcpInitialize(precvpacket: *const ::std::ffi::c_void, urecvpacketlen: u32, preplypacket: *mut ::std::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeDhcpInitialize(precvpacket: *const ::std::ffi::c_void, urecvpacketlen: u32, preplypacket: *mut ::std::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32) -> u32;
        }
        ::std::mem::transmute(PxeDhcpInitialize(::std::mem::transmute(precvpacket), ::std::mem::transmute(urecvpacketlen), ::std::mem::transmute(preplypacket), ::std::mem::transmute(umaxreplypacketlen), ::std::mem::transmute(pureplypacketlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeDhcpIsValid<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(ppacket: *const ::std::ffi::c_void, upacketlen: u32, brequestpacket: Param2, pbpxeoptionpresent: *mut super::super::Foundation::BOOL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeDhcpIsValid(ppacket: *const ::std::ffi::c_void, upacketlen: u32, brequestpacket: super::super::Foundation::BOOL, pbpxeoptionpresent: *mut super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(PxeDhcpIsValid(::std::mem::transmute(ppacket), ::std::mem::transmute(upacketlen), brequestpacket.into_param().abi(), ::std::mem::transmute(pbpxeoptionpresent)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PxeDhcpv6AppendOption(preply: *mut ::std::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32, woptiontype: u16, cboption: u16, poption: *const ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeDhcpv6AppendOption(preply: *mut ::std::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32, woptiontype: u16, cboption: u16, poption: *const ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(PxeDhcpv6AppendOption(::std::mem::transmute(preply), ::std::mem::transmute(cbreply), ::std::mem::transmute(pcbreplyused), ::std::mem::transmute(woptiontype), ::std::mem::transmute(cboption), ::std::mem::transmute(poption)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PxeDhcpv6AppendOptionRaw(preply: *mut ::std::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32, cbbuffer: u16, pbuffer: *const ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeDhcpv6AppendOptionRaw(preply: *mut ::std::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32, cbbuffer: u16, pbuffer: *const ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(PxeDhcpv6AppendOptionRaw(::std::mem::transmute(preply), ::std::mem::transmute(cbreply), ::std::mem::transmute(pcbreplyused), ::std::mem::transmute(cbbuffer), ::std::mem::transmute(pbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PxeDhcpv6CreateRelayRepl(prelaymessages: *const PXE_DHCPV6_NESTED_RELAY_MESSAGE, nrelaymessages: u32, pinnerpacket: *const u8, cbinnerpacket: u32, preplybuffer: *mut ::std::ffi::c_void, cbreplybuffer: u32, pcbreplybuffer: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeDhcpv6CreateRelayRepl(prelaymessages: *const PXE_DHCPV6_NESTED_RELAY_MESSAGE, nrelaymessages: u32, pinnerpacket: *const u8, cbinnerpacket: u32, preplybuffer: *mut ::std::ffi::c_void, cbreplybuffer: u32, pcbreplybuffer: *mut u32) -> u32;
        }
        ::std::mem::transmute(PxeDhcpv6CreateRelayRepl(::std::mem::transmute(prelaymessages), ::std::mem::transmute(nrelaymessages), ::std::mem::transmute(pinnerpacket), ::std::mem::transmute(cbinnerpacket), ::std::mem::transmute(preplybuffer), ::std::mem::transmute(cbreplybuffer), ::std::mem::transmute(pcbreplybuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PxeDhcpv6GetOptionValue(ppacket: *const ::std::ffi::c_void, upacketlen: u32, uinstance: u32, woption: u16, pwoptionlen: *mut u16, ppoptionvalue: *mut *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeDhcpv6GetOptionValue(ppacket: *const ::std::ffi::c_void, upacketlen: u32, uinstance: u32, woption: u16, pwoptionlen: *mut u16, ppoptionvalue: *mut *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(PxeDhcpv6GetOptionValue(::std::mem::transmute(ppacket), ::std::mem::transmute(upacketlen), ::std::mem::transmute(uinstance), ::std::mem::transmute(woption), ::std::mem::transmute(pwoptionlen), ::std::mem::transmute(ppoptionvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PxeDhcpv6GetVendorOptionValue(ppacket: *const ::std::ffi::c_void, upacketlen: u32, dwenterprisenumber: u32, woption: u16, uinstance: u32, pwoptionlen: *mut u16, ppoptionvalue: *mut *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeDhcpv6GetVendorOptionValue(ppacket: *const ::std::ffi::c_void, upacketlen: u32, dwenterprisenumber: u32, woption: u16, uinstance: u32, pwoptionlen: *mut u16, ppoptionvalue: *mut *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(PxeDhcpv6GetVendorOptionValue(::std::mem::transmute(ppacket), ::std::mem::transmute(upacketlen), ::std::mem::transmute(dwenterprisenumber), ::std::mem::transmute(woption), ::std::mem::transmute(uinstance), ::std::mem::transmute(pwoptionlen), ::std::mem::transmute(ppoptionvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PxeDhcpv6Initialize(prequest: *const ::std::ffi::c_void, cbrequest: u32, preply: *mut ::std::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeDhcpv6Initialize(prequest: *const ::std::ffi::c_void, cbrequest: u32, preply: *mut ::std::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32) -> u32;
        }
        ::std::mem::transmute(PxeDhcpv6Initialize(::std::mem::transmute(prequest), ::std::mem::transmute(cbrequest), ::std::mem::transmute(preply), ::std::mem::transmute(cbreply), ::std::mem::transmute(pcbreplyused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeDhcpv6IsValid<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(ppacket: *const ::std::ffi::c_void, upacketlen: u32, brequestpacket: Param2, pbpxeoptionpresent: *mut super::super::Foundation::BOOL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeDhcpv6IsValid(ppacket: *const ::std::ffi::c_void, upacketlen: u32, brequestpacket: super::super::Foundation::BOOL, pbpxeoptionpresent: *mut super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(PxeDhcpv6IsValid(::std::mem::transmute(ppacket), ::std::mem::transmute(upacketlen), brequestpacket.into_param().abi(), ::std::mem::transmute(pbpxeoptionpresent)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PxeDhcpv6ParseRelayForw(prelayforwpacket: *const ::std::ffi::c_void, urelayforwpacketlen: u32, prelaymessages: *mut PXE_DHCPV6_NESTED_RELAY_MESSAGE, nrelaymessages: u32, pnrelaymessages: *mut u32, ppinnerpacket: *mut *mut u8, pcbinnerpacket: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeDhcpv6ParseRelayForw(prelayforwpacket: *const ::std::ffi::c_void, urelayforwpacketlen: u32, prelaymessages: *mut PXE_DHCPV6_NESTED_RELAY_MESSAGE, nrelaymessages: u32, pnrelaymessages: *mut u32, ppinnerpacket: *mut *mut u8, pcbinnerpacket: *mut u32) -> u32;
        }
        ::std::mem::transmute(PxeDhcpv6ParseRelayForw(
            ::std::mem::transmute(prelayforwpacket),
            ::std::mem::transmute(urelayforwpacketlen),
            ::std::mem::transmute(prelaymessages),
            ::std::mem::transmute(nrelaymessages),
            ::std::mem::transmute(pnrelaymessages),
            ::std::mem::transmute(ppinnerpacket),
            ::std::mem::transmute(pcbinnerpacket),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PxeGetServerInfo(uinfotype: u32, pbuffer: *mut ::std::ffi::c_void, ubufferlen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeGetServerInfo(uinfotype: u32, pbuffer: *mut ::std::ffi::c_void, ubufferlen: u32) -> u32;
        }
        ::std::mem::transmute(PxeGetServerInfo(::std::mem::transmute(uinfotype), ::std::mem::transmute(pbuffer), ::std::mem::transmute(ubufferlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PxeGetServerInfoEx(uinfotype: u32, pbuffer: *mut ::std::ffi::c_void, ubufferlen: u32, pubufferused: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeGetServerInfoEx(uinfotype: u32, pbuffer: *mut ::std::ffi::c_void, ubufferlen: u32, pubufferused: *mut u32) -> u32;
        }
        ::std::mem::transmute(PxeGetServerInfoEx(::std::mem::transmute(uinfotype), ::std::mem::transmute(pbuffer), ::std::mem::transmute(ubufferlen), ::std::mem::transmute(pubufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxePacketAllocate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprovider: Param0, hclientrequest: Param1, usize: u32) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxePacketAllocate(hprovider: super::super::Foundation::HANDLE, hclientrequest: super::super::Foundation::HANDLE, usize: u32) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(PxePacketAllocate(hprovider.into_param().abi(), hclientrequest.into_param().abi(), ::std::mem::transmute(usize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxePacketFree<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprovider: Param0, hclientrequest: Param1, ppacket: *const ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxePacketFree(hprovider: super::super::Foundation::HANDLE, hclientrequest: super::super::Foundation::HANDLE, ppacket: *const ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(PxePacketFree(hprovider.into_param().abi(), hclientrequest.into_param().abi(), ::std::mem::transmute(ppacket)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeProviderEnumClose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(henum: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeProviderEnumClose(henum: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(PxeProviderEnumClose(henum.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeProviderEnumFirst(phenum: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeProviderEnumFirst(phenum: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(PxeProviderEnumFirst(::std::mem::transmute(phenum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeProviderEnumNext<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(henum: Param0, ppprovider: *mut *mut PXE_PROVIDER) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeProviderEnumNext(henum: super::super::Foundation::HANDLE, ppprovider: *mut *mut PXE_PROVIDER) -> u32;
        }
        ::std::mem::transmute(PxeProviderEnumNext(henum.into_param().abi(), ::std::mem::transmute(ppprovider)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeProviderFreeInfo(pprovider: *const PXE_PROVIDER) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeProviderFreeInfo(pprovider: *const PXE_PROVIDER) -> u32;
        }
        ::std::mem::transmute(PxeProviderFreeInfo(::std::mem::transmute(pprovider)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeProviderQueryIndex<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszprovidername: Param0, puindex: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeProviderQueryIndex(pszprovidername: super::super::Foundation::PWSTR, puindex: *mut u32) -> u32;
        }
        ::std::mem::transmute(PxeProviderQueryIndex(pszprovidername.into_param().abi(), ::std::mem::transmute(puindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn PxeProviderRegister<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(pszprovidername: Param0, pszmodulepath: Param1, index: u32, biscritical: Param3, phproviderkey: *mut super::Registry::HKEY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeProviderRegister(pszprovidername: super::super::Foundation::PWSTR, pszmodulepath: super::super::Foundation::PWSTR, index: u32, biscritical: super::super::Foundation::BOOL, phproviderkey: *mut super::Registry::HKEY) -> u32;
        }
        ::std::mem::transmute(PxeProviderRegister(pszprovidername.into_param().abi(), pszmodulepath.into_param().abi(), ::std::mem::transmute(index), biscritical.into_param().abi(), ::std::mem::transmute(phproviderkey)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeProviderSetAttribute<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprovider: Param0, attribute: u32, pparameterbuffer: *const ::std::ffi::c_void, uparamlen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeProviderSetAttribute(hprovider: super::super::Foundation::HANDLE, attribute: u32, pparameterbuffer: *const ::std::ffi::c_void, uparamlen: u32) -> u32;
        }
        ::std::mem::transmute(PxeProviderSetAttribute(hprovider.into_param().abi(), ::std::mem::transmute(attribute), ::std::mem::transmute(pparameterbuffer), ::std::mem::transmute(uparamlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeProviderUnRegister<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszprovidername: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeProviderUnRegister(pszprovidername: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(PxeProviderUnRegister(pszprovidername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeRegisterCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprovider: Param0, callbacktype: u32, pcallbackfunction: *const ::std::ffi::c_void, pcontext: *const ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeRegisterCallback(hprovider: super::super::Foundation::HANDLE, callbacktype: u32, pcallbackfunction: *const ::std::ffi::c_void, pcontext: *const ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(PxeRegisterCallback(hprovider.into_param().abi(), ::std::mem::transmute(callbacktype), ::std::mem::transmute(pcallbackfunction), ::std::mem::transmute(pcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeSendReply<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hclientrequest: Param0, ppacket: *const ::std::ffi::c_void, upacketlen: u32, paddress: *const PXE_ADDRESS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeSendReply(hclientrequest: super::super::Foundation::HANDLE, ppacket: *const ::std::ffi::c_void, upacketlen: u32, paddress: *const PXE_ADDRESS) -> u32;
        }
        ::std::mem::transmute(PxeSendReply(hclientrequest.into_param().abi(), ::std::mem::transmute(ppacket), ::std::mem::transmute(upacketlen), ::std::mem::transmute(paddress)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeTrace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hprovider: Param0, severity: u32, pszformat: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeTrace(hprovider: super::super::Foundation::HANDLE, severity: u32, pszformat: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(PxeTrace(hprovider.into_param().abi(), ::std::mem::transmute(severity), pszformat.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeTraceV<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hprovider: Param0, severity: u32, pszformat: Param2, params: *const i8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PxeTraceV(hprovider: super::super::Foundation::HANDLE, severity: u32, pszformat: super::super::Foundation::PWSTR, params: *const i8) -> u32;
        }
        ::std::mem::transmute(PxeTraceV(hprovider.into_param().abi(), ::std::mem::transmute(severity), pszformat.into_param().abi(), ::std::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TRANSPORTCLIENT_CALLBACK_ID(pub i32);
pub const WDS_TRANSPORTCLIENT_SESSION_START: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(0i32);
pub const WDS_TRANSPORTCLIENT_RECEIVE_CONTENTS: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(1i32);
pub const WDS_TRANSPORTCLIENT_SESSION_COMPLETE: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(2i32);
pub const WDS_TRANSPORTCLIENT_RECEIVE_METADATA: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(3i32);
pub const WDS_TRANSPORTCLIENT_SESSION_STARTEX: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(4i32);
pub const WDS_TRANSPORTCLIENT_SESSION_NEGOTIATE: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(5i32);
pub const WDS_TRANSPORTCLIENT_MAX_CALLBACKS: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(6i32);
impl ::std::convert::From<i32> for TRANSPORTCLIENT_CALLBACK_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TRANSPORTCLIENT_CALLBACK_ID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct TRANSPORTCLIENT_SESSION_INFO {
    pub ulStructureLength: u32,
    pub ullFileSize: u64,
    pub ulBlockSize: u32,
}
impl TRANSPORTCLIENT_SESSION_INFO {}
impl ::std::default::Default for TRANSPORTCLIENT_SESSION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRANSPORTCLIENT_SESSION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRANSPORTCLIENT_SESSION_INFO").field("ulStructureLength", &self.ulStructureLength).field("ullFileSize", &self.ullFileSize).field("ulBlockSize", &self.ulBlockSize).finish()
    }
}
impl ::std::cmp::PartialEq for TRANSPORTCLIENT_SESSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulStructureLength == other.ulStructureLength && self.ullFileSize == other.ullFileSize && self.ulBlockSize == other.ulBlockSize
    }
}
impl ::std::cmp::Eq for TRANSPORTCLIENT_SESSION_INFO {}
unsafe impl ::windows::runtime::Abi for TRANSPORTCLIENT_SESSION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TRANSPORTPROVIDER_CALLBACK_ID(pub i32);
pub const WDS_TRANSPORTPROVIDER_CREATE_INSTANCE: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(0i32);
pub const WDS_TRANSPORTPROVIDER_COMPARE_CONTENT: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(1i32);
pub const WDS_TRANSPORTPROVIDER_OPEN_CONTENT: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(2i32);
pub const WDS_TRANSPORTPROVIDER_USER_ACCESS_CHECK: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(3i32);
pub const WDS_TRANSPORTPROVIDER_GET_CONTENT_SIZE: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(4i32);
pub const WDS_TRANSPORTPROVIDER_READ_CONTENT: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(5i32);
pub const WDS_TRANSPORTPROVIDER_CLOSE_CONTENT: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(6i32);
pub const WDS_TRANSPORTPROVIDER_CLOSE_INSTANCE: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(7i32);
pub const WDS_TRANSPORTPROVIDER_SHUTDOWN: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(8i32);
pub const WDS_TRANSPORTPROVIDER_DUMP_STATE: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(9i32);
pub const WDS_TRANSPORTPROVIDER_REFRESH_SETTINGS: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(10i32);
pub const WDS_TRANSPORTPROVIDER_GET_CONTENT_METADATA: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(11i32);
pub const WDS_TRANSPORTPROVIDER_MAX_CALLBACKS: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(12i32);
impl ::std::convert::From<i32> for TRANSPORTPROVIDER_CALLBACK_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TRANSPORTPROVIDER_CALLBACK_ID {
    type Abi = Self;
    type DefaultType = Self;
}
pub const TRANSPORTPROVIDER_CURRENT_VERSION: u32 = 1u32;
pub const WDSBP_OPTVAL_ACTION_ABORT: u32 = 5u32;
pub const WDSBP_OPTVAL_ACTION_APPROVAL: u32 = 1u32;
pub const WDSBP_OPTVAL_ACTION_REFERRAL: u32 = 3u32;
pub const WDSBP_OPTVAL_NBP_VER_7: u32 = 1792u32;
pub const WDSBP_OPTVAL_NBP_VER_8: u32 = 2048u32;
pub const WDSBP_OPTVAL_PXE_PROMPT_NOPROMPT: u32 = 2u32;
pub const WDSBP_OPTVAL_PXE_PROMPT_OPTIN: u32 = 1u32;
pub const WDSBP_OPTVAL_PXE_PROMPT_OPTOUT: u32 = 3u32;
pub const WDSBP_OPT_TYPE_BYTE: u32 = 1u32;
pub const WDSBP_OPT_TYPE_IP4: u32 = 6u32;
pub const WDSBP_OPT_TYPE_IP6: u32 = 7u32;
pub const WDSBP_OPT_TYPE_NONE: u32 = 0u32;
pub const WDSBP_OPT_TYPE_STR: u32 = 5u32;
pub const WDSBP_OPT_TYPE_ULONG: u32 = 3u32;
pub const WDSBP_OPT_TYPE_USHORT: u32 = 2u32;
pub const WDSBP_OPT_TYPE_WSTR: u32 = 4u32;
pub const WDSBP_PK_TYPE_BCD: u32 = 4u32;
pub const WDSBP_PK_TYPE_DHCP: u32 = 1u32;
pub const WDSBP_PK_TYPE_DHCPV6: u32 = 8u32;
pub const WDSBP_PK_TYPE_WDSNBP: u32 = 2u32;
pub const WDSMCCLIENT_CATEGORY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(2i32 as _);
pub const WDSMCSERVER_CATEGORY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(1i32 as _);
pub const WDSMCS_E_CLIENT_DOESNOT_SUPPORT_SECURITY_MODE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801648i32 as _);
pub const WDSMCS_E_CLIENT_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801660i32 as _);
pub const WDSMCS_E_CONTENT_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801661i32 as _);
pub const WDSMCS_E_CONTENT_PROVIDER_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801658i32 as _);
pub const WDSMCS_E_INCOMPATIBLE_VERSION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801662i32 as _);
pub const WDSMCS_E_NAMESPACE_ALREADY_EXISTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801657i32 as _);
pub const WDSMCS_E_NAMESPACE_ALREADY_STARTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801655i32 as _);
pub const WDSMCS_E_NAMESPACE_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801659i32 as _);
pub const WDSMCS_E_NAMESPACE_SHUTDOWN_IN_PROGRESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801656i32 as _);
pub const WDSMCS_E_NS_START_FAILED_NO_CLIENTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801654i32 as _);
pub const WDSMCS_E_PACKET_HAS_SECURITY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801650i32 as _);
pub const WDSMCS_E_PACKET_NOT_CHECKSUMED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801649i32 as _);
pub const WDSMCS_E_PACKET_NOT_HASHED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801652i32 as _);
pub const WDSMCS_E_PACKET_NOT_SIGNED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801651i32 as _);
pub const WDSMCS_E_REQCALLBACKS_NOT_REG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801663i32 as _);
pub const WDSMCS_E_SESSION_SHUTDOWN_IN_PROGRESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801664i32 as _);
pub const WDSMCS_E_START_TIME_IN_PAST: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054801653i32 as _);
pub const WDSTPC_E_ALREADY_COMPLETED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054735615i32 as _);
pub const WDSTPC_E_ALREADY_IN_LOWEST_SESSION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054735606i32 as _);
pub const WDSTPC_E_ALREADY_IN_PROGRESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054735614i32 as _);
pub const WDSTPC_E_CALLBACKS_NOT_REG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054735616i32 as _);
pub const WDSTPC_E_CLIENT_DEMOTE_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054735605i32 as _);
pub const WDSTPC_E_KICKED_FAIL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054735609i32 as _);
pub const WDSTPC_E_KICKED_FALLBACK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054735610i32 as _);
pub const WDSTPC_E_KICKED_POLICY_NOT_MET: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054735611i32 as _);
pub const WDSTPC_E_KICKED_UNKNOWN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054735608i32 as _);
pub const WDSTPC_E_MULTISTREAM_NOT_ENABLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054735607i32 as _);
pub const WDSTPC_E_NOT_INITIALIZED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054735612i32 as _);
pub const WDSTPC_E_NO_IP4_INTERFACE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054735604i32 as _);
pub const WDSTPC_E_UNKNOWN_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054735613i32 as _);
pub const WDSTPTC_E_WIM_APPLY_REQUIRES_REFERENCE_IMAGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1054735603i32 as _);
pub const WDSTPTMGMT_CATEGORY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(1i32 as _);
pub const WDSTPTMGMT_E_CANNOT_REFRESH_DIRTY_OBJECT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915761i32 as _);
pub const WDSTPTMGMT_E_CANNOT_REINITIALIZE_OBJECT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915767i32 as _);
pub const WDSTPTMGMT_E_CONTENT_PROVIDER_ALREADY_REGISTERED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915773i32 as _);
pub const WDSTPTMGMT_E_CONTENT_PROVIDER_NOT_REGISTERED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915772i32 as _);
pub const WDSTPTMGMT_E_INVALID_AUTO_DISCONNECT_THRESHOLD: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915748i32 as _);
pub const WDSTPTMGMT_E_INVALID_CLASS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915774i32 as _);
pub const WDSTPTMGMT_E_INVALID_CONTENT_PROVIDER_NAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915771i32 as _);
pub const WDSTPTMGMT_E_INVALID_DIAGNOSTICS_COMPONENTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915762i32 as _);
pub const WDSTPTMGMT_E_INVALID_IPV4_MULTICAST_ADDRESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915753i32 as _);
pub const WDSTPTMGMT_E_INVALID_IPV6_MULTICAST_ADDRESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915752i32 as _);
pub const WDSTPTMGMT_E_INVALID_IPV6_MULTICAST_ADDRESS_SOURCE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915750i32 as _);
pub const WDSTPTMGMT_E_INVALID_IP_ADDRESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915754i32 as _);
pub const WDSTPTMGMT_E_INVALID_MULTISTREAM_STREAM_COUNT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915749i32 as _);
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_DATA: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915765i32 as _);
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_NAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915766i32 as _);
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_START_PARAMETERS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915758i32 as _);
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_START_TIME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915763i32 as _);
pub const WDSTPTMGMT_E_INVALID_OPERATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915775i32 as _);
pub const WDSTPTMGMT_E_INVALID_PROPERTY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915776i32 as _);
pub const WDSTPTMGMT_E_INVALID_SERVICE_IP_ADDRESS_RANGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915760i32 as _);
pub const WDSTPTMGMT_E_INVALID_SERVICE_PORT_RANGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915759i32 as _);
pub const WDSTPTMGMT_E_INVALID_SLOW_CLIENT_HANDLING_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915746i32 as _);
pub const WDSTPTMGMT_E_INVALID_TFTP_MAX_BLOCKSIZE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915741i32 as _);
pub const WDSTPTMGMT_E_IPV6_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915751i32 as _);
pub const WDSTPTMGMT_E_MULTICAST_SESSION_POLICY_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915747i32 as _);
pub const WDSTPTMGMT_E_NAMESPACE_ALREADY_REGISTERED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915769i32 as _);
pub const WDSTPTMGMT_E_NAMESPACE_NOT_ON_SERVER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915756i32 as _);
pub const WDSTPTMGMT_E_NAMESPACE_NOT_REGISTERED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915768i32 as _);
pub const WDSTPTMGMT_E_NAMESPACE_READ_ONLY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915764i32 as _);
pub const WDSTPTMGMT_E_NAMESPACE_REMOVED_FROM_SERVER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915755i32 as _);
pub const WDSTPTMGMT_E_NETWORK_PROFILES_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915745i32 as _);
pub const WDSTPTMGMT_E_TFTP_MAX_BLOCKSIZE_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915743i32 as _);
pub const WDSTPTMGMT_E_TFTP_VAR_WINDOW_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915742i32 as _);
pub const WDSTPTMGMT_E_TRANSPORT_SERVER_ROLE_NOT_CONFIGURED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915770i32 as _);
pub const WDSTPTMGMT_E_TRANSPORT_SERVER_UNAVAILABLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915757i32 as _);
pub const WDSTPTMGMT_E_UDP_PORT_POLICY_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1055915744i32 as _);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS(pub i32);
pub const WdsTptDiagnosticsComponentPxe: WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS = WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS(1i32);
pub const WdsTptDiagnosticsComponentTftp: WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS = WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS(2i32);
pub const WdsTptDiagnosticsComponentImageServer: WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS = WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS(4i32);
pub const WdsTptDiagnosticsComponentMulticast: WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS = WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS(8i32);
impl ::std::convert::From<i32> for WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WDSTRANSPORT_DISCONNECT_TYPE(pub i32);
pub const WdsTptDisconnectUnknown: WDSTRANSPORT_DISCONNECT_TYPE = WDSTRANSPORT_DISCONNECT_TYPE(0i32);
pub const WdsTptDisconnectFallback: WDSTRANSPORT_DISCONNECT_TYPE = WDSTRANSPORT_DISCONNECT_TYPE(1i32);
pub const WdsTptDisconnectAbort: WDSTRANSPORT_DISCONNECT_TYPE = WDSTRANSPORT_DISCONNECT_TYPE(2i32);
impl ::std::convert::From<i32> for WDSTRANSPORT_DISCONNECT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WDSTRANSPORT_DISCONNECT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WDSTRANSPORT_FEATURE_FLAGS(pub i32);
pub const WdsTptFeatureAdminPack: WDSTRANSPORT_FEATURE_FLAGS = WDSTRANSPORT_FEATURE_FLAGS(1i32);
pub const WdsTptFeatureTransportServer: WDSTRANSPORT_FEATURE_FLAGS = WDSTRANSPORT_FEATURE_FLAGS(2i32);
pub const WdsTptFeatureDeploymentServer: WDSTRANSPORT_FEATURE_FLAGS = WDSTRANSPORT_FEATURE_FLAGS(4i32);
impl ::std::convert::From<i32> for WDSTRANSPORT_FEATURE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WDSTRANSPORT_FEATURE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE(pub i32);
pub const WdsTptIpAddressSourceUnknown: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE = WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE(0i32);
pub const WdsTptIpAddressSourceDhcp: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE = WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE(1i32);
pub const WdsTptIpAddressSourceRange: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE = WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE(2i32);
impl ::std::convert::From<i32> for WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WDSTRANSPORT_IP_ADDRESS_TYPE(pub i32);
pub const WdsTptIpAddressUnknown: WDSTRANSPORT_IP_ADDRESS_TYPE = WDSTRANSPORT_IP_ADDRESS_TYPE(0i32);
pub const WdsTptIpAddressIpv4: WDSTRANSPORT_IP_ADDRESS_TYPE = WDSTRANSPORT_IP_ADDRESS_TYPE(1i32);
pub const WdsTptIpAddressIpv6: WDSTRANSPORT_IP_ADDRESS_TYPE = WDSTRANSPORT_IP_ADDRESS_TYPE(2i32);
impl ::std::convert::From<i32> for WDSTRANSPORT_IP_ADDRESS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WDSTRANSPORT_IP_ADDRESS_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WDSTRANSPORT_NAMESPACE_TYPE(pub i32);
pub const WdsTptNamespaceTypeUnknown: WDSTRANSPORT_NAMESPACE_TYPE = WDSTRANSPORT_NAMESPACE_TYPE(0i32);
pub const WdsTptNamespaceTypeAutoCast: WDSTRANSPORT_NAMESPACE_TYPE = WDSTRANSPORT_NAMESPACE_TYPE(1i32);
pub const WdsTptNamespaceTypeScheduledCastManualStart: WDSTRANSPORT_NAMESPACE_TYPE = WDSTRANSPORT_NAMESPACE_TYPE(2i32);
pub const WdsTptNamespaceTypeScheduledCastAutoStart: WDSTRANSPORT_NAMESPACE_TYPE = WDSTRANSPORT_NAMESPACE_TYPE(3i32);
impl ::std::convert::From<i32> for WDSTRANSPORT_NAMESPACE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WDSTRANSPORT_NAMESPACE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WDSTRANSPORT_NETWORK_PROFILE_TYPE(pub i32);
pub const WdsTptNetworkProfileUnknown: WDSTRANSPORT_NETWORK_PROFILE_TYPE = WDSTRANSPORT_NETWORK_PROFILE_TYPE(0i32);
pub const WdsTptNetworkProfileCustom: WDSTRANSPORT_NETWORK_PROFILE_TYPE = WDSTRANSPORT_NETWORK_PROFILE_TYPE(1i32);
pub const WdsTptNetworkProfile10Mbps: WDSTRANSPORT_NETWORK_PROFILE_TYPE = WDSTRANSPORT_NETWORK_PROFILE_TYPE(2i32);
pub const WdsTptNetworkProfile100Mbps: WDSTRANSPORT_NETWORK_PROFILE_TYPE = WDSTRANSPORT_NETWORK_PROFILE_TYPE(3i32);
pub const WdsTptNetworkProfile1Gbps: WDSTRANSPORT_NETWORK_PROFILE_TYPE = WDSTRANSPORT_NETWORK_PROFILE_TYPE(4i32);
impl ::std::convert::From<i32> for WDSTRANSPORT_NETWORK_PROFILE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WDSTRANSPORT_NETWORK_PROFILE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WDSTRANSPORT_PROTOCOL_FLAGS(pub i32);
pub const WdsTptProtocolUnicast: WDSTRANSPORT_PROTOCOL_FLAGS = WDSTRANSPORT_PROTOCOL_FLAGS(1i32);
pub const WdsTptProtocolMulticast: WDSTRANSPORT_PROTOCOL_FLAGS = WDSTRANSPORT_PROTOCOL_FLAGS(2i32);
impl ::std::convert::From<i32> for WDSTRANSPORT_PROTOCOL_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WDSTRANSPORT_PROTOCOL_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WDSTRANSPORT_RESOURCE_UTILIZATION_UNKNOWN: u32 = 255u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WDSTRANSPORT_SERVICE_NOTIFICATION(pub i32);
pub const WdsTptServiceNotifyUnknown: WDSTRANSPORT_SERVICE_NOTIFICATION = WDSTRANSPORT_SERVICE_NOTIFICATION(0i32);
pub const WdsTptServiceNotifyReadSettings: WDSTRANSPORT_SERVICE_NOTIFICATION = WDSTRANSPORT_SERVICE_NOTIFICATION(1i32);
impl ::std::convert::From<i32> for WDSTRANSPORT_SERVICE_NOTIFICATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WDSTRANSPORT_SERVICE_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE(pub i32);
pub const WdsTptSlowClientHandlingUnknown: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE = WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE(0i32);
pub const WdsTptSlowClientHandlingNone: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE = WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE(1i32);
pub const WdsTptSlowClientHandlingAutoDisconnect: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE = WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE(2i32);
pub const WdsTptSlowClientHandlingMultistream: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE = WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE(3i32);
impl ::std::convert::From<i32> for WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WDSTRANSPORT_TFTP_CAPABILITY(pub i32);
pub const WdsTptTftpCapMaximumBlockSize: WDSTRANSPORT_TFTP_CAPABILITY = WDSTRANSPORT_TFTP_CAPABILITY(1i32);
pub const WdsTptTftpCapVariableWindow: WDSTRANSPORT_TFTP_CAPABILITY = WDSTRANSPORT_TFTP_CAPABILITY(2i32);
impl ::std::convert::From<i32> for WDSTRANSPORT_TFTP_CAPABILITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WDSTRANSPORT_TFTP_CAPABILITY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WDSTRANSPORT_UDP_PORT_POLICY(pub i32);
pub const WdsTptUdpPortPolicyDynamic: WDSTRANSPORT_UDP_PORT_POLICY = WDSTRANSPORT_UDP_PORT_POLICY(0i32);
pub const WdsTptUdpPortPolicyFixed: WDSTRANSPORT_UDP_PORT_POLICY = WDSTRANSPORT_UDP_PORT_POLICY(1i32);
impl ::std::convert::From<i32> for WDSTRANSPORT_UDP_PORT_POLICY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WDSTRANSPORT_UDP_PORT_POLICY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WDS_CLI_CRED {
    pub pwszUserName: super::super::Foundation::PWSTR,
    pub pwszDomain: super::super::Foundation::PWSTR,
    pub pwszPassword: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WDS_CLI_CRED {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WDS_CLI_CRED {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WDS_CLI_CRED {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WDS_CLI_CRED").field("pwszUserName", &self.pwszUserName).field("pwszDomain", &self.pwszDomain).field("pwszPassword", &self.pwszPassword).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WDS_CLI_CRED {
    fn eq(&self, other: &Self) -> bool {
        self.pwszUserName == other.pwszUserName && self.pwszDomain == other.pwszDomain && self.pwszPassword == other.pwszPassword
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WDS_CLI_CRED {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WDS_CLI_CRED {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WDS_CLI_FIRMWARE_TYPE(pub i32);
pub const WDS_CLI_FIRMWARE_UNKNOWN: WDS_CLI_FIRMWARE_TYPE = WDS_CLI_FIRMWARE_TYPE(0i32);
pub const WDS_CLI_FIRMWARE_BIOS: WDS_CLI_FIRMWARE_TYPE = WDS_CLI_FIRMWARE_TYPE(1i32);
pub const WDS_CLI_FIRMWARE_EFI: WDS_CLI_FIRMWARE_TYPE = WDS_CLI_FIRMWARE_TYPE(2i32);
impl ::std::convert::From<i32> for WDS_CLI_FIRMWARE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WDS_CLI_FIRMWARE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WDS_CLI_IMAGE_PARAM_TYPE(pub i32);
pub const WDS_CLI_IMAGE_PARAM_UNKNOWN: WDS_CLI_IMAGE_PARAM_TYPE = WDS_CLI_IMAGE_PARAM_TYPE(0i32);
pub const WDS_CLI_IMAGE_PARAM_SPARSE_FILE: WDS_CLI_IMAGE_PARAM_TYPE = WDS_CLI_IMAGE_PARAM_TYPE(1i32);
pub const WDS_CLI_IMAGE_PARAM_SUPPORTED_FIRMWARES: WDS_CLI_IMAGE_PARAM_TYPE = WDS_CLI_IMAGE_PARAM_TYPE(2i32);
impl ::std::convert::From<i32> for WDS_CLI_IMAGE_PARAM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WDS_CLI_IMAGE_PARAM_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WDS_CLI_IMAGE_TYPE(pub i32);
pub const WDS_CLI_IMAGE_TYPE_UNKNOWN: WDS_CLI_IMAGE_TYPE = WDS_CLI_IMAGE_TYPE(0i32);
pub const WDS_CLI_IMAGE_TYPE_WIM: WDS_CLI_IMAGE_TYPE = WDS_CLI_IMAGE_TYPE(1i32);
pub const WDS_CLI_IMAGE_TYPE_VHD: WDS_CLI_IMAGE_TYPE = WDS_CLI_IMAGE_TYPE(2i32);
pub const WDS_CLI_IMAGE_TYPE_VHDX: WDS_CLI_IMAGE_TYPE = WDS_CLI_IMAGE_TYPE(3i32);
impl ::std::convert::From<i32> for WDS_CLI_IMAGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WDS_CLI_IMAGE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WDS_CLI_NO_SPARSE_FILE: u32 = 2u32;
pub const WDS_CLI_TRANSFER_ASYNCHRONOUS: u32 = 1u32;
pub const WDS_LOG_LEVEL_DISABLED: i32 = 0i32;
pub const WDS_LOG_LEVEL_ERROR: i32 = 1i32;
pub const WDS_LOG_LEVEL_INFO: i32 = 3i32;
pub const WDS_LOG_LEVEL_WARNING: i32 = 2i32;
pub const WDS_LOG_TYPE_CLIENT_APPLY_FINISHED: i32 = 6i32;
pub const WDS_LOG_TYPE_CLIENT_APPLY_FINISHED_2: i32 = 16i32;
pub const WDS_LOG_TYPE_CLIENT_APPLY_STARTED: i32 = 5i32;
pub const WDS_LOG_TYPE_CLIENT_APPLY_STARTED_2: i32 = 15i32;
pub const WDS_LOG_TYPE_CLIENT_DOMAINJOINERROR: i32 = 12i32;
pub const WDS_LOG_TYPE_CLIENT_DOMAINJOINERROR_2: i32 = 17i32;
pub const WDS_LOG_TYPE_CLIENT_DRIVER_PACKAGE_NOT_ACCESSIBLE: i32 = 18i32;
pub const WDS_LOG_TYPE_CLIENT_ERROR: i32 = 1i32;
pub const WDS_LOG_TYPE_CLIENT_FINISHED: i32 = 3i32;
pub const WDS_LOG_TYPE_CLIENT_GENERIC_MESSAGE: i32 = 7i32;
pub const WDS_LOG_TYPE_CLIENT_IMAGE_SELECTED: i32 = 4i32;
pub const WDS_LOG_TYPE_CLIENT_IMAGE_SELECTED2: i32 = 22i32;
pub const WDS_LOG_TYPE_CLIENT_IMAGE_SELECTED3: i32 = 23i32;
pub const WDS_LOG_TYPE_CLIENT_MAX_CODE: i32 = 24i32;
pub const WDS_LOG_TYPE_CLIENT_OFFLINE_DRIVER_INJECTION_END: i32 = 20i32;
pub const WDS_LOG_TYPE_CLIENT_OFFLINE_DRIVER_INJECTION_FAILURE: i32 = 21i32;
pub const WDS_LOG_TYPE_CLIENT_OFFLINE_DRIVER_INJECTION_START: i32 = 19i32;
pub const WDS_LOG_TYPE_CLIENT_POST_ACTIONS_END: i32 = 14i32;
pub const WDS_LOG_TYPE_CLIENT_POST_ACTIONS_START: i32 = 13i32;
pub const WDS_LOG_TYPE_CLIENT_STARTED: i32 = 2i32;
pub const WDS_LOG_TYPE_CLIENT_TRANSFER_DOWNGRADE: i32 = 11i32;
pub const WDS_LOG_TYPE_CLIENT_TRANSFER_END: i32 = 10i32;
pub const WDS_LOG_TYPE_CLIENT_TRANSFER_START: i32 = 9i32;
pub const WDS_LOG_TYPE_CLIENT_UNATTEND_MODE: i32 = 8i32;
pub const WDS_MC_TRACE_ERROR: u32 = 524288u32;
pub const WDS_MC_TRACE_FATAL: u32 = 1048576u32;
pub const WDS_MC_TRACE_INFO: u32 = 131072u32;
pub const WDS_MC_TRACE_VERBOSE: u32 = 65536u32;
pub const WDS_MC_TRACE_WARNING: u32 = 262144u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WDS_TRANSPORTCLIENT_CALLBACKS {
    pub SessionStart: ::std::option::Option<PFN_WdsTransportClientSessionStart>,
    pub SessionStartEx: ::std::option::Option<PFN_WdsTransportClientSessionStartEx>,
    pub ReceiveContents: ::std::option::Option<PFN_WdsTransportClientReceiveContents>,
    pub ReceiveMetadata: ::std::option::Option<PFN_WdsTransportClientReceiveMetadata>,
    pub SessionComplete: ::std::option::Option<PFN_WdsTransportClientSessionComplete>,
    pub SessionNegotiate: ::std::option::Option<PFN_WdsTransportClientSessionNegotiate>,
}
#[cfg(feature = "Win32_Foundation")]
impl WDS_TRANSPORTCLIENT_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WDS_TRANSPORTCLIENT_CALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WDS_TRANSPORTCLIENT_CALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WDS_TRANSPORTCLIENT_CALLBACKS").finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WDS_TRANSPORTCLIENT_CALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.SessionStart.map(|f| f as usize) == other.SessionStart.map(|f| f as usize)
            && self.SessionStartEx.map(|f| f as usize) == other.SessionStartEx.map(|f| f as usize)
            && self.ReceiveContents.map(|f| f as usize) == other.ReceiveContents.map(|f| f as usize)
            && self.ReceiveMetadata.map(|f| f as usize) == other.ReceiveMetadata.map(|f| f as usize)
            && self.SessionComplete.map(|f| f as usize) == other.SessionComplete.map(|f| f as usize)
            && self.SessionNegotiate.map(|f| f as usize) == other.SessionNegotiate.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WDS_TRANSPORTCLIENT_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WDS_TRANSPORTCLIENT_CALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const WDS_TRANSPORTCLIENT_CURRENT_API_VERSION: u32 = 1u32;
pub const WDS_TRANSPORTCLIENT_NO_CACHE: u32 = 0u32;
pub const WDS_TRANSPORTCLIENT_PROTOCOL_MULTICAST: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WDS_TRANSPORTCLIENT_REQUEST {
    pub ulLength: u32,
    pub ulApiVersion: u32,
    pub ulAuthLevel: WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL,
    pub pwszServer: super::super::Foundation::PWSTR,
    pub pwszNamespace: super::super::Foundation::PWSTR,
    pub pwszObjectName: super::super::Foundation::PWSTR,
    pub ulCacheSize: u32,
    pub ulProtocol: u32,
    pub pvProtocolData: *mut ::std::ffi::c_void,
    pub ulProtocolDataLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WDS_TRANSPORTCLIENT_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WDS_TRANSPORTCLIENT_REQUEST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WDS_TRANSPORTCLIENT_REQUEST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WDS_TRANSPORTCLIENT_REQUEST")
            .field("ulLength", &self.ulLength)
            .field("ulApiVersion", &self.ulApiVersion)
            .field("ulAuthLevel", &self.ulAuthLevel)
            .field("pwszServer", &self.pwszServer)
            .field("pwszNamespace", &self.pwszNamespace)
            .field("pwszObjectName", &self.pwszObjectName)
            .field("ulCacheSize", &self.ulCacheSize)
            .field("ulProtocol", &self.ulProtocol)
            .field("pvProtocolData", &self.pvProtocolData)
            .field("ulProtocolDataLength", &self.ulProtocolDataLength)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WDS_TRANSPORTCLIENT_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.ulLength == other.ulLength && self.ulApiVersion == other.ulApiVersion && self.ulAuthLevel == other.ulAuthLevel && self.pwszServer == other.pwszServer && self.pwszNamespace == other.pwszNamespace && self.pwszObjectName == other.pwszObjectName && self.ulCacheSize == other.ulCacheSize && self.ulProtocol == other.ulProtocol && self.pvProtocolData == other.pvProtocolData && self.ulProtocolDataLength == other.ulProtocolDataLength
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WDS_TRANSPORTCLIENT_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WDS_TRANSPORTCLIENT_REQUEST {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL(pub u32);
pub const WDS_TRANSPORTCLIENT_AUTH: WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL = WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL(1u32);
pub const WDS_TRANSPORTCLIENT_NO_AUTH: WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL = WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL(2u32);
impl ::std::convert::From<u32> for WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const WDS_TRANSPORTCLIENT_STATUS_FAILURE: u32 = 3u32;
pub const WDS_TRANSPORTCLIENT_STATUS_IN_PROGRESS: u32 = 1u32;
pub const WDS_TRANSPORTCLIENT_STATUS_SUCCESS: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct WDS_TRANSPORTPROVIDER_INIT_PARAMS {
    pub ulLength: u32,
    pub ulMcServerVersion: u32,
    pub hRegistryKey: super::Registry::HKEY,
    pub hProvider: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl WDS_TRANSPORTPROVIDER_INIT_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::std::default::Default for WDS_TRANSPORTPROVIDER_INIT_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::std::fmt::Debug for WDS_TRANSPORTPROVIDER_INIT_PARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WDS_TRANSPORTPROVIDER_INIT_PARAMS").field("ulLength", &self.ulLength).field("ulMcServerVersion", &self.ulMcServerVersion).field("hRegistryKey", &self.hRegistryKey).field("hProvider", &self.hProvider).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::std::cmp::PartialEq for WDS_TRANSPORTPROVIDER_INIT_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.ulLength == other.ulLength && self.ulMcServerVersion == other.ulMcServerVersion && self.hRegistryKey == other.hRegistryKey && self.hProvider == other.hProvider
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::std::cmp::Eq for WDS_TRANSPORTPROVIDER_INIT_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
unsafe impl ::windows::runtime::Abi for WDS_TRANSPORTPROVIDER_INIT_PARAMS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WDS_TRANSPORTPROVIDER_SETTINGS {
    pub ulLength: u32,
    pub ulProviderVersion: u32,
}
impl WDS_TRANSPORTPROVIDER_SETTINGS {}
impl ::std::default::Default for WDS_TRANSPORTPROVIDER_SETTINGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WDS_TRANSPORTPROVIDER_SETTINGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WDS_TRANSPORTPROVIDER_SETTINGS").field("ulLength", &self.ulLength).field("ulProviderVersion", &self.ulProviderVersion).finish()
    }
}
impl ::std::cmp::PartialEq for WDS_TRANSPORTPROVIDER_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.ulLength == other.ulLength && self.ulProviderVersion == other.ulProviderVersion
    }
}
impl ::std::cmp::Eq for WDS_TRANSPORTPROVIDER_SETTINGS {}
unsafe impl ::windows::runtime::Abi for WDS_TRANSPORTPROVIDER_SETTINGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsBpAddOption<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hhandle: Param0, uoption: u32, uvaluelen: u32, pvalue: *const ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsBpAddOption(hhandle: super::super::Foundation::HANDLE, uoption: u32, uvaluelen: u32, pvalue: *const ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(WdsBpAddOption(hhandle.into_param().abi(), ::std::mem::transmute(uoption), ::std::mem::transmute(uvaluelen), ::std::mem::transmute(pvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsBpCloseHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hhandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsBpCloseHandle(hhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(WdsBpCloseHandle(hhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsBpGetOptionBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hhandle: Param0, ubufferlen: u32, pbuffer: *mut ::std::ffi::c_void, pubytes: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsBpGetOptionBuffer(hhandle: super::super::Foundation::HANDLE, ubufferlen: u32, pbuffer: *mut ::std::ffi::c_void, pubytes: *mut u32) -> u32;
        }
        ::std::mem::transmute(WdsBpGetOptionBuffer(hhandle.into_param().abi(), ::std::mem::transmute(ubufferlen), ::std::mem::transmute(pbuffer), ::std::mem::transmute(pubytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsBpInitialize(bpackettype: u8, phhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsBpInitialize(bpackettype: u8, phhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(WdsBpInitialize(::std::mem::transmute(bpackettype), ::std::mem::transmute(phhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsBpParseInitialize(ppacket: *const ::std::ffi::c_void, upacketlen: u32, pbpackettype: *mut u8, phhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsBpParseInitialize(ppacket: *const ::std::ffi::c_void, upacketlen: u32, pbpackettype: *mut u8, phhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(WdsBpParseInitialize(::std::mem::transmute(ppacket), ::std::mem::transmute(upacketlen), ::std::mem::transmute(pbpackettype), ::std::mem::transmute(phhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsBpParseInitializev6(ppacket: *const ::std::ffi::c_void, upacketlen: u32, pbpackettype: *mut u8, phhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsBpParseInitializev6(ppacket: *const ::std::ffi::c_void, upacketlen: u32, pbpackettype: *mut u8, phhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(WdsBpParseInitializev6(::std::mem::transmute(ppacket), ::std::mem::transmute(upacketlen), ::std::mem::transmute(pbpackettype), ::std::mem::transmute(phhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsBpQueryOption<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hhandle: Param0, uoption: u32, uvaluelen: u32, pvalue: *mut ::std::ffi::c_void, pubytes: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsBpQueryOption(hhandle: super::super::Foundation::HANDLE, uoption: u32, uvaluelen: u32, pvalue: *mut ::std::ffi::c_void, pubytes: *mut u32) -> u32;
        }
        ::std::mem::transmute(WdsBpQueryOption(hhandle.into_param().abi(), ::std::mem::transmute(uoption), ::std::mem::transmute(uvaluelen), ::std::mem::transmute(pvalue), ::std::mem::transmute(pubytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliAuthorizeSession<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hsession: Param0, pcred: *const WDS_CLI_CRED) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliAuthorizeSession(hsession: super::super::Foundation::HANDLE, pcred: *const WDS_CLI_CRED) -> ::windows::runtime::HRESULT;
        }
        WdsCliAuthorizeSession(hsession.into_param().abi(), ::std::mem::transmute(pcred)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliCancelTransfer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(htransfer: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliCancelTransfer(htransfer: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        WdsCliCancelTransfer(htransfer.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliClose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(handle: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliClose(handle: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        WdsCliClose(handle.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliCreateSession<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszserver: Param0, pcred: *const WDS_CLI_CRED) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliCreateSession(pwszserver: super::super::Foundation::PWSTR, pcred: *const WDS_CLI_CRED, phsession: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliCreateSession(pwszserver.into_param().abi(), ::std::mem::transmute(pcred), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliFindFirstImage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hsession: Param0) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliFindFirstImage(hsession: super::super::Foundation::HANDLE, phfindhandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliFindFirstImage(hsession.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliFindNextImage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(handle: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliFindNextImage(handle: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        WdsCliFindNextImage(handle.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WdsCliFlagEnumFilterFirmware: i32 = 2i32;
pub const WdsCliFlagEnumFilterVersion: i32 = 1i32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliFreeStringArray(ppwszarray: *mut super::super::Foundation::PWSTR, ulcount: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliFreeStringArray(ppwszarray: *mut super::super::Foundation::PWSTR, ulcount: u32) -> ::windows::runtime::HRESULT;
        }
        WdsCliFreeStringArray(::std::mem::transmute(ppwszarray), ::std::mem::transmute(ulcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetDriverQueryXml<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszwindirpath: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetDriverQueryXml(pwszwindirpath: super::super::Foundation::PWSTR, ppwszdriverquery: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliGetDriverQueryXml(pwszwindirpath.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetEnumerationFlags<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(handle: Param0) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetEnumerationFlags(handle: super::super::Foundation::HANDLE, pdwflags: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliGetEnumerationFlags(handle.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageArchitecture<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hifh: Param0) -> ::windows::runtime::Result<CPU_ARCHITECTURE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetImageArchitecture(hifh: super::super::Foundation::HANDLE, pdwvalue: *mut CPU_ARCHITECTURE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <CPU_ARCHITECTURE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliGetImageArchitecture(hifh.into_param().abi(), &mut result__).from_abi::<CPU_ARCHITECTURE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hifh: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetImageDescription(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliGetImageDescription(hifh.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageFiles<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hifh: Param0, pppwszfiles: *mut *mut super::super::Foundation::PWSTR, pdwcount: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetImageFiles(hifh: super::super::Foundation::HANDLE, pppwszfiles: *mut *mut super::super::Foundation::PWSTR, pdwcount: *mut u32) -> ::windows::runtime::HRESULT;
        }
        WdsCliGetImageFiles(hifh.into_param().abi(), ::std::mem::transmute(pppwszfiles), ::std::mem::transmute(pdwcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hifh: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetImageGroup(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliGetImageGroup(hifh.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageHalName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hifh: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetImageHalName(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliGetImageHalName(hifh.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageHandleFromFindHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(findhandle: Param0) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetImageHandleFromFindHandle(findhandle: super::super::Foundation::HANDLE, phimagehandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliGetImageHandleFromFindHandle(findhandle.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageHandleFromTransferHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(htransfer: Param0) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetImageHandleFromTransferHandle(htransfer: super::super::Foundation::HANDLE, phimagehandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliGetImageHandleFromTransferHandle(htransfer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageIndex<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hifh: Param0) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetImageIndex(hifh: super::super::Foundation::HANDLE, pdwvalue: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliGetImageIndex(hifh.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hifh: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetImageLanguage(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliGetImageLanguage(hifh.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageLanguages<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hifh: Param0, pppszvalues: *mut *mut *mut i8, pdwnumvalues: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetImageLanguages(hifh: super::super::Foundation::HANDLE, pppszvalues: *mut *mut *mut i8, pdwnumvalues: *mut u32) -> ::windows::runtime::HRESULT;
        }
        WdsCliGetImageLanguages(hifh.into_param().abi(), ::std::mem::transmute(pppszvalues), ::std::mem::transmute(pdwnumvalues)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageLastModifiedTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hifh: Param0) -> ::windows::runtime::Result<*mut super::super::Foundation::SYSTEMTIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetImageLastModifiedTime(hifh: super::super::Foundation::HANDLE, ppsystimevalue: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut super::super::Foundation::SYSTEMTIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliGetImageLastModifiedTime(hifh.into_param().abi(), &mut result__).from_abi::<*mut super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hifh: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetImageName(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliGetImageName(hifh.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hifh: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetImageNamespace(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliGetImageNamespace(hifh.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageParameter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hifh: Param0, paramtype: WDS_CLI_IMAGE_PARAM_TYPE, presponse: *mut ::std::ffi::c_void, uresponselen: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetImageParameter(hifh: super::super::Foundation::HANDLE, paramtype: WDS_CLI_IMAGE_PARAM_TYPE, presponse: *mut ::std::ffi::c_void, uresponselen: u32) -> ::windows::runtime::HRESULT;
        }
        WdsCliGetImageParameter(hifh.into_param().abi(), ::std::mem::transmute(paramtype), ::std::mem::transmute(presponse), ::std::mem::transmute(uresponselen)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImagePath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hifh: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetImagePath(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliGetImagePath(hifh.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hifh: Param0) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetImageSize(hifh: super::super::Foundation::HANDLE, pullvalue: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliGetImageSize(hifh.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageType<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hifh: Param0) -> ::windows::runtime::Result<WDS_CLI_IMAGE_TYPE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetImageType(hifh: super::super::Foundation::HANDLE, pimagetype: *mut WDS_CLI_IMAGE_TYPE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <WDS_CLI_IMAGE_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliGetImageType(hifh.into_param().abi(), &mut result__).from_abi::<WDS_CLI_IMAGE_TYPE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageVersion<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hifh: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetImageVersion(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliGetImageVersion(hifh.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetTransferSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hifh: Param0) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliGetTransferSize(hifh: super::super::Foundation::HANDLE, pullvalue: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliGetTransferSize(hifh.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliInitializeLog<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hsession: Param0, ulclientarchitecture: CPU_ARCHITECTURE, pwszclientid: Param2, pwszclientaddress: Param3) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliInitializeLog(hsession: super::super::Foundation::HANDLE, ulclientarchitecture: CPU_ARCHITECTURE, pwszclientid: super::super::Foundation::PWSTR, pwszclientaddress: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        WdsCliInitializeLog(hsession.into_param().abi(), ::std::mem::transmute(ulclientarchitecture), pwszclientid.into_param().abi(), pwszclientaddress.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliLog<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hsession: Param0, ulloglevel: u32, ulmessagecode: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliLog(hsession: super::super::Foundation::HANDLE, ulloglevel: u32, ulmessagecode: u32) -> ::windows::runtime::HRESULT;
        }
        WdsCliLog(hsession.into_param().abi(), ::std::mem::transmute(ulloglevel), ::std::mem::transmute(ulmessagecode)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliObtainDriverPackages<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(himage: Param0, ppwszservername: *mut super::super::Foundation::PWSTR, pppwszdriverpackages: *mut *mut super::super::Foundation::PWSTR, pulcount: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliObtainDriverPackages(himage: super::super::Foundation::HANDLE, ppwszservername: *mut super::super::Foundation::PWSTR, pppwszdriverpackages: *mut *mut super::super::Foundation::PWSTR, pulcount: *mut u32) -> ::windows::runtime::HRESULT;
        }
        WdsCliObtainDriverPackages(himage.into_param().abi(), ::std::mem::transmute(ppwszservername), ::std::mem::transmute(pppwszdriverpackages), ::std::mem::transmute(pulcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliObtainDriverPackagesEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hsession: Param0, pwszmachineinfo: Param1, ppwszservername: *mut super::super::Foundation::PWSTR, pppwszdriverpackages: *mut *mut super::super::Foundation::PWSTR, pulcount: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliObtainDriverPackagesEx(hsession: super::super::Foundation::HANDLE, pwszmachineinfo: super::super::Foundation::PWSTR, ppwszservername: *mut super::super::Foundation::PWSTR, pppwszdriverpackages: *mut *mut super::super::Foundation::PWSTR, pulcount: *mut u32) -> ::windows::runtime::HRESULT;
        }
        WdsCliObtainDriverPackagesEx(hsession.into_param().abi(), pwszmachineinfo.into_param().abi(), ::std::mem::transmute(ppwszservername), ::std::mem::transmute(pppwszdriverpackages), ::std::mem::transmute(pulcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliRegisterTrace(pfn: ::std::option::Option<PFN_WdsCliTraceFunction>) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliRegisterTrace(pfn: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        WdsCliRegisterTrace(::std::mem::transmute(pfn)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WdsCliSetTransferBufferSize(ulsizeinbytes: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliSetTransferBufferSize(ulsizeinbytes: u32);
        }
        ::std::mem::transmute(WdsCliSetTransferBufferSize(::std::mem::transmute(ulsizeinbytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliTransferFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    pwszserver: Param0,
    pwsznamespace: Param1,
    pwszremotefilepath: Param2,
    pwszlocalfilepath: Param3,
    dwflags: u32,
    dwreserved: u32,
    pfnwdsclicallback: ::std::option::Option<PFN_WdsCliCallback>,
    pvuserdata: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliTransferFile(pwszserver: super::super::Foundation::PWSTR, pwsznamespace: super::super::Foundation::PWSTR, pwszremotefilepath: super::super::Foundation::PWSTR, pwszlocalfilepath: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: u32, pfnwdsclicallback: ::windows::runtime::RawPtr, pvuserdata: *const ::std::ffi::c_void, phtransfer: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliTransferFile(pwszserver.into_param().abi(), pwsznamespace.into_param().abi(), pwszremotefilepath.into_param().abi(), pwszlocalfilepath.into_param().abi(), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwreserved), ::std::mem::transmute(pfnwdsclicallback), ::std::mem::transmute(pvuserdata), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliTransferImage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(himage: Param0, pwszlocalpath: Param1, dwflags: u32, dwreserved: u32, pfnwdsclicallback: ::std::option::Option<PFN_WdsCliCallback>, pvuserdata: *const ::std::ffi::c_void) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliTransferImage(himage: super::super::Foundation::HANDLE, pwszlocalpath: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: u32, pfnwdsclicallback: ::windows::runtime::RawPtr, pvuserdata: *const ::std::ffi::c_void, phtransfer: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WdsCliTransferImage(himage.into_param().abi(), pwszlocalpath.into_param().abi(), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwreserved), ::std::mem::transmute(pfnwdsclicallback), ::std::mem::transmute(pvuserdata), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliWaitForTransfer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(htransfer: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsCliWaitForTransfer(htransfer: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        WdsCliWaitForTransfer(htransfer.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WdsTransportCacheable: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1884883734, 61766, 18109, [189, 157, 74, 170, 144, 8, 75, 245]);
pub const WdsTransportClient: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1725089257, 4086, 18924, [151, 51, 218, 251, 30, 1, 223, 28]);
#[inline]
pub unsafe fn WdsTransportClientAddRefBuffer(pvbuffer: *const ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportClientAddRefBuffer(pvbuffer: *const ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(WdsTransportClientAddRefBuffer(::std::mem::transmute(pvbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportClientCancelSession<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hsessionkey: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportClientCancelSession(hsessionkey: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(WdsTransportClientCancelSession(hsessionkey.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportClientCancelSessionEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hsessionkey: Param0, dwerrorcode: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportClientCancelSessionEx(hsessionkey: super::super::Foundation::HANDLE, dwerrorcode: u32) -> u32;
        }
        ::std::mem::transmute(WdsTransportClientCancelSessionEx(hsessionkey.into_param().abi(), ::std::mem::transmute(dwerrorcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportClientCloseSession<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hsessionkey: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportClientCloseSession(hsessionkey: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(WdsTransportClientCloseSession(hsessionkey.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportClientCompleteReceive<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hsessionkey: Param0, ulsize: u32, pulloffset: *const u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportClientCompleteReceive(hsessionkey: super::super::Foundation::HANDLE, ulsize: u32, pulloffset: *const u64) -> u32;
        }
        ::std::mem::transmute(WdsTransportClientCompleteReceive(hsessionkey.into_param().abi(), ::std::mem::transmute(ulsize), ::std::mem::transmute(pulloffset)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WdsTransportClientInitialize() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportClientInitialize() -> u32;
        }
        ::std::mem::transmute(WdsTransportClientInitialize())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportClientInitializeSession(psessionrequest: *const WDS_TRANSPORTCLIENT_REQUEST, pcallerdata: *const ::std::ffi::c_void, hsessionkey: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportClientInitializeSession(psessionrequest: *const WDS_TRANSPORTCLIENT_REQUEST, pcallerdata: *const ::std::ffi::c_void, hsessionkey: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(WdsTransportClientInitializeSession(::std::mem::transmute(psessionrequest), ::std::mem::transmute(pcallerdata), ::std::mem::transmute(hsessionkey)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportClientQueryStatus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hsessionkey: Param0, pustatus: *mut u32, puerrorcode: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportClientQueryStatus(hsessionkey: super::super::Foundation::HANDLE, pustatus: *mut u32, puerrorcode: *mut u32) -> u32;
        }
        ::std::mem::transmute(WdsTransportClientQueryStatus(hsessionkey.into_param().abi(), ::std::mem::transmute(pustatus), ::std::mem::transmute(puerrorcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportClientRegisterCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hsessionkey: Param0, callbackid: TRANSPORTCLIENT_CALLBACK_ID, pfncallback: *const ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportClientRegisterCallback(hsessionkey: super::super::Foundation::HANDLE, callbackid: TRANSPORTCLIENT_CALLBACK_ID, pfncallback: *const ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(WdsTransportClientRegisterCallback(hsessionkey.into_param().abi(), ::std::mem::transmute(callbackid), ::std::mem::transmute(pfncallback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WdsTransportClientReleaseBuffer(pvbuffer: *const ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportClientReleaseBuffer(pvbuffer: *const ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(WdsTransportClientReleaseBuffer(::std::mem::transmute(pvbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WdsTransportClientShutdown() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportClientShutdown() -> u32;
        }
        ::std::mem::transmute(WdsTransportClientShutdown())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportClientStartSession<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hsessionkey: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportClientStartSession(hsessionkey: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(WdsTransportClientStartSession(hsessionkey.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportClientWaitForCompletion<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hsessionkey: Param0, utimeout: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportClientWaitForCompletion(hsessionkey: super::super::Foundation::HANDLE, utimeout: u32) -> u32;
        }
        ::std::mem::transmute(WdsTransportClientWaitForCompletion(hsessionkey.into_param().abi(), ::std::mem::transmute(utimeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WdsTransportCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3354495753, 14622, 17262, [177, 11, 195, 239, 70, 242, 195, 79]);
pub const WdsTransportConfigurationManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2269378164, 36940, 18378, [133, 18, 53, 254, 152, 246, 176, 172]);
pub const WdsTransportContent: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(176758759, 19007, 19557, [182, 242, 20, 103, 97, 150, 121, 234]);
pub const WdsTransportContentProvider: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3770577951, 23157, 20153, [138, 45, 94, 24, 155, 69, 243, 39]);
pub const WdsTransportDiagnosticsPolicy: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3946001377, 42925, 18165, [128, 214, 107, 116, 2, 4, 229, 9]);
pub const WdsTransportManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4061471734, 33660, 19032, [175, 153, 138, 126, 39, 248, 255, 89]);
pub const WdsTransportMulticastSessionPolicy: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1013695476, 25624, 18218, [182, 241, 82, 212, 87, 25, 84, 55]);
pub const WdsTransportNamespace: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3627571048, 1842, 20161, [149, 234, 22, 218, 88, 25, 8, 161]);
pub const WdsTransportNamespaceAutoCast: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2962355624, 27289, 18317, [178, 59, 9, 232, 254, 224, 69, 116]);
pub const WdsTransportNamespaceManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4035763043, 34270, 18984, [161, 169, 92, 163, 231, 239, 218, 115]);
pub const WdsTransportNamespaceScheduledCast: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3134986391, 28709, 17643, [145, 8, 251, 97, 196, 5, 87, 146]);
pub const WdsTransportNamespaceScheduledCastAutoStart: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2702209106, 4652, 19329, [155, 124, 56, 110, 104, 85, 56, 63]);
pub const WdsTransportNamespaceScheduledCastManualStart: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3554779818, 51884, 17934, [185, 138, 71, 249, 243, 24, 161, 250]);
pub const WdsTransportServer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3927553603, 19167, 17427, [148, 44, 20, 243, 121, 17, 135, 96]);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportServerAllocateBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprovider: Param0, ulbuffersize: u32) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportServerAllocateBuffer(hprovider: super::super::Foundation::HANDLE, ulbuffersize: u32) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(WdsTransportServerAllocateBuffer(hprovider.into_param().abi(), ::std::mem::transmute(ulbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportServerCompleteRead<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprovider: Param0, ulbytesread: u32, pvuserdata: *const ::std::ffi::c_void, hreadresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportServerCompleteRead(hprovider: super::super::Foundation::HANDLE, ulbytesread: u32, pvuserdata: *const ::std::ffi::c_void, hreadresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT;
        }
        WdsTransportServerCompleteRead(hprovider.into_param().abi(), ::std::mem::transmute(ulbytesread), ::std::mem::transmute(pvuserdata), ::std::mem::transmute(hreadresult)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportServerFreeBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprovider: Param0, pvbuffer: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportServerFreeBuffer(hprovider: super::super::Foundation::HANDLE, pvbuffer: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        WdsTransportServerFreeBuffer(hprovider.into_param().abi(), ::std::mem::transmute(pvbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportServerRegisterCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprovider: Param0, callbackid: TRANSPORTPROVIDER_CALLBACK_ID, pfncallback: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportServerRegisterCallback(hprovider: super::super::Foundation::HANDLE, callbackid: TRANSPORTPROVIDER_CALLBACK_ID, pfncallback: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        WdsTransportServerRegisterCallback(hprovider.into_param().abi(), ::std::mem::transmute(callbackid), ::std::mem::transmute(pfncallback)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportServerTrace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hprovider: Param0, severity: u32, pwszformat: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportServerTrace(hprovider: super::super::Foundation::HANDLE, severity: u32, pwszformat: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        WdsTransportServerTrace(hprovider.into_param().abi(), ::std::mem::transmute(severity), pwszformat.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportServerTraceV<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hprovider: Param0, severity: u32, pwszformat: Param2, params: *const i8) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WdsTransportServerTraceV(hprovider: super::super::Foundation::HANDLE, severity: u32, pwszformat: super::super::Foundation::PWSTR, params: *const i8) -> ::windows::runtime::HRESULT;
        }
        WdsTransportServerTraceV(hprovider.into_param().abi(), ::std::mem::transmute(severity), pwszformat.into_param().abi(), ::std::mem::transmute(params)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WdsTransportServicePolicy: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1705831132, 12043, 20291, [159, 77, 129, 24, 101, 216, 206, 173]);
pub const WdsTransportSession: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1956300000, 26556, 18243, [191, 229, 202, 203, 31, 38, 245, 127]);
pub const WdsTransportSetupManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3351177901, 40708, 18723, [159, 12, 251, 245, 43, 199, 89, 15]);
pub const WdsTransportTftpClient: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1345599781, 31836, 19596, [150, 196, 173, 159, 165, 0, 95, 186]);
pub const WdsTransportTftpManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3370769570, 12865, 20045, [184, 6, 188, 116, 1, 157, 254, 218]);
