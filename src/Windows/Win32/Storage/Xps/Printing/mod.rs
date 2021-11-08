#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const ID_DOCUMENTPACKAGETARGET_MSXPS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2628665512, 57041, 16841, [169, 253, 215, 53, 239, 51, 174, 218]);
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(5684082, 35996, 17938, [189, 15, 147, 1, 42, 135, 9, 157]);
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS_WITH_3D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1675351840, 35604, 17783, [176, 116, 123, 177, 27, 89, 109, 40]);
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrintDocumentPackageStatusEvent(pub ::windows::runtime::IUnknown);
impl IPrintDocumentPackageStatusEvent {
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn PackageStatusUpdated(&self, packagestatus: *const PrintDocumentPackageStatus) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(packagestatus)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPrintDocumentPackageStatusEvent {
    type Vtable = IPrintDocumentPackageStatusEvent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3985688749, 23604, 19717, [161, 236, 14, 138, 155, 58, 215, 175]);
}
impl ::core::convert::From<IPrintDocumentPackageStatusEvent> for ::windows::runtime::IUnknown {
    fn from(value: IPrintDocumentPackageStatusEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintDocumentPackageStatusEvent> for ::windows::runtime::IUnknown {
    fn from(value: &IPrintDocumentPackageStatusEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrintDocumentPackageStatusEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPrintDocumentPackageStatusEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::core::convert::From<IPrintDocumentPackageStatusEvent> for super::super::super::System::Ole::Automation::IDispatch {
    fn from(value: IPrintDocumentPackageStatusEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::core::convert::From<&IPrintDocumentPackageStatusEvent> for super::super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IPrintDocumentPackageStatusEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Ole::Automation::IDispatch> for IPrintDocumentPackageStatusEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Ole::Automation::IDispatch> for &IPrintDocumentPackageStatusEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPackageStatusEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagestatus: *const PrintDocumentPackageStatus) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrintDocumentPackageTarget(pub ::windows::runtime::IUnknown);
impl IPrintDocumentPackageTarget {
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn GetPackageTargetTypes(&self, targetcount: *mut u32, targettypes: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(targetcount), ::core::mem::transmute(targettypes)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn GetPackageTarget<T: ::windows::runtime::Interface>(&self, guidtargettype: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtargettype), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPrintDocumentPackageTarget {
    type Vtable = IPrintDocumentPackageTarget_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(462356164, 12313, 19495, [150, 78, 54, 114, 2, 21, 105, 6]);
}
impl ::core::convert::From<IPrintDocumentPackageTarget> for ::windows::runtime::IUnknown {
    fn from(value: IPrintDocumentPackageTarget) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintDocumentPackageTarget> for ::windows::runtime::IUnknown {
    fn from(value: &IPrintDocumentPackageTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrintDocumentPackageTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPrintDocumentPackageTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPackageTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetcount: *mut u32, targettypes: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtargettype: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrintDocumentPackageTargetFactory(pub ::windows::runtime::IUnknown);
impl IPrintDocumentPackageTargetFactory {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreateDocumentPackageTargetForPrintJob<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(
        &self,
        printername: Param0,
        jobname: Param1,
        joboutputstream: Param2,
        jobprintticketstream: Param3,
    ) -> ::windows::runtime::Result<IPrintDocumentPackageTarget> {
        let mut result__: <IPrintDocumentPackageTarget as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), printername.into_param().abi(), jobname.into_param().abi(), joboutputstream.into_param().abi(), jobprintticketstream.into_param().abi(), &mut result__).from_abi::<IPrintDocumentPackageTarget>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPrintDocumentPackageTargetFactory {
    type Vtable = IPrintDocumentPackageTargetFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3533020151, 45851, 19005, [150, 0, 113, 46, 177, 51, 91, 164]);
}
impl ::core::convert::From<IPrintDocumentPackageTargetFactory> for ::windows::runtime::IUnknown {
    fn from(value: IPrintDocumentPackageTargetFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintDocumentPackageTargetFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IPrintDocumentPackageTargetFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrintDocumentPackageTargetFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPrintDocumentPackageTargetFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPackageTargetFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, printername: super::super::super::Foundation::PWSTR, jobname: super::super::super::Foundation::PWSTR, joboutputstream: ::windows::runtime::RawPtr, jobprintticketstream: ::windows::runtime::RawPtr, docpackagetarget: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsPrintJob(pub ::windows::runtime::IUnknown);
impl IXpsPrintJob {
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn GetJobStatus(&self) -> ::windows::runtime::Result<XPS_JOB_STATUS> {
        let mut result__: <XPS_JOB_STATUS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_JOB_STATUS>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsPrintJob {
    type Vtable = IXpsPrintJob_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1522047750, 33172, 16991, [171, 59, 215, 169, 110, 53, 1, 97]);
}
impl ::core::convert::From<IXpsPrintJob> for ::windows::runtime::IUnknown {
    fn from(value: IXpsPrintJob) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsPrintJob> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsPrintJob) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsPrintJob_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, jobstatus: *mut XPS_JOB_STATUS) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsPrintJobStream(pub ::windows::runtime::IUnknown);
impl IXpsPrintJobStream {
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsPrintJobStream {
    type Vtable = IXpsPrintJobStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2054675551, 17878, 19967, [147, 7, 216, 203, 132, 99, 71, 202]);
}
impl ::core::convert::From<IXpsPrintJobStream> for ::windows::runtime::IUnknown {
    fn from(value: IXpsPrintJobStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsPrintJobStream> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsPrintJobStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsPrintJobStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXpsPrintJobStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IXpsPrintJobStream> for super::super::super::System::Com::ISequentialStream {
    fn from(value: IXpsPrintJobStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IXpsPrintJobStream> for super::super::super::System::Com::ISequentialStream {
    fn from(value: &IXpsPrintJobStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::ISequentialStream> for IXpsPrintJobStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::ISequentialStream> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::ISequentialStream> for &IXpsPrintJobStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::ISequentialStream> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsPrintJobStream_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PrintDocumentPackageCompletion(pub i32);
pub const PrintDocumentPackageCompletion_InProgress: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(0i32);
pub const PrintDocumentPackageCompletion_Completed: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(1i32);
pub const PrintDocumentPackageCompletion_Canceled: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(2i32);
pub const PrintDocumentPackageCompletion_Failed: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(3i32);
impl ::core::convert::From<i32> for PrintDocumentPackageCompletion {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PrintDocumentPackageCompletion {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
pub struct PrintDocumentPackageStatus {
    pub JobId: u32,
    pub CurrentDocument: i32,
    pub CurrentPage: i32,
    pub CurrentPageTotal: i32,
    pub Completion: PrintDocumentPackageCompletion,
    pub PackageStatus: ::windows::runtime::HRESULT,
}
impl PrintDocumentPackageStatus {}
impl ::core::default::Default for PrintDocumentPackageStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PrintDocumentPackageStatus {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PrintDocumentPackageStatus")
            .field("JobId", &self.JobId)
            .field("CurrentDocument", &self.CurrentDocument)
            .field("CurrentPage", &self.CurrentPage)
            .field("CurrentPageTotal", &self.CurrentPageTotal)
            .field("Completion", &self.Completion)
            .field("PackageStatus", &self.PackageStatus)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PrintDocumentPackageStatus {
    fn eq(&self, other: &Self) -> bool {
        self.JobId == other.JobId && self.CurrentDocument == other.CurrentDocument && self.CurrentPage == other.CurrentPage && self.CurrentPageTotal == other.CurrentPageTotal && self.Completion == other.Completion && self.PackageStatus == other.PackageStatus
    }
}
impl ::core::cmp::Eq for PrintDocumentPackageStatus {}
unsafe impl ::windows::runtime::Abi for PrintDocumentPackageStatus {
    type Abi = Self;
}
pub const PrintDocumentPackageTarget: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1212311198, 39239, 18154, [139, 162, 216, 204, 228, 50, 194, 202]);
pub const PrintDocumentPackageTargetFactory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(881783165, 27777, 18818, [146, 180, 238, 24, 138, 67, 134, 122]);
#[doc = "*Required features: `Win32_Storage_Xps_Printing`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartXpsPrintJob<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(
    printername: Param0,
    jobname: Param1,
    outputfilename: Param2,
    progressevent: Param3,
    completionevent: Param4,
    printablepageson: *const u8,
    printablepagesoncount: u32,
    xpsprintjob: *mut ::core::option::Option<IXpsPrintJob>,
    documentstream: *mut ::core::option::Option<IXpsPrintJobStream>,
    printticketstream: *mut ::core::option::Option<IXpsPrintJobStream>,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartXpsPrintJob(
                printername: super::super::super::Foundation::PWSTR,
                jobname: super::super::super::Foundation::PWSTR,
                outputfilename: super::super::super::Foundation::PWSTR,
                progressevent: super::super::super::Foundation::HANDLE,
                completionevent: super::super::super::Foundation::HANDLE,
                printablepageson: *const u8,
                printablepagesoncount: u32,
                xpsprintjob: *mut ::windows::runtime::RawPtr,
                documentstream: *mut ::windows::runtime::RawPtr,
                printticketstream: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        StartXpsPrintJob(
            printername.into_param().abi(),
            jobname.into_param().abi(),
            outputfilename.into_param().abi(),
            progressevent.into_param().abi(),
            completionevent.into_param().abi(),
            ::core::mem::transmute(printablepageson),
            ::core::mem::transmute(printablepagesoncount),
            ::core::mem::transmute(xpsprintjob),
            ::core::mem::transmute(documentstream),
            ::core::mem::transmute(printticketstream),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Xps_Printing`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartXpsPrintJob1<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(
    printername: Param0,
    jobname: Param1,
    outputfilename: Param2,
    progressevent: Param3,
    completionevent: Param4,
    xpsprintjob: *mut ::core::option::Option<IXpsPrintJob>,
    printcontentreceiver: *mut ::core::option::Option<super::IXpsOMPackageTarget>,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartXpsPrintJob1(printername: super::super::super::Foundation::PWSTR, jobname: super::super::super::Foundation::PWSTR, outputfilename: super::super::super::Foundation::PWSTR, progressevent: super::super::super::Foundation::HANDLE, completionevent: super::super::super::Foundation::HANDLE, xpsprintjob: *mut ::windows::runtime::RawPtr, printcontentreceiver: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        StartXpsPrintJob1(printername.into_param().abi(), jobname.into_param().abi(), outputfilename.into_param().abi(), progressevent.into_param().abi(), completionevent.into_param().abi(), ::core::mem::transmute(xpsprintjob), ::core::mem::transmute(printcontentreceiver)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_JOB_COMPLETION(pub i32);
pub const XPS_JOB_IN_PROGRESS: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(0i32);
pub const XPS_JOB_COMPLETED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(1i32);
pub const XPS_JOB_CANCELLED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(2i32);
pub const XPS_JOB_FAILED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(3i32);
impl ::core::convert::From<i32> for XPS_JOB_COMPLETION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_JOB_COMPLETION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
pub struct XPS_JOB_STATUS {
    pub jobId: u32,
    pub currentDocument: i32,
    pub currentPage: i32,
    pub currentPageTotal: i32,
    pub completion: XPS_JOB_COMPLETION,
    pub jobStatus: ::windows::runtime::HRESULT,
}
impl XPS_JOB_STATUS {}
impl ::core::default::Default for XPS_JOB_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XPS_JOB_STATUS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XPS_JOB_STATUS").field("jobId", &self.jobId).field("currentDocument", &self.currentDocument).field("currentPage", &self.currentPage).field("currentPageTotal", &self.currentPageTotal).field("completion", &self.completion).field("jobStatus", &self.jobStatus).finish()
    }
}
impl ::core::cmp::PartialEq for XPS_JOB_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.jobId == other.jobId && self.currentDocument == other.currentDocument && self.currentPage == other.currentPage && self.currentPageTotal == other.currentPageTotal && self.completion == other.completion && self.jobStatus == other.jobStatus
    }
}
impl ::core::cmp::Eq for XPS_JOB_STATUS {}
unsafe impl ::windows::runtime::Abi for XPS_JOB_STATUS {
    type Abi = Self;
}
