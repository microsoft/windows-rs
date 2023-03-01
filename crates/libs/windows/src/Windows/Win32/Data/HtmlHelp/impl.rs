#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IITDatabase_Impl: Sized {
    fn Open(&self, lpszhost: &::windows::core::PCWSTR, lpszmoniker: &::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn CreateObject(&self, rclsid: *const ::windows::core::GUID, pdwobjinstance: *mut u32) -> ::windows::core::Result<()>;
    fn GetObject(&self, dwobjinstance: u32, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetObjectPersistence(&self, lpwszobject: &::windows::core::PCWSTR, dwobjinstance: u32, ppvpersistence: *mut *mut ::core::ffi::c_void, fstream: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IITDatabase {}
#[cfg(feature = "Win32_Foundation")]
impl IITDatabase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITDatabase_Impl, const OFFSET: isize>() -> IITDatabase_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITDatabase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszhost: ::windows::core::PCWSTR, lpszmoniker: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::core::mem::transmute(&lpszhost), ::core::mem::transmute(&lpszmoniker), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITDatabase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn CreateObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITDatabase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, pdwobjinstance: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateObject(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&pdwobjinstance)).into()
        }
        unsafe extern "system" fn GetObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITDatabase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwobjinstance: u32, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObject(::core::mem::transmute_copy(&dwobjinstance), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into()
        }
        unsafe extern "system" fn GetObjectPersistence<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITDatabase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpwszobject: ::windows::core::PCWSTR, dwobjinstance: u32, ppvpersistence: *mut *mut ::core::ffi::c_void, fstream: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectPersistence(::core::mem::transmute(&lpwszobject), ::core::mem::transmute_copy(&dwobjinstance), ::core::mem::transmute_copy(&ppvpersistence), ::core::mem::transmute_copy(&fstream)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            CreateObject: CreateObject::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            GetObjectPersistence: GetObjectPersistence::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IITDatabase as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IITPropList_Impl: Sized + super::super::System::Com::IPersistStreamInit_Impl {
    fn Set(&self, propid: u32, lpszwstring: &::windows::core::PCWSTR, dwoperation: u32) -> ::windows::core::Result<()>;
    fn Set2(&self, propid: u32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32, dwoperation: u32) -> ::windows::core::Result<()>;
    fn Set3(&self, propid: u32, dwdata: u32, dwoperation: u32) -> ::windows::core::Result<()>;
    fn Add(&self, prop: *mut CProperty) -> ::windows::core::Result<()>;
    fn Get(&self, propid: u32, property: *mut CProperty) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn SetPersist(&self, fpersist: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetPersist2(&self, propid: u32, fpersist: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetFirst(&self, property: *mut CProperty) -> ::windows::core::Result<()>;
    fn GetNext(&self, property: *mut CProperty) -> ::windows::core::Result<()>;
    fn GetPropCount(&self, cprop: *mut i32) -> ::windows::core::Result<()>;
    fn SaveHeader(&self, lpvdata: *mut ::core::ffi::c_void, dwhdrsize: u32) -> ::windows::core::Result<()>;
    fn SaveData(&self, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::Result<()>;
    fn GetHeaderSize(&self, dwhdrsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetDataSize(&self, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> ::windows::core::Result<()>;
    fn SaveDataToStream(&self, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn LoadFromMem(&self, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::Result<()>;
    fn SaveToMem(&self, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IITPropList {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IITPropList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>() -> IITPropList_Vtbl {
        unsafe extern "system" fn Set<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, lpszwstring: ::windows::core::PCWSTR, dwoperation: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Set(::core::mem::transmute_copy(&propid), ::core::mem::transmute(&lpszwstring), ::core::mem::transmute_copy(&dwoperation)).into()
        }
        unsafe extern "system" fn Set2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32, dwoperation: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Set2(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&dwoperation)).into()
        }
        unsafe extern "system" fn Set3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, dwdata: u32, dwoperation: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Set3(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&dwdata), ::core::mem::transmute_copy(&dwoperation)).into()
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prop: *mut CProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute_copy(&prop)).into()
        }
        unsafe extern "system" fn Get<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, property: *mut CProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Get(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&property)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        unsafe extern "system" fn SetPersist<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpersist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPersist(::core::mem::transmute_copy(&fpersist)).into()
        }
        unsafe extern "system" fn SetPersist2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, fpersist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPersist2(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&fpersist)).into()
        }
        unsafe extern "system" fn GetFirst<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *mut CProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFirst(::core::mem::transmute_copy(&property)).into()
        }
        unsafe extern "system" fn GetNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *mut CProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNext(::core::mem::transmute_copy(&property)).into()
        }
        unsafe extern "system" fn GetPropCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropCount(::core::mem::transmute_copy(&cprop)).into()
        }
        unsafe extern "system" fn SaveHeader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void, dwhdrsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveHeader(::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&dwhdrsize)).into()
        }
        unsafe extern "system" fn SaveData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveData(::core::mem::transmute_copy(&lpvheader), ::core::mem::transmute_copy(&dwhdrsize), ::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&dwbufsize)).into()
        }
        unsafe extern "system" fn GetHeaderSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhdrsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHeaderSize(::core::mem::transmute_copy(&dwhdrsize)).into()
        }
        unsafe extern "system" fn GetDataSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDataSize(::core::mem::transmute_copy(&lpvheader), ::core::mem::transmute_copy(&dwhdrsize), ::core::mem::transmute_copy(&dwdatasize)).into()
        }
        unsafe extern "system" fn SaveDataToStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, pstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveDataToStream(::core::mem::transmute_copy(&lpvheader), ::core::mem::transmute_copy(&dwhdrsize), ::windows::core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn LoadFromMem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadFromMem(::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&dwbufsize)).into()
        }
        unsafe extern "system" fn SaveToMem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveToMem(::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&dwbufsize)).into()
        }
        Self {
            base__: super::super::System::Com::IPersistStreamInit_Vtbl::new::<Identity, Impl, OFFSET>(),
            Set: Set::<Identity, Impl, OFFSET>,
            Set2: Set2::<Identity, Impl, OFFSET>,
            Set3: Set3::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            SetPersist: SetPersist::<Identity, Impl, OFFSET>,
            SetPersist2: SetPersist2::<Identity, Impl, OFFSET>,
            GetFirst: GetFirst::<Identity, Impl, OFFSET>,
            GetNext: GetNext::<Identity, Impl, OFFSET>,
            GetPropCount: GetPropCount::<Identity, Impl, OFFSET>,
            SaveHeader: SaveHeader::<Identity, Impl, OFFSET>,
            SaveData: SaveData::<Identity, Impl, OFFSET>,
            GetHeaderSize: GetHeaderSize::<Identity, Impl, OFFSET>,
            GetDataSize: GetDataSize::<Identity, Impl, OFFSET>,
            SaveDataToStream: SaveDataToStream::<Identity, Impl, OFFSET>,
            LoadFromMem: LoadFromMem::<Identity, Impl, OFFSET>,
            SaveToMem: SaveToMem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IITPropList as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IPersist as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IPersistStreamInit as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IITResultSet_Impl: Sized {
    fn SetColumnPriority(&self, lcolumnindex: i32, columnpriority: PRIORITY) -> ::windows::core::Result<()>;
    fn SetColumnHeap(&self, lcolumnindex: i32, lpvheap: *mut ::core::ffi::c_void, pfncolheapfree: PFNCOLHEAPFREE) -> ::windows::core::Result<()>;
    fn SetKeyProp(&self, propid: u32) -> ::windows::core::Result<()>;
    fn Add(&self, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> ::windows::core::Result<()>;
    fn Add2(&self, propid: u32, lpszwdefault: &::windows::core::PCWSTR, priority: PRIORITY) -> ::windows::core::Result<()>;
    fn Add3(&self, propid: u32, lpvdefaultdata: *mut ::core::ffi::c_void, cbdata: u32, priority: PRIORITY) -> ::windows::core::Result<()>;
    fn Add4(&self, lpvhdr: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Append(&self, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Set(&self, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32) -> ::windows::core::Result<()>;
    fn Set2(&self, lrowindex: i32, lcolumnindex: i32, lpwstr: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Set3(&self, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> ::windows::core::Result<()>;
    fn Set4(&self, lrowindex: i32, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Copy(&self, prscopy: ::core::option::Option<&IITResultSet>) -> ::windows::core::Result<()>;
    fn AppendRows(&self, pressrc: ::core::option::Option<&IITResultSet>, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> ::windows::core::Result<()>;
    fn Get(&self, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> ::windows::core::Result<()>;
    fn GetKeyProp(&self, keypropid: *mut u32) -> ::windows::core::Result<()>;
    fn GetColumnPriority(&self, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> ::windows::core::Result<()>;
    fn GetRowCount(&self, lnumberofrows: *mut i32) -> ::windows::core::Result<()>;
    fn GetColumnCount(&self, lnumberofcolumns: *mut i32) -> ::windows::core::Result<()>;
    fn GetColumn(&self, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut ::core::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> ::windows::core::Result<()>;
    fn GetColumn2(&self, lcolumnindex: i32, propid: *mut u32) -> ::windows::core::Result<()>;
    fn GetColumnFromPropID(&self, propid: u32, lcolumnindex: *mut i32) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn ClearRows(&self) -> ::windows::core::Result<()>;
    fn Free(&self) -> ::windows::core::Result<()>;
    fn IsCompleted(&self) -> ::windows::core::Result<()>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn Pause(&self, fpause: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetRowStatus(&self, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> ::windows::core::Result<()>;
    fn GetColumnStatus(&self, lpcolstatus: *mut COLUMNSTATUS) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IITResultSet {}
#[cfg(feature = "Win32_Foundation")]
impl IITResultSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>() -> IITResultSet_Vtbl {
        unsafe extern "system" fn SetColumnPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, columnpriority: PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColumnPriority(::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&columnpriority)).into()
        }
        unsafe extern "system" fn SetColumnHeap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, lpvheap: *mut ::core::ffi::c_void, pfncolheapfree: PFNCOLHEAPFREE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColumnHeap(::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&lpvheap), ::core::mem::transmute_copy(&pfncolheapfree)).into()
        }
        unsafe extern "system" fn SetKeyProp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetKeyProp(::core::mem::transmute_copy(&propid)).into()
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&dwdefaultdata), ::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn Add2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, lpszwdefault: ::windows::core::PCWSTR, priority: PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add2(::core::mem::transmute_copy(&propid), ::core::mem::transmute(&lpszwdefault), ::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn Add3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, lpvdefaultdata: *mut ::core::ffi::c_void, cbdata: u32, priority: PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add3(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&lpvdefaultdata), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn Add4<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvhdr: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add4(::core::mem::transmute_copy(&lpvhdr)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::core::mem::transmute_copy(&lpvhdr), ::core::mem::transmute_copy(&lpvdata)).into()
        }
        unsafe extern "system" fn Set<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Set(::core::mem::transmute_copy(&lrowindex), ::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn Set2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, lpwstr: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Set2(::core::mem::transmute_copy(&lrowindex), ::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute(&lpwstr)).into()
        }
        unsafe extern "system" fn Set3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Set3(::core::mem::transmute_copy(&lrowindex), ::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&dwdata)).into()
        }
        unsafe extern "system" fn Set4<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Set4(::core::mem::transmute_copy(&lrowindex), ::core::mem::transmute_copy(&lpvhdr), ::core::mem::transmute_copy(&lpvdata)).into()
        }
        unsafe extern "system" fn Copy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prscopy: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Copy(::windows::core::from_raw_borrowed(&prscopy)).into()
        }
        unsafe extern "system" fn AppendRows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pressrc: *mut ::core::ffi::c_void, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AppendRows(::windows::core::from_raw_borrowed(&pressrc), ::core::mem::transmute_copy(&lrowsrcfirst), ::core::mem::transmute_copy(&csrcrows), ::core::mem::transmute_copy(&lrowfirstdest)).into()
        }
        unsafe extern "system" fn Get<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Get(::core::mem::transmute_copy(&lrowindex), ::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&prop)).into()
        }
        unsafe extern "system" fn GetKeyProp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keypropid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetKeyProp(::core::mem::transmute_copy(&keypropid)).into()
        }
        unsafe extern "system" fn GetColumnPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColumnPriority(::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&columnpriority)).into()
        }
        unsafe extern "system" fn GetRowCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumberofrows: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRowCount(::core::mem::transmute_copy(&lnumberofrows)).into()
        }
        unsafe extern "system" fn GetColumnCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumberofcolumns: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColumnCount(::core::mem::transmute_copy(&lnumberofcolumns)).into()
        }
        unsafe extern "system" fn GetColumn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut ::core::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColumn(::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&lpvdefaultvalue), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&columnpriority)).into()
        }
        unsafe extern "system" fn GetColumn2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, propid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColumn2(::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&propid)).into()
        }
        unsafe extern "system" fn GetColumnFromPropID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, lcolumnindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColumnFromPropID(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&lcolumnindex)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        unsafe extern "system" fn ClearRows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearRows().into()
        }
        unsafe extern "system" fn Free<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Free().into()
        }
        unsafe extern "system" fn IsCompleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsCompleted().into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpause: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pause(::core::mem::transmute_copy(&fpause)).into()
        }
        unsafe extern "system" fn GetRowStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRowStatus(::core::mem::transmute_copy(&lrowfirst), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&lprowstatus)).into()
        }
        unsafe extern "system" fn GetColumnStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcolstatus: *mut COLUMNSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColumnStatus(::core::mem::transmute_copy(&lpcolstatus)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetColumnPriority: SetColumnPriority::<Identity, Impl, OFFSET>,
            SetColumnHeap: SetColumnHeap::<Identity, Impl, OFFSET>,
            SetKeyProp: SetKeyProp::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Add2: Add2::<Identity, Impl, OFFSET>,
            Add3: Add3::<Identity, Impl, OFFSET>,
            Add4: Add4::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            Set: Set::<Identity, Impl, OFFSET>,
            Set2: Set2::<Identity, Impl, OFFSET>,
            Set3: Set3::<Identity, Impl, OFFSET>,
            Set4: Set4::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
            AppendRows: AppendRows::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            GetKeyProp: GetKeyProp::<Identity, Impl, OFFSET>,
            GetColumnPriority: GetColumnPriority::<Identity, Impl, OFFSET>,
            GetRowCount: GetRowCount::<Identity, Impl, OFFSET>,
            GetColumnCount: GetColumnCount::<Identity, Impl, OFFSET>,
            GetColumn: GetColumn::<Identity, Impl, OFFSET>,
            GetColumn2: GetColumn2::<Identity, Impl, OFFSET>,
            GetColumnFromPropID: GetColumnFromPropID::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            ClearRows: ClearRows::<Identity, Impl, OFFSET>,
            Free: Free::<Identity, Impl, OFFSET>,
            IsCompleted: IsCompleted::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            GetRowStatus: GetRowStatus::<Identity, Impl, OFFSET>,
            GetColumnStatus: GetColumnStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IITResultSet as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IITWordWheel_Impl: Sized {
    fn Open(&self, lpitdb: ::core::option::Option<&IITDatabase>, lpszmoniker: &::windows::core::PCWSTR, dwflags: WORD_WHEEL_OPEN_FLAGS) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::Result<()>;
    fn GetSorterInstance(&self, pdwobjinstance: *mut u32) -> ::windows::core::Result<()>;
    fn Count(&self, pcentries: *mut i32) -> ::windows::core::Result<()>;
    fn Lookup(&self, lpcvprefix: *const ::core::ffi::c_void, fexactmatch: super::super::Foundation::BOOL, plentry: *mut i32) -> ::windows::core::Result<()>;
    fn Lookup2(&self, lentry: i32, lpitresult: ::core::option::Option<&IITResultSet>, centries: i32) -> ::windows::core::Result<()>;
    fn Lookup3(&self, lentry: i32, lpvkeybuf: *mut ::core::ffi::c_void, cbkeybuf: u32) -> ::windows::core::Result<()>;
    fn SetGroup(&self, piitgroup: *mut IITGroup) -> ::windows::core::Result<()>;
    fn GetGroup(&self, ppiitgroup: *mut *mut IITGroup) -> ::windows::core::Result<()>;
    fn GetDataCount(&self, lentry: i32, pdwcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetData(&self, lentry: i32, lpitresult: ::core::option::Option<&IITResultSet>) -> ::windows::core::Result<()>;
    fn GetDataColumns(&self, prs: ::core::option::Option<&IITResultSet>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IITWordWheel {}
#[cfg(feature = "Win32_Foundation")]
impl IITWordWheel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITWordWheel_Impl, const OFFSET: isize>() -> IITWordWheel_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpitdb: *mut ::core::ffi::c_void, lpszmoniker: ::windows::core::PCWSTR, dwflags: WORD_WHEEL_OPEN_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::windows::core::from_raw_borrowed(&lpitdb), ::core::mem::transmute(&lpszmoniker), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn GetLocaleInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocaleInfo(::core::mem::transmute_copy(&pdwcodepageid), ::core::mem::transmute_copy(&plcid)).into()
        }
        unsafe extern "system" fn GetSorterInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwobjinstance: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSorterInstance(::core::mem::transmute_copy(&pdwobjinstance)).into()
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcentries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Count(::core::mem::transmute_copy(&pcentries)).into()
        }
        unsafe extern "system" fn Lookup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcvprefix: *const ::core::ffi::c_void, fexactmatch: super::super::Foundation::BOOL, plentry: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Lookup(::core::mem::transmute_copy(&lpcvprefix), ::core::mem::transmute_copy(&fexactmatch), ::core::mem::transmute_copy(&plentry)).into()
        }
        unsafe extern "system" fn Lookup2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lentry: i32, lpitresult: *mut ::core::ffi::c_void, centries: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Lookup2(::core::mem::transmute_copy(&lentry), ::windows::core::from_raw_borrowed(&lpitresult), ::core::mem::transmute_copy(&centries)).into()
        }
        unsafe extern "system" fn Lookup3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lentry: i32, lpvkeybuf: *mut ::core::ffi::c_void, cbkeybuf: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Lookup3(::core::mem::transmute_copy(&lentry), ::core::mem::transmute_copy(&lpvkeybuf), ::core::mem::transmute_copy(&cbkeybuf)).into()
        }
        unsafe extern "system" fn SetGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piitgroup: *mut IITGroup) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGroup(::core::mem::transmute_copy(&piitgroup)).into()
        }
        unsafe extern "system" fn GetGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiitgroup: *mut *mut IITGroup) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGroup(::core::mem::transmute_copy(&ppiitgroup)).into()
        }
        unsafe extern "system" fn GetDataCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lentry: i32, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDataCount(::core::mem::transmute_copy(&lentry), ::core::mem::transmute_copy(&pdwcount)).into()
        }
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lentry: i32, lpitresult: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetData(::core::mem::transmute_copy(&lentry), ::windows::core::from_raw_borrowed(&lpitresult)).into()
        }
        unsafe extern "system" fn GetDataColumns<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IITWordWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prs: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDataColumns(::windows::core::from_raw_borrowed(&prs)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetLocaleInfo: GetLocaleInfo::<Identity, Impl, OFFSET>,
            GetSorterInstance: GetSorterInstance::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Lookup: Lookup::<Identity, Impl, OFFSET>,
            Lookup2: Lookup2::<Identity, Impl, OFFSET>,
            Lookup3: Lookup3::<Identity, Impl, OFFSET>,
            SetGroup: SetGroup::<Identity, Impl, OFFSET>,
            GetGroup: GetGroup::<Identity, Impl, OFFSET>,
            GetDataCount: GetDataCount::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            GetDataColumns: GetDataColumns::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IITWordWheel as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"implement\"`*"]
