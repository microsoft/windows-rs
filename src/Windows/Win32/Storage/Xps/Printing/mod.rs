#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const ID_DOCUMENTPACKAGETARGET_MSXPS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2628665512, 57041, 16841, [169, 253, 215, 53, 239, 51, 174, 218]);
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(5684082, 35996, 17938, [189, 15, 147, 1, 42, 135, 9, 157]);
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS_WITH_3D: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1675351840, 35604, 17783, [176, 116, 123, 177, 27, 89, 109, 40]);
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPrintDocumentPackageStatusEvent(::windows::runtime::IUnknown);
impl IPrintDocumentPackageStatusEvent {
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn PackageStatusUpdated(&self, packagestatus: *const PrintDocumentPackageStatus) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(packagestatus)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPrintDocumentPackageStatusEvent {
    type Vtable = IPrintDocumentPackageStatusEvent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3985688749, 23604, 19717, [161, 236, 14, 138, 155, 58, 215, 175]);
}
impl ::std::convert::From<IPrintDocumentPackageStatusEvent> for ::windows::runtime::IUnknown {
    fn from(value: IPrintDocumentPackageStatusEvent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPrintDocumentPackageStatusEvent> for ::windows::runtime::IUnknown {
    fn from(value: &IPrintDocumentPackageStatusEvent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrintDocumentPackageStatusEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPrintDocumentPackageStatusEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IPrintDocumentPackageStatusEvent> for super::super::super::System::Ole::Automation::IDispatch {
    fn from(value: IPrintDocumentPackageStatusEvent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IPrintDocumentPackageStatusEvent> for super::super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IPrintDocumentPackageStatusEvent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Ole::Automation::IDispatch> for IPrintDocumentPackageStatusEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Ole::Automation::IDispatch> for &IPrintDocumentPackageStatusEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
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
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagestatus: *const PrintDocumentPackageStatus) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPrintDocumentPackageTarget(::windows::runtime::IUnknown);
impl IPrintDocumentPackageTarget {
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn GetPackageTargetTypes(&self, targetcount: *mut u32, targettypes: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(targetcount), ::std::mem::transmute(targettypes)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn GetPackageTarget<T: ::windows::runtime::Interface>(&self, guidtargettype: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtargettype), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPrintDocumentPackageTarget {
    type Vtable = IPrintDocumentPackageTarget_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(462356164, 12313, 19495, [150, 78, 54, 114, 2, 21, 105, 6]);
}
impl ::std::convert::From<IPrintDocumentPackageTarget> for ::windows::runtime::IUnknown {
    fn from(value: IPrintDocumentPackageTarget) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPrintDocumentPackageTarget> for ::windows::runtime::IUnknown {
    fn from(value: &IPrintDocumentPackageTarget) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrintDocumentPackageTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPrintDocumentPackageTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPackageTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetcount: *mut u32, targettypes: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtargettype: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppvtarget: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPrintDocumentPackageTargetFactory(::windows::runtime::IUnknown);
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
        let mut result__: <IPrintDocumentPackageTarget as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), printername.into_param().abi(), jobname.into_param().abi(), joboutputstream.into_param().abi(), jobprintticketstream.into_param().abi(), &mut result__).from_abi::<IPrintDocumentPackageTarget>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPrintDocumentPackageTargetFactory {
    type Vtable = IPrintDocumentPackageTargetFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3533020151, 45851, 19005, [150, 0, 113, 46, 177, 51, 91, 164]);
}
impl ::std::convert::From<IPrintDocumentPackageTargetFactory> for ::windows::runtime::IUnknown {
    fn from(value: IPrintDocumentPackageTargetFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPrintDocumentPackageTargetFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IPrintDocumentPackageTargetFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrintDocumentPackageTargetFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPrintDocumentPackageTargetFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IXpsPrintJob(::windows::runtime::IUnknown);
impl IXpsPrintJob {
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn GetJobStatus(&self) -> ::windows::runtime::Result<XPS_JOB_STATUS> {
        let mut result__: <XPS_JOB_STATUS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<XPS_JOB_STATUS>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXpsPrintJob {
    type Vtable = IXpsPrintJob_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1522047750, 33172, 16991, [171, 59, 215, 169, 110, 53, 1, 97]);
}
impl ::std::convert::From<IXpsPrintJob> for ::windows::runtime::IUnknown {
    fn from(value: IXpsPrintJob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXpsPrintJob> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsPrintJob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXpsPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IXpsPrintJobStream(::windows::runtime::IUnknown);
impl IXpsPrintJobStream {
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn Read(&self, pv: *mut ::std::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pv), ::std::mem::transmute(cb), ::std::mem::transmute(pcbread)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn Write(&self, pv: *const ::std::ffi::c_void, cb: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pv), ::std::mem::transmute(cb), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXpsPrintJobStream {
    type Vtable = IXpsPrintJobStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2054675551, 17878, 19967, [147, 7, 216, 203, 132, 99, 71, 202]);
}
impl ::std::convert::From<IXpsPrintJobStream> for ::windows::runtime::IUnknown {
    fn from(value: IXpsPrintJobStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXpsPrintJobStream> for ::windows::runtime::IUnknown {
    fn from(value: &IXpsPrintJobStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXpsPrintJobStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXpsPrintJobStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<IXpsPrintJobStream> for super::super::super::System::Com::ISequentialStream {
    fn from(value: IXpsPrintJobStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<&IXpsPrintJobStream> for super::super::super::System::Com::ISequentialStream {
    fn from(value: &IXpsPrintJobStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::ISequentialStream> for IXpsPrintJobStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::ISequentialStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Com::ISequentialStream>::into(self))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::ISequentialStream> for &IXpsPrintJobStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::ISequentialStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Com::ISequentialStream>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsPrintJobStream_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *mut ::std::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::std::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PrintDocumentPackageCompletion(pub i32);
pub const PrintDocumentPackageCompletion_InProgress: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(0i32);
pub const PrintDocumentPackageCompletion_Completed: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(1i32);
pub const PrintDocumentPackageCompletion_Canceled: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(2i32);
pub const PrintDocumentPackageCompletion_Failed: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(3i32);
impl ::std::convert::From<i32> for PrintDocumentPackageCompletion {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PrintDocumentPackageCompletion {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for PrintDocumentPackageStatus {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PrintDocumentPackageStatus {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for PrintDocumentPackageStatus {
    fn eq(&self, other: &Self) -> bool {
        self.JobId == other.JobId && self.CurrentDocument == other.CurrentDocument && self.CurrentPage == other.CurrentPage && self.CurrentPageTotal == other.CurrentPageTotal && self.Completion == other.Completion && self.PackageStatus == other.PackageStatus
    }
}
impl ::std::cmp::Eq for PrintDocumentPackageStatus {}
unsafe impl ::windows::runtime::Abi for PrintDocumentPackageStatus {
    type Abi = Self;
}
pub const PrintDocumentPackageTarget: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1212311198, 39239, 18154, [139, 162, 216, 204, 228, 50, 194, 202]);
pub const PrintDocumentPackageTargetFactory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(881783165, 27777, 18818, [146, 180, 238, 24, 138, 67, 134, 122]);
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Storage_Xps_Printing`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn StartXpsPrintJob<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(
    printername: Param0,
    jobname: Param1,
    outputfilename: Param2,
    progressevent: Param3,
    completionevent: Param4,
    printablepageson: *const u8,
    printablepagesoncount: u32,
    xpsprintjob: *mut ::std::option::Option<IXpsPrintJob>,
    documentstream: *mut ::std::option::Option<IXpsPrintJobStream>,
    printticketstream: *mut ::std::option::Option<IXpsPrintJobStream>,
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
            ::std::mem::transmute(printablepageson),
            ::std::mem::transmute(printablepagesoncount),
            ::std::mem::transmute(xpsprintjob),
            ::std::mem::transmute(documentstream),
            ::std::mem::transmute(printticketstream),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Storage_Xps_Printing`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn StartXpsPrintJob1<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(
    printername: Param0,
    jobname: Param1,
    outputfilename: Param2,
    progressevent: Param3,
    completionevent: Param4,
    xpsprintjob: *mut ::std::option::Option<IXpsPrintJob>,
    printcontentreceiver: *mut ::std::option::Option<super::IXpsOMPackageTarget>,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartXpsPrintJob1(printername: super::super::super::Foundation::PWSTR, jobname: super::super::super::Foundation::PWSTR, outputfilename: super::super::super::Foundation::PWSTR, progressevent: super::super::super::Foundation::HANDLE, completionevent: super::super::super::Foundation::HANDLE, xpsprintjob: *mut ::windows::runtime::RawPtr, printcontentreceiver: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        StartXpsPrintJob1(printername.into_param().abi(), jobname.into_param().abi(), outputfilename.into_param().abi(), progressevent.into_param().abi(), completionevent.into_param().abi(), ::std::mem::transmute(xpsprintjob), ::std::mem::transmute(printcontentreceiver)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XPS_JOB_COMPLETION(pub i32);
pub const XPS_JOB_IN_PROGRESS: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(0i32);
pub const XPS_JOB_COMPLETED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(1i32);
pub const XPS_JOB_CANCELLED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(2i32);
pub const XPS_JOB_FAILED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(3i32);
impl ::std::convert::From<i32> for XPS_JOB_COMPLETION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XPS_JOB_COMPLETION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for XPS_JOB_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for XPS_JOB_STATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("XPS_JOB_STATUS").field("jobId", &self.jobId).field("currentDocument", &self.currentDocument).field("currentPage", &self.currentPage).field("currentPageTotal", &self.currentPageTotal).field("completion", &self.completion).field("jobStatus", &self.jobStatus).finish()
    }
}
impl ::std::cmp::PartialEq for XPS_JOB_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.jobId == other.jobId && self.currentDocument == other.currentDocument && self.currentPage == other.currentPage && self.currentPageTotal == other.currentPageTotal && self.completion == other.completion && self.jobStatus == other.jobStatus
    }
}
impl ::std::cmp::Eq for XPS_JOB_STATUS {}
unsafe impl ::windows::runtime::Abi for XPS_JOB_STATUS {
    type Abi = Self;
}
