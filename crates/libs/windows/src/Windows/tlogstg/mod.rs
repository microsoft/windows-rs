windows_core::imp::define_interface!(IEnumTravelLogEntry, IEnumTravelLogEntry_Vtbl, 0x7ebfdd85_ad18_11d3_a4c5_00c04f72d6b8);
windows_core::imp::interface_hierarchy!(IEnumTravelLogEntry, windows_core::IUnknown);
impl IEnumTravelLogEntry {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut Option<ITravelLogEntry>, pceltfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
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
pub struct IEnumTravelLogEntry_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumTravelLogEntry_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: windows_core::OutRef<ITravelLogEntry>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumTravelLogEntry>;
}
impl IEnumTravelLogEntry_Vtbl {
    pub const fn new<Identity: IEnumTravelLogEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumTravelLogEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTravelLogEntry_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumTravelLogEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTravelLogEntry_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumTravelLogEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTravelLogEntry_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumTravelLogEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumTravelLogEntry_Impl::Clone(this) {
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
        iid == &<IEnumTravelLogEntry as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTravelLogEntry {}
windows_core::imp::define_interface!(ITravelLogClient, ITravelLogClient_Vtbl, 0x241c033e_e659_43da_aa4d_4086dbc4758d);
windows_core::imp::interface_hierarchy!(ITravelLogClient, windows_core::IUnknown);
impl ITravelLogClient {
    pub unsafe fn FindWindowByIndex(&self, dwid: u32) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindWindowByIndex)(windows_core::Interface::as_raw(self), dwid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "objidlbase", feature = "shtypes"))]
    pub unsafe fn GetWindowData<P0>(&self, pstream: P0, pwindata: *mut WINDOWDATA) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetWindowData)(windows_core::Interface::as_raw(self), pstream.param().abi(), pwindata as _) }
    }
    pub unsafe fn LoadHistoryPosition<P0>(&self, pszurllocation: P0, dwposition: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadHistoryPosition)(windows_core::Interface::as_raw(self), pszurllocation.param().abi(), dwposition) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITravelLogClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindWindowByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "objidlbase", feature = "shtypes"))]
    pub GetWindowData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut WINDOWDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "shtypes")))]
    GetWindowData: usize,
    pub LoadHistoryPosition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidlbase", feature = "shtypes"))]
