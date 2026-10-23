pub const IID_IRichEditOle: windows_core::GUID = windows_core::GUID::from_u128(0x00020d00_0000_0000_c000_000000000046);
pub const IID_IRichEditOleCallback: windows_core::GUID = windows_core::GUID::from_u128(0x00020d03_0000_0000_c000_000000000046);
windows_core::imp::define_interface!(IRichEditOle, IRichEditOle_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IRichEditOle, windows_core::IUnknown);
impl IRichEditOle {
    #[cfg(feature = "oleidl")]
    pub unsafe fn GetClientSite(&self, lplpolesite: *mut Option<super::LPOLECLIENTSITE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClientSite)(windows_core::Interface::as_raw(self), lplpolesite as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetObjectCount(&self) -> super::LONG {
        unsafe { (windows_core::Interface::vtable(self).GetObjectCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetLinkCount(&self) -> super::LONG {
        unsafe { (windows_core::Interface::vtable(self).GetLinkCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "guiddef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "windef"))]
    pub unsafe fn GetObjectA(&self, iob: super::LONG, lpreobject: *mut REOBJECT, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetObjectA)(windows_core::Interface::as_raw(self), iob, lpreobject as _, dwflags) }
    }
    #[cfg(all(feature = "guiddef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "windef"))]
    pub unsafe fn InsertObject(&self, lpreobject: *mut REOBJECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InsertObject)(windows_core::Interface::as_raw(self), lpreobject as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn ConvertObject(&self, iob: super::LONG, rclsidnew: *const super::IID, lpstrusertypenew: &windows_core::PCSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ConvertObject)(windows_core::Interface::as_raw(self), iob, rclsidnew as _, core::mem::transmute_copy(lpstrusertypenew)) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn ActivateAs(&self, rclsid: *const super::IID, rclsidas: *const super::IID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ActivateAs)(windows_core::Interface::as_raw(self), rclsid as _, rclsidas as _) }
    }
    pub unsafe fn SetHostNames(&self, lpstrcontainerapp: &windows_core::PCSTR, lpstrcontainerobj: &windows_core::PCSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHostNames)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lpstrcontainerapp), core::mem::transmute_copy(lpstrcontainerobj)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetLinkAvailable(&self, iob: super::LONG, favailable: windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLinkAvailable)(windows_core::Interface::as_raw(self), iob, favailable) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetDvaspect(&self, iob: super::LONG, dvaspect: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDvaspect)(windows_core::Interface::as_raw(self), iob, dvaspect) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn HandsOffStorage(&self, iob: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HandsOffStorage)(windows_core::Interface::as_raw(self), iob) }
    }
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub unsafe fn SaveCompleted(&self, iob: super::LONG, lpstg: &Option<super::LPSTORAGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SaveCompleted)(windows_core::Interface::as_raw(self), iob, core::mem::transmute_copy(lpstg)) }
    }
    pub unsafe fn InPlaceDeactivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InPlaceDeactivate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ContextSensitiveHelp(&self, fentermode: windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ContextSensitiveHelp)(windows_core::Interface::as_raw(self), fentermode) }
    }
    #[cfg(all(feature = "objidl", feature = "richedit", feature = "shared"))]
    pub unsafe fn GetClipboardData(&self, lpchrg: *mut super::CHARRANGE, reco: super::DWORD, lplpdataobj: *mut Option<super::LPDATAOBJECT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClipboardData)(windows_core::Interface::as_raw(self), lpchrg as _, reco, lplpdataobj as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "objidl", feature = "wtypes"))]
    pub unsafe fn ImportDataObject(&self, lpdataobj: &Option<super::LPDATAOBJECT>, cf: super::CLIPFORMAT, hmetapict: super::HGLOBAL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ImportDataObject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lpdataobj), cf, hmetapict as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichEditOle_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oleidl")]
    pub GetClientSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oleidl"))]
    GetClientSite: usize,
    #[cfg(feature = "shared")]
    pub GetObjectCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::LONG,
    #[cfg(not(feature = "shared"))]
    GetObjectCount: usize,
    #[cfg(feature = "shared")]
    pub GetLinkCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::LONG,
    #[cfg(not(feature = "shared"))]
    GetLinkCount: usize,
    #[cfg(all(feature = "guiddef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "windef"))]
    pub GetObjectA: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut REOBJECT, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "windef")))]
    GetObjectA: usize,
    #[cfg(all(feature = "guiddef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "windef"))]
    pub InsertObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut REOBJECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "windef")))]
    InsertObject: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub ConvertObject: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *const super::IID, windows_core::PCSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    ConvertObject: usize,
    #[cfg(feature = "guiddef")]
    pub ActivateAs: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *const super::IID) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    ActivateAs: usize,
    pub SetHostNames: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, windows_core::PCSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetLinkAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetLinkAvailable: usize,
    #[cfg(feature = "shared")]
    pub SetDvaspect: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetDvaspect: usize,
    #[cfg(feature = "shared")]
    pub HandsOffStorage: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    HandsOffStorage: usize,
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub SaveCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared")))]
    SaveCompleted: usize,
    pub InPlaceDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContextSensitiveHelp: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "objidl", feature = "richedit", feature = "shared"))]
    pub GetClipboardData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CHARRANGE, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "richedit", feature = "shared")))]
    GetClipboardData: usize,
    #[cfg(all(feature = "minwindef", feature = "objidl", feature = "wtypes"))]
    pub ImportDataObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::CLIPFORMAT, super::HGLOBAL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "objidl", feature = "wtypes")))]
    ImportDataObject: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "richedit", feature = "shared", feature = "windef", feature = "wtypes"))]
