#[cfg(feature = "Win32_Foundation")]
pub trait IITDatabaseImpl: Sized {
    fn Open();
    fn Close();
    fn CreateObject();
    fn GetObject();
    fn GetObjectPersistence();
}
#[cfg(feature = "Win32_Foundation")]
impl IITDatabaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IITDatabaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IITDatabaseVtbl {
        unsafe extern "system" fn Open<Impl: IITDatabaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszhost: super::super::Foundation::PWSTR, lpszmoniker: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IITDatabaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateObject<Impl: IITDatabaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, pdwobjinstance: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObject<Impl: IITDatabaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobjinstance: u32, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjectPersistence<Impl: IITDatabaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpwszobject: super::super::Foundation::PWSTR, dwobjinstance: u32, ppvpersistence: *mut *mut ::core::ffi::c_void, fstream: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            CreateObject: CreateObject::<Impl, IMPL_OFFSET>,
            GetObject: GetObject::<Impl, IMPL_OFFSET>,
            GetObjectPersistence: GetObjectPersistence::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IITDatabase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IITPropListImpl: Sized + IPersistImpl + IPersistStreamInitImpl {
    fn Set();
    fn Set();
    fn Set();
    fn Add();
    fn Get();
    fn Clear();
    fn SetPersist();
    fn SetPersist();
    fn GetFirst();
    fn GetNext();
    fn GetPropCount();
    fn SaveHeader();
    fn SaveData();
    fn GetHeaderSize();
    fn GetDataSize();
    fn SaveDataToStream();
    fn LoadFromMem();
    fn SaveToMem();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IITPropListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IITPropListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IITPropListVtbl {
        unsafe extern "system" fn Set<Impl: IITPropListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, lpszwstring: super::super::Foundation::PWSTR, dwoperation: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Set<Impl: IITPropListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32, dwoperation: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Set<Impl: IITPropListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, dwdata: u32, dwoperation: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IITPropListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prop: *mut CProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Get<Impl: IITPropListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, property: *mut CProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IITPropListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPersist<Impl: IITPropListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpersist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPersist<Impl: IITPropListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, fpersist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFirst<Impl: IITPropListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *mut CProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNext<Impl: IITPropListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *mut CProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropCount<Impl: IITPropListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveHeader<Impl: IITPropListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void, dwhdrsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveData<Impl: IITPropListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHeaderSize<Impl: IITPropListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhdrsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDataSize<Impl: IITPropListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveDataToStream<Impl: IITPropListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadFromMem<Impl: IITPropListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveToMem<Impl: IITPropListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IPersistStreamInitVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Set: Set::<Impl, IMPL_OFFSET>,
            Set: Set::<Impl, IMPL_OFFSET>,
            Set: Set::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Get: Get::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            SetPersist: SetPersist::<Impl, IMPL_OFFSET>,
            SetPersist: SetPersist::<Impl, IMPL_OFFSET>,
            GetFirst: GetFirst::<Impl, IMPL_OFFSET>,
            GetNext: GetNext::<Impl, IMPL_OFFSET>,
            GetPropCount: GetPropCount::<Impl, IMPL_OFFSET>,
            SaveHeader: SaveHeader::<Impl, IMPL_OFFSET>,
            SaveData: SaveData::<Impl, IMPL_OFFSET>,
            GetHeaderSize: GetHeaderSize::<Impl, IMPL_OFFSET>,
            GetDataSize: GetDataSize::<Impl, IMPL_OFFSET>,
            SaveDataToStream: SaveDataToStream::<Impl, IMPL_OFFSET>,
            LoadFromMem: LoadFromMem::<Impl, IMPL_OFFSET>,
            SaveToMem: SaveToMem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IITPropList as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IITResultSetImpl: Sized {
    fn SetColumnPriority();
    fn SetColumnHeap();
    fn SetKeyProp();
    fn Add();
    fn Add();
    fn Add();
    fn Add();
    fn Append();
    fn Set();
    fn Set();
    fn Set();
    fn Set();
    fn Copy();
    fn AppendRows();
    fn Get();
    fn GetKeyProp();
    fn GetColumnPriority();
    fn GetRowCount();
    fn GetColumnCount();
    fn GetColumn();
    fn GetColumn();
    fn GetColumnFromPropID();
    fn Clear();
    fn ClearRows();
    fn Free();
    fn IsCompleted();
    fn Cancel();
    fn Pause();
    fn GetRowStatus();
    fn GetColumnStatus();
}
#[cfg(feature = "Win32_Foundation")]
impl IITResultSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IITResultSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IITResultSetVtbl {
        unsafe extern "system" fn SetColumnPriority<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, columnpriority: PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColumnHeap<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, lpvheap: *mut ::core::ffi::c_void, pfncolheapfree: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKeyProp<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, lpszwdefault: super::super::Foundation::PWSTR, priority: PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, lpvdefaultdata: *mut ::core::ffi::c_void, cbdata: u32, priority: PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvhdr: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Set<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Set<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, lpwstr: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Set<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Set<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Copy<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prscopy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AppendRows<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pressrc: ::windows::core::RawPtr, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Get<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeyProp<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keypropid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumnPriority<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRowCount<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumberofrows: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumnCount<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumberofcolumns: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumn<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut ::core::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumn<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, propid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumnFromPropID<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, lcolumnindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearRows<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Free<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsCompleted<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pause<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpause: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRowStatus<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumnStatus<Impl: IITResultSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcolstatus: *mut COLUMNSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetColumnPriority: SetColumnPriority::<Impl, IMPL_OFFSET>,
            SetColumnHeap: SetColumnHeap::<Impl, IMPL_OFFSET>,
            SetKeyProp: SetKeyProp::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
            Set: Set::<Impl, IMPL_OFFSET>,
            Set: Set::<Impl, IMPL_OFFSET>,
            Set: Set::<Impl, IMPL_OFFSET>,
            Set: Set::<Impl, IMPL_OFFSET>,
            Copy: Copy::<Impl, IMPL_OFFSET>,
            AppendRows: AppendRows::<Impl, IMPL_OFFSET>,
            Get: Get::<Impl, IMPL_OFFSET>,
            GetKeyProp: GetKeyProp::<Impl, IMPL_OFFSET>,
            GetColumnPriority: GetColumnPriority::<Impl, IMPL_OFFSET>,
            GetRowCount: GetRowCount::<Impl, IMPL_OFFSET>,
            GetColumnCount: GetColumnCount::<Impl, IMPL_OFFSET>,
            GetColumn: GetColumn::<Impl, IMPL_OFFSET>,
            GetColumn: GetColumn::<Impl, IMPL_OFFSET>,
            GetColumnFromPropID: GetColumnFromPropID::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            ClearRows: ClearRows::<Impl, IMPL_OFFSET>,
            Free: Free::<Impl, IMPL_OFFSET>,
            IsCompleted: IsCompleted::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            GetRowStatus: GetRowStatus::<Impl, IMPL_OFFSET>,
            GetColumnStatus: GetColumnStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IITResultSet as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IITWordWheelImpl: Sized {
    fn Open();
    fn Close();
    fn GetLocaleInfo();
    fn GetSorterInstance();
    fn Count();
    fn Lookup();
    fn Lookup();
    fn Lookup();
    fn SetGroup();
    fn GetGroup();
    fn GetDataCount();
    fn GetData();
    fn GetDataColumns();
}
#[cfg(feature = "Win32_Foundation")]
impl IITWordWheelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IITWordWheelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IITWordWheelVtbl {
        unsafe extern "system" fn Open<Impl: IITWordWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpitdb: ::windows::core::RawPtr, lpszmoniker: super::super::Foundation::PWSTR, dwflags: WORD_WHEEL_OPEN_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IITWordWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocaleInfo<Impl: IITWordWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSorterInstance<Impl: IITWordWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwobjinstance: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IITWordWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcentries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Lookup<Impl: IITWordWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcvprefix: *const ::core::ffi::c_void, fexactmatch: super::super::Foundation::BOOL, plentry: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Lookup<Impl: IITWordWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lentry: i32, lpitresult: ::windows::core::RawPtr, centries: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Lookup<Impl: IITWordWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lentry: i32, lpvkeybuf: *mut ::core::ffi::c_void, cbkeybuf: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGroup<Impl: IITWordWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piitgroup: *mut IITGroup) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGroup<Impl: IITWordWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiitgroup: *mut *mut IITGroup) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDataCount<Impl: IITWordWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lentry: i32, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetData<Impl: IITWordWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lentry: i32, lpitresult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDataColumns<Impl: IITWordWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            GetLocaleInfo: GetLocaleInfo::<Impl, IMPL_OFFSET>,
            GetSorterInstance: GetSorterInstance::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Lookup: Lookup::<Impl, IMPL_OFFSET>,
            Lookup: Lookup::<Impl, IMPL_OFFSET>,
            Lookup: Lookup::<Impl, IMPL_OFFSET>,
            SetGroup: SetGroup::<Impl, IMPL_OFFSET>,
            GetGroup: GetGroup::<Impl, IMPL_OFFSET>,
            GetDataCount: GetDataCount::<Impl, IMPL_OFFSET>,
            GetData: GetData::<Impl, IMPL_OFFSET>,
            GetDataColumns: GetDataColumns::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IITWordWheel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IStemSinkImpl: Sized {
    fn PutAltWord();
    fn PutWord();
}
#[cfg(feature = "Win32_Foundation")]
impl IStemSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStemSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStemSinkVtbl {
        unsafe extern "system" fn PutAltWord<Impl: IStemSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutWord<Impl: IStemSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PutAltWord: PutAltWord::<Impl, IMPL_OFFSET>,
            PutWord: PutWord::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStemSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStemmerConfigImpl: Sized {
    fn SetLocaleInfo();
    fn GetLocaleInfo();
    fn SetControlInfo();
    fn GetControlInfo();
    fn LoadExternalStemmerData();
}
#[cfg(feature = "Win32_System_Com")]
impl IStemmerConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStemmerConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStemmerConfigVtbl {
        unsafe extern "system" fn SetLocaleInfo<Impl: IStemmerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcodepageid: u32, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocaleInfo<Impl: IStemmerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetControlInfo<Impl: IStemmerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfstemflags: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetControlInfo<Impl: IStemmerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadExternalStemmerData<Impl: IStemmerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, dwextdatatype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetLocaleInfo: SetLocaleInfo::<Impl, IMPL_OFFSET>,
            GetLocaleInfo: GetLocaleInfo::<Impl, IMPL_OFFSET>,
            SetControlInfo: SetControlInfo::<Impl, IMPL_OFFSET>,
            GetControlInfo: GetControlInfo::<Impl, IMPL_OFFSET>,
            LoadExternalStemmerData: LoadExternalStemmerData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStemmerConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search"))]
pub trait IWordBreakerConfigImpl: Sized {
    fn SetLocaleInfo();
    fn GetLocaleInfo();
    fn SetBreakWordType();
    fn GetBreakWordType();
    fn SetControlInfo();
    fn GetControlInfo();
    fn LoadExternalBreakerData();
    fn SetWordStemmer();
    fn GetWordStemmer();
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search"))]
impl IWordBreakerConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWordBreakerConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWordBreakerConfigVtbl {
        unsafe extern "system" fn SetLocaleInfo<Impl: IWordBreakerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcodepageid: u32, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocaleInfo<Impl: IWordBreakerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBreakWordType<Impl: IWordBreakerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbreakwordtype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBreakWordType<Impl: IWordBreakerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbreakwordtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetControlInfo<Impl: IWordBreakerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbreakflags: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetControlInfo<Impl: IWordBreakerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadExternalBreakerData<Impl: IWordBreakerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, dwextdatatype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWordStemmer<Impl: IWordBreakerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, pstemmer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWordStemmer<Impl: IWordBreakerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstemmer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetLocaleInfo: SetLocaleInfo::<Impl, IMPL_OFFSET>,
            GetLocaleInfo: GetLocaleInfo::<Impl, IMPL_OFFSET>,
            SetBreakWordType: SetBreakWordType::<Impl, IMPL_OFFSET>,
            GetBreakWordType: GetBreakWordType::<Impl, IMPL_OFFSET>,
            SetControlInfo: SetControlInfo::<Impl, IMPL_OFFSET>,
            GetControlInfo: GetControlInfo::<Impl, IMPL_OFFSET>,
            LoadExternalBreakerData: LoadExternalBreakerData::<Impl, IMPL_OFFSET>,
            SetWordStemmer: SetWordStemmer::<Impl, IMPL_OFFSET>,
            GetWordStemmer: GetWordStemmer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWordBreakerConfig as ::windows::core::Interface>::IID
    }
}