pub trait IStemSink_Impl: Sized {
    fn PutAltWord(&self, pwcinbuf: &::windows::core::PCWSTR, cwc: u32) -> ::windows::core::Result<()>;
    fn PutWord(&self, pwcinbuf: &::windows::core::PCWSTR, cwc: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IStemSink {}
impl IStemSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStemSink_Impl, const OFFSET: isize>() -> IStemSink_Vtbl {
        unsafe extern "system" fn PutAltWord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStemSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcinbuf: ::windows::core::PCWSTR, cwc: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutAltWord(::core::mem::transmute(&pwcinbuf), ::core::mem::transmute_copy(&cwc)).into()
        }
        unsafe extern "system" fn PutWord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStemSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcinbuf: ::windows::core::PCWSTR, cwc: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutWord(::core::mem::transmute(&pwcinbuf), ::core::mem::transmute_copy(&cwc)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PutAltWord: PutAltWord::<Identity, Impl, OFFSET>,
            PutWord: PutWord::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStemSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IStemmerConfig_Impl: Sized {
    fn SetLocaleInfo(&self, dwcodepageid: u32, lcid: u32) -> ::windows::core::Result<()>;
    fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::Result<()>;
    fn SetControlInfo(&self, grfstemflags: u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn GetControlInfo(&self, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
    fn LoadExternalStemmerData(&self, pstream: ::core::option::Option<&super::super::System::Com::IStream>, dwextdatatype: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IStemmerConfig {}
#[cfg(feature = "Win32_System_Com")]
impl IStemmerConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStemmerConfig_Impl, const OFFSET: isize>() -> IStemmerConfig_Vtbl {
        unsafe extern "system" fn SetLocaleInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcodepageid: u32, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocaleInfo(::core::mem::transmute_copy(&dwcodepageid), ::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn GetLocaleInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocaleInfo(::core::mem::transmute_copy(&pdwcodepageid), ::core::mem::transmute_copy(&plcid)).into()
        }
        unsafe extern "system" fn SetControlInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfstemflags: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetControlInfo(::core::mem::transmute_copy(&grfstemflags), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetControlInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetControlInfo(::core::mem::transmute_copy(&pgrfstemflags), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        unsafe extern "system" fn LoadExternalStemmerData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, dwextdatatype: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadExternalStemmerData(::windows::core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&dwextdatatype)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLocaleInfo: SetLocaleInfo::<Identity, Impl, OFFSET>,
            GetLocaleInfo: GetLocaleInfo::<Identity, Impl, OFFSET>,
            SetControlInfo: SetControlInfo::<Identity, Impl, OFFSET>,
            GetControlInfo: GetControlInfo::<Identity, Impl, OFFSET>,
            LoadExternalStemmerData: LoadExternalStemmerData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStemmerConfig as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_System_Com\"`, `\"Win32_System_Search\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search"))]
pub trait IWordBreakerConfig_Impl: Sized {
    fn SetLocaleInfo(&self, dwcodepageid: u32, lcid: u32) -> ::windows::core::Result<()>;
    fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::Result<()>;
    fn SetBreakWordType(&self, dwbreakwordtype: u32) -> ::windows::core::Result<()>;
    fn GetBreakWordType(&self, pdwbreakwordtype: *mut u32) -> ::windows::core::Result<()>;
    fn SetControlInfo(&self, grfbreakflags: u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn GetControlInfo(&self, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
    fn LoadExternalBreakerData(&self, pstream: ::core::option::Option<&super::super::System::Com::IStream>, dwextdatatype: u32) -> ::windows::core::Result<()>;
    fn SetWordStemmer(&self, rclsid: *const ::windows::core::GUID, pstemmer: ::core::option::Option<&super::super::System::Search::IStemmer>) -> ::windows::core::Result<()>;
    fn GetWordStemmer(&self) -> ::windows::core::Result<super::super::System::Search::IStemmer>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search"))]
impl ::windows::core::RuntimeName for IWordBreakerConfig {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search"))]
impl IWordBreakerConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: isize>() -> IWordBreakerConfig_Vtbl {
        unsafe extern "system" fn SetLocaleInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcodepageid: u32, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocaleInfo(::core::mem::transmute_copy(&dwcodepageid), ::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn GetLocaleInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocaleInfo(::core::mem::transmute_copy(&pdwcodepageid), ::core::mem::transmute_copy(&plcid)).into()
        }
        unsafe extern "system" fn SetBreakWordType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbreakwordtype: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBreakWordType(::core::mem::transmute_copy(&dwbreakwordtype)).into()
        }
        unsafe extern "system" fn GetBreakWordType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbreakwordtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBreakWordType(::core::mem::transmute_copy(&pdwbreakwordtype)).into()
        }
        unsafe extern "system" fn SetControlInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbreakflags: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetControlInfo(::core::mem::transmute_copy(&grfbreakflags), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetControlInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetControlInfo(::core::mem::transmute_copy(&pgrfbreakflags), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        unsafe extern "system" fn LoadExternalBreakerData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, dwextdatatype: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadExternalBreakerData(::windows::core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&dwextdatatype)).into()
        }
        unsafe extern "system" fn SetWordStemmer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, pstemmer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWordStemmer(::core::mem::transmute_copy(&rclsid), ::windows::core::from_raw_borrowed(&pstemmer)).into()
        }
        unsafe extern "system" fn GetWordStemmer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstemmer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWordStemmer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstemmer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLocaleInfo: SetLocaleInfo::<Identity, Impl, OFFSET>,
            GetLocaleInfo: GetLocaleInfo::<Identity, Impl, OFFSET>,
            SetBreakWordType: SetBreakWordType::<Identity, Impl, OFFSET>,
            GetBreakWordType: GetBreakWordType::<Identity, Impl, OFFSET>,
            SetControlInfo: SetControlInfo::<Identity, Impl, OFFSET>,
            GetControlInfo: GetControlInfo::<Identity, Impl, OFFSET>,
            LoadExternalBreakerData: LoadExternalBreakerData::<Identity, Impl, OFFSET>,
            SetWordStemmer: SetWordStemmer::<Identity, Impl, OFFSET>,
            GetWordStemmer: GetWordStemmer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWordBreakerConfig as ::windows::core::ComInterface>::IID
    }
}