pub trait IRichEditOle_Impl: windows_core::IUnknownImpl {
    fn GetClientSite(&self, lplpolesite: *mut Option<super::LPOLECLIENTSITE>) -> windows_core::Result<()>;
    fn GetObjectCount(&self) -> super::LONG;
    fn GetLinkCount(&self) -> super::LONG;
    fn GetObjectA(&self, iob: super::LONG, lpreobject: *mut REOBJECT, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn InsertObject(&self, lpreobject: *mut REOBJECT) -> windows_core::Result<()>;
    fn ConvertObject(&self, iob: super::LONG, rclsidnew: *const super::IID, lpstrusertypenew: windows_core::PCSTR) -> windows_core::Result<()>;
    fn ActivateAs(&self, rclsid: *const super::IID, rclsidas: *const super::IID) -> windows_core::Result<()>;
    fn SetHostNames(&self, lpstrcontainerapp: windows_core::PCSTR, lpstrcontainerobj: windows_core::PCSTR) -> windows_core::Result<()>;
    fn SetLinkAvailable(&self, iob: super::LONG, favailable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetDvaspect(&self, iob: super::LONG, dvaspect: super::DWORD) -> windows_core::Result<()>;
    fn HandsOffStorage(&self, iob: super::LONG) -> windows_core::Result<()>;
    fn SaveCompleted(&self, iob: super::LONG, lpstg: Option<super::LPSTORAGE>) -> windows_core::Result<()>;
    fn InPlaceDeactivate(&self) -> windows_core::Result<()>;
    fn ContextSensitiveHelp(&self, fentermode: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetClipboardData(&self, lpchrg: *mut super::CHARRANGE, reco: super::DWORD, lplpdataobj: *mut Option<super::LPDATAOBJECT>) -> windows_core::Result<()>;
    fn ImportDataObject(&self, lpdataobj: Option<super::LPDATAOBJECT>, cf: super::CLIPFORMAT, hmetapict: super::HGLOBAL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "richedit", feature = "shared", feature = "windef", feature = "wtypes"))]
impl IRichEditOle_Vtbl {
    pub const fn new<Identity: IRichEditOle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClientSite<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lplpolesite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::GetClientSite(this, core::mem::transmute_copy(&lplpolesite)).into()
            }
        }
        unsafe extern "system" fn GetObjectCount<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::LONG {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::GetObjectCount(this)
            }
        }
        unsafe extern "system" fn GetLinkCount<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::LONG {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::GetLinkCount(this)
            }
        }
        unsafe extern "system" fn GetObjectA<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: super::LONG, lpreobject: *mut REOBJECT, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::GetObjectA(this, core::mem::transmute_copy(&iob), core::mem::transmute_copy(&lpreobject), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn InsertObject<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpreobject: *mut REOBJECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::InsertObject(this, core::mem::transmute_copy(&lpreobject)).into()
            }
        }
        unsafe extern "system" fn ConvertObject<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: super::LONG, rclsidnew: *const super::IID, lpstrusertypenew: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::ConvertObject(this, core::mem::transmute_copy(&iob), core::mem::transmute_copy(&rclsidnew), core::mem::transmute_copy(&lpstrusertypenew)).into()
            }
        }
        unsafe extern "system" fn ActivateAs<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const super::IID, rclsidas: *const super::IID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::ActivateAs(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rclsidas)).into()
            }
        }
        unsafe extern "system" fn SetHostNames<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpstrcontainerapp: windows_core::PCSTR, lpstrcontainerobj: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::SetHostNames(this, core::mem::transmute_copy(&lpstrcontainerapp), core::mem::transmute_copy(&lpstrcontainerobj)).into()
            }
        }
        unsafe extern "system" fn SetLinkAvailable<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: super::LONG, favailable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::SetLinkAvailable(this, core::mem::transmute_copy(&iob), core::mem::transmute_copy(&favailable)).into()
            }
        }
        unsafe extern "system" fn SetDvaspect<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: super::LONG, dvaspect: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::SetDvaspect(this, core::mem::transmute_copy(&iob), core::mem::transmute_copy(&dvaspect)).into()
            }
        }
        unsafe extern "system" fn HandsOffStorage<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::HandsOffStorage(this, core::mem::transmute_copy(&iob)).into()
            }
        }
        unsafe extern "system" fn SaveCompleted<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: super::LONG, lpstg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::SaveCompleted(this, core::mem::transmute_copy(&iob), core::mem::transmute(&lpstg)).into()
            }
        }
        unsafe extern "system" fn InPlaceDeactivate<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::InPlaceDeactivate(this).into()
            }
        }
        unsafe extern "system" fn ContextSensitiveHelp<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fentermode: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::ContextSensitiveHelp(this, core::mem::transmute_copy(&fentermode)).into()
            }
        }
        unsafe extern "system" fn GetClipboardData<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpchrg: *mut super::CHARRANGE, reco: super::DWORD, lplpdataobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::GetClipboardData(this, core::mem::transmute_copy(&lpchrg), core::mem::transmute_copy(&reco), core::mem::transmute_copy(&lplpdataobj)).into()
            }
        }
        unsafe extern "system" fn ImportDataObject<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdataobj: *mut core::ffi::c_void, cf: super::CLIPFORMAT, hmetapict: super::HGLOBAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::ImportDataObject(this, core::mem::transmute(&lpdataobj), core::mem::transmute_copy(&cf), core::mem::transmute_copy(&hmetapict)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClientSite: GetClientSite::<Identity, OFFSET>,
            GetObjectCount: GetObjectCount::<Identity, OFFSET>,
            GetLinkCount: GetLinkCount::<Identity, OFFSET>,
            GetObjectA: GetObjectA::<Identity, OFFSET>,
            InsertObject: InsertObject::<Identity, OFFSET>,
            ConvertObject: ConvertObject::<Identity, OFFSET>,
            ActivateAs: ActivateAs::<Identity, OFFSET>,
            SetHostNames: SetHostNames::<Identity, OFFSET>,
            SetLinkAvailable: SetLinkAvailable::<Identity, OFFSET>,
            SetDvaspect: SetDvaspect::<Identity, OFFSET>,
            HandsOffStorage: HandsOffStorage::<Identity, OFFSET>,
            SaveCompleted: SaveCompleted::<Identity, OFFSET>,
            InPlaceDeactivate: InPlaceDeactivate::<Identity, OFFSET>,
            ContextSensitiveHelp: ContextSensitiveHelp::<Identity, OFFSET>,
            GetClipboardData: GetClipboardData::<Identity, OFFSET>,
            ImportDataObject: ImportDataObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRichEditOle as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "richedit", feature = "shared", feature = "windef", feature = "wtypes"))]
