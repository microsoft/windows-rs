#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn StartXpsPrintJob<P0, P1, P2, P3, P4>(printername: P0, jobname: P1, outputfilename: P2, progressevent: P3, completionevent: P4, printablepageson: &[u8], xpsprintjob: *mut ::core::option::Option<IXpsPrintJob>, documentstream: *mut ::core::option::Option<IXpsPrintJobStream>, printticketstream: *mut ::core::option::Option<IXpsPrintJobStream>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<super::super::super::Foundation::HANDLE>,
    P4: ::windows_core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("xpsprint.dll" "system" fn StartXpsPrintJob(printername : ::windows_core::PCWSTR, jobname : ::windows_core::PCWSTR, outputfilename : ::windows_core::PCWSTR, progressevent : super::super::super::Foundation:: HANDLE, completionevent : super::super::super::Foundation:: HANDLE, printablepageson : *const u8, printablepagesoncount : u32, xpsprintjob : *mut * mut::core::ffi::c_void, documentstream : *mut * mut::core::ffi::c_void, printticketstream : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    StartXpsPrintJob(printername.into_param().abi(), jobname.into_param().abi(), outputfilename.into_param().abi(), progressevent.into_param().abi(), completionevent.into_param().abi(), ::core::mem::transmute(printablepageson.as_ptr()), printablepageson.len() as _, ::core::mem::transmute(xpsprintjob), ::core::mem::transmute(documentstream), ::core::mem::transmute(printticketstream)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartXpsPrintJob1<P0, P1, P2, P3, P4>(printername: P0, jobname: P1, outputfilename: P2, progressevent: P3, completionevent: P4, xpsprintjob: *mut ::core::option::Option<IXpsPrintJob>, printcontentreceiver: *mut ::core::option::Option<super::IXpsOMPackageTarget>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<super::super::super::Foundation::HANDLE>,
    P4: ::windows_core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("xpsprint.dll" "system" fn StartXpsPrintJob1(printername : ::windows_core::PCWSTR, jobname : ::windows_core::PCWSTR, outputfilename : ::windows_core::PCWSTR, progressevent : super::super::super::Foundation:: HANDLE, completionevent : super::super::super::Foundation:: HANDLE, xpsprintjob : *mut * mut::core::ffi::c_void, printcontentreceiver : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    StartXpsPrintJob1(printername.into_param().abi(), jobname.into_param().abi(), outputfilename.into_param().abi(), progressevent.into_param().abi(), completionevent.into_param().abi(), ::core::mem::transmute(xpsprintjob), ::core::mem::transmute(printcontentreceiver)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrintDocumentPackageStatusEvent(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrintDocumentPackageStatusEvent {
    pub unsafe fn PackageStatusUpdated(&self, packagestatus: *const PrintDocumentPackageStatus) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PackageStatusUpdated)(::windows_core::Interface::as_raw(self), packagestatus).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IPrintDocumentPackageStatusEvent, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrintDocumentPackageStatusEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrintDocumentPackageStatusEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrintDocumentPackageStatusEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintDocumentPackageStatusEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IPrintDocumentPackageStatusEvent {
    type Vtable = IPrintDocumentPackageStatusEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrintDocumentPackageStatusEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IPrintDocumentPackageStatusEvent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed90c8ad_5c34_4d05_a1ec_0e8a9b3ad7af);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPackageStatusEvent_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub PackageStatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagestatus: *const PrintDocumentPackageStatus) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintDocumentPackageTarget(::windows_core::IUnknown);
impl IPrintDocumentPackageTarget {
    pub unsafe fn GetPackageTargetTypes(&self, targetcount: *mut u32, targettypes: *mut *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPackageTargetTypes)(::windows_core::Interface::as_raw(self), targetcount, targettypes).ok()
    }
    pub unsafe fn GetPackageTarget<T>(&self, guidtargettype: *const ::windows_core::GUID) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetPackageTarget)(::windows_core::Interface::as_raw(self), guidtargettype, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPrintDocumentPackageTarget, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPrintDocumentPackageTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintDocumentPackageTarget {}
impl ::core::fmt::Debug for IPrintDocumentPackageTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintDocumentPackageTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPrintDocumentPackageTarget {
    type Vtable = IPrintDocumentPackageTarget_Vtbl;
}
impl ::core::clone::Clone for IPrintDocumentPackageTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintDocumentPackageTarget {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b8efec4_3019_4c27_964e_367202156906);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPackageTarget_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetPackageTargetTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetcount: *mut u32, targettypes: *mut *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetPackageTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtargettype: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintDocumentPackageTarget2(::windows_core::IUnknown);
impl IPrintDocumentPackageTarget2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsTargetIppPrinter(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIsTargetIppPrinter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTargetIppPrintDevice<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetTargetIppPrintDevice)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPrintDocumentPackageTarget2, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPrintDocumentPackageTarget2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintDocumentPackageTarget2 {}
impl ::core::fmt::Debug for IPrintDocumentPackageTarget2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintDocumentPackageTarget2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPrintDocumentPackageTarget2 {
    type Vtable = IPrintDocumentPackageTarget2_Vtbl;
}
impl ::core::clone::Clone for IPrintDocumentPackageTarget2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintDocumentPackageTarget2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc560298a_535c_48f9_866a_632540660cb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPackageTarget2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsTargetIppPrinter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isippprinter: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsTargetIppPrinter: usize,
    pub GetTargetIppPrintDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintDocumentPackageTargetFactory(::windows_core::IUnknown);
impl IPrintDocumentPackageTargetFactory {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDocumentPackageTargetForPrintJob<P0, P1, P2, P3>(&self, printername: P0, jobname: P1, joboutputstream: P2, jobprintticketstream: P3) -> ::windows_core::Result<IPrintDocumentPackageTarget>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
        P3: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateDocumentPackageTargetForPrintJob)(::windows_core::Interface::as_raw(self), printername.into_param().abi(), jobname.into_param().abi(), joboutputstream.into_param().abi(), jobprintticketstream.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPrintDocumentPackageTargetFactory, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPrintDocumentPackageTargetFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintDocumentPackageTargetFactory {}
impl ::core::fmt::Debug for IPrintDocumentPackageTargetFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintDocumentPackageTargetFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPrintDocumentPackageTargetFactory {
    type Vtable = IPrintDocumentPackageTargetFactory_Vtbl;
}
impl ::core::clone::Clone for IPrintDocumentPackageTargetFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintDocumentPackageTargetFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2959bf7_b31b_4a3d_9600_712eb1335ba4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPackageTargetFactory_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDocumentPackageTargetForPrintJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printername: ::windows_core::PCWSTR, jobname: ::windows_core::PCWSTR, joboutputstream: *mut ::core::ffi::c_void, jobprintticketstream: *mut ::core::ffi::c_void, docpackagetarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDocumentPackageTargetForPrintJob: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
#[repr(transparent)]
pub struct IXpsPrintJob(::windows_core::IUnknown);
impl IXpsPrintJob {
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetJobStatus(&self, jobstatus: *mut XPS_JOB_STATUS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetJobStatus)(::windows_core::Interface::as_raw(self), jobstatus).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsPrintJob, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IXpsPrintJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsPrintJob {}
impl ::core::fmt::Debug for IXpsPrintJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsPrintJob").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IXpsPrintJob {
    type Vtable = IXpsPrintJob_Vtbl;
}
impl ::core::clone::Clone for IXpsPrintJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXpsPrintJob {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ab89b06_8194_425f_ab3b_d7a96e350161);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsPrintJob_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetJobStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobstatus: *mut XPS_JOB_STATUS) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IXpsPrintJobStream(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IXpsPrintJobStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).base__.Read)(::windows_core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).base__.Write)(::windows_core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IXpsPrintJobStream, ::windows_core::IUnknown, super::super::super::System::Com::ISequentialStream);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IXpsPrintJobStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IXpsPrintJobStream {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IXpsPrintJobStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsPrintJobStream").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IXpsPrintJobStream {
    type Vtable = IXpsPrintJobStream_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IXpsPrintJobStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IXpsPrintJobStream {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a77dc5f_45d6_4dff_9307_d8cb846347ca);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IXpsPrintJobStream_Vtbl {
    pub base__: super::super::super::System::Com::ISequentialStream_Vtbl,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const ID_DOCUMENTPACKAGETARGET_MSXPS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9cae40a8_ded1_41c9_a9fd_d735ef33aeda);
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0056bb72_8c9c_4612_bd0f_93012a87099d);
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS_WITH_3D: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63dbd720_8b14_4577_b074_7bb11b596d28);
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const PrintDocumentPackageCompletion_Canceled: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const PrintDocumentPackageCompletion_Completed: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const PrintDocumentPackageCompletion_Failed: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const PrintDocumentPackageCompletion_InProgress: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(0i32);
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const PrintDocumentPackageTarget: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4842669e_9947_46ea_8ba2_d8cce432c2ca);
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const PrintDocumentPackageTargetFactory: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x348ef17d_6c81_4982_92b4_ee188a43867a);
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const XPS_JOB_CANCELLED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const XPS_JOB_COMPLETED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const XPS_JOB_FAILED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const XPS_JOB_IN_PROGRESS: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(0i32);
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintDocumentPackageCompletion(pub i32);
impl ::core::marker::Copy for PrintDocumentPackageCompletion {}
impl ::core::clone::Clone for PrintDocumentPackageCompletion {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintDocumentPackageCompletion {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PrintDocumentPackageCompletion {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PrintDocumentPackageCompletion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintDocumentPackageCompletion").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_JOB_COMPLETION(pub i32);
impl ::core::marker::Copy for XPS_JOB_COMPLETION {}
impl ::core::clone::Clone for XPS_JOB_COMPLETION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_JOB_COMPLETION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_JOB_COMPLETION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_JOB_COMPLETION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_JOB_COMPLETION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub struct PrintDocumentPackageStatus {
    pub JobId: u32,
    pub CurrentDocument: i32,
    pub CurrentPage: i32,
    pub CurrentPageTotal: i32,
    pub Completion: PrintDocumentPackageCompletion,
    pub PackageStatus: ::windows_core::HRESULT,
}
impl ::core::marker::Copy for PrintDocumentPackageStatus {}
impl ::core::clone::Clone for PrintDocumentPackageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PrintDocumentPackageStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PrintDocumentPackageStatus").field("JobId", &self.JobId).field("CurrentDocument", &self.CurrentDocument).field("CurrentPage", &self.CurrentPage).field("CurrentPageTotal", &self.CurrentPageTotal).field("Completion", &self.Completion).field("PackageStatus", &self.PackageStatus).finish()
    }
}
impl ::windows_core::TypeKind for PrintDocumentPackageStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PrintDocumentPackageStatus {
    fn eq(&self, other: &Self) -> bool {
        self.JobId == other.JobId && self.CurrentDocument == other.CurrentDocument && self.CurrentPage == other.CurrentPage && self.CurrentPageTotal == other.CurrentPageTotal && self.Completion == other.Completion && self.PackageStatus == other.PackageStatus
    }
}
impl ::core::cmp::Eq for PrintDocumentPackageStatus {}
impl ::core::default::Default for PrintDocumentPackageStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub struct XPS_JOB_STATUS {
    pub jobId: u32,
    pub currentDocument: i32,
    pub currentPage: i32,
    pub currentPageTotal: i32,
    pub completion: XPS_JOB_COMPLETION,
    pub jobStatus: ::windows_core::HRESULT,
}
impl ::core::marker::Copy for XPS_JOB_STATUS {}
impl ::core::clone::Clone for XPS_JOB_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XPS_JOB_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_JOB_STATUS").field("jobId", &self.jobId).field("currentDocument", &self.currentDocument).field("currentPage", &self.currentPage).field("currentPageTotal", &self.currentPageTotal).field("completion", &self.completion).field("jobStatus", &self.jobStatus).finish()
    }
}
impl ::windows_core::TypeKind for XPS_JOB_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for XPS_JOB_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.jobId == other.jobId && self.currentDocument == other.currentDocument && self.currentPage == other.currentPage && self.currentPageTotal == other.currentPageTotal && self.completion == other.completion && self.jobStatus == other.jobStatus
    }
}
impl ::core::cmp::Eq for XPS_JOB_STATUS {}
impl ::core::default::Default for XPS_JOB_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
