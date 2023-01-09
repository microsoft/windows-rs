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
impl IXpsPrintJobStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.Read)(::windows::core::Vtable::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.Write)(::windows::core::Vtable::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
}
impl ::core::default::Default for PrintDocumentPackageCompletion {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintDocumentPackageCompletion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintDocumentPackageCompletion").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintDocumentPackageStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PrintDocumentPackageStatus {
    fn eq(&self, other: &Self) -> bool {
        self.JobId == other.JobId && self.CurrentDocument == other.CurrentDocument && self.CurrentPage == other.CurrentPage && self.CurrentPageTotal == other.CurrentPageTotal && self.Completion == other.Completion && self.PackageStatus == other.PackageStatus
    }
}
impl ::core::cmp::Eq for PrintDocumentPackageStatus {}
impl ::core::fmt::Debug for PrintDocumentPackageStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PrintDocumentPackageStatus").field("JobId", &self.JobId).field("CurrentDocument", &self.CurrentDocument).field("CurrentPage", &self.CurrentPage).field("CurrentPageTotal", &self.CurrentPageTotal).field("Completion", &self.Completion).field("PackageStatus", &self.PackageStatus).finish()
    }
}
impl ::core::default::Default for XPS_JOB_COMPLETION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_JOB_COMPLETION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_JOB_COMPLETION").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_JOB_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XPS_JOB_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.jobId == other.jobId && self.currentDocument == other.currentDocument && self.currentPage == other.currentPage && self.currentPageTotal == other.currentPageTotal && self.completion == other.completion && self.jobStatus == other.jobStatus
    }
}
impl ::core::cmp::Eq for XPS_JOB_STATUS {}
impl ::core::fmt::Debug for XPS_JOB_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_JOB_STATUS").field("jobId", &self.jobId).field("currentDocument", &self.currentDocument).field("currentPage", &self.currentPage).field("currentPageTotal", &self.currentPageTotal).field("completion", &self.completion).field("jobStatus", &self.jobStatus).finish()
    }
}