impl windows_core::RuntimeName for IRichEditOle {}
windows_core::imp::define_interface!(IRichEditOleCallback, IRichEditOleCallback_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IRichEditOleCallback, windows_core::IUnknown);
impl IRichEditOleCallback {
    #[cfg(feature = "objidl")]
    pub unsafe fn GetNewStorage(&self, lplpstg: *mut Option<super::LPSTORAGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNewStorage)(windows_core::Interface::as_raw(self), lplpstg as _) }
    }
    #[cfg(all(feature = "oleidl", feature = "shared", feature = "windef"))]
    pub unsafe fn GetInPlaceContext(&self, lplpframe: *mut Option<super::LPOLEINPLACEFRAME>, lplpdoc: *mut Option<super::LPOLEINPLACEUIWINDOW>, lpframeinfo: super::LPOLEINPLACEFRAMEINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInPlaceContext)(windows_core::Interface::as_raw(self), lplpframe as _, lplpdoc as _, lpframeinfo as _) }
    }
    pub unsafe fn ShowContainerUI(&self, fshow: windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShowContainerUI)(windows_core::Interface::as_raw(self), fshow) }
    }
    #[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared"))]
    pub unsafe fn QueryInsertObject(&self, lpclsid: super::LPCLSID, lpstg: &Option<super::LPSTORAGE>, cp: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInsertObject)(windows_core::Interface::as_raw(self), lpclsid as _, core::mem::transmute_copy(lpstg), cp) }
    }
    #[cfg(feature = "oleidl")]
    pub unsafe fn DeleteObject(&self, lpoleobj: &Option<super::LPOLEOBJECT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteObject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lpoleobj)) }
    }
    #[cfg(all(feature = "minwindef", feature = "objidl", feature = "shared", feature = "wtypes"))]
    pub unsafe fn QueryAcceptData(&self, lpdataobj: &Option<super::LPDATAOBJECT>, lpcfformat: *mut super::CLIPFORMAT, reco: super::DWORD, freally: windows_core::BOOL, hmetapict: super::HGLOBAL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryAcceptData)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lpdataobj), lpcfformat as _, reco, freally, hmetapict as _) }
    }
    pub unsafe fn ContextSensitiveHelp(&self, fentermode: windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ContextSensitiveHelp)(windows_core::Interface::as_raw(self), fentermode) }
    }
    #[cfg(all(feature = "objidl", feature = "richedit", feature = "shared"))]
    pub unsafe fn GetClipboardData(&self, lpchrg: *mut super::CHARRANGE, reco: super::DWORD, lplpdataobj: *mut Option<super::LPDATAOBJECT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClipboardData)(windows_core::Interface::as_raw(self), lpchrg as _, reco, lplpdataobj as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn GetDragDropEffect(&self, fdrag: windows_core::BOOL, grfkeystate: super::DWORD, pdweffect: super::LPDWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDragDropEffect)(windows_core::Interface::as_raw(self), fdrag, grfkeystate, pdweffect as _) }
    }
    #[cfg(all(feature = "oleidl", feature = "richedit", feature = "shared", feature = "windef"))]
    pub unsafe fn GetContextMenu(&self, seltype: super::WORD, lpoleobj: &Option<super::LPOLEOBJECT>, lpchrg: *mut super::CHARRANGE, lphmenu: *mut super::HMENU) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetContextMenu)(windows_core::Interface::as_raw(self), seltype, core::mem::transmute_copy(lpoleobj), lpchrg as _, lphmenu as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichEditOleCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "objidl")]
    pub GetNewStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    GetNewStorage: usize,
    #[cfg(all(feature = "oleidl", feature = "shared", feature = "windef"))]
    pub GetInPlaceContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, super::LPOLEINPLACEFRAMEINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oleidl", feature = "shared", feature = "windef")))]
    GetInPlaceContext: usize,
    pub ShowContainerUI: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared"))]
    pub QueryInsertObject: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCLSID, *mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "objidl", feature = "shared")))]
    QueryInsertObject: usize,
    #[cfg(feature = "oleidl")]
    pub DeleteObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oleidl"))]
    DeleteObject: usize,
    #[cfg(all(feature = "minwindef", feature = "objidl", feature = "shared", feature = "wtypes"))]
    pub QueryAcceptData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::CLIPFORMAT, super::DWORD, windows_core::BOOL, super::HGLOBAL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "objidl", feature = "shared", feature = "wtypes")))]
    QueryAcceptData: usize,
    pub ContextSensitiveHelp: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "objidl", feature = "richedit", feature = "shared"))]
    pub GetClipboardData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CHARRANGE, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "richedit", feature = "shared")))]
    GetClipboardData: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub GetDragDropEffect: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, super::DWORD, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    GetDragDropEffect: usize,
    #[cfg(all(feature = "oleidl", feature = "richedit", feature = "shared", feature = "windef"))]
    pub GetContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut core::ffi::c_void, *mut super::CHARRANGE, *mut super::HMENU) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oleidl", feature = "richedit", feature = "shared", feature = "windef")))]
    GetContextMenu: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "richedit", feature = "shared", feature = "windef", feature = "wtypes"))]
