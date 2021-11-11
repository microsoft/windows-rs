#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const ID_DOCUMENTPACKAGETARGET_MSXPS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cae40a8_ded1_41c9_a9fd_d735ef33aeda);
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0056bb72_8c9c_4612_bd0f_93012a87099d);
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS_WITH_3D: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63dbd720_8b14_4577_b074_7bb11b596d28);
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrintDocumentPackageStatusEvent(pub ::windows::core::IUnknown);
impl IPrintDocumentPackageStatusEvent {
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn PackageStatusUpdated(&self, packagestatus: *const PrintDocumentPackageStatus) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(packagestatus)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPrintDocumentPackageStatusEvent {
    type Vtable = IPrintDocumentPackageStatusEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed90c8ad_5c34_4d05_a1ec_0e8a9b3ad7af);
}
impl ::core::convert::From<IPrintDocumentPackageStatusEvent> for ::windows::core::IUnknown {
    fn from(value: IPrintDocumentPackageStatusEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintDocumentPackageStatusEvent> for ::windows::core::IUnknown {
    fn from(value: &IPrintDocumentPackageStatusEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintDocumentPackageStatusEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintDocumentPackageStatusEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPrintDocumentPackageStatusEvent> for super::super::super::System::Com::IDispatch {
    fn from(value: IPrintDocumentPackageStatusEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPrintDocumentPackageStatusEvent> for super::super::super::System::Com::IDispatch {
    fn from(value: &IPrintDocumentPackageStatusEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for IPrintDocumentPackageStatusEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &IPrintDocumentPackageStatusEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPackageStatusEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagestatus: *const PrintDocumentPackageStatus) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrintDocumentPackageTarget(pub ::windows::core::IUnknown);
impl IPrintDocumentPackageTarget {
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn GetPackageTargetTypes(&self, targetcount: *mut u32, targettypes: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(targetcount), ::core::mem::transmute(targettypes)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn GetPackageTarget<T: ::windows::core::Interface>(&self, guidtargettype: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtargettype), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPrintDocumentPackageTarget {
    type Vtable = IPrintDocumentPackageTarget_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b8efec4_3019_4c27_964e_367202156906);
}
impl ::core::convert::From<IPrintDocumentPackageTarget> for ::windows::core::IUnknown {
    fn from(value: IPrintDocumentPackageTarget) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintDocumentPackageTarget> for ::windows::core::IUnknown {
    fn from(value: &IPrintDocumentPackageTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintDocumentPackageTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintDocumentPackageTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPackageTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, targetcount: *mut u32, targettypes: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guidtargettype: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrintDocumentPackageTargetFactory(pub ::windows::core::IUnknown);
impl IPrintDocumentPackageTargetFactory {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreateDocumentPackageTargetForPrintJob<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param3: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(
        &self,
        printername: Param0,
        jobname: Param1,
        joboutputstream: Param2,
        jobprintticketstream: Param3,
    ) -> ::windows::core::Result<IPrintDocumentPackageTarget> {
        let mut result__: <IPrintDocumentPackageTarget as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), printername.into_param().abi(), jobname.into_param().abi(), joboutputstream.into_param().abi(), jobprintticketstream.into_param().abi(), &mut result__).from_abi::<IPrintDocumentPackageTarget>(result__)
    }
}
unsafe impl ::windows::core::Interface for IPrintDocumentPackageTargetFactory {
    type Vtable = IPrintDocumentPackageTargetFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2959bf7_b31b_4a3d_9600_712eb1335ba4);
}
impl ::core::convert::From<IPrintDocumentPackageTargetFactory> for ::windows::core::IUnknown {
    fn from(value: IPrintDocumentPackageTargetFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintDocumentPackageTargetFactory> for ::windows::core::IUnknown {
    fn from(value: &IPrintDocumentPackageTargetFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintDocumentPackageTargetFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintDocumentPackageTargetFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPackageTargetFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, printername: super::super::super::Foundation::PWSTR, jobname: super::super::super::Foundation::PWSTR, joboutputstream: ::windows::core::RawPtr, jobprintticketstream: ::windows::core::RawPtr, docpackagetarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsPrintJob(pub ::windows::core::IUnknown);
impl IXpsPrintJob {
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn GetJobStatus(&self) -> ::windows::core::Result<XPS_JOB_STATUS> {
        let mut result__: <XPS_JOB_STATUS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<XPS_JOB_STATUS>(result__)
    }
}
unsafe impl ::windows::core::Interface for IXpsPrintJob {
    type Vtable = IXpsPrintJob_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ab89b06_8194_425f_ab3b_d7a96e350161);
}
impl ::core::convert::From<IXpsPrintJob> for ::windows::core::IUnknown {
    fn from(value: IXpsPrintJob) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsPrintJob> for ::windows::core::IUnknown {
    fn from(value: &IXpsPrintJob) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsPrintJob_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, jobstatus: *mut XPS_JOB_STATUS) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXpsPrintJobStream(pub ::windows::core::IUnknown);
impl IXpsPrintJobStream {
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IXpsPrintJobStream {
    type Vtable = IXpsPrintJobStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a77dc5f_45d6_4dff_9307_d8cb846347ca);
}
impl ::core::convert::From<IXpsPrintJobStream> for ::windows::core::IUnknown {
    fn from(value: IXpsPrintJobStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXpsPrintJobStream> for ::windows::core::IUnknown {
    fn from(value: &IXpsPrintJobStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXpsPrintJobStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXpsPrintJobStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::ISequentialStream> for IXpsPrintJobStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::ISequentialStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::ISequentialStream> for &IXpsPrintJobStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::ISequentialStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsPrintJobStream_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Abi for PrintDocumentPackageCompletion {
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
    pub PackageStatus: ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Abi for PrintDocumentPackageStatus {
    type Abi = Self;
}
pub const PrintDocumentPackageTarget: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4842669e_9947_46ea_8ba2_d8cce432c2ca);
pub const PrintDocumentPackageTargetFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x348ef17d_6c81_4982_92b4_ee188a43867a);
#[doc = "*Required features: `Win32_Storage_Xps_Printing`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartXpsPrintJob<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(
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
) -> ::windows::core::Result<()> {
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
                xpsprintjob: *mut ::windows::core::RawPtr,
                documentstream: *mut ::windows::core::RawPtr,
                printticketstream: *mut ::windows::core::RawPtr,
            ) -> ::windows::core::HRESULT;
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
pub unsafe fn StartXpsPrintJob1<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(
    printername: Param0,
    jobname: Param1,
    outputfilename: Param2,
    progressevent: Param3,
    completionevent: Param4,
    xpsprintjob: *mut ::core::option::Option<IXpsPrintJob>,
    printcontentreceiver: *mut ::core::option::Option<super::IXpsOMPackageTarget>,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartXpsPrintJob1(printername: super::super::super::Foundation::PWSTR, jobname: super::super::super::Foundation::PWSTR, outputfilename: super::super::super::Foundation::PWSTR, progressevent: super::super::super::Foundation::HANDLE, completionevent: super::super::super::Foundation::HANDLE, xpsprintjob: *mut ::windows::core::RawPtr, printcontentreceiver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
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
unsafe impl ::windows::core::Abi for XPS_JOB_COMPLETION {
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
    pub jobStatus: ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Abi for XPS_JOB_STATUS {
    type Abi = Self;
}