pub trait ITravelLogClient_Impl: windows_core::IUnknownImpl {
    fn FindWindowByIndex(&self, dwid: u32) -> windows_core::Result<windows_core::IUnknown>;
    fn GetWindowData(&self, pstream: windows_core::Ref<super::objidlbase::IStream>, pwindata: *mut WINDOWDATA) -> windows_core::Result<()>;
    fn LoadHistoryPosition(&self, pszurllocation: &windows_core::PCWSTR, dwposition: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidlbase", feature = "shtypes"))]
impl ITravelLogClient_Vtbl {
    pub const fn new<Identity: ITravelLogClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindWindowByIndex<Identity: ITravelLogClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwid: u32, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITravelLogClient_Impl::FindWindowByIndex(this, core::mem::transmute_copy(&dwid)) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWindowData<Identity: ITravelLogClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, pwindata: *mut WINDOWDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITravelLogClient_Impl::GetWindowData(this, core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&pwindata)).into()
            }
        }
        unsafe extern "system" fn LoadHistoryPosition<Identity: ITravelLogClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurllocation: windows_core::PCWSTR, dwposition: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITravelLogClient_Impl::LoadHistoryPosition(this, core::mem::transmute(&pszurllocation), core::mem::transmute_copy(&dwposition)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindWindowByIndex: FindWindowByIndex::<Identity, OFFSET>,
            GetWindowData: GetWindowData::<Identity, OFFSET>,
            LoadHistoryPosition: LoadHistoryPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITravelLogClient as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidlbase", feature = "shtypes"))]
impl windows_core::RuntimeName for ITravelLogClient {}
windows_core::imp::define_interface!(ITravelLogEntry, ITravelLogEntry_Vtbl, 0x7ebfdd87_ad18_11d3_a4c5_00c04f72d6b8);
windows_core::imp::interface_hierarchy!(ITravelLogEntry, windows_core::IUnknown);
impl ITravelLogEntry {
    pub unsafe fn GetTitle(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTitle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetURL(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITravelLogEntry_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait ITravelLogEntry_Impl: windows_core::IUnknownImpl {
    fn GetTitle(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetURL(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl ITravelLogEntry_Vtbl {
    pub const fn new<Identity: ITravelLogEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTitle<Identity: ITravelLogEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsztitle: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITravelLogEntry_Impl::GetTitle(this) {
                    Ok(ok__) => {
                        ppsztitle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetURL<Identity: ITravelLogEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszurl: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITravelLogEntry_Impl::GetURL(this) {
                    Ok(ok__) => {
                        ppszurl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetTitle: GetTitle::<Identity, OFFSET>, GetURL: GetURL::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITravelLogEntry as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITravelLogEntry {}
windows_core::imp::define_interface!(ITravelLogStg, ITravelLogStg_Vtbl, 0x7ebfdd80_ad18_11d3_a4c5_00c04f72d6b8);
windows_core::imp::interface_hierarchy!(ITravelLogStg, windows_core::IUnknown);
impl ITravelLogStg {
    pub unsafe fn CreateEntry<P0, P1, P2>(&self, pszurl: P0, psztitle: P1, ptlerelativeto: P2, fprepend: bool) -> windows_core::Result<ITravelLogEntry>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<ITravelLogEntry>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEntry)(windows_core::Interface::as_raw(self), pszurl.param().abi(), psztitle.param().abi(), ptlerelativeto.param().abi(), fprepend.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn TravelTo<P0>(&self, ptle: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITravelLogEntry>,
    {
        unsafe { (windows_core::Interface::vtable(self).TravelTo)(windows_core::Interface::as_raw(self), ptle.param().abi()) }
    }
    pub unsafe fn EnumEntries(&self, flags: TLENUMF) -> windows_core::Result<IEnumTravelLogEntry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumEntries)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindEntries<P1>(&self, flags: TLENUMF, pszurl: P1) -> windows_core::Result<IEnumTravelLogEntry>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindEntries)(windows_core::Interface::as_raw(self), flags, pszurl.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self, flags: TLENUMF) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), flags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveEntry<P0>(&self, ptle: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITravelLogEntry>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveEntry)(windows_core::Interface::as_raw(self), ptle.param().abi()) }
    }
    pub unsafe fn GetRelativeEntry(&self, ioffset: i32) -> windows_core::Result<ITravelLogEntry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRelativeEntry)(windows_core::Interface::as_raw(self), ioffset, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITravelLogStg_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateEntry: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TravelTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumEntries: unsafe extern "system" fn(*mut core::ffi::c_void, TLENUMF, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindEntries: unsafe extern "system" fn(*mut core::ffi::c_void, TLENUMF, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, TLENUMF, *mut u32) -> windows_core::HRESULT,
    pub RemoveEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRelativeEntry: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITravelLogStg_Impl: windows_core::IUnknownImpl {
    fn CreateEntry(&self, pszurl: &windows_core::PCWSTR, psztitle: &windows_core::PCWSTR, ptlerelativeto: windows_core::Ref<ITravelLogEntry>, fprepend: windows_core::BOOL) -> windows_core::Result<ITravelLogEntry>;
    fn TravelTo(&self, ptle: windows_core::Ref<ITravelLogEntry>) -> windows_core::Result<()>;
    fn EnumEntries(&self, flags: TLENUMF) -> windows_core::Result<IEnumTravelLogEntry>;
    fn FindEntries(&self, flags: TLENUMF, pszurl: &windows_core::PCWSTR) -> windows_core::Result<IEnumTravelLogEntry>;
    fn GetCount(&self, flags: TLENUMF) -> windows_core::Result<u32>;
    fn RemoveEntry(&self, ptle: windows_core::Ref<ITravelLogEntry>) -> windows_core::Result<()>;
    fn GetRelativeEntry(&self, ioffset: i32) -> windows_core::Result<ITravelLogEntry>;
}
impl ITravelLogStg_Vtbl {
    pub const fn new<Identity: ITravelLogStg_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateEntry<Identity: ITravelLogStg_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, psztitle: windows_core::PCWSTR, ptlerelativeto: *mut core::ffi::c_void, fprepend: windows_core::BOOL, pptle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITravelLogStg_Impl::CreateEntry(this, core::mem::transmute(&pszurl), core::mem::transmute(&psztitle), core::mem::transmute_copy(&ptlerelativeto), core::mem::transmute_copy(&fprepend)) {
                    Ok(ok__) => {
                        pptle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TravelTo<Identity: ITravelLogStg_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptle: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITravelLogStg_Impl::TravelTo(this, core::mem::transmute_copy(&ptle)).into()
            }
        }
        unsafe extern "system" fn EnumEntries<Identity: ITravelLogStg_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: TLENUMF, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITravelLogStg_Impl::EnumEntries(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindEntries<Identity: ITravelLogStg_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: TLENUMF, pszurl: windows_core::PCWSTR, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITravelLogStg_Impl::FindEntries(this, core::mem::transmute_copy(&flags), core::mem::transmute(&pszurl)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: ITravelLogStg_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: TLENUMF, pcentries: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITravelLogStg_Impl::GetCount(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pcentries.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveEntry<Identity: ITravelLogStg_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptle: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITravelLogStg_Impl::RemoveEntry(this, core::mem::transmute_copy(&ptle)).into()
            }
        }
        unsafe extern "system" fn GetRelativeEntry<Identity: ITravelLogStg_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ioffset: i32, ptle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITravelLogStg_Impl::GetRelativeEntry(this, core::mem::transmute_copy(&ioffset)) {
                    Ok(ok__) => {
                        ptle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateEntry: CreateEntry::<Identity, OFFSET>,
            TravelTo: TravelTo::<Identity, OFFSET>,
            EnumEntries: EnumEntries::<Identity, OFFSET>,
            FindEntries: FindEntries::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            RemoveEntry: RemoveEntry::<Identity, OFFSET>,
            GetRelativeEntry: GetRelativeEntry::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITravelLogStg as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITravelLogStg {}
#[cfg(feature = "shtypes")]
pub type LPCWINDOWDATA = *const WINDOWDATA;
#[cfg(feature = "shtypes")]
pub type LPWINDOWDATA = *mut WINDOWDATA;
pub const TLEF_ABSOLUTE: tagTLENUMF = 49;
pub const TLEF_EXCLUDE_ABOUT_PAGES: tagTLENUMF = 256;
pub const TLEF_EXCLUDE_SUBFRAME_ENTRIES: tagTLENUMF = 128;
pub const TLEF_INCLUDE_UNINVOKEABLE: tagTLENUMF = 64;
pub const TLEF_RELATIVE_BACK: tagTLENUMF = 16;
pub const TLEF_RELATIVE_FORE: tagTLENUMF = 32;
pub const TLEF_RELATIVE_INCLUDE_CURRENT: tagTLENUMF = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TLENUMF(pub u32);
#[repr(C)]
#[cfg(feature = "shtypes")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINDOWDATA {
    pub dwWindowID: u32,
    pub uiCP: u32,
    pub pidl: super::shtypes::LPITEMIDLIST,
    pub lpszUrl: windows_core::PWSTR,
    pub lpszUrlLocation: windows_core::PWSTR,
    pub lpszTitle: windows_core::PWSTR,
}
pub type tagTLENUMF = i32;