pub trait IRichEditOleCallback_Impl: windows_core::IUnknownImpl {
    fn GetNewStorage(&self, lplpstg: *mut Option<super::LPSTORAGE>) -> windows_core::Result<()>;
    fn GetInPlaceContext(&self, lplpframe: *mut Option<super::LPOLEINPLACEFRAME>, lplpdoc: *mut Option<super::LPOLEINPLACEUIWINDOW>, lpframeinfo: super::LPOLEINPLACEFRAMEINFO) -> windows_core::Result<()>;
    fn ShowContainerUI(&self, fshow: windows_core::BOOL) -> windows_core::Result<()>;
    fn QueryInsertObject(&self, lpclsid: super::LPCLSID, lpstg: Option<super::LPSTORAGE>, cp: super::LONG) -> windows_core::Result<()>;
    fn DeleteObject(&self, lpoleobj: Option<super::LPOLEOBJECT>) -> windows_core::Result<()>;
    fn QueryAcceptData(&self, lpdataobj: Option<super::LPDATAOBJECT>, lpcfformat: *mut super::CLIPFORMAT, reco: super::DWORD, freally: windows_core::BOOL, hmetapict: super::HGLOBAL) -> windows_core::Result<()>;
    fn ContextSensitiveHelp(&self, fentermode: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetClipboardData(&self, lpchrg: *mut super::CHARRANGE, reco: super::DWORD, lplpdataobj: *mut Option<super::LPDATAOBJECT>) -> windows_core::Result<()>;
    fn GetDragDropEffect(&self, fdrag: windows_core::BOOL, grfkeystate: super::DWORD, pdweffect: super::LPDWORD) -> windows_core::Result<()>;
    fn GetContextMenu(&self, seltype: super::WORD, lpoleobj: Option<super::LPOLEOBJECT>, lpchrg: *mut super::CHARRANGE, lphmenu: *mut super::HMENU) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "richedit", feature = "shared", feature = "windef", feature = "wtypes"))]
