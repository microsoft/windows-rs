pub trait IITDatabase_Impl: Sized + windows_core::IUnknownImpl {
    fn Open(&self, lpszhost: &windows_core::PCWSTR, lpszmoniker: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn CreateObject(&self, rclsid: *const windows_core::GUID, pdwobjinstance: *mut u32) -> windows_core::Result<()>;
    fn GetObject(&self, dwobjinstance: u32, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetObjectPersistence(&self, lpwszobject: &windows_core::PCWSTR, dwobjinstance: u32, ppvpersistence: *mut *mut core::ffi::c_void, fstream: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IITDatabase {}
impl IITDatabase_Vtbl {
    pub const fn new<Identity: IITDatabase_Impl, const OFFSET: isize>() -> IITDatabase_Vtbl {
        unsafe extern "system" fn Open<Identity: IITDatabase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszhost: windows_core::PCWSTR, lpszmoniker: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITDatabase_Impl::Open(this, core::mem::transmute(&lpszhost), core::mem::transmute(&lpszmoniker), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Close<Identity: IITDatabase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITDatabase_Impl::Close(this).into()
        }
        unsafe extern "system" fn CreateObject<Identity: IITDatabase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, pdwobjinstance: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITDatabase_Impl::CreateObject(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&pdwobjinstance)).into()
        }
        unsafe extern "system" fn GetObject<Identity: IITDatabase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwobjinstance: u32, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITDatabase_Impl::GetObject(this, core::mem::transmute_copy(&dwobjinstance), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
        }
        unsafe extern "system" fn GetObjectPersistence<Identity: IITDatabase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpwszobject: windows_core::PCWSTR, dwobjinstance: u32, ppvpersistence: *mut *mut core::ffi::c_void, fstream: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITDatabase_Impl::GetObjectPersistence(this, core::mem::transmute(&lpwszobject), core::mem::transmute_copy(&dwobjinstance), core::mem::transmute_copy(&ppvpersistence), core::mem::transmute_copy(&fstream)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            CreateObject: CreateObject::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            GetObjectPersistence: GetObjectPersistence::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IITDatabase as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IITPropList_Impl: Sized + super::super::System::Com::IPersistStreamInit_Impl {
    fn Set(&self, propid: u32, lpszwstring: &windows_core::PCWSTR, dwoperation: u32) -> windows_core::Result<()>;
    fn Set2(&self, propid: u32, lpvdata: *mut core::ffi::c_void, cbdata: u32, dwoperation: u32) -> windows_core::Result<()>;
    fn Set3(&self, propid: u32, dwdata: u32, dwoperation: u32) -> windows_core::Result<()>;
    fn Add(&self, prop: *mut CProperty) -> windows_core::Result<()>;
    fn Get(&self, propid: u32, property: *mut CProperty) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn SetPersist(&self, fpersist: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetPersist2(&self, propid: u32, fpersist: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetFirst(&self, property: *mut CProperty) -> windows_core::Result<()>;
    fn GetNext(&self, property: *mut CProperty) -> windows_core::Result<()>;
    fn GetPropCount(&self, cprop: *mut i32) -> windows_core::Result<()>;
    fn SaveHeader(&self, lpvdata: *mut core::ffi::c_void, dwhdrsize: u32) -> windows_core::Result<()>;
    fn SaveData(&self, lpvheader: *mut core::ffi::c_void, dwhdrsize: u32, lpvdata: *mut core::ffi::c_void, dwbufsize: u32) -> windows_core::Result<()>;
    fn GetHeaderSize(&self, dwhdrsize: *mut u32) -> windows_core::Result<()>;
    fn GetDataSize(&self, lpvheader: *mut core::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> windows_core::Result<()>;
    fn SaveDataToStream(&self, lpvheader: *mut core::ffi::c_void, dwhdrsize: u32, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn LoadFromMem(&self, lpvdata: *mut core::ffi::c_void, dwbufsize: u32) -> windows_core::Result<()>;
    fn SaveToMem(&self, lpvdata: *mut core::ffi::c_void, dwbufsize: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IITPropList {}
#[cfg(feature = "Win32_System_Com")]
impl IITPropList_Vtbl {
    pub const fn new<Identity: IITPropList_Impl, const OFFSET: isize>() -> IITPropList_Vtbl {
        unsafe extern "system" fn Set<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, lpszwstring: windows_core::PCWSTR, dwoperation: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITPropList_Impl::Set(this, core::mem::transmute_copy(&propid), core::mem::transmute(&lpszwstring), core::mem::transmute_copy(&dwoperation)).into()
        }
        unsafe extern "system" fn Set2<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, lpvdata: *mut core::ffi::c_void, cbdata: u32, dwoperation: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITPropList_Impl::Set2(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&lpvdata), core::mem::transmute_copy(&cbdata), core::mem::transmute_copy(&dwoperation)).into()
        }
        unsafe extern "system" fn Set3<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, dwdata: u32, dwoperation: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITPropList_Impl::Set3(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&dwdata), core::mem::transmute_copy(&dwoperation)).into()
        }
        unsafe extern "system" fn Add<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prop: *mut CProperty) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITPropList_Impl::Add(this, core::mem::transmute_copy(&prop)).into()
        }
        unsafe extern "system" fn Get<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, property: *mut CProperty) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITPropList_Impl::Get(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&property)).into()
        }
        unsafe extern "system" fn Clear<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITPropList_Impl::Clear(this).into()
        }
        unsafe extern "system" fn SetPersist<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fpersist: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITPropList_Impl::SetPersist(this, core::mem::transmute_copy(&fpersist)).into()
        }
        unsafe extern "system" fn SetPersist2<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, fpersist: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITPropList_Impl::SetPersist2(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&fpersist)).into()
        }
        unsafe extern "system" fn GetFirst<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *mut CProperty) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITPropList_Impl::GetFirst(this, core::mem::transmute_copy(&property)).into()
        }
        unsafe extern "system" fn GetNext<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *mut CProperty) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITPropList_Impl::GetNext(this, core::mem::transmute_copy(&property)).into()
        }
        unsafe extern "system" fn GetPropCount<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cprop: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITPropList_Impl::GetPropCount(this, core::mem::transmute_copy(&cprop)).into()
        }
        unsafe extern "system" fn SaveHeader<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvdata: *mut core::ffi::c_void, dwhdrsize: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITPropList_Impl::SaveHeader(this, core::mem::transmute_copy(&lpvdata), core::mem::transmute_copy(&dwhdrsize)).into()
        }
        unsafe extern "system" fn SaveData<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvheader: *mut core::ffi::c_void, dwhdrsize: u32, lpvdata: *mut core::ffi::c_void, dwbufsize: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITPropList_Impl::SaveData(this, core::mem::transmute_copy(&lpvheader), core::mem::transmute_copy(&dwhdrsize), core::mem::transmute_copy(&lpvdata), core::mem::transmute_copy(&dwbufsize)).into()
        }
        unsafe extern "system" fn GetHeaderSize<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhdrsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITPropList_Impl::GetHeaderSize(this, core::mem::transmute_copy(&dwhdrsize)).into()
        }
        unsafe extern "system" fn GetDataSize<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvheader: *mut core::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITPropList_Impl::GetDataSize(this, core::mem::transmute_copy(&lpvheader), core::mem::transmute_copy(&dwhdrsize), core::mem::transmute_copy(&dwdatasize)).into()
        }
        unsafe extern "system" fn SaveDataToStream<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvheader: *mut core::ffi::c_void, dwhdrsize: u32, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITPropList_Impl::SaveDataToStream(this, core::mem::transmute_copy(&lpvheader), core::mem::transmute_copy(&dwhdrsize), windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn LoadFromMem<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvdata: *mut core::ffi::c_void, dwbufsize: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITPropList_Impl::LoadFromMem(this, core::mem::transmute_copy(&lpvdata), core::mem::transmute_copy(&dwbufsize)).into()
        }
        unsafe extern "system" fn SaveToMem<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvdata: *mut core::ffi::c_void, dwbufsize: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITPropList_Impl::SaveToMem(this, core::mem::transmute_copy(&lpvdata), core::mem::transmute_copy(&dwbufsize)).into()
        }
        Self {
            base__: super::super::System::Com::IPersistStreamInit_Vtbl::new::<Identity, OFFSET>(),
            Set: Set::<Identity, OFFSET>,
            Set2: Set2::<Identity, OFFSET>,
            Set3: Set3::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            SetPersist: SetPersist::<Identity, OFFSET>,
            SetPersist2: SetPersist2::<Identity, OFFSET>,
            GetFirst: GetFirst::<Identity, OFFSET>,
            GetNext: GetNext::<Identity, OFFSET>,
            GetPropCount: GetPropCount::<Identity, OFFSET>,
            SaveHeader: SaveHeader::<Identity, OFFSET>,
            SaveData: SaveData::<Identity, OFFSET>,
            GetHeaderSize: GetHeaderSize::<Identity, OFFSET>,
            GetDataSize: GetDataSize::<Identity, OFFSET>,
            SaveDataToStream: SaveDataToStream::<Identity, OFFSET>,
            LoadFromMem: LoadFromMem::<Identity, OFFSET>,
            SaveToMem: SaveToMem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IITPropList as windows_core::Interface>::IID || iid == &<super::super::System::Com::IPersist as windows_core::Interface>::IID || iid == &<super::super::System::Com::IPersistStreamInit as windows_core::Interface>::IID
    }
}
pub trait IITResultSet_Impl: Sized + windows_core::IUnknownImpl {
    fn SetColumnPriority(&self, lcolumnindex: i32, columnpriority: PRIORITY) -> windows_core::Result<()>;
    fn SetColumnHeap(&self, lcolumnindex: i32, lpvheap: *mut core::ffi::c_void, pfncolheapfree: PFNCOLHEAPFREE) -> windows_core::Result<()>;
    fn SetKeyProp(&self, propid: u32) -> windows_core::Result<()>;
    fn Add(&self, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> windows_core::Result<()>;
    fn Add2(&self, propid: u32, lpszwdefault: &windows_core::PCWSTR, priority: PRIORITY) -> windows_core::Result<()>;
    fn Add3(&self, propid: u32, lpvdefaultdata: *mut core::ffi::c_void, cbdata: u32, priority: PRIORITY) -> windows_core::Result<()>;
    fn Add4(&self, lpvhdr: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Append(&self, lpvhdr: *mut core::ffi::c_void, lpvdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Set(&self, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut core::ffi::c_void, cbdata: u32) -> windows_core::Result<()>;
    fn Set2(&self, lrowindex: i32, lcolumnindex: i32, lpwstr: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Set3(&self, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> windows_core::Result<()>;
    fn Set4(&self, lrowindex: i32, lpvhdr: *mut core::ffi::c_void, lpvdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Copy(&self, prscopy: Option<&IITResultSet>) -> windows_core::Result<()>;
    fn AppendRows(&self, pressrc: Option<&IITResultSet>, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> windows_core::Result<()>;
    fn Get(&self, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> windows_core::Result<()>;
    fn GetKeyProp(&self, keypropid: *mut u32) -> windows_core::Result<()>;
    fn GetColumnPriority(&self, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> windows_core::Result<()>;
    fn GetRowCount(&self, lnumberofrows: *mut i32) -> windows_core::Result<()>;
    fn GetColumnCount(&self, lnumberofcolumns: *mut i32) -> windows_core::Result<()>;
    fn GetColumn(&self, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut core::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> windows_core::Result<()>;
    fn GetColumn2(&self, lcolumnindex: i32, propid: *mut u32) -> windows_core::Result<()>;
    fn GetColumnFromPropID(&self, propid: u32, lcolumnindex: *mut i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn ClearRows(&self) -> windows_core::Result<()>;
    fn Free(&self) -> windows_core::Result<()>;
    fn IsCompleted(&self) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Pause(&self, fpause: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetRowStatus(&self, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> windows_core::Result<()>;
    fn GetColumnStatus(&self, lpcolstatus: *mut COLUMNSTATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IITResultSet {}
impl IITResultSet_Vtbl {
    pub const fn new<Identity: IITResultSet_Impl, const OFFSET: isize>() -> IITResultSet_Vtbl {
        unsafe extern "system" fn SetColumnPriority<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcolumnindex: i32, columnpriority: PRIORITY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::SetColumnPriority(this, core::mem::transmute_copy(&lcolumnindex), core::mem::transmute_copy(&columnpriority)).into()
        }
        unsafe extern "system" fn SetColumnHeap<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcolumnindex: i32, lpvheap: *mut core::ffi::c_void, pfncolheapfree: PFNCOLHEAPFREE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::SetColumnHeap(this, core::mem::transmute_copy(&lcolumnindex), core::mem::transmute_copy(&lpvheap), core::mem::transmute_copy(&pfncolheapfree)).into()
        }
        unsafe extern "system" fn SetKeyProp<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::SetKeyProp(this, core::mem::transmute_copy(&propid)).into()
        }
        unsafe extern "system" fn Add<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::Add(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&dwdefaultdata), core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn Add2<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, lpszwdefault: windows_core::PCWSTR, priority: PRIORITY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::Add2(this, core::mem::transmute_copy(&propid), core::mem::transmute(&lpszwdefault), core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn Add3<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, lpvdefaultdata: *mut core::ffi::c_void, cbdata: u32, priority: PRIORITY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::Add3(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&lpvdefaultdata), core::mem::transmute_copy(&cbdata), core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn Add4<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvhdr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::Add4(this, core::mem::transmute_copy(&lpvhdr)).into()
        }
        unsafe extern "system" fn Append<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvhdr: *mut core::ffi::c_void, lpvdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::Append(this, core::mem::transmute_copy(&lpvhdr), core::mem::transmute_copy(&lpvdata)).into()
        }
        unsafe extern "system" fn Set<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut core::ffi::c_void, cbdata: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::Set(this, core::mem::transmute_copy(&lrowindex), core::mem::transmute_copy(&lcolumnindex), core::mem::transmute_copy(&lpvdata), core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn Set2<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, lpwstr: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::Set2(this, core::mem::transmute_copy(&lrowindex), core::mem::transmute_copy(&lcolumnindex), core::mem::transmute(&lpwstr)).into()
        }
        unsafe extern "system" fn Set3<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::Set3(this, core::mem::transmute_copy(&lrowindex), core::mem::transmute_copy(&lcolumnindex), core::mem::transmute_copy(&dwdata)).into()
        }
        unsafe extern "system" fn Set4<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrowindex: i32, lpvhdr: *mut core::ffi::c_void, lpvdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::Set4(this, core::mem::transmute_copy(&lrowindex), core::mem::transmute_copy(&lpvhdr), core::mem::transmute_copy(&lpvdata)).into()
        }
        unsafe extern "system" fn Copy<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prscopy: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::Copy(this, windows_core::from_raw_borrowed(&prscopy)).into()
        }
        unsafe extern "system" fn AppendRows<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pressrc: *mut core::ffi::c_void, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::AppendRows(this, windows_core::from_raw_borrowed(&pressrc), core::mem::transmute_copy(&lrowsrcfirst), core::mem::transmute_copy(&csrcrows), core::mem::transmute_copy(&lrowfirstdest)).into()
        }
        unsafe extern "system" fn Get<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::Get(this, core::mem::transmute_copy(&lrowindex), core::mem::transmute_copy(&lcolumnindex), core::mem::transmute_copy(&prop)).into()
        }
        unsafe extern "system" fn GetKeyProp<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keypropid: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::GetKeyProp(this, core::mem::transmute_copy(&keypropid)).into()
        }
        unsafe extern "system" fn GetColumnPriority<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::GetColumnPriority(this, core::mem::transmute_copy(&lcolumnindex), core::mem::transmute_copy(&columnpriority)).into()
        }
        unsafe extern "system" fn GetRowCount<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnumberofrows: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::GetRowCount(this, core::mem::transmute_copy(&lnumberofrows)).into()
        }
        unsafe extern "system" fn GetColumnCount<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnumberofcolumns: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::GetColumnCount(this, core::mem::transmute_copy(&lnumberofcolumns)).into()
        }
        unsafe extern "system" fn GetColumn<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut core::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::GetColumn(this, core::mem::transmute_copy(&lcolumnindex), core::mem::transmute_copy(&propid), core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&lpvdefaultvalue), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&columnpriority)).into()
        }
        unsafe extern "system" fn GetColumn2<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcolumnindex: i32, propid: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::GetColumn2(this, core::mem::transmute_copy(&lcolumnindex), core::mem::transmute_copy(&propid)).into()
        }
        unsafe extern "system" fn GetColumnFromPropID<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, lcolumnindex: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::GetColumnFromPropID(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&lcolumnindex)).into()
        }
        unsafe extern "system" fn Clear<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::Clear(this).into()
        }
        unsafe extern "system" fn ClearRows<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::ClearRows(this).into()
        }
        unsafe extern "system" fn Free<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::Free(this).into()
        }
        unsafe extern "system" fn IsCompleted<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::IsCompleted(this).into()
        }
        unsafe extern "system" fn Cancel<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn Pause<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fpause: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::Pause(this, core::mem::transmute_copy(&fpause)).into()
        }
        unsafe extern "system" fn GetRowStatus<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::GetRowStatus(this, core::mem::transmute_copy(&lrowfirst), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&lprowstatus)).into()
        }
        unsafe extern "system" fn GetColumnStatus<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpcolstatus: *mut COLUMNSTATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IITResultSet_Impl::GetColumnStatus(this, core::mem::transmute_copy(&lpcolstatus)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetColumnPriority: SetColumnPriority::<Identity, OFFSET>,
            SetColumnHeap: SetColumnHeap::<Identity, OFFSET>,
            SetKeyProp: SetKeyProp::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Add2: Add2::<Identity, OFFSET>,
            Add3: Add3::<Identity, OFFSET>,
            Add4: Add4::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            Set: Set::<Identity, OFFSET>,
            Set2: Set2::<Identity, OFFSET>,
            Set3: Set3::<Identity, OFFSET>,
            Set4: Set4::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            AppendRows: AppendRows::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            GetKeyProp: GetKeyProp::<Identity, OFFSET>,
            GetColumnPriority: GetColumnPriority::<Identity, OFFSET>,
            GetRowCount: GetRowCount::<Identity, OFFSET>,
            GetColumnCount: GetColumnCount::<Identity, OFFSET>,
            GetColumn: GetColumn::<Identity, OFFSET>,
            GetColumn2: GetColumn2::<Identity, OFFSET>,
            GetColumnFromPropID: GetColumnFromPropID::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            ClearRows: ClearRows::<Identity, OFFSET>,
            Free: Free::<Identity, OFFSET>,
            IsCompleted: IsCompleted::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            GetRowStatus: GetRowStatus::<Identity, OFFSET>,
            GetColumnStatus: GetColumnStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IITResultSet as windows_core::Interface>::IID
    }
}
pub trait IStemSink_Impl: Sized + windows_core::IUnknownImpl {
    fn PutAltWord(&self, pwcinbuf: &windows_core::PCWSTR, cwc: u32) -> windows_core::Result<()>;
    fn PutWord(&self, pwcinbuf: &windows_core::PCWSTR, cwc: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IStemSink {}
impl IStemSink_Vtbl {
    pub const fn new<Identity: IStemSink_Impl, const OFFSET: isize>() -> IStemSink_Vtbl {
        unsafe extern "system" fn PutAltWord<Identity: IStemSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcinbuf: windows_core::PCWSTR, cwc: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStemSink_Impl::PutAltWord(this, core::mem::transmute(&pwcinbuf), core::mem::transmute_copy(&cwc)).into()
        }
        unsafe extern "system" fn PutWord<Identity: IStemSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcinbuf: windows_core::PCWSTR, cwc: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStemSink_Impl::PutWord(this, core::mem::transmute(&pwcinbuf), core::mem::transmute_copy(&cwc)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), PutAltWord: PutAltWord::<Identity, OFFSET>, PutWord: PutWord::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStemSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStemmerConfig_Impl: Sized + windows_core::IUnknownImpl {
    fn SetLocaleInfo(&self, dwcodepageid: u32, lcid: u32) -> windows_core::Result<()>;
    fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> windows_core::Result<()>;
    fn SetControlInfo(&self, grfstemflags: u32, dwreserved: u32) -> windows_core::Result<()>;
    fn GetControlInfo(&self, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()>;
    fn LoadExternalStemmerData(&self, pstream: Option<&super::super::System::Com::IStream>, dwextdatatype: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IStemmerConfig {}
#[cfg(feature = "Win32_System_Com")]
impl IStemmerConfig_Vtbl {
    pub const fn new<Identity: IStemmerConfig_Impl, const OFFSET: isize>() -> IStemmerConfig_Vtbl {
        unsafe extern "system" fn SetLocaleInfo<Identity: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcodepageid: u32, lcid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStemmerConfig_Impl::SetLocaleInfo(this, core::mem::transmute_copy(&dwcodepageid), core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn GetLocaleInfo<Identity: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcodepageid: *mut u32, plcid: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStemmerConfig_Impl::GetLocaleInfo(this, core::mem::transmute_copy(&pdwcodepageid), core::mem::transmute_copy(&plcid)).into()
        }
        unsafe extern "system" fn SetControlInfo<Identity: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfstemflags: u32, dwreserved: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStemmerConfig_Impl::SetControlInfo(this, core::mem::transmute_copy(&grfstemflags), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetControlInfo<Identity: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStemmerConfig_Impl::GetControlInfo(this, core::mem::transmute_copy(&pgrfstemflags), core::mem::transmute_copy(&pdwreserved)).into()
        }
        unsafe extern "system" fn LoadExternalStemmerData<Identity: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, dwextdatatype: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStemmerConfig_Impl::LoadExternalStemmerData(this, windows_core::from_raw_borrowed(&pstream), core::mem::transmute_copy(&dwextdatatype)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLocaleInfo: SetLocaleInfo::<Identity, OFFSET>,
            GetLocaleInfo: GetLocaleInfo::<Identity, OFFSET>,
            SetControlInfo: SetControlInfo::<Identity, OFFSET>,
            GetControlInfo: GetControlInfo::<Identity, OFFSET>,
            LoadExternalStemmerData: LoadExternalStemmerData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStemmerConfig as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search"))]
pub trait IWordBreakerConfig_Impl: Sized + windows_core::IUnknownImpl {
    fn SetLocaleInfo(&self, dwcodepageid: u32, lcid: u32) -> windows_core::Result<()>;
    fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> windows_core::Result<()>;
    fn SetBreakWordType(&self, dwbreakwordtype: u32) -> windows_core::Result<()>;
    fn GetBreakWordType(&self, pdwbreakwordtype: *mut u32) -> windows_core::Result<()>;
    fn SetControlInfo(&self, grfbreakflags: u32, dwreserved: u32) -> windows_core::Result<()>;
    fn GetControlInfo(&self, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()>;
    fn LoadExternalBreakerData(&self, pstream: Option<&super::super::System::Com::IStream>, dwextdatatype: u32) -> windows_core::Result<()>;
    fn SetWordStemmer(&self, rclsid: *const windows_core::GUID, pstemmer: Option<&super::super::System::Search::IStemmer>) -> windows_core::Result<()>;
    fn GetWordStemmer(&self) -> windows_core::Result<super::super::System::Search::IStemmer>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search"))]
impl windows_core::RuntimeName for IWordBreakerConfig {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search"))]
impl IWordBreakerConfig_Vtbl {
    pub const fn new<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>() -> IWordBreakerConfig_Vtbl {
        unsafe extern "system" fn SetLocaleInfo<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcodepageid: u32, lcid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordBreakerConfig_Impl::SetLocaleInfo(this, core::mem::transmute_copy(&dwcodepageid), core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn GetLocaleInfo<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcodepageid: *mut u32, plcid: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordBreakerConfig_Impl::GetLocaleInfo(this, core::mem::transmute_copy(&pdwcodepageid), core::mem::transmute_copy(&plcid)).into()
        }
        unsafe extern "system" fn SetBreakWordType<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbreakwordtype: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordBreakerConfig_Impl::SetBreakWordType(this, core::mem::transmute_copy(&dwbreakwordtype)).into()
        }
        unsafe extern "system" fn GetBreakWordType<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwbreakwordtype: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordBreakerConfig_Impl::GetBreakWordType(this, core::mem::transmute_copy(&pdwbreakwordtype)).into()
        }
        unsafe extern "system" fn SetControlInfo<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfbreakflags: u32, dwreserved: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordBreakerConfig_Impl::SetControlInfo(this, core::mem::transmute_copy(&grfbreakflags), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetControlInfo<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordBreakerConfig_Impl::GetControlInfo(this, core::mem::transmute_copy(&pgrfbreakflags), core::mem::transmute_copy(&pdwreserved)).into()
        }
        unsafe extern "system" fn LoadExternalBreakerData<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, dwextdatatype: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordBreakerConfig_Impl::LoadExternalBreakerData(this, windows_core::from_raw_borrowed(&pstream), core::mem::transmute_copy(&dwextdatatype)).into()
        }
        unsafe extern "system" fn SetWordStemmer<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, pstemmer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordBreakerConfig_Impl::SetWordStemmer(this, core::mem::transmute_copy(&rclsid), windows_core::from_raw_borrowed(&pstemmer)).into()
        }
        unsafe extern "system" fn GetWordStemmer<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstemmer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWordBreakerConfig_Impl::GetWordStemmer(this) {
                Ok(ok__) => {
                    ppstemmer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLocaleInfo: SetLocaleInfo::<Identity, OFFSET>,
            GetLocaleInfo: GetLocaleInfo::<Identity, OFFSET>,
            SetBreakWordType: SetBreakWordType::<Identity, OFFSET>,
            GetBreakWordType: GetBreakWordType::<Identity, OFFSET>,
            SetControlInfo: SetControlInfo::<Identity, OFFSET>,
            GetControlInfo: GetControlInfo::<Identity, OFFSET>,
            LoadExternalBreakerData: LoadExternalBreakerData::<Identity, OFFSET>,
            SetWordStemmer: SetWordStemmer::<Identity, OFFSET>,
            GetWordStemmer: GetWordStemmer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWordBreakerConfig as windows_core::Interface>::IID
    }
}
