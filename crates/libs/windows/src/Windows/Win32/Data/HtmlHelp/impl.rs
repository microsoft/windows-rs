#[cfg(feature = "Win32_Foundation")]
pub trait IITDatabase_Impl: Sized {
    fn Open(&mut self, lpszhost: super::super::Foundation::PWSTR, lpszmoniker: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn CreateObject(&mut self, rclsid: *const ::windows::core::GUID, pdwobjinstance: *mut u32) -> ::windows::core::Result<()>;
    fn GetObject(&mut self, dwobjinstance: u32, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetObjectPersistence(&mut self, lpwszobject: super::super::Foundation::PWSTR, dwobjinstance: u32, ppvpersistence: *mut *mut ::core::ffi::c_void, fstream: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IITDatabase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IITDatabase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IITDatabase_Vtbl {
        unsafe extern "system" fn Open<Impl: IITDatabase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszhost: super::super::Foundation::PWSTR, lpszmoniker: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&lpszhost), ::core::mem::transmute_copy(&lpszmoniker), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Close<Impl: IITDatabase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn CreateObject<Impl: IITDatabase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, pdwobjinstance: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateObject(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&pdwobjinstance)).into()
        }
        unsafe extern "system" fn GetObject<Impl: IITDatabase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobjinstance: u32, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObject(::core::mem::transmute_copy(&dwobjinstance), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into()
        }
        unsafe extern "system" fn GetObjectPersistence<Impl: IITDatabase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpwszobject: super::super::Foundation::PWSTR, dwobjinstance: u32, ppvpersistence: *mut *mut ::core::ffi::c_void, fstream: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectPersistence(::core::mem::transmute_copy(&lpwszobject), ::core::mem::transmute_copy(&dwobjinstance), ::core::mem::transmute_copy(&ppvpersistence), ::core::mem::transmute_copy(&fstream)).into()
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
pub trait IITPropList_Impl: Sized + super::super::System::Com::IPersist_Impl + super::super::System::Com::IPersistStreamInit_Impl {
    fn Set(&mut self, propid: u32, lpszwstring: super::super::Foundation::PWSTR, dwoperation: u32) -> ::windows::core::Result<()>;
    fn Set2(&mut self, propid: u32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32, dwoperation: u32) -> ::windows::core::Result<()>;
    fn Set3(&mut self, propid: u32, dwdata: u32, dwoperation: u32) -> ::windows::core::Result<()>;
    fn Add(&mut self, prop: *mut CProperty) -> ::windows::core::Result<()>;
    fn Get(&mut self, propid: u32, property: *mut CProperty) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn SetPersist(&mut self, fpersist: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetPersist2(&mut self, propid: u32, fpersist: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetFirst(&mut self, property: *mut CProperty) -> ::windows::core::Result<()>;
    fn GetNext(&mut self, property: *mut CProperty) -> ::windows::core::Result<()>;
    fn GetPropCount(&mut self, cprop: *mut i32) -> ::windows::core::Result<()>;
    fn SaveHeader(&mut self, lpvdata: *mut ::core::ffi::c_void, dwhdrsize: u32) -> ::windows::core::Result<()>;
    fn SaveData(&mut self, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::Result<()>;
    fn GetHeaderSize(&mut self, dwhdrsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetDataSize(&mut self, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> ::windows::core::Result<()>;
    fn SaveDataToStream(&mut self, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, pstream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn LoadFromMem(&mut self, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::Result<()>;
    fn SaveToMem(&mut self, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IITPropList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IITPropList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IITPropList_Vtbl {
        unsafe extern "system" fn Set<Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, lpszwstring: super::super::Foundation::PWSTR, dwoperation: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Set(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&lpszwstring), ::core::mem::transmute_copy(&dwoperation)).into()
        }
        unsafe extern "system" fn Set2<Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32, dwoperation: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Set2(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&dwoperation)).into()
        }
        unsafe extern "system" fn Set3<Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, dwdata: u32, dwoperation: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Set3(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&dwdata), ::core::mem::transmute_copy(&dwoperation)).into()
        }
        unsafe extern "system" fn Add<Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prop: *mut CProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&prop)).into()
        }
        unsafe extern "system" fn Get<Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, property: *mut CProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Get(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&property)).into()
        }
        unsafe extern "system" fn Clear<Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn SetPersist<Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpersist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPersist(::core::mem::transmute_copy(&fpersist)).into()
        }
        unsafe extern "system" fn SetPersist2<Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, fpersist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPersist2(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&fpersist)).into()
        }
        unsafe extern "system" fn GetFirst<Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *mut CProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFirst(::core::mem::transmute_copy(&property)).into()
        }
        unsafe extern "system" fn GetNext<Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *mut CProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNext(::core::mem::transmute_copy(&property)).into()
        }
        unsafe extern "system" fn GetPropCount<Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropCount(::core::mem::transmute_copy(&cprop)).into()
        }
        unsafe extern "system" fn SaveHeader<Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void, dwhdrsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveHeader(::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&dwhdrsize)).into()
        }
        unsafe extern "system" fn SaveData<Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveData(::core::mem::transmute_copy(&lpvheader), ::core::mem::transmute_copy(&dwhdrsize), ::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&dwbufsize)).into()
        }
        unsafe extern "system" fn GetHeaderSize<Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhdrsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetHeaderSize(::core::mem::transmute_copy(&dwhdrsize)).into()
        }
        unsafe extern "system" fn GetDataSize<Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDataSize(::core::mem::transmute_copy(&lpvheader), ::core::mem::transmute_copy(&dwhdrsize), ::core::mem::transmute_copy(&dwdatasize)).into()
        }
        unsafe extern "system" fn SaveDataToStream<Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveDataToStream(::core::mem::transmute_copy(&lpvheader), ::core::mem::transmute_copy(&dwhdrsize), ::core::mem::transmute(&pstream)).into()
        }
        unsafe extern "system" fn LoadFromMem<Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadFromMem(::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&dwbufsize)).into()
        }
        unsafe extern "system" fn SaveToMem<Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveToMem(::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&dwbufsize)).into()
        }
        Self {
            base: super::super::System::Com::IPersistStreamInit_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Set: Set::<Impl, IMPL_OFFSET>,
            Set2: Set2::<Impl, IMPL_OFFSET>,
            Set3: Set3::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Get: Get::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            SetPersist: SetPersist::<Impl, IMPL_OFFSET>,
            SetPersist2: SetPersist2::<Impl, IMPL_OFFSET>,
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
        iid == &<IITPropList as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IPersist as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IPersistStreamInit as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IITResultSet_Impl: Sized {
    fn SetColumnPriority(&mut self, lcolumnindex: i32, columnpriority: PRIORITY) -> ::windows::core::Result<()>;
    fn SetColumnHeap(&mut self, lcolumnindex: i32, lpvheap: *mut ::core::ffi::c_void, pfncolheapfree: &PFNCOLHEAPFREE) -> ::windows::core::Result<()>;
    fn SetKeyProp(&mut self, propid: u32) -> ::windows::core::Result<()>;
    fn Add(&mut self, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> ::windows::core::Result<()>;
    fn Add2(&mut self, propid: u32, lpszwdefault: super::super::Foundation::PWSTR, priority: PRIORITY) -> ::windows::core::Result<()>;
    fn Add3(&mut self, propid: u32, lpvdefaultdata: *mut ::core::ffi::c_void, cbdata: u32, priority: PRIORITY) -> ::windows::core::Result<()>;
    fn Add4(&mut self, lpvhdr: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Append(&mut self, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Set(&mut self, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32) -> ::windows::core::Result<()>;
    fn Set2(&mut self, lrowindex: i32, lcolumnindex: i32, lpwstr: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Set3(&mut self, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> ::windows::core::Result<()>;
    fn Set4(&mut self, lrowindex: i32, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Copy(&mut self, prscopy: &::core::option::Option<IITResultSet>) -> ::windows::core::Result<()>;
    fn AppendRows(&mut self, pressrc: &::core::option::Option<IITResultSet>, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> ::windows::core::Result<()>;
    fn Get(&mut self, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> ::windows::core::Result<()>;
    fn GetKeyProp(&mut self, keypropid: *mut u32) -> ::windows::core::Result<()>;
    fn GetColumnPriority(&mut self, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> ::windows::core::Result<()>;
    fn GetRowCount(&mut self, lnumberofrows: *mut i32) -> ::windows::core::Result<()>;
    fn GetColumnCount(&mut self, lnumberofcolumns: *mut i32) -> ::windows::core::Result<()>;
    fn GetColumn(&mut self, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut ::core::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> ::windows::core::Result<()>;
    fn GetColumn2(&mut self, lcolumnindex: i32, propid: *mut u32) -> ::windows::core::Result<()>;
    fn GetColumnFromPropID(&mut self, propid: u32, lcolumnindex: *mut i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn ClearRows(&mut self) -> ::windows::core::Result<()>;
    fn Free(&mut self) -> ::windows::core::Result<()>;
    fn IsCompleted(&mut self) -> ::windows::core::Result<()>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn Pause(&mut self, fpause: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetRowStatus(&mut self, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> ::windows::core::Result<()>;
    fn GetColumnStatus(&mut self, lpcolstatus: *mut COLUMNSTATUS) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IITResultSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IITResultSet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IITResultSet_Vtbl {
        unsafe extern "system" fn SetColumnPriority<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, columnpriority: PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColumnPriority(::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&columnpriority)).into()
        }
        unsafe extern "system" fn SetColumnHeap<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, lpvheap: *mut ::core::ffi::c_void, pfncolheapfree: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColumnHeap(::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&lpvheap), ::core::mem::transmute_copy(&pfncolheapfree)).into()
        }
        unsafe extern "system" fn SetKeyProp<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyProp(::core::mem::transmute_copy(&propid)).into()
        }
        unsafe extern "system" fn Add<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&dwdefaultdata), ::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn Add2<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, lpszwdefault: super::super::Foundation::PWSTR, priority: PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add2(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&lpszwdefault), ::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn Add3<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, lpvdefaultdata: *mut ::core::ffi::c_void, cbdata: u32, priority: PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add3(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&lpvdefaultdata), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn Add4<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvhdr: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add4(::core::mem::transmute_copy(&lpvhdr)).into()
        }
        unsafe extern "system" fn Append<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute_copy(&lpvhdr), ::core::mem::transmute_copy(&lpvdata)).into()
        }
        unsafe extern "system" fn Set<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Set(::core::mem::transmute_copy(&lrowindex), ::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn Set2<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, lpwstr: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Set2(::core::mem::transmute_copy(&lrowindex), ::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&lpwstr)).into()
        }
        unsafe extern "system" fn Set3<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Set3(::core::mem::transmute_copy(&lrowindex), ::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&dwdata)).into()
        }
        unsafe extern "system" fn Set4<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Set4(::core::mem::transmute_copy(&lrowindex), ::core::mem::transmute_copy(&lpvhdr), ::core::mem::transmute_copy(&lpvdata)).into()
        }
        unsafe extern "system" fn Copy<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prscopy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Copy(::core::mem::transmute(&prscopy)).into()
        }
        unsafe extern "system" fn AppendRows<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pressrc: ::windows::core::RawPtr, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AppendRows(::core::mem::transmute(&pressrc), ::core::mem::transmute_copy(&lrowsrcfirst), ::core::mem::transmute_copy(&csrcrows), ::core::mem::transmute_copy(&lrowfirstdest)).into()
        }
        unsafe extern "system" fn Get<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Get(::core::mem::transmute_copy(&lrowindex), ::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&prop)).into()
        }
        unsafe extern "system" fn GetKeyProp<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keypropid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetKeyProp(::core::mem::transmute_copy(&keypropid)).into()
        }
        unsafe extern "system" fn GetColumnPriority<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColumnPriority(::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&columnpriority)).into()
        }
        unsafe extern "system" fn GetRowCount<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumberofrows: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRowCount(::core::mem::transmute_copy(&lnumberofrows)).into()
        }
        unsafe extern "system" fn GetColumnCount<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumberofcolumns: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColumnCount(::core::mem::transmute_copy(&lnumberofcolumns)).into()
        }
        unsafe extern "system" fn GetColumn<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut ::core::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColumn(::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&lpvdefaultvalue), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&columnpriority)).into()
        }
        unsafe extern "system" fn GetColumn2<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, propid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColumn2(::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&propid)).into()
        }
        unsafe extern "system" fn GetColumnFromPropID<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, lcolumnindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColumnFromPropID(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&lcolumnindex)).into()
        }
        unsafe extern "system" fn Clear<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn ClearRows<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearRows().into()
        }
        unsafe extern "system" fn Free<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Free().into()
        }
        unsafe extern "system" fn IsCompleted<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsCompleted().into()
        }
        unsafe extern "system" fn Cancel<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Pause<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpause: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause(::core::mem::transmute_copy(&fpause)).into()
        }
        unsafe extern "system" fn GetRowStatus<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRowStatus(::core::mem::transmute_copy(&lrowfirst), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&lprowstatus)).into()
        }
        unsafe extern "system" fn GetColumnStatus<Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcolstatus: *mut COLUMNSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColumnStatus(::core::mem::transmute_copy(&lpcolstatus)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetColumnPriority: SetColumnPriority::<Impl, IMPL_OFFSET>,
            SetColumnHeap: SetColumnHeap::<Impl, IMPL_OFFSET>,
            SetKeyProp: SetKeyProp::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Add2: Add2::<Impl, IMPL_OFFSET>,
            Add3: Add3::<Impl, IMPL_OFFSET>,
            Add4: Add4::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
            Set: Set::<Impl, IMPL_OFFSET>,
            Set2: Set2::<Impl, IMPL_OFFSET>,
            Set3: Set3::<Impl, IMPL_OFFSET>,
            Set4: Set4::<Impl, IMPL_OFFSET>,
            Copy: Copy::<Impl, IMPL_OFFSET>,
            AppendRows: AppendRows::<Impl, IMPL_OFFSET>,
            Get: Get::<Impl, IMPL_OFFSET>,
            GetKeyProp: GetKeyProp::<Impl, IMPL_OFFSET>,
            GetColumnPriority: GetColumnPriority::<Impl, IMPL_OFFSET>,
            GetRowCount: GetRowCount::<Impl, IMPL_OFFSET>,
            GetColumnCount: GetColumnCount::<Impl, IMPL_OFFSET>,
            GetColumn: GetColumn::<Impl, IMPL_OFFSET>,
            GetColumn2: GetColumn2::<Impl, IMPL_OFFSET>,
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
pub trait IITWordWheel_Impl: Sized {
    fn Open(&mut self, lpitdb: &::core::option::Option<IITDatabase>, lpszmoniker: super::super::Foundation::PWSTR, dwflags: WORD_WHEEL_OPEN_FLAGS) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn GetLocaleInfo(&mut self, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::Result<()>;
    fn GetSorterInstance(&mut self, pdwobjinstance: *mut u32) -> ::windows::core::Result<()>;
    fn Count(&mut self, pcentries: *mut i32) -> ::windows::core::Result<()>;
    fn Lookup(&mut self, lpcvprefix: *const ::core::ffi::c_void, fexactmatch: super::super::Foundation::BOOL, plentry: *mut i32) -> ::windows::core::Result<()>;
    fn Lookup2(&mut self, lentry: i32, lpitresult: &::core::option::Option<IITResultSet>, centries: i32) -> ::windows::core::Result<()>;
    fn Lookup3(&mut self, lentry: i32, lpvkeybuf: *mut ::core::ffi::c_void, cbkeybuf: u32) -> ::windows::core::Result<()>;
    fn SetGroup(&mut self, piitgroup: *mut IITGroup) -> ::windows::core::Result<()>;
    fn GetGroup(&mut self, ppiitgroup: *mut *mut IITGroup) -> ::windows::core::Result<()>;
    fn GetDataCount(&mut self, lentry: i32, pdwcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetData(&mut self, lentry: i32, lpitresult: &::core::option::Option<IITResultSet>) -> ::windows::core::Result<()>;
    fn GetDataColumns(&mut self, prs: &::core::option::Option<IITResultSet>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IITWordWheel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IITWordWheel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IITWordWheel_Vtbl {
        unsafe extern "system" fn Open<Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpitdb: ::windows::core::RawPtr, lpszmoniker: super::super::Foundation::PWSTR, dwflags: WORD_WHEEL_OPEN_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute(&lpitdb), ::core::mem::transmute_copy(&lpszmoniker), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Close<Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn GetLocaleInfo<Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocaleInfo(::core::mem::transmute_copy(&pdwcodepageid), ::core::mem::transmute_copy(&plcid)).into()
        }
        unsafe extern "system" fn GetSorterInstance<Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwobjinstance: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSorterInstance(::core::mem::transmute_copy(&pdwobjinstance)).into()
        }
        unsafe extern "system" fn Count<Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcentries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Count(::core::mem::transmute_copy(&pcentries)).into()
        }
        unsafe extern "system" fn Lookup<Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcvprefix: *const ::core::ffi::c_void, fexactmatch: super::super::Foundation::BOOL, plentry: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Lookup(::core::mem::transmute_copy(&lpcvprefix), ::core::mem::transmute_copy(&fexactmatch), ::core::mem::transmute_copy(&plentry)).into()
        }
        unsafe extern "system" fn Lookup2<Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lentry: i32, lpitresult: ::windows::core::RawPtr, centries: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Lookup2(::core::mem::transmute_copy(&lentry), ::core::mem::transmute(&lpitresult), ::core::mem::transmute_copy(&centries)).into()
        }
        unsafe extern "system" fn Lookup3<Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lentry: i32, lpvkeybuf: *mut ::core::ffi::c_void, cbkeybuf: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Lookup3(::core::mem::transmute_copy(&lentry), ::core::mem::transmute_copy(&lpvkeybuf), ::core::mem::transmute_copy(&cbkeybuf)).into()
        }
        unsafe extern "system" fn SetGroup<Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piitgroup: *mut IITGroup) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroup(::core::mem::transmute_copy(&piitgroup)).into()
        }
        unsafe extern "system" fn GetGroup<Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiitgroup: *mut *mut IITGroup) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGroup(::core::mem::transmute_copy(&ppiitgroup)).into()
        }
        unsafe extern "system" fn GetDataCount<Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lentry: i32, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDataCount(::core::mem::transmute_copy(&lentry), ::core::mem::transmute_copy(&pdwcount)).into()
        }
        unsafe extern "system" fn GetData<Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lentry: i32, lpitresult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetData(::core::mem::transmute_copy(&lentry), ::core::mem::transmute(&lpitresult)).into()
        }
        unsafe extern "system" fn GetDataColumns<Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDataColumns(::core::mem::transmute(&prs)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            GetLocaleInfo: GetLocaleInfo::<Impl, IMPL_OFFSET>,
            GetSorterInstance: GetSorterInstance::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Lookup: Lookup::<Impl, IMPL_OFFSET>,
            Lookup2: Lookup2::<Impl, IMPL_OFFSET>,
            Lookup3: Lookup3::<Impl, IMPL_OFFSET>,
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
pub trait IStemSink_Impl: Sized {
    fn PutAltWord(&mut self, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32) -> ::windows::core::Result<()>;
    fn PutWord(&mut self, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IStemSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStemSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStemSink_Vtbl {
        unsafe extern "system" fn PutAltWord<Impl: IStemSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutAltWord(::core::mem::transmute_copy(&pwcinbuf), ::core::mem::transmute_copy(&cwc)).into()
        }
        unsafe extern "system" fn PutWord<Impl: IStemSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutWord(::core::mem::transmute_copy(&pwcinbuf), ::core::mem::transmute_copy(&cwc)).into()
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
pub trait IStemmerConfig_Impl: Sized {
    fn SetLocaleInfo(&mut self, dwcodepageid: u32, lcid: u32) -> ::windows::core::Result<()>;
    fn GetLocaleInfo(&mut self, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::Result<()>;
    fn SetControlInfo(&mut self, grfstemflags: u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn GetControlInfo(&mut self, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
    fn LoadExternalStemmerData(&mut self, pstream: &::core::option::Option<super::super::System::Com::IStream>, dwextdatatype: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IStemmerConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStemmerConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStemmerConfig_Vtbl {
        unsafe extern "system" fn SetLocaleInfo<Impl: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcodepageid: u32, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocaleInfo(::core::mem::transmute_copy(&dwcodepageid), ::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn GetLocaleInfo<Impl: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocaleInfo(::core::mem::transmute_copy(&pdwcodepageid), ::core::mem::transmute_copy(&plcid)).into()
        }
        unsafe extern "system" fn SetControlInfo<Impl: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfstemflags: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControlInfo(::core::mem::transmute_copy(&grfstemflags), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetControlInfo<Impl: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetControlInfo(::core::mem::transmute_copy(&pgrfstemflags), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        unsafe extern "system" fn LoadExternalStemmerData<Impl: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, dwextdatatype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadExternalStemmerData(::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&dwextdatatype)).into()
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
pub trait IWordBreakerConfig_Impl: Sized {
    fn SetLocaleInfo(&mut self, dwcodepageid: u32, lcid: u32) -> ::windows::core::Result<()>;
    fn GetLocaleInfo(&mut self, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::Result<()>;
    fn SetBreakWordType(&mut self, dwbreakwordtype: u32) -> ::windows::core::Result<()>;
    fn GetBreakWordType(&mut self, pdwbreakwordtype: *mut u32) -> ::windows::core::Result<()>;
    fn SetControlInfo(&mut self, grfbreakflags: u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn GetControlInfo(&mut self, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
    fn LoadExternalBreakerData(&mut self, pstream: &::core::option::Option<super::super::System::Com::IStream>, dwextdatatype: u32) -> ::windows::core::Result<()>;
    fn SetWordStemmer(&mut self, rclsid: *const ::windows::core::GUID, pstemmer: &::core::option::Option<super::super::System::Search::IStemmer>) -> ::windows::core::Result<()>;
    fn GetWordStemmer(&mut self) -> ::windows::core::Result<super::super::System::Search::IStemmer>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search"))]
impl IWordBreakerConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWordBreakerConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWordBreakerConfig_Vtbl {
        unsafe extern "system" fn SetLocaleInfo<Impl: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcodepageid: u32, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocaleInfo(::core::mem::transmute_copy(&dwcodepageid), ::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn GetLocaleInfo<Impl: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocaleInfo(::core::mem::transmute_copy(&pdwcodepageid), ::core::mem::transmute_copy(&plcid)).into()
        }
        unsafe extern "system" fn SetBreakWordType<Impl: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbreakwordtype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBreakWordType(::core::mem::transmute_copy(&dwbreakwordtype)).into()
        }
        unsafe extern "system" fn GetBreakWordType<Impl: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbreakwordtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBreakWordType(::core::mem::transmute_copy(&pdwbreakwordtype)).into()
        }
        unsafe extern "system" fn SetControlInfo<Impl: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbreakflags: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControlInfo(::core::mem::transmute_copy(&grfbreakflags), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetControlInfo<Impl: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetControlInfo(::core::mem::transmute_copy(&pgrfbreakflags), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        unsafe extern "system" fn LoadExternalBreakerData<Impl: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, dwextdatatype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadExternalBreakerData(::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&dwextdatatype)).into()
        }
        unsafe extern "system" fn SetWordStemmer<Impl: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, pstemmer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWordStemmer(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute(&pstemmer)).into()
        }
        unsafe extern "system" fn GetWordStemmer<Impl: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstemmer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWordStemmer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstemmer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
