pub type DOCMISC = i32;
pub const DOCMISC_CANCREATEMULTIPLEVIEWS: DOCMISC = 1;
pub const DOCMISC_CANTOPENEDIT: DOCMISC = 4;
pub const DOCMISC_NOFILESUPPORT: DOCMISC = 8;
pub const DOCMISC_SUPPORTCOMPLEXRECTANGLES: DOCMISC = 2;
windows_core::imp::define_interface!(IContinueCallback, IContinueCallback_Vtbl, 0xb722bcca_4e68_101b_a2bc_00aa00404770);
windows_core::imp::interface_hierarchy!(IContinueCallback, windows_core::IUnknown);
impl IContinueCallback {
    pub unsafe fn FContinue(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FContinue)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn FContinuePrinting(&self, ncntprinted: i32, ncurpage: i32, pwszprintstatus: *const u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FContinuePrinting)(windows_core::Interface::as_raw(self), ncntprinted, ncurpage, pwszprintstatus) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinueCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FContinue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FContinuePrinting: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *const u16) -> windows_core::HRESULT,
}
pub trait IContinueCallback_Impl: windows_core::IUnknownImpl {
    fn FContinue(&self) -> windows_core::Result<()>;
    fn FContinuePrinting(&self, ncntprinted: i32, ncurpage: i32, pwszprintstatus: *const u16) -> windows_core::Result<()>;
}
impl IContinueCallback_Vtbl {
    pub const fn new<Identity: IContinueCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FContinue<Identity: IContinueCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContinueCallback_Impl::FContinue(this).into()
            }
        }
        unsafe extern "system" fn FContinuePrinting<Identity: IContinueCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncntprinted: i32, ncurpage: i32, pwszprintstatus: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContinueCallback_Impl::FContinuePrinting(this, core::mem::transmute_copy(&ncntprinted), core::mem::transmute_copy(&ncurpage), core::mem::transmute_copy(&pwszprintstatus)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FContinue: FContinue::<Identity, OFFSET>,
            FContinuePrinting: FContinuePrinting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContinueCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContinueCallback {}
windows_core::imp::define_interface!(IEnumOleDocumentViews, IEnumOleDocumentViews_Vtbl, 0xb722bcc8_4e68_101b_a2bc_00aa00404770);
windows_core::imp::interface_hierarchy!(IEnumOleDocumentViews, windows_core::IUnknown);
impl IEnumOleDocumentViews {
    pub unsafe fn Next(&self, cviews: u32, rgpview: *mut Option<IOleDocumentView>, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), cviews, core::mem::transmute(rgpview), pcfetched as _) }
    }
    pub unsafe fn Skip(&self, cviews: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cviews) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOleDocumentViews_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumOleDocumentViews_Impl: windows_core::IUnknownImpl {
    fn Next(&self, cviews: u32, rgpview: windows_core::OutRef<IOleDocumentView>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cviews: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumOleDocumentViews>;
}
impl IEnumOleDocumentViews_Vtbl {
    pub const fn new<Identity: IEnumOleDocumentViews_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumOleDocumentViews_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cviews: u32, rgpview: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOleDocumentViews_Impl::Next(this, core::mem::transmute_copy(&cviews), core::mem::transmute_copy(&rgpview), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumOleDocumentViews_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cviews: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOleDocumentViews_Impl::Skip(this, core::mem::transmute_copy(&cviews)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumOleDocumentViews_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOleDocumentViews_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumOleDocumentViews_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumOleDocumentViews_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumOleDocumentViews as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumOleDocumentViews {}
pub type IGNOREMIME = i32;
pub const IGNOREMIME_PROMPT: IGNOREMIME = 1;
pub const IGNOREMIME_TEXT: IGNOREMIME = 2;
pub const INSTALL_SCOPE_INVALID: u32 = 0;
pub const INSTALL_SCOPE_MACHINE: u32 = 1;
pub const INSTALL_SCOPE_USER: u32 = 2;
windows_core::imp::define_interface!(IOleCommandTarget, IOleCommandTarget_Vtbl, 0xb722bccb_4e68_101b_a2bc_00aa00404770);
windows_core::imp::interface_hierarchy!(IOleCommandTarget, windows_core::IUnknown);
impl IOleCommandTarget {
    pub unsafe fn QueryStatus(&self, pguidcmdgroup: *const windows_core::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryStatus)(windows_core::Interface::as_raw(self), pguidcmdgroup, ccmds, prgcmds as _, pcmdtext as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Exec(&self, pguidcmdgroup: *const windows_core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const super::VARIANT, pvaout: *mut super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Exec)(windows_core::Interface::as_raw(self), pguidcmdgroup, ncmdid, ncmdexecopt, pvain, pvaout) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleCommandTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut OLECMD, *mut OLECMDTEXT) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub Exec: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32, *const super::VARIANT, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    Exec: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IOleCommandTarget_Impl: windows_core::IUnknownImpl {
    fn QueryStatus(&self, pguidcmdgroup: *const windows_core::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> windows_core::Result<()>;
    fn Exec(&self, pguidcmdgroup: *const windows_core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const super::VARIANT, pvaout: *mut super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IOleCommandTarget_Vtbl {
    pub const fn new<Identity: IOleCommandTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryStatus<Identity: IOleCommandTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcmdgroup: *const windows_core::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleCommandTarget_Impl::QueryStatus(this, core::mem::transmute_copy(&pguidcmdgroup), core::mem::transmute_copy(&ccmds), core::mem::transmute_copy(&prgcmds), core::mem::transmute_copy(&pcmdtext)).into()
            }
        }
        unsafe extern "system" fn Exec<Identity: IOleCommandTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcmdgroup: *const windows_core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const super::VARIANT, pvaout: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleCommandTarget_Impl::Exec(this, core::mem::transmute_copy(&pguidcmdgroup), core::mem::transmute_copy(&ncmdid), core::mem::transmute_copy(&ncmdexecopt), core::mem::transmute_copy(&pvain), core::mem::transmute_copy(&pvaout)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryStatus: QueryStatus::<Identity, OFFSET>, Exec: Exec::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleCommandTarget as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IOleCommandTarget {}
windows_core::imp::define_interface!(IOleDocument, IOleDocument_Vtbl, 0xb722bcc5_4e68_101b_a2bc_00aa00404770);
windows_core::imp::interface_hierarchy!(IOleDocument, windows_core::IUnknown);
impl IOleDocument {
    #[cfg(all(feature = "objidlbase", feature = "oleidl"))]
    pub unsafe fn CreateView<P0, P1>(&self, pipsite: P0, pstm: P1, dwreserved: u32) -> windows_core::Result<IOleDocumentView>
    where
        P0: windows_core::Param<super::IOleInPlaceSite>,
        P1: windows_core::Param<super::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateView)(windows_core::Interface::as_raw(self), pipsite.param().abi(), pstm.param().abi(), dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDocMiscStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocMiscStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumViews(&self, ppenum: *mut Option<IEnumOleDocumentViews>, ppview: *mut Option<IOleDocumentView>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumViews)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum), core::mem::transmute(ppview)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleDocument_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "objidlbase", feature = "oleidl"))]
    pub CreateView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "oleidl")))]
    CreateView: usize,
    pub GetDocMiscStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EnumViews: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidlbase", feature = "oleidl"))]
pub trait IOleDocument_Impl: windows_core::IUnknownImpl {
    fn CreateView(&self, pipsite: windows_core::Ref<super::IOleInPlaceSite>, pstm: windows_core::Ref<super::IStream>, dwreserved: u32) -> windows_core::Result<IOleDocumentView>;
    fn GetDocMiscStatus(&self) -> windows_core::Result<u32>;
    fn EnumViews(&self, ppenum: windows_core::OutRef<IEnumOleDocumentViews>, ppview: windows_core::OutRef<IOleDocumentView>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidlbase", feature = "oleidl"))]
impl IOleDocument_Vtbl {
    pub const fn new<Identity: IOleDocument_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateView<Identity: IOleDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipsite: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, dwreserved: u32, ppview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleDocument_Impl::CreateView(this, core::mem::transmute_copy(&pipsite), core::mem::transmute_copy(&pstm), core::mem::transmute_copy(&dwreserved)) {
                    Ok(ok__) => {
                        ppview.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocMiscStatus<Identity: IOleDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleDocument_Impl::GetDocMiscStatus(this) {
                    Ok(ok__) => {
                        pdwstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumViews<Identity: IOleDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void, ppview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleDocument_Impl::EnumViews(this, core::mem::transmute_copy(&ppenum), core::mem::transmute_copy(&ppview)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateView: CreateView::<Identity, OFFSET>,
            GetDocMiscStatus: GetDocMiscStatus::<Identity, OFFSET>,
            EnumViews: EnumViews::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleDocument as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidlbase", feature = "oleidl"))]
impl windows_core::RuntimeName for IOleDocument {}
windows_core::imp::define_interface!(IOleDocumentSite, IOleDocumentSite_Vtbl, 0xb722bcc7_4e68_101b_a2bc_00aa00404770);
windows_core::imp::interface_hierarchy!(IOleDocumentSite, windows_core::IUnknown);
impl IOleDocumentSite {
    pub unsafe fn ActivateMe<P0>(&self, pviewtoactivate: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IOleDocumentView>,
    {
        unsafe { (windows_core::Interface::vtable(self).ActivateMe)(windows_core::Interface::as_raw(self), pviewtoactivate.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleDocumentSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ActivateMe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOleDocumentSite_Impl: windows_core::IUnknownImpl {
    fn ActivateMe(&self, pviewtoactivate: windows_core::Ref<IOleDocumentView>) -> windows_core::Result<()>;
}
impl IOleDocumentSite_Vtbl {
    pub const fn new<Identity: IOleDocumentSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateMe<Identity: IOleDocumentSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pviewtoactivate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleDocumentSite_Impl::ActivateMe(this, core::mem::transmute_copy(&pviewtoactivate)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ActivateMe: ActivateMe::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleDocumentSite as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOleDocumentSite {}
windows_core::imp::define_interface!(IOleDocumentView, IOleDocumentView_Vtbl, 0xb722bcc6_4e68_101b_a2bc_00aa00404770);
windows_core::imp::interface_hierarchy!(IOleDocumentView, windows_core::IUnknown);
impl IOleDocumentView {
    #[cfg(feature = "oleidl")]
    pub unsafe fn SetInPlaceSite<P0>(&self, pipsite: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IOleInPlaceSite>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInPlaceSite)(windows_core::Interface::as_raw(self), pipsite.param().abi()) }
    }
    #[cfg(feature = "oleidl")]
    pub unsafe fn GetInPlaceSite(&self) -> windows_core::Result<super::IOleInPlaceSite> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInPlaceSite)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDocument(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetRect(&self, prcview: *const super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRect)(windows_core::Interface::as_raw(self), prcview) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetRect(&self) -> windows_core::Result<super::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetRectComplex(&self, prcview: *const super::RECT, prchscroll: *const super::RECT, prcvscroll: *const super::RECT, prcsizebox: *const super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRectComplex)(windows_core::Interface::as_raw(self), prcview, prchscroll, prcvscroll, prcsizebox) }
    }
    pub unsafe fn Show(&self, fshow: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), fshow.into()) }
    }
    pub unsafe fn UIActivate(&self, fuiactivate: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UIActivate)(windows_core::Interface::as_raw(self), fuiactivate.into()) }
    }
    pub unsafe fn Open(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CloseView(&self, dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CloseView)(windows_core::Interface::as_raw(self), dwreserved) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn SaveViewState<P0>(&self, pstm: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveViewState)(windows_core::Interface::as_raw(self), pstm.param().abi()) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn ApplyViewState<P0>(&self, pstm: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).ApplyViewState)(windows_core::Interface::as_raw(self), pstm.param().abi()) }
    }
    #[cfg(feature = "oleidl")]
    pub unsafe fn Clone<P0>(&self, pipsitenew: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<super::IOleInPlaceSite>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), pipsitenew.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleDocumentView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oleidl")]
    pub SetInPlaceSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oleidl"))]
    SetInPlaceSite: usize,
    #[cfg(feature = "oleidl")]
    pub GetInPlaceSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oleidl"))]
    GetInPlaceSite: usize,
    pub GetDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetRect: usize,
    #[cfg(feature = "windef")]
    pub GetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetRect: usize,
    #[cfg(feature = "windef")]
    pub SetRectComplex: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::RECT, *const super::RECT, *const super::RECT, *const super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetRectComplex: usize,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub UIActivate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseView: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub SaveViewState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    SaveViewState: usize,
    #[cfg(feature = "objidlbase")]
    pub ApplyViewState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    ApplyViewState: usize,
    #[cfg(feature = "oleidl")]
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oleidl"))]
    Clone: usize,
}
#[cfg(all(feature = "objidlbase", feature = "oleidl", feature = "windef"))]
pub trait IOleDocumentView_Impl: windows_core::IUnknownImpl {
    fn SetInPlaceSite(&self, pipsite: windows_core::Ref<super::IOleInPlaceSite>) -> windows_core::Result<()>;
    fn GetInPlaceSite(&self) -> windows_core::Result<super::IOleInPlaceSite>;
    fn GetDocument(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SetRect(&self, prcview: *const super::RECT) -> windows_core::Result<()>;
    fn GetRect(&self) -> windows_core::Result<super::RECT>;
    fn SetRectComplex(&self, prcview: *const super::RECT, prchscroll: *const super::RECT, prcvscroll: *const super::RECT, prcsizebox: *const super::RECT) -> windows_core::Result<()>;
    fn Show(&self, fshow: windows_core::BOOL) -> windows_core::Result<()>;
    fn UIActivate(&self, fuiactivate: windows_core::BOOL) -> windows_core::Result<()>;
    fn Open(&self) -> windows_core::Result<()>;
    fn CloseView(&self, dwreserved: u32) -> windows_core::Result<()>;
    fn SaveViewState(&self, pstm: windows_core::Ref<super::IStream>) -> windows_core::Result<()>;
    fn ApplyViewState(&self, pstm: windows_core::Ref<super::IStream>) -> windows_core::Result<()>;
    fn Clone(&self, pipsitenew: windows_core::Ref<super::IOleInPlaceSite>) -> windows_core::Result<IOleDocumentView>;
}
#[cfg(all(feature = "objidlbase", feature = "oleidl", feature = "windef"))]
impl IOleDocumentView_Vtbl {
    pub const fn new<Identity: IOleDocumentView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetInPlaceSite<Identity: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipsite: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleDocumentView_Impl::SetInPlaceSite(this, core::mem::transmute_copy(&pipsite)).into()
            }
        }
        unsafe extern "system" fn GetInPlaceSite<Identity: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppipsite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleDocumentView_Impl::GetInPlaceSite(this) {
                    Ok(ok__) => {
                        ppipsite.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocument<Identity: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleDocumentView_Impl::GetDocument(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRect<Identity: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcview: *const super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleDocumentView_Impl::SetRect(this, core::mem::transmute_copy(&prcview)).into()
            }
        }
        unsafe extern "system" fn GetRect<Identity: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcview: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleDocumentView_Impl::GetRect(this) {
                    Ok(ok__) => {
                        prcview.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRectComplex<Identity: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcview: *const super::RECT, prchscroll: *const super::RECT, prcvscroll: *const super::RECT, prcsizebox: *const super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleDocumentView_Impl::SetRectComplex(this, core::mem::transmute_copy(&prcview), core::mem::transmute_copy(&prchscroll), core::mem::transmute_copy(&prcvscroll), core::mem::transmute_copy(&prcsizebox)).into()
            }
        }
        unsafe extern "system" fn Show<Identity: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleDocumentView_Impl::Show(this, core::mem::transmute_copy(&fshow)).into()
            }
        }
        unsafe extern "system" fn UIActivate<Identity: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuiactivate: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleDocumentView_Impl::UIActivate(this, core::mem::transmute_copy(&fuiactivate)).into()
            }
        }
        unsafe extern "system" fn Open<Identity: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleDocumentView_Impl::Open(this).into()
            }
        }
        unsafe extern "system" fn CloseView<Identity: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleDocumentView_Impl::CloseView(this, core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn SaveViewState<Identity: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleDocumentView_Impl::SaveViewState(this, core::mem::transmute_copy(&pstm)).into()
            }
        }
        unsafe extern "system" fn ApplyViewState<Identity: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleDocumentView_Impl::ApplyViewState(this, core::mem::transmute_copy(&pstm)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IOleDocumentView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipsitenew: *mut core::ffi::c_void, ppviewnew: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleDocumentView_Impl::Clone(this, core::mem::transmute_copy(&pipsitenew)) {
                    Ok(ok__) => {
                        ppviewnew.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInPlaceSite: SetInPlaceSite::<Identity, OFFSET>,
            GetInPlaceSite: GetInPlaceSite::<Identity, OFFSET>,
            GetDocument: GetDocument::<Identity, OFFSET>,
            SetRect: SetRect::<Identity, OFFSET>,
            GetRect: GetRect::<Identity, OFFSET>,
            SetRectComplex: SetRectComplex::<Identity, OFFSET>,
            Show: Show::<Identity, OFFSET>,
            UIActivate: UIActivate::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            CloseView: CloseView::<Identity, OFFSET>,
            SaveViewState: SaveViewState::<Identity, OFFSET>,
            ApplyViewState: ApplyViewState::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleDocumentView as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidlbase", feature = "oleidl", feature = "windef"))]
impl windows_core::RuntimeName for IOleDocumentView {}
windows_core::imp::define_interface!(IPrint, IPrint_Vtbl, 0xb722bcc9_4e68_101b_a2bc_00aa00404770);
windows_core::imp::interface_hierarchy!(IPrint, windows_core::IUnknown);
impl IPrint {
    pub unsafe fn SetInitialPageNum(&self, nfirstpage: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInitialPageNum)(windows_core::Interface::as_raw(self), nfirstpage) }
    }
    pub unsafe fn GetPageInfo(&self, pnfirstpage: *mut i32, pcpages: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPageInfo)(windows_core::Interface::as_raw(self), pnfirstpage as _, pcpages as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub unsafe fn Print<P4>(&self, grfflags: u32, pptd: *mut *mut super::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::STGMEDIUM, pcallback: P4, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> windows_core::HRESULT
    where
        P4: windows_core::Param<IContinueCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Print)(windows_core::Interface::as_raw(self), grfflags, pptd as _, pppageset as _, pstgmoptions, pcallback.param().abi(), nfirstpage, pcpagesprinted as _, pnlastpage as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetInitialPageNum: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub Print: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut super::DVTARGETDEVICE, *mut *mut PAGESET, *mut super::STGMEDIUM, *mut core::ffi::c_void, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes")))]
    Print: usize,
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub trait IPrint_Impl: windows_core::IUnknownImpl {
    fn SetInitialPageNum(&self, nfirstpage: i32) -> windows_core::Result<()>;
    fn GetPageInfo(&self, pnfirstpage: *mut i32, pcpages: *mut i32) -> windows_core::Result<()>;
    fn Print(&self, grfflags: u32, pptd: *mut *mut super::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::STGMEDIUM, pcallback: windows_core::Ref<IContinueCallback>, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl IPrint_Vtbl {
    pub const fn new<Identity: IPrint_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetInitialPageNum<Identity: IPrint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nfirstpage: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrint_Impl::SetInitialPageNum(this, core::mem::transmute_copy(&nfirstpage)).into()
            }
        }
        unsafe extern "system" fn GetPageInfo<Identity: IPrint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnfirstpage: *mut i32, pcpages: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrint_Impl::GetPageInfo(this, core::mem::transmute_copy(&pnfirstpage), core::mem::transmute_copy(&pcpages)).into()
            }
        }
        unsafe extern "system" fn Print<Identity: IPrint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfflags: u32, pptd: *mut *mut super::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::STGMEDIUM, pcallback: *mut core::ffi::c_void, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrint_Impl::Print(this, core::mem::transmute_copy(&grfflags), core::mem::transmute_copy(&pptd), core::mem::transmute_copy(&pppageset), core::mem::transmute_copy(&pstgmoptions), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&nfirstpage), core::mem::transmute_copy(&pcpagesprinted), core::mem::transmute_copy(&pnlastpage)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInitialPageNum: SetInitialPageNum::<Identity, OFFSET>,
            GetPageInfo: GetPageInfo::<Identity, OFFSET>,
            Print: Print::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrint as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl windows_core::RuntimeName for IPrint {}
windows_core::imp::define_interface!(IProtectFocus, IProtectFocus_Vtbl, 0xd81f90a3_8156_44f7_ad28_5abb87003274);
windows_core::imp::interface_hierarchy!(IProtectFocus, windows_core::IUnknown);
impl IProtectFocus {
    pub unsafe fn AllowFocusChange(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowFocusChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectFocus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AllowFocusChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IProtectFocus_Impl: windows_core::IUnknownImpl {
    fn AllowFocusChange(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IProtectFocus_Vtbl {
    pub const fn new<Identity: IProtectFocus_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AllowFocusChange<Identity: IProtectFocus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfallow: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProtectFocus_Impl::AllowFocusChange(this) {
                    Ok(ok__) => {
                        pfallow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AllowFocusChange: AllowFocusChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProtectFocus as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IProtectFocus {}
windows_core::imp::define_interface!(IProtectedModeMenuServices, IProtectedModeMenuServices_Vtbl, 0x73c105ee_9dff_4a07_b83c_7eff290c266e);
windows_core::imp::interface_hierarchy!(IProtectedModeMenuServices, windows_core::IUnknown);
impl IProtectedModeMenuServices {
    #[cfg(feature = "windef")]
    pub unsafe fn CreateMenu(&self) -> windows_core::Result<super::HMENU> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMenu)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn LoadMenu<P0, P1>(&self, pszmodulename: P0, pszmenuname: P1) -> windows_core::Result<super::HMENU>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadMenu)(windows_core::Interface::as_raw(self), pszmodulename.param().abi(), pszmenuname.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn LoadMenuID<P0>(&self, pszmodulename: P0, wresourceid: u16) -> windows_core::Result<super::HMENU>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadMenuID)(windows_core::Interface::as_raw(self), pszmodulename.param().abi(), wresourceid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedModeMenuServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub CreateMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HMENU) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    CreateMenu: usize,
    #[cfg(feature = "windef")]
    pub LoadMenu: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut super::HMENU) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    LoadMenu: usize,
    #[cfg(feature = "windef")]
    pub LoadMenuID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u16, *mut super::HMENU) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    LoadMenuID: usize,
}
#[cfg(feature = "windef")]
pub trait IProtectedModeMenuServices_Impl: windows_core::IUnknownImpl {
    fn CreateMenu(&self) -> windows_core::Result<super::HMENU>;
    fn LoadMenu(&self, pszmodulename: &windows_core::PCWSTR, pszmenuname: &windows_core::PCWSTR) -> windows_core::Result<super::HMENU>;
    fn LoadMenuID(&self, pszmodulename: &windows_core::PCWSTR, wresourceid: u16) -> windows_core::Result<super::HMENU>;
}
#[cfg(feature = "windef")]
impl IProtectedModeMenuServices_Vtbl {
    pub const fn new<Identity: IProtectedModeMenuServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateMenu<Identity: IProtectedModeMenuServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phmenu: *mut super::HMENU) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProtectedModeMenuServices_Impl::CreateMenu(this) {
                    Ok(ok__) => {
                        phmenu.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LoadMenu<Identity: IProtectedModeMenuServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmodulename: windows_core::PCWSTR, pszmenuname: windows_core::PCWSTR, phmenu: *mut super::HMENU) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProtectedModeMenuServices_Impl::LoadMenu(this, core::mem::transmute(&pszmodulename), core::mem::transmute(&pszmenuname)) {
                    Ok(ok__) => {
                        phmenu.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LoadMenuID<Identity: IProtectedModeMenuServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmodulename: windows_core::PCWSTR, wresourceid: u16, phmenu: *mut super::HMENU) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProtectedModeMenuServices_Impl::LoadMenuID(this, core::mem::transmute(&pszmodulename), core::mem::transmute_copy(&wresourceid)) {
                    Ok(ok__) => {
                        phmenu.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateMenu: CreateMenu::<Identity, OFFSET>,
            LoadMenu: LoadMenu::<Identity, OFFSET>,
            LoadMenuID: LoadMenuID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProtectedModeMenuServices as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IProtectedModeMenuServices {}
windows_core::imp::define_interface!(IZoomEvents, IZoomEvents_Vtbl, 0x41b68150_904c_4e17_a0ba_a438182e359d);
windows_core::imp::interface_hierarchy!(IZoomEvents, windows_core::IUnknown);
impl IZoomEvents {
    pub unsafe fn OnZoomPercentChanged(&self, ulzoompercent: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnZoomPercentChanged)(windows_core::Interface::as_raw(self), ulzoompercent) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IZoomEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnZoomPercentChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IZoomEvents_Impl: windows_core::IUnknownImpl {
    fn OnZoomPercentChanged(&self, ulzoompercent: u32) -> windows_core::Result<()>;
}
impl IZoomEvents_Vtbl {
    pub const fn new<Identity: IZoomEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnZoomPercentChanged<Identity: IZoomEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulzoompercent: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IZoomEvents_Impl::OnZoomPercentChanged(this, core::mem::transmute_copy(&ulzoompercent)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnZoomPercentChanged: OnZoomPercentChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IZoomEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IZoomEvents {}
pub const MEDIAPLAYBACK_PAUSE: MEDIAPLAYBACK_STATE = 1;
pub const MEDIAPLAYBACK_PAUSE_AND_SUSPEND: MEDIAPLAYBACK_STATE = 2;
pub const MEDIAPLAYBACK_RESUME: MEDIAPLAYBACK_STATE = 0;
pub const MEDIAPLAYBACK_RESUME_FROM_SUSPEND: MEDIAPLAYBACK_STATE = 3;
pub type MEDIAPLAYBACK_STATE = i32;
pub const MSOCMDERR_E_CANCELED: i32 = -2147221245;
pub const MSOCMDERR_E_DISABLED: i32 = -2147221247;
pub const MSOCMDERR_E_FIRST: i32 = -2147221248;
pub const MSOCMDERR_E_NOHELP: i32 = -2147221246;
pub const MSOCMDERR_E_NOTSUPPORTED: i32 = -2147221248;
pub const MSOCMDERR_E_UNKNOWNGROUP: i32 = -2147221244;
pub const MSOCMDEXECOPT_DODEFAULT: u32 = 0;
pub const MSOCMDEXECOPT_DONTPROMPTUSER: u32 = 2;
pub const MSOCMDEXECOPT_PROMPTUSER: u32 = 1;
pub const MSOCMDEXECOPT_SHOWHELP: u32 = 3;
pub const MSOCMDF_ENABLED: u32 = 2;
pub const MSOCMDF_LATCHED: u32 = 4;
pub const MSOCMDF_NINCHED: u32 = 8;
pub const MSOCMDF_SUPPORTED: u32 = 1;
pub const MSOCMDID_CLEARSELECTION: u32 = 18;
pub const MSOCMDID_COPY: u32 = 12;
pub const MSOCMDID_CUT: u32 = 11;
pub const MSOCMDID_GETZOOMRANGE: u32 = 20;
pub const MSOCMDID_NEW: u32 = 2;
pub const MSOCMDID_OPEN: u32 = 1;
pub const MSOCMDID_PAGESETUP: u32 = 8;
pub const MSOCMDID_PASTE: u32 = 13;
pub const MSOCMDID_PASTESPECIAL: u32 = 14;
pub const MSOCMDID_PRINT: u32 = 6;
pub const MSOCMDID_PRINTPREVIEW: u32 = 7;
pub const MSOCMDID_PROPERTIES: u32 = 10;
pub const MSOCMDID_REDO: u32 = 16;
pub const MSOCMDID_SAVE: u32 = 3;
pub const MSOCMDID_SAVEAS: u32 = 4;
pub const MSOCMDID_SAVECOPYAS: u32 = 5;
pub const MSOCMDID_SELECTALL: u32 = 17;
pub const MSOCMDID_SPELL: u32 = 9;
pub const MSOCMDID_UNDO: u32 = 15;
pub const MSOCMDID_ZOOM: u32 = 19;
pub const MSOCMDTEXTF_NAME: u32 = 1;
pub const MSOCMDTEXTF_NONE: u32 = 0;
pub const MSOCMDTEXTF_STATUS: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OLECMD {
    pub cmdID: u32,
    pub cmdf: u32,
}
pub const OLECMDARGINDEX_ACTIVEXINSTALL_CLSID: u32 = 2;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_DISPLAYNAME: u32 = 1;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_INSTALLSCOPE: u32 = 3;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_PUBLISHER: u32 = 0;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_SOURCEURL: u32 = 4;
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_HWND: u32 = 0;
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_X: u32 = 1;
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_Y: u32 = 2;
pub const OLECMDERR_E_CANCELED: i32 = -2147221245;
pub const OLECMDERR_E_DISABLED: i32 = -2147221247;
pub const OLECMDERR_E_FIRST: i32 = -2147221248;
pub const OLECMDERR_E_NOHELP: i32 = -2147221246;
pub const OLECMDERR_E_NOTSUPPORTED: i32 = -2147221248;
pub const OLECMDERR_E_UNKNOWNGROUP: i32 = -2147221244;
pub type OLECMDEXECOPT = i32;
pub const OLECMDEXECOPT_DODEFAULT: OLECMDEXECOPT = 0;
pub const OLECMDEXECOPT_DONTPROMPTUSER: OLECMDEXECOPT = 2;
pub const OLECMDEXECOPT_PROMPTUSER: OLECMDEXECOPT = 1;
pub const OLECMDEXECOPT_SHOWHELP: OLECMDEXECOPT = 3;
pub type OLECMDF = i32;
pub const OLECMDF_DEFHIDEONCTXTMENU: OLECMDF = 32;
pub const OLECMDF_ENABLED: OLECMDF = 2;
pub const OLECMDF_INVISIBLE: OLECMDF = 16;
pub const OLECMDF_LATCHED: OLECMDF = 4;
pub const OLECMDF_NINCHED: OLECMDF = 8;
pub const OLECMDF_SUPPORTED: OLECMDF = 1;
pub type OLECMDID = i32;
pub const OLECMDIDF_BROWSERSTATE_BLOCKEDVERSION: OLECMDID_BROWSERSTATEFLAG = 64;
pub const OLECMDIDF_BROWSERSTATE_DESKTOPHTMLDIALOG: OLECMDID_BROWSERSTATEFLAG = 32;
pub const OLECMDIDF_BROWSERSTATE_EXTENSIONSOFF: OLECMDID_BROWSERSTATEFLAG = 1;
pub const OLECMDIDF_BROWSERSTATE_IESECURITY: OLECMDID_BROWSERSTATEFLAG = 2;
pub const OLECMDIDF_BROWSERSTATE_PROTECTEDMODE_OFF: OLECMDID_BROWSERSTATEFLAG = 4;
pub const OLECMDIDF_BROWSERSTATE_REQUIRESACTIVEX: OLECMDID_BROWSERSTATEFLAG = 16;
pub const OLECMDIDF_BROWSERSTATE_RESET: OLECMDID_BROWSERSTATEFLAG = 8;
pub const OLECMDIDF_OPTICAL_ZOOM_NOLAYOUT: OLECMDID_OPTICAL_ZOOMFLAG = 16;
pub const OLECMDIDF_OPTICAL_ZOOM_NOPERSIST: OLECMDID_OPTICAL_ZOOMFLAG = 1;
pub const OLECMDIDF_OPTICAL_ZOOM_NOTRANSIENT: OLECMDID_OPTICAL_ZOOMFLAG = 32;
pub const OLECMDIDF_OPTICAL_ZOOM_RELOADFORNEWTAB: OLECMDID_OPTICAL_ZOOMFLAG = 64;
pub const OLECMDIDF_PAGEACTION_ACTIVEXDISALLOW: OLECMDID_PAGEACTIONFLAG = 16;
pub const OLECMDIDF_PAGEACTION_ACTIVEXINSTALL: OLECMDID_PAGEACTIONFLAG = 2;
pub const OLECMDIDF_PAGEACTION_ACTIVEXTRUSTFAIL: OLECMDID_PAGEACTIONFLAG = 4;
pub const OLECMDIDF_PAGEACTION_ACTIVEXUNSAFE: OLECMDID_PAGEACTIONFLAG = 32;
pub const OLECMDIDF_PAGEACTION_ACTIVEXUSERAPPROVAL: OLECMDID_PAGEACTIONFLAG = 262144;
pub const OLECMDIDF_PAGEACTION_ACTIVEXUSERDISABLE: OLECMDID_PAGEACTIONFLAG = 8;
pub const OLECMDIDF_PAGEACTION_ACTIVEX_EPM_INCOMPATIBLE: OLECMDID_PAGEACTIONFLAG = 16777216;
pub const OLECMDIDF_PAGEACTION_EXTENSION_COMPAT_BLOCKED: OLECMDID_PAGEACTIONFLAG = 268435456;
pub const OLECMDIDF_PAGEACTION_FILEDOWNLOAD: OLECMDID_PAGEACTIONFLAG = 1;
pub const OLECMDIDF_PAGEACTION_GENERIC_STATE: OLECMDID_PAGEACTIONFLAG = 1073741824;
pub const OLECMDIDF_PAGEACTION_INTRANETZONEREQUEST: OLECMDID_PAGEACTIONFLAG = 2097152;
pub const OLECMDIDF_PAGEACTION_INVALID_CERT: OLECMDID_PAGEACTIONFLAG = 1048576;
pub const OLECMDIDF_PAGEACTION_LOCALMACHINE: OLECMDID_PAGEACTIONFLAG = 128;
pub const OLECMDIDF_PAGEACTION_MIMETEXTPLAIN: OLECMDID_PAGEACTIONFLAG = 256;
pub const OLECMDIDF_PAGEACTION_MIXEDCONTENT: OLECMDID_PAGEACTIONFLAG = 524288;
pub const OLECMDIDF_PAGEACTION_NORESETACTIVEX: OLECMDID_PAGEACTIONFLAG = 536870912;
pub const OLECMDIDF_PAGEACTION_POPUPALLOWED: OLECMDID_PAGEACTIONFLAG = 65536;
pub const OLECMDIDF_PAGEACTION_POPUPWINDOW: OLECMDID_PAGEACTIONFLAG = 64;
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNDENY: OLECMDID_PAGEACTIONFLAG = 32768;
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNINTERNET: OLECMDID_PAGEACTIONFLAG = 8192;
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNINTRANET: OLECMDID_PAGEACTIONFLAG = 4096;
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNLOCALMACHINE: OLECMDID_PAGEACTIONFLAG = 1024;
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNRESTRICTED: OLECMDID_PAGEACTIONFLAG = 16384;
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNTRUSTED: OLECMDID_PAGEACTIONFLAG = 2048;
pub const OLECMDIDF_PAGEACTION_RESET: OLECMDID_PAGEACTIONFLAG = -2147483648;
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE: OLECMDID_PAGEACTIONFLAG = 512;
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE_ACTIVEXINSTALL: OLECMDID_PAGEACTIONFLAG = 512;
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE_ACTIVEXUSERAPPROVAL: OLECMDID_PAGEACTIONFLAG = 33554432;
pub const OLECMDIDF_PAGEACTION_SCRIPTPROMPT: OLECMDID_PAGEACTIONFLAG = 131072;
pub const OLECMDIDF_PAGEACTION_SPOOFABLEIDNHOST: OLECMDID_PAGEACTIONFLAG = 8388608;
pub const OLECMDIDF_PAGEACTION_WPCBLOCKED: OLECMDID_PAGEACTIONFLAG = 67108864;
pub const OLECMDIDF_PAGEACTION_WPCBLOCKED_ACTIVEX: OLECMDID_PAGEACTIONFLAG = 134217728;
pub const OLECMDIDF_PAGEACTION_XSSFILTERED: OLECMDID_PAGEACTIONFLAG = 4194304;
pub const OLECMDIDF_REFRESH_CLEARUSERINPUT: OLECMDID_REFRESHFLAG = 4096;
pub const OLECMDIDF_REFRESH_COMPLETELY: OLECMDID_REFRESHFLAG = 3;
pub const OLECMDIDF_REFRESH_CONTINUE: OLECMDID_REFRESHFLAG = 2;
pub const OLECMDIDF_REFRESH_IFEXPIRED: OLECMDID_REFRESHFLAG = 1;
pub const OLECMDIDF_REFRESH_LEVELMASK: OLECMDID_REFRESHFLAG = 255;
pub const OLECMDIDF_REFRESH_NORMAL: OLECMDID_REFRESHFLAG = 0;
pub const OLECMDIDF_REFRESH_NO_CACHE: OLECMDID_REFRESHFLAG = 4;
pub const OLECMDIDF_REFRESH_PAGEACTION_ACTIVEXINSTALL: OLECMDID_REFRESHFLAG = 65536;
pub const OLECMDIDF_REFRESH_PAGEACTION_ALLOW_VERSION: OLECMDID_REFRESHFLAG = 134217728;
pub const OLECMDIDF_REFRESH_PAGEACTION_FILEDOWNLOAD: OLECMDID_REFRESHFLAG = 131072;
pub const OLECMDIDF_REFRESH_PAGEACTION_INVALID_CERT: OLECMDID_REFRESHFLAG = 67108864;
pub const OLECMDIDF_REFRESH_PAGEACTION_LOCALMACHINE: OLECMDID_REFRESHFLAG = 262144;
pub const OLECMDIDF_REFRESH_PAGEACTION_MIXEDCONTENT: OLECMDID_REFRESHFLAG = 33554432;
pub const OLECMDIDF_REFRESH_PAGEACTION_POPUPWINDOW: OLECMDID_REFRESHFLAG = 524288;
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNINTERNET: OLECMDID_REFRESHFLAG = 8388608;
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNINTRANET: OLECMDID_REFRESHFLAG = 4194304;
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNLOCALMACHINE: OLECMDID_REFRESHFLAG = 1048576;
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNRESTRICTED: OLECMDID_REFRESHFLAG = 16777216;
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNTRUSTED: OLECMDID_REFRESHFLAG = 2097152;
pub const OLECMDIDF_REFRESH_PROMPTIFOFFLINE: OLECMDID_REFRESHFLAG = 8192;
pub const OLECMDIDF_REFRESH_RELOAD: OLECMDID_REFRESHFLAG = 5;
pub const OLECMDIDF_REFRESH_SKIPBEFOREUNLOADEVENT: OLECMDID_REFRESHFLAG = 32768;
pub const OLECMDIDF_REFRESH_THROUGHSCRIPT: OLECMDID_REFRESHFLAG = 16384;
pub const OLECMDIDF_VIEWPORTMODE_EXCLUDE_VISUAL_BOTTOM: OLECMDID_VIEWPORT_MODE_FLAG = 2;
pub const OLECMDIDF_VIEWPORTMODE_EXCLUDE_VISUAL_BOTTOM_VALID: OLECMDID_VIEWPORT_MODE_FLAG = 131072;
pub const OLECMDIDF_VIEWPORTMODE_FIXED_LAYOUT_WIDTH: OLECMDID_VIEWPORT_MODE_FLAG = 1;
pub const OLECMDIDF_VIEWPORTMODE_FIXED_LAYOUT_WIDTH_VALID: OLECMDID_VIEWPORT_MODE_FLAG = 65536;
pub const OLECMDIDF_WINDOWSTATE_ENABLED: OLECMDID_WINDOWSTATE_FLAG = 2;
pub const OLECMDIDF_WINDOWSTATE_ENABLED_VALID: OLECMDID_WINDOWSTATE_FLAG = 131072;
pub const OLECMDIDF_WINDOWSTATE_USERVISIBLE: OLECMDID_WINDOWSTATE_FLAG = 1;
pub const OLECMDIDF_WINDOWSTATE_USERVISIBLE_VALID: OLECMDID_WINDOWSTATE_FLAG = 65536;
pub const OLECMDID_ACTIVEXINSTALLSCOPE: OLECMDID = 66;
pub const OLECMDID_ADDTRAVELENTRY: OLECMDID = 60;
pub const OLECMDID_ALLOWUILESSSAVEAS: OLECMDID = 46;
pub type OLECMDID_BROWSERSTATEFLAG = i32;
pub const OLECMDID_CLEARSELECTION: OLECMDID = 18;
pub const OLECMDID_CLOSE: OLECMDID = 45;
pub const OLECMDID_COPY: OLECMDID = 12;
pub const OLECMDID_CUT: OLECMDID = 11;
pub const OLECMDID_DELETE: OLECMDID = 33;
pub const OLECMDID_DONTDOWNLOADCSS: OLECMDID = 47;
pub const OLECMDID_ENABLE_INTERACTION: OLECMDID = 36;
pub const OLECMDID_ENABLE_VISIBILITY: OLECMDID = 77;
pub const OLECMDID_EXITFULLSCREEN: OLECMDID = 81;
pub const OLECMDID_FIND: OLECMDID = 32;
pub const OLECMDID_FOCUSVIEWCONTROLS: OLECMDID = 57;
pub const OLECMDID_FOCUSVIEWCONTROLSQUERY: OLECMDID = 58;
pub const OLECMDID_GETPRINTTEMPLATE: OLECMDID = 52;
pub const OLECMDID_GETUSERSCALABLE: OLECMDID = 75;
pub const OLECMDID_GETZOOMRANGE: OLECMDID = 20;
pub const OLECMDID_HIDETOOLBARS: OLECMDID = 24;
pub const OLECMDID_HTTPEQUIV: OLECMDID = 34;
pub const OLECMDID_HTTPEQUIV_DONE: OLECMDID = 35;
pub const OLECMDID_LAYOUT_VIEWPORT_WIDTH: OLECMDID = 71;
pub const OLECMDID_MEDIA_PLAYBACK: OLECMDID = 78;
pub const OLECMDID_NEW: OLECMDID = 2;
pub const OLECMDID_ONBEFOREUNLOAD: OLECMDID = 83;
pub const OLECMDID_ONTOOLBARACTIVATED: OLECMDID = 31;
pub const OLECMDID_ONUNLOAD: OLECMDID = 37;
pub const OLECMDID_OPEN: OLECMDID = 1;
pub const OLECMDID_OPTICAL_GETZOOMRANGE: OLECMDID = 64;
pub const OLECMDID_OPTICAL_ZOOM: OLECMDID = 63;
pub type OLECMDID_OPTICAL_ZOOMFLAG = i32;
pub const OLECMDID_PAGEACTIONBLOCKED: OLECMDID = 55;
pub type OLECMDID_PAGEACTIONFLAG = i32;
pub const OLECMDID_PAGEACTIONUIQUERY: OLECMDID = 56;
pub const OLECMDID_PAGEAVAILABLE: OLECMDID = 74;
pub const OLECMDID_PAGESETUP: OLECMDID = 8;
pub const OLECMDID_PASTE: OLECMDID = 13;
pub const OLECMDID_PASTESPECIAL: OLECMDID = 14;
pub const OLECMDID_POPSTATEEVENT: OLECMDID = 69;
pub const OLECMDID_PREREFRESH: OLECMDID = 39;
pub const OLECMDID_PRINT: OLECMDID = 6;
pub const OLECMDID_PRINT2: OLECMDID = 49;
pub const OLECMDID_PRINTPREVIEW: OLECMDID = 7;
pub const OLECMDID_PRINTPREVIEW2: OLECMDID = 50;
pub const OLECMDID_PROPERTIES: OLECMDID = 10;
pub const OLECMDID_PROPERTYBAG2: OLECMDID = 38;
pub const OLECMDID_REDO: OLECMDID = 16;
pub const OLECMDID_REFRESH: OLECMDID = 22;
pub type OLECMDID_REFRESHFLAG = i32;
pub const OLECMDID_SAVE: OLECMDID = 3;
pub const OLECMDID_SAVEAS: OLECMDID = 4;
pub const OLECMDID_SAVECOPYAS: OLECMDID = 5;
pub const OLECMDID_SCROLLCOMPLETE: OLECMDID = 82;
pub const OLECMDID_SELECTALL: OLECMDID = 17;
pub const OLECMDID_SETDOWNLOADSTATE: OLECMDID = 29;
pub const OLECMDID_SETFAVICON: OLECMDID = 79;
pub const OLECMDID_SETPRINTTEMPLATE: OLECMDID = 51;
pub const OLECMDID_SETPROGRESSMAX: OLECMDID = 25;
pub const OLECMDID_SETPROGRESSPOS: OLECMDID = 26;
pub const OLECMDID_SETPROGRESSTEXT: OLECMDID = 27;
pub const OLECMDID_SETTITLE: OLECMDID = 28;
pub const OLECMDID_SET_HOST_FULLSCREENMODE: OLECMDID = 80;
pub const OLECMDID_SHOWFIND: OLECMDID = 42;
pub const OLECMDID_SHOWMESSAGE: OLECMDID = 41;
pub const OLECMDID_SHOWMESSAGE_BLOCKABLE: OLECMDID = 84;
pub const OLECMDID_SHOWPAGEACTIONMENU: OLECMDID = 59;
pub const OLECMDID_SHOWPAGESETUP: OLECMDID = 43;
pub const OLECMDID_SHOWPRINT: OLECMDID = 44;
pub const OLECMDID_SHOWSCRIPTERROR: OLECMDID = 40;
pub const OLECMDID_SHOWTASKDLG: OLECMDID = 68;
pub const OLECMDID_SHOWTASKDLG_BLOCKABLE: OLECMDID = 85;
pub const OLECMDID_SPELL: OLECMDID = 9;
pub const OLECMDID_STOP: OLECMDID = 23;
pub const OLECMDID_STOPDOWNLOAD: OLECMDID = 30;
pub const OLECMDID_UNDO: OLECMDID = 15;
pub const OLECMDID_UPDATEBACKFORWARDSTATE: OLECMDID = 62;
pub const OLECMDID_UPDATECOMMANDS: OLECMDID = 21;
pub const OLECMDID_UPDATEPAGESTATUS: OLECMDID = 48;
pub const OLECMDID_UPDATETRAVELENTRY: OLECMDID = 61;
pub const OLECMDID_UPDATETRAVELENTRY_DATARECOVERY: OLECMDID = 67;
pub const OLECMDID_UPDATE_CARET: OLECMDID = 76;
pub const OLECMDID_USER_OPTICAL_ZOOM: OLECMDID = 73;
pub const OLECMDID_VIEWPORT_MODE: OLECMDID = 70;
pub type OLECMDID_VIEWPORT_MODE_FLAG = i32;
pub const OLECMDID_VISUAL_VIEWPORT_EXCLUDE_BOTTOM: OLECMDID = 72;
pub const OLECMDID_WINDOWSTATECHANGED: OLECMDID = 65;
pub type OLECMDID_WINDOWSTATE_FLAG = i32;
pub const OLECMDID_ZOOM: OLECMDID = 19;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OLECMDTEXT {
    pub cmdtextf: u32,
    pub cwActual: u32,
    pub cwBuf: u32,
    pub rgwz: [u16; 1],
}
impl Default for OLECMDTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OLECMDTEXTF = i32;
pub const OLECMDTEXTF_NAME: OLECMDTEXTF = 1;
pub const OLECMDTEXTF_NONE: OLECMDTEXTF = 0;
pub const OLECMDTEXTF_STATUS: OLECMDTEXTF = 2;
pub const OLECMD_TASKDLGID_ONBEFOREUNLOAD: u32 = 1;
pub type PAGEACTION_UI = i32;
pub const PAGEACTION_UI_DEFAULT: PAGEACTION_UI = 0;
pub const PAGEACTION_UI_MODAL: PAGEACTION_UI = 1;
pub const PAGEACTION_UI_MODELESS: PAGEACTION_UI = 2;
pub const PAGEACTION_UI_SILENT: PAGEACTION_UI = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAGERANGE {
    pub nFromPage: i32,
    pub nToPage: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PAGESET {
    pub cbStruct: u32,
    pub fOddPages: windows_core::BOOL,
    pub fEvenPages: windows_core::BOOL,
    pub cPageRange: u32,
    pub rgPages: [PAGERANGE; 1],
}
impl Default for PAGESET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PAGESET_TOLASTPAGE: u32 = 65535;
pub type PRINTFLAG = i32;
pub const PRINTFLAG_DONTACTUALLYPRINT: PRINTFLAG = 16;
pub const PRINTFLAG_FORCEPROPERTIES: PRINTFLAG = 32;
pub const PRINTFLAG_MAYBOTHERUSER: PRINTFLAG = 1;
pub const PRINTFLAG_PRINTTOFILE: PRINTFLAG = 64;
pub const PRINTFLAG_PROMPTUSER: PRINTFLAG = 2;
pub const PRINTFLAG_RECOMPOSETODEVICE: PRINTFLAG = 8;
pub const PRINTFLAG_USERMAYCHANGEPRINTER: PRINTFLAG = 4;
pub type WPCSETTING = i32;
pub const WPCSETTING_FILEDOWNLOAD_BLOCKED: WPCSETTING = 2;
pub const WPCSETTING_LOGGING_ENABLED: WPCSETTING = 1;