impl IRichEditOleCallback_Vtbl {
    pub const fn new<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNewStorage<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lplpstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::GetNewStorage(this, core::mem::transmute_copy(&lplpstg)).into()
            }
        }
        unsafe extern "system" fn GetInPlaceContext<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lplpframe: *mut *mut core::ffi::c_void, lplpdoc: *mut *mut core::ffi::c_void, lpframeinfo: super::LPOLEINPLACEFRAMEINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::GetInPlaceContext(this, core::mem::transmute_copy(&lplpframe), core::mem::transmute_copy(&lplpdoc), core::mem::transmute_copy(&lpframeinfo)).into()
            }
        }
        unsafe extern "system" fn ShowContainerUI<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::ShowContainerUI(this, core::mem::transmute_copy(&fshow)).into()
            }
        }
        unsafe extern "system" fn QueryInsertObject<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpclsid: super::LPCLSID, lpstg: *mut core::ffi::c_void, cp: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::QueryInsertObject(this, core::mem::transmute_copy(&lpclsid), core::mem::transmute(&lpstg), core::mem::transmute_copy(&cp)).into()
            }
        }
        unsafe extern "system" fn DeleteObject<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpoleobj: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::DeleteObject(this, core::mem::transmute(&lpoleobj)).into()
            }
        }
        unsafe extern "system" fn QueryAcceptData<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdataobj: *mut core::ffi::c_void, lpcfformat: *mut super::CLIPFORMAT, reco: super::DWORD, freally: windows_core::BOOL, hmetapict: super::HGLOBAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::QueryAcceptData(this, core::mem::transmute(&lpdataobj), core::mem::transmute_copy(&lpcfformat), core::mem::transmute_copy(&reco), core::mem::transmute_copy(&freally), core::mem::transmute_copy(&hmetapict)).into()
            }
        }
        unsafe extern "system" fn ContextSensitiveHelp<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fentermode: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::ContextSensitiveHelp(this, core::mem::transmute_copy(&fentermode)).into()
            }
        }
        unsafe extern "system" fn GetClipboardData<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpchrg: *mut super::CHARRANGE, reco: super::DWORD, lplpdataobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::GetClipboardData(this, core::mem::transmute_copy(&lpchrg), core::mem::transmute_copy(&reco), core::mem::transmute_copy(&lplpdataobj)).into()
            }
        }
        unsafe extern "system" fn GetDragDropEffect<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdrag: windows_core::BOOL, grfkeystate: super::DWORD, pdweffect: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::GetDragDropEffect(this, core::mem::transmute_copy(&fdrag), core::mem::transmute_copy(&grfkeystate), core::mem::transmute_copy(&pdweffect)).into()
            }
        }
        unsafe extern "system" fn GetContextMenu<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seltype: super::WORD, lpoleobj: *mut core::ffi::c_void, lpchrg: *mut super::CHARRANGE, lphmenu: *mut super::HMENU) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::GetContextMenu(this, core::mem::transmute_copy(&seltype), core::mem::transmute(&lpoleobj), core::mem::transmute_copy(&lpchrg), core::mem::transmute_copy(&lphmenu)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNewStorage: GetNewStorage::<Identity, OFFSET>,
            GetInPlaceContext: GetInPlaceContext::<Identity, OFFSET>,
            ShowContainerUI: ShowContainerUI::<Identity, OFFSET>,
            QueryInsertObject: QueryInsertObject::<Identity, OFFSET>,
            DeleteObject: DeleteObject::<Identity, OFFSET>,
            QueryAcceptData: QueryAcceptData::<Identity, OFFSET>,
            ContextSensitiveHelp: ContextSensitiveHelp::<Identity, OFFSET>,
            GetClipboardData: GetClipboardData::<Identity, OFFSET>,
            GetDragDropEffect: GetDragDropEffect::<Identity, OFFSET>,
            GetContextMenu: GetContextMenu::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRichEditOleCallback as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "richedit", feature = "shared", feature = "windef", feature = "wtypes"))]
