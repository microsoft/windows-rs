windows_core::imp::define_interface!(IRichEditOle, IRichEditOle_Vtbl, 0x00020d00_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRichEditOle, windows_core::IUnknown);
impl IRichEditOle {
    #[cfg(feature = "oleidl")]
    pub unsafe fn GetClientSite(&self) -> windows_core::Result<super::IOleClientSite> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClientSite)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetObjectCount(&self) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).GetObjectCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLinkCount(&self) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).GetLinkCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "objidl", feature = "oleidl", feature = "windef"))]
    pub unsafe fn GetObject(&self, iob: i32, lpreobject: *mut REOBJECT, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), iob, lpreobject, dwflags) }
    }
    #[cfg(all(feature = "objidl", feature = "oleidl", feature = "windef"))]
    pub unsafe fn InsertObject(&self, lpreobject: *mut REOBJECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InsertObject)(windows_core::Interface::as_raw(self), lpreobject) }
    }
    pub unsafe fn ConvertObject<P2>(&self, iob: i32, rclsidnew: *const windows_core::GUID, lpstrusertypenew: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConvertObject)(windows_core::Interface::as_raw(self), iob, rclsidnew, lpstrusertypenew.param().abi()) }
    }
    pub unsafe fn ActivateAs(&self, rclsid: *const windows_core::GUID, rclsidas: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ActivateAs)(windows_core::Interface::as_raw(self), rclsid, rclsidas) }
    }
    pub unsafe fn SetHostNames<P0, P1>(&self, lpstrcontainerapp: P0, lpstrcontainerobj: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHostNames)(windows_core::Interface::as_raw(self), lpstrcontainerapp.param().abi(), lpstrcontainerobj.param().abi()) }
    }
    pub unsafe fn SetLinkAvailable(&self, iob: i32, favailable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLinkAvailable)(windows_core::Interface::as_raw(self), iob, favailable.into()) }
    }
    pub unsafe fn SetDvaspect(&self, iob: i32, dvaspect: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDvaspect)(windows_core::Interface::as_raw(self), iob, dvaspect) }
    }
    pub unsafe fn HandsOffStorage(&self, iob: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HandsOffStorage)(windows_core::Interface::as_raw(self), iob) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn SaveCompleted<P1>(&self, iob: i32, lpstg: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveCompleted)(windows_core::Interface::as_raw(self), iob, lpstg.param().abi()) }
    }
    pub unsafe fn InPlaceDeactivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InPlaceDeactivate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ContextSensitiveHelp(&self, fentermode: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ContextSensitiveHelp)(windows_core::Interface::as_raw(self), fentermode.into()) }
    }
    #[cfg(all(feature = "objidl", feature = "richedit"))]
    pub unsafe fn GetClipboardData(&self, lpchrg: *mut super::CHARRANGE, reco: u32, lplpdataobj: *mut Option<super::IDataObject>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClipboardData)(windows_core::Interface::as_raw(self), lpchrg as _, reco, core::mem::transmute(lplpdataobj)) }
    }
    #[cfg(all(feature = "minwindef", feature = "objidl", feature = "winnt", feature = "wtypes"))]
    pub unsafe fn ImportDataObject<P0>(&self, lpdataobj: P0, cf: super::CLIPFORMAT, hmetapict: super::HGLOBAL) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).ImportDataObject)(windows_core::Interface::as_raw(self), lpdataobj.param().abi(), cf, hmetapict) }
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
    pub GetObjectCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> i32,
    pub GetLinkCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> i32,
    #[cfg(all(feature = "objidl", feature = "oleidl", feature = "windef"))]
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut REOBJECT, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "oleidl", feature = "windef")))]
    GetObject: usize,
    #[cfg(all(feature = "objidl", feature = "oleidl", feature = "windef"))]
    pub InsertObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut REOBJECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "oleidl", feature = "windef")))]
    InsertObject: usize,
    pub ConvertObject: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::GUID, windows_core::PCSTR) -> windows_core::HRESULT,
    pub ActivateAs: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetHostNames: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, windows_core::PCSTR) -> windows_core::HRESULT,
    pub SetLinkAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetDvaspect: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32) -> windows_core::HRESULT,
    pub HandsOffStorage: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub SaveCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    SaveCompleted: usize,
    pub InPlaceDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContextSensitiveHelp: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "objidl", feature = "richedit"))]
    pub GetClipboardData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CHARRANGE, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "richedit")))]
    GetClipboardData: usize,
    #[cfg(all(feature = "minwindef", feature = "objidl", feature = "winnt", feature = "wtypes"))]
    pub ImportDataObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::CLIPFORMAT, super::HGLOBAL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "objidl", feature = "winnt", feature = "wtypes")))]
    ImportDataObject: usize,
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "richedit", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub trait IRichEditOle_Impl: windows_core::IUnknownImpl {
    fn GetClientSite(&self) -> windows_core::Result<super::IOleClientSite>;
    fn GetObjectCount(&self) -> i32;
    fn GetLinkCount(&self) -> i32;
    fn GetObject(&self, iob: i32, lpreobject: *mut REOBJECT, dwflags: u32) -> windows_core::Result<()>;
    fn InsertObject(&self, lpreobject: *mut REOBJECT) -> windows_core::Result<()>;
    fn ConvertObject(&self, iob: i32, rclsidnew: *const windows_core::GUID, lpstrusertypenew: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn ActivateAs(&self, rclsid: *const windows_core::GUID, rclsidas: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetHostNames(&self, lpstrcontainerapp: &windows_core::PCSTR, lpstrcontainerobj: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn SetLinkAvailable(&self, iob: i32, favailable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetDvaspect(&self, iob: i32, dvaspect: u32) -> windows_core::Result<()>;
    fn HandsOffStorage(&self, iob: i32) -> windows_core::Result<()>;
    fn SaveCompleted(&self, iob: i32, lpstg: windows_core::Ref<super::IStorage>) -> windows_core::Result<()>;
    fn InPlaceDeactivate(&self) -> windows_core::Result<()>;
    fn ContextSensitiveHelp(&self, fentermode: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetClipboardData(&self, lpchrg: *mut super::CHARRANGE, reco: u32, lplpdataobj: windows_core::OutRef<super::IDataObject>) -> windows_core::Result<()>;
    fn ImportDataObject(&self, lpdataobj: windows_core::Ref<super::IDataObject>, cf: super::CLIPFORMAT, hmetapict: super::HGLOBAL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "richedit", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl IRichEditOle_Vtbl {
    pub const fn new<Identity: IRichEditOle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClientSite<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lplpolesite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRichEditOle_Impl::GetClientSite(this) {
                    Ok(ok__) => {
                        lplpolesite.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObjectCount<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::GetObjectCount(this)
            }
        }
        unsafe extern "system" fn GetLinkCount<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::GetLinkCount(this)
            }
        }
        unsafe extern "system" fn GetObject<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: i32, lpreobject: *mut REOBJECT, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::GetObject(this, core::mem::transmute_copy(&iob), core::mem::transmute_copy(&lpreobject), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn InsertObject<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpreobject: *mut REOBJECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::InsertObject(this, core::mem::transmute_copy(&lpreobject)).into()
            }
        }
        unsafe extern "system" fn ConvertObject<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: i32, rclsidnew: *const windows_core::GUID, lpstrusertypenew: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::ConvertObject(this, core::mem::transmute_copy(&iob), core::mem::transmute_copy(&rclsidnew), core::mem::transmute(&lpstrusertypenew)).into()
            }
        }
        unsafe extern "system" fn ActivateAs<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rclsidas: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::ActivateAs(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rclsidas)).into()
            }
        }
        unsafe extern "system" fn SetHostNames<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpstrcontainerapp: windows_core::PCSTR, lpstrcontainerobj: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::SetHostNames(this, core::mem::transmute(&lpstrcontainerapp), core::mem::transmute(&lpstrcontainerobj)).into()
            }
        }
        unsafe extern "system" fn SetLinkAvailable<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: i32, favailable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::SetLinkAvailable(this, core::mem::transmute_copy(&iob), core::mem::transmute_copy(&favailable)).into()
            }
        }
        unsafe extern "system" fn SetDvaspect<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: i32, dvaspect: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::SetDvaspect(this, core::mem::transmute_copy(&iob), core::mem::transmute_copy(&dvaspect)).into()
            }
        }
        unsafe extern "system" fn HandsOffStorage<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::HandsOffStorage(this, core::mem::transmute_copy(&iob)).into()
            }
        }
        unsafe extern "system" fn SaveCompleted<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: i32, lpstg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::SaveCompleted(this, core::mem::transmute_copy(&iob), core::mem::transmute_copy(&lpstg)).into()
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
        unsafe extern "system" fn GetClipboardData<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpchrg: *mut super::CHARRANGE, reco: u32, lplpdataobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::GetClipboardData(this, core::mem::transmute_copy(&lpchrg), core::mem::transmute_copy(&reco), core::mem::transmute_copy(&lplpdataobj)).into()
            }
        }
        unsafe extern "system" fn ImportDataObject<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdataobj: *mut core::ffi::c_void, cf: super::CLIPFORMAT, hmetapict: super::HGLOBAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::ImportDataObject(this, core::mem::transmute_copy(&lpdataobj), core::mem::transmute_copy(&cf), core::mem::transmute_copy(&hmetapict)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClientSite: GetClientSite::<Identity, OFFSET>,
            GetObjectCount: GetObjectCount::<Identity, OFFSET>,
            GetLinkCount: GetLinkCount::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
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
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "richedit", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl windows_core::RuntimeName for IRichEditOle {}
windows_core::imp::define_interface!(IRichEditOleCallback, IRichEditOleCallback_Vtbl, 0x00020d03_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRichEditOleCallback, windows_core::IUnknown);
impl IRichEditOleCallback {
    #[cfg(feature = "objidl")]
    pub unsafe fn GetNewStorage(&self) -> windows_core::Result<super::IStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNewStorage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oleidl", feature = "windef"))]
    pub unsafe fn GetInPlaceContext(&self, lplpframe: *mut Option<super::IOleInPlaceFrame>, lplpdoc: *mut Option<super::IOleInPlaceUIWindow>, lpframeinfo: *mut super::OLEINPLACEFRAMEINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInPlaceContext)(windows_core::Interface::as_raw(self), core::mem::transmute(lplpframe), core::mem::transmute(lplpdoc), lpframeinfo as _) }
    }
    pub unsafe fn ShowContainerUI(&self, fshow: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShowContainerUI)(windows_core::Interface::as_raw(self), fshow.into()) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn QueryInsertObject<P1>(&self, lpclsid: *mut windows_core::GUID, lpstg: P1, cp: i32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryInsertObject)(windows_core::Interface::as_raw(self), lpclsid as _, lpstg.param().abi(), cp) }
    }
    #[cfg(feature = "oleidl")]
    pub unsafe fn DeleteObject<P0>(&self, lpoleobj: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IOleObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteObject)(windows_core::Interface::as_raw(self), lpoleobj.param().abi()) }
    }
    #[cfg(all(feature = "minwindef", feature = "objidl", feature = "winnt", feature = "wtypes"))]
    pub unsafe fn QueryAcceptData<P0>(&self, lpdataobj: P0, lpcfformat: *mut super::CLIPFORMAT, reco: u32, freally: bool, hmetapict: super::HGLOBAL) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryAcceptData)(windows_core::Interface::as_raw(self), lpdataobj.param().abi(), lpcfformat as _, reco, freally.into(), hmetapict) }
    }
    pub unsafe fn ContextSensitiveHelp(&self, fentermode: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ContextSensitiveHelp)(windows_core::Interface::as_raw(self), fentermode.into()) }
    }
    #[cfg(all(feature = "objidl", feature = "richedit"))]
    pub unsafe fn GetClipboardData(&self, lpchrg: *mut super::CHARRANGE, reco: u32, lplpdataobj: *mut Option<super::IDataObject>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClipboardData)(windows_core::Interface::as_raw(self), lpchrg as _, reco, core::mem::transmute(lplpdataobj)) }
    }
    pub unsafe fn GetDragDropEffect(&self, fdrag: bool, grfkeystate: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDragDropEffect)(windows_core::Interface::as_raw(self), fdrag.into(), grfkeystate, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oleidl", feature = "richedit", feature = "windef"))]
    pub unsafe fn GetContextMenu<P1>(&self, seltype: u16, lpoleobj: P1, lpchrg: *mut super::CHARRANGE, lphmenu: *mut super::HMENU) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IOleObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetContextMenu)(windows_core::Interface::as_raw(self), seltype, lpoleobj.param().abi(), lpchrg as _, lphmenu as _) }
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
    #[cfg(all(feature = "oleidl", feature = "windef"))]
    pub GetInPlaceContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::OLEINPLACEFRAMEINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oleidl", feature = "windef")))]
    GetInPlaceContext: usize,
    pub ShowContainerUI: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub QueryInsertObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    QueryInsertObject: usize,
    #[cfg(feature = "oleidl")]
    pub DeleteObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oleidl"))]
    DeleteObject: usize,
    #[cfg(all(feature = "minwindef", feature = "objidl", feature = "winnt", feature = "wtypes"))]
    pub QueryAcceptData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::CLIPFORMAT, u32, windows_core::BOOL, super::HGLOBAL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "objidl", feature = "winnt", feature = "wtypes")))]
    QueryAcceptData: usize,
    pub ContextSensitiveHelp: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "objidl", feature = "richedit"))]
    pub GetClipboardData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CHARRANGE, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "richedit")))]
    GetClipboardData: usize,
    pub GetDragDropEffect: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "oleidl", feature = "richedit", feature = "windef"))]
    pub GetContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut core::ffi::c_void, *mut super::CHARRANGE, *mut super::HMENU) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oleidl", feature = "richedit", feature = "windef")))]
    GetContextMenu: usize,
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "richedit", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub trait IRichEditOleCallback_Impl: windows_core::IUnknownImpl {
    fn GetNewStorage(&self) -> windows_core::Result<super::IStorage>;
    fn GetInPlaceContext(&self, lplpframe: windows_core::OutRef<super::IOleInPlaceFrame>, lplpdoc: windows_core::OutRef<super::IOleInPlaceUIWindow>, lpframeinfo: *mut super::OLEINPLACEFRAMEINFO) -> windows_core::Result<()>;
    fn ShowContainerUI(&self, fshow: windows_core::BOOL) -> windows_core::Result<()>;
    fn QueryInsertObject(&self, lpclsid: *mut windows_core::GUID, lpstg: windows_core::Ref<super::IStorage>, cp: i32) -> windows_core::Result<()>;
    fn DeleteObject(&self, lpoleobj: windows_core::Ref<super::IOleObject>) -> windows_core::Result<()>;
    fn QueryAcceptData(&self, lpdataobj: windows_core::Ref<super::IDataObject>, lpcfformat: *mut super::CLIPFORMAT, reco: u32, freally: windows_core::BOOL, hmetapict: super::HGLOBAL) -> windows_core::Result<()>;
    fn ContextSensitiveHelp(&self, fentermode: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetClipboardData(&self, lpchrg: *mut super::CHARRANGE, reco: u32, lplpdataobj: windows_core::OutRef<super::IDataObject>) -> windows_core::Result<()>;
    fn GetDragDropEffect(&self, fdrag: windows_core::BOOL, grfkeystate: u32) -> windows_core::Result<u32>;
    fn GetContextMenu(&self, seltype: u16, lpoleobj: windows_core::Ref<super::IOleObject>, lpchrg: *mut super::CHARRANGE, lphmenu: *mut super::HMENU) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "richedit", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl IRichEditOleCallback_Vtbl {
    pub const fn new<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNewStorage<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lplpstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRichEditOleCallback_Impl::GetNewStorage(this) {
                    Ok(ok__) => {
                        lplpstg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInPlaceContext<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lplpframe: *mut *mut core::ffi::c_void, lplpdoc: *mut *mut core::ffi::c_void, lpframeinfo: *mut super::OLEINPLACEFRAMEINFO) -> windows_core::HRESULT {
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
        unsafe extern "system" fn QueryInsertObject<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpclsid: *mut windows_core::GUID, lpstg: *mut core::ffi::c_void, cp: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::QueryInsertObject(this, core::mem::transmute_copy(&lpclsid), core::mem::transmute_copy(&lpstg), core::mem::transmute_copy(&cp)).into()
            }
        }
        unsafe extern "system" fn DeleteObject<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpoleobj: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::DeleteObject(this, core::mem::transmute_copy(&lpoleobj)).into()
            }
        }
        unsafe extern "system" fn QueryAcceptData<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdataobj: *mut core::ffi::c_void, lpcfformat: *mut super::CLIPFORMAT, reco: u32, freally: windows_core::BOOL, hmetapict: super::HGLOBAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::QueryAcceptData(this, core::mem::transmute_copy(&lpdataobj), core::mem::transmute_copy(&lpcfformat), core::mem::transmute_copy(&reco), core::mem::transmute_copy(&freally), core::mem::transmute_copy(&hmetapict)).into()
            }
        }
        unsafe extern "system" fn ContextSensitiveHelp<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fentermode: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::ContextSensitiveHelp(this, core::mem::transmute_copy(&fentermode)).into()
            }
        }
        unsafe extern "system" fn GetClipboardData<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpchrg: *mut super::CHARRANGE, reco: u32, lplpdataobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::GetClipboardData(this, core::mem::transmute_copy(&lpchrg), core::mem::transmute_copy(&reco), core::mem::transmute_copy(&lplpdataobj)).into()
            }
        }
        unsafe extern "system" fn GetDragDropEffect<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdrag: windows_core::BOOL, grfkeystate: u32, pdweffect: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRichEditOleCallback_Impl::GetDragDropEffect(this, core::mem::transmute_copy(&fdrag), core::mem::transmute_copy(&grfkeystate)) {
                    Ok(ok__) => {
                        pdweffect.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContextMenu<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seltype: u16, lpoleobj: *mut core::ffi::c_void, lpchrg: *mut super::CHARRANGE, lphmenu: *mut super::HMENU) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::GetContextMenu(this, core::mem::transmute_copy(&seltype), core::mem::transmute_copy(&lpoleobj), core::mem::transmute_copy(&lpchrg), core::mem::transmute_copy(&lphmenu)).into()
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
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "richedit", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl windows_core::RuntimeName for IRichEditOleCallback {}
pub const RECO_COPY: u32 = 2;
pub const RECO_CUT: u32 = 3;
pub const RECO_DRAG: u32 = 4;
pub const RECO_DROP: u32 = 1;
pub const RECO_PASTE: u32 = 0;
#[repr(C)]
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "windef"))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct REOBJECT {
    pub cbStruct: u32,
    pub cp: i32,
    pub clsid: windows_core::GUID,
    pub poleobj: core::mem::ManuallyDrop<Option<super::IOleObject>>,
    pub pstg: core::mem::ManuallyDrop<Option<super::IStorage>>,
    pub polesite: core::mem::ManuallyDrop<Option<super::IOleClientSite>>,
    pub sizel: super::SIZEL,
    pub dvaspect: u32,
    pub dwFlags: u32,
    pub dwUser: u32,
}
pub const REO_ALIGNTORIGHT: u32 = 256;
pub const REO_BELOWBASELINE: u32 = 2;
pub const REO_BLANK: u32 = 16;
pub const REO_CANROTATE: u32 = 128;
pub const REO_CP_SELECTION: u32 = 4294967295;
pub const REO_DONTNEEDPALETTE: u32 = 32;
pub const REO_DYNAMICSIZE: u32 = 8;
pub const REO_GETMETAFILE: u32 = 4194304;
pub const REO_GETOBJ_ALL_INTERFACES: u32 = 7;
pub const REO_GETOBJ_NO_INTERFACES: u32 = 0;
pub const REO_GETOBJ_POLEOBJ: u32 = 1;
pub const REO_GETOBJ_POLESITE: u32 = 4;
pub const REO_GETOBJ_PSTG: u32 = 2;
pub const REO_HILITED: u32 = 16777216;
pub const REO_INPLACEACTIVE: u32 = 33554432;
pub const REO_INVERTEDSELECT: u32 = 4;
pub const REO_IOB_SELECTION: u32 = 4294967295;
pub const REO_IOB_USE_CP: u32 = 4294967294;
pub const REO_LINK: u32 = 2147483648;
pub const REO_LINKAVAILABLE: u32 = 8388608;
pub const REO_NULL: u32 = 0;
pub const REO_OPEN: u32 = 67108864;
pub const REO_OWNERDRAWSELECT: u32 = 64;
pub const REO_READWRITEMASK: u32 = 2047;
pub const REO_RESIZABLE: u32 = 1;
pub const REO_SELECTED: u32 = 134217728;
pub const REO_STATIC: u32 = 1073741824;
pub const REO_USEASBACKGROUND: u32 = 1024;
pub const REO_WRAPTEXTAROUND: u32 = 512;