impl windows_core::RuntimeName for IRichEditOleCallback {}
pub type LPRICHEDITOLE = IRichEditOle;
pub type LPRICHEDITOLECALLBACK = IRichEditOleCallback;
pub const RECO_COPY: i32 = 2;
pub const RECO_CUT: i32 = 3;
pub const RECO_DRAG: i32 = 4;
pub const RECO_DROP: i32 = 1;
pub const RECO_PASTE: i32 = 0;
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "windef"))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct REOBJECT {
    pub cbStruct: super::DWORD,
    pub cp: super::LONG,
    pub clsid: super::CLSID,
    pub poleobj: Option<super::LPOLEOBJECT>,
    pub pstg: Option<super::LPSTORAGE>,
    pub polesite: Option<super::LPOLECLIENTSITE>,
    pub sizel: super::SIZEL,
    pub dvaspect: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwUser: super::DWORD,
}
pub const REO_ALIGNTORIGHT: i32 = 256;
pub const REO_BELOWBASELINE: i32 = 2;
pub const REO_BLANK: i32 = 16;
pub const REO_CANROTATE: i32 = 128;
#[cfg(feature = "shared")]
pub const REO_CP_SELECTION: super::ULONG = 4294967295;
pub const REO_DONTNEEDPALETTE: i32 = 32;
pub const REO_DYNAMICSIZE: i32 = 8;
pub const REO_GETMETAFILE: i32 = 4194304;
pub const REO_GETOBJ_ALL_INTERFACES: i32 = 7;
pub const REO_GETOBJ_NO_INTERFACES: i32 = 0;
pub const REO_GETOBJ_POLEOBJ: i32 = 1;
pub const REO_GETOBJ_POLESITE: i32 = 4;
pub const REO_GETOBJ_PSTG: i32 = 2;
pub const REO_HILITED: i32 = 16777216;
pub const REO_INPLACEACTIVE: i32 = 33554432;
pub const REO_INVERTEDSELECT: i32 = 4;
#[cfg(feature = "shared")]
pub const REO_IOB_SELECTION: super::ULONG = 4294967295;
#[cfg(feature = "shared")]
pub const REO_IOB_USE_CP: super::ULONG = 4294967294;
pub const REO_LINK: u32 = 2147483648;
pub const REO_LINKAVAILABLE: i32 = 8388608;
pub const REO_NULL: i32 = 0;
pub const REO_OPEN: i32 = 67108864;
pub const REO_OWNERDRAWSELECT: i32 = 64;
pub const REO_READWRITEMASK: i32 = 2047;
pub const REO_RESIZABLE: i32 = 1;
pub const REO_SELECTED: i32 = 134217728;
pub const REO_STATIC: i32 = 1073741824;
pub const REO_USEASBACKGROUND: i32 = 1024;
pub const REO_WRAPTEXTAROUND: i32 = 512;
