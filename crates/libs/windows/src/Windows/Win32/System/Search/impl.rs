pub trait DataSource_Impl: Sized {
    fn getDataMember(&mut self, bstrdm: *const u16, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn getDataMemberName(&mut self, lindex: i32) -> ::windows::core::Result<*mut u16>;
    fn getDataMemberCount(&mut self) -> ::windows::core::Result<i32>;
    fn addDataSourceListener(&mut self, pdsl: &::core::option::Option<DataSourceListener>) -> ::windows::core::Result<()>;
    fn removeDataSourceListener(&mut self, pdsl: &::core::option::Option<DataSourceListener>) -> ::windows::core::Result<()>;
}
impl DataSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DataSource_Impl, const OFFSET: isize>() -> DataSource_Vtbl {
        unsafe extern "system" fn getDataMember<Identity: ::windows::core::IUnknownImpl, Impl: DataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdm: *const u16, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getDataMember(::core::mem::transmute_copy(&bstrdm), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getDataMemberName<Identity: ::windows::core::IUnknownImpl, Impl: DataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrdm: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getDataMemberName(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getDataMemberCount<Identity: ::windows::core::IUnknownImpl, Impl: DataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getDataMemberCount() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addDataSourceListener<Identity: ::windows::core::IUnknownImpl, Impl: DataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsl: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).addDataSourceListener(::core::mem::transmute(&pdsl)).into()
        }
        unsafe extern "system" fn removeDataSourceListener<Identity: ::windows::core::IUnknownImpl, Impl: DataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsl: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).removeDataSourceListener(::core::mem::transmute(&pdsl)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            getDataMember: getDataMember::<Identity, Impl, OFFSET>,
            getDataMemberName: getDataMemberName::<Identity, Impl, OFFSET>,
            getDataMemberCount: getDataMemberCount::<Identity, Impl, OFFSET>,
            addDataSourceListener: addDataSourceListener::<Identity, Impl, OFFSET>,
            removeDataSourceListener: removeDataSourceListener::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DataSource as ::windows::core::Interface>::IID
    }
}
pub trait DataSourceListener_Impl: Sized {
    fn dataMemberChanged(&mut self, bstrdm: *const u16) -> ::windows::core::Result<()>;
    fn dataMemberAdded(&mut self, bstrdm: *const u16) -> ::windows::core::Result<()>;
    fn dataMemberRemoved(&mut self, bstrdm: *const u16) -> ::windows::core::Result<()>;
}
impl DataSourceListener_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DataSourceListener_Impl, const OFFSET: isize>() -> DataSourceListener_Vtbl {
        unsafe extern "system" fn dataMemberChanged<Identity: ::windows::core::IUnknownImpl, Impl: DataSourceListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdm: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).dataMemberChanged(::core::mem::transmute_copy(&bstrdm)).into()
        }
        unsafe extern "system" fn dataMemberAdded<Identity: ::windows::core::IUnknownImpl, Impl: DataSourceListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdm: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).dataMemberAdded(::core::mem::transmute_copy(&bstrdm)).into()
        }
        unsafe extern "system" fn dataMemberRemoved<Identity: ::windows::core::IUnknownImpl, Impl: DataSourceListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdm: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).dataMemberRemoved(::core::mem::transmute_copy(&bstrdm)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            dataMemberChanged: dataMemberChanged::<Identity, Impl, OFFSET>,
            dataMemberAdded: dataMemberAdded::<Identity, Impl, OFFSET>,
            dataMemberRemoved: dataMemberRemoved::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DataSourceListener as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DataSourceObject_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DataSourceObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DataSourceObject_Impl, const OFFSET: isize>() -> DataSourceObject_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DataSourceObject as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessor_Impl: Sized {
    fn AddRefAccessor(&mut self, haccessor: usize) -> ::windows::core::Result<u32>;
    fn CreateAccessor(&mut self, dwaccessorflags: u32, cbindings: usize, rgbindings: *const DBBINDING, cbrowsize: usize, phaccessor: *mut usize, rgstatus: *mut u32) -> ::windows::core::Result<()>;
    fn GetBindings(&mut self, haccessor: usize, pdwaccessorflags: *mut u32, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> ::windows::core::Result<()>;
    fn ReleaseAccessor(&mut self, haccessor: usize) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAccessor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessor_Impl, const OFFSET: isize>() -> IAccessor_Vtbl {
        unsafe extern "system" fn AddRefAccessor<Identity: ::windows::core::IUnknownImpl, Impl: IAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haccessor: usize, pcrefcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddRefAccessor(::core::mem::transmute_copy(&haccessor)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcrefcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAccessor<Identity: ::windows::core::IUnknownImpl, Impl: IAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaccessorflags: u32, cbindings: usize, rgbindings: *const DBBINDING, cbrowsize: usize, phaccessor: *mut usize, rgstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateAccessor(::core::mem::transmute_copy(&dwaccessorflags), ::core::mem::transmute_copy(&cbindings), ::core::mem::transmute_copy(&rgbindings), ::core::mem::transmute_copy(&cbrowsize), ::core::mem::transmute_copy(&phaccessor), ::core::mem::transmute_copy(&rgstatus)).into()
        }
        unsafe extern "system" fn GetBindings<Identity: ::windows::core::IUnknownImpl, Impl: IAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haccessor: usize, pdwaccessorflags: *mut u32, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBindings(::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdwaccessorflags), ::core::mem::transmute_copy(&pcbindings), ::core::mem::transmute_copy(&prgbindings)).into()
        }
        unsafe extern "system" fn ReleaseAccessor<Identity: ::windows::core::IUnknownImpl, Impl: IAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haccessor: usize, pcrefcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReleaseAccessor(::core::mem::transmute_copy(&haccessor)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcrefcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddRefAccessor: AddRefAccessor::<Identity, Impl, OFFSET>,
            CreateAccessor: CreateAccessor::<Identity, Impl, OFFSET>,
            GetBindings: GetBindings::<Identity, Impl, OFFSET>,
            ReleaseAccessor: ReleaseAccessor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAlterIndex_Impl: Sized {
    fn AlterIndex(&mut self, ptableid: *mut super::super::Storage::IndexServer::DBID, pindexid: *mut super::super::Storage::IndexServer::DBID, pnewindexid: *mut super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAlterIndex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAlterIndex_Impl, const OFFSET: isize>() -> IAlterIndex_Vtbl {
        unsafe extern "system" fn AlterIndex<Identity: ::windows::core::IUnknownImpl, Impl: IAlterIndex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, pindexid: *mut super::super::Storage::IndexServer::DBID, pnewindexid: *mut super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AlterIndex(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pindexid), ::core::mem::transmute_copy(&pnewindexid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AlterIndex: AlterIndex::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAlterIndex as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAlterTable_Impl: Sized {
    fn AlterColumn(&mut self, ptableid: *mut super::super::Storage::IndexServer::DBID, pcolumnid: *mut super::super::Storage::IndexServer::DBID, dwcolumndescflags: u32, pcolumndesc: *mut DBCOLUMNDESC) -> ::windows::core::Result<()>;
    fn AlterTable(&mut self, ptableid: *mut super::super::Storage::IndexServer::DBID, pnewtableid: *mut super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAlterTable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAlterTable_Impl, const OFFSET: isize>() -> IAlterTable_Vtbl {
        unsafe extern "system" fn AlterColumn<Identity: ::windows::core::IUnknownImpl, Impl: IAlterTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, pcolumnid: *mut super::super::Storage::IndexServer::DBID, dwcolumndescflags: u32, pcolumndesc: *mut DBCOLUMNDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AlterColumn(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pcolumnid), ::core::mem::transmute_copy(&dwcolumndescflags), ::core::mem::transmute_copy(&pcolumndesc)).into()
        }
        unsafe extern "system" fn AlterTable<Identity: ::windows::core::IUnknownImpl, Impl: IAlterTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, pnewtableid: *mut super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AlterTable(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pnewtableid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AlterColumn: AlterColumn::<Identity, Impl, OFFSET>,
            AlterTable: AlterTable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAlterTable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IBindResource_Impl: Sized {
    fn Bind(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, pwszurl: super::super::Foundation::PWSTR, dwbindurlflags: u32, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pauthenticate: &::core::option::Option<super::Com::IAuthenticate>, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IBindResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindResource_Impl, const OFFSET: isize>() -> IBindResource_Vtbl {
        unsafe extern "system" fn Bind<Identity: ::windows::core::IUnknownImpl, Impl: IBindResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, dwbindurlflags: u32, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pauthenticate: ::windows::core::RawPtr, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Bind(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&dwbindurlflags), ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute(&pauthenticate), ::core::mem::transmute_copy(&pimplsession), ::core::mem::transmute_copy(&pdwbindstatus), ::core::mem::transmute_copy(&ppunk)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Bind: Bind::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindResource as ::windows::core::Interface>::IID
    }
}
pub trait IChapteredRowset_Impl: Sized {
    fn AddRefChapter(&mut self, hchapter: usize) -> ::windows::core::Result<u32>;
    fn ReleaseChapter(&mut self, hchapter: usize) -> ::windows::core::Result<u32>;
}
impl IChapteredRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChapteredRowset_Impl, const OFFSET: isize>() -> IChapteredRowset_Vtbl {
        unsafe extern "system" fn AddRefChapter<Identity: ::windows::core::IUnknownImpl, Impl: IChapteredRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, pcrefcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddRefChapter(::core::mem::transmute_copy(&hchapter)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcrefcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseChapter<Identity: ::windows::core::IUnknownImpl, Impl: IChapteredRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, pcrefcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReleaseChapter(::core::mem::transmute_copy(&hchapter)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcrefcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddRefChapter: AddRefChapter::<Identity, Impl, OFFSET>,
            ReleaseChapter: ReleaseChapter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChapteredRowset as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
pub trait IColumnMapper_Impl: Sized {
    fn GetPropInfoFromName(&mut self, wcspropname: super::super::Foundation::PWSTR, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows::core::Result<()>;
    fn GetPropInfoFromId(&mut self, ppropid: *const super::super::Storage::IndexServer::DBID, pwcsname: *mut *mut u16, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows::core::Result<()>;
    fn EnumPropInfo(&mut self, ientry: u32, pwcsname: *const *const u16, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows::core::Result<()>;
    fn IsMapUpToDate(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
impl IColumnMapper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColumnMapper_Impl, const OFFSET: isize>() -> IColumnMapper_Vtbl {
        unsafe extern "system" fn GetPropInfoFromName<Identity: ::windows::core::IUnknownImpl, Impl: IColumnMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wcspropname: super::super::Foundation::PWSTR, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropInfoFromName(::core::mem::transmute_copy(&wcspropname), ::core::mem::transmute_copy(&pppropid), ::core::mem::transmute_copy(&pproptype), ::core::mem::transmute_copy(&puiwidth)).into()
        }
        unsafe extern "system" fn GetPropInfoFromId<Identity: ::windows::core::IUnknownImpl, Impl: IColumnMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropid: *const super::super::Storage::IndexServer::DBID, pwcsname: *mut *mut u16, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropInfoFromId(::core::mem::transmute_copy(&ppropid), ::core::mem::transmute_copy(&pwcsname), ::core::mem::transmute_copy(&pproptype), ::core::mem::transmute_copy(&puiwidth)).into()
        }
        unsafe extern "system" fn EnumPropInfo<Identity: ::windows::core::IUnknownImpl, Impl: IColumnMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ientry: u32, pwcsname: *const *const u16, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumPropInfo(::core::mem::transmute_copy(&ientry), ::core::mem::transmute_copy(&pwcsname), ::core::mem::transmute_copy(&pppropid), ::core::mem::transmute_copy(&pproptype), ::core::mem::transmute_copy(&puiwidth)).into()
        }
        unsafe extern "system" fn IsMapUpToDate<Identity: ::windows::core::IUnknownImpl, Impl: IColumnMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsMapUpToDate().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPropInfoFromName: GetPropInfoFromName::<Identity, Impl, OFFSET>,
            GetPropInfoFromId: GetPropInfoFromId::<Identity, Impl, OFFSET>,
            EnumPropInfo: EnumPropInfo::<Identity, Impl, OFFSET>,
            IsMapUpToDate: IsMapUpToDate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColumnMapper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IColumnMapperCreator_Impl: Sized {
    fn GetColumnMapper(&mut self, wcsmachinename: super::super::Foundation::PWSTR, wcscatalogname: super::super::Foundation::PWSTR) -> ::windows::core::Result<IColumnMapper>;
}
#[cfg(feature = "Win32_Foundation")]
impl IColumnMapperCreator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColumnMapperCreator_Impl, const OFFSET: isize>() -> IColumnMapperCreator_Vtbl {
        unsafe extern "system" fn GetColumnMapper<Identity: ::windows::core::IUnknownImpl, Impl: IColumnMapperCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wcsmachinename: super::super::Foundation::PWSTR, wcscatalogname: super::super::Foundation::PWSTR, ppcolumnmapper: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetColumnMapper(::core::mem::transmute_copy(&wcsmachinename), ::core::mem::transmute_copy(&wcscatalogname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolumnmapper = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetColumnMapper: GetColumnMapper::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColumnMapperCreator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait IColumnsInfo_Impl: Sized {
    fn GetColumnInfo(&mut self, pccolumns: *mut usize, prginfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
    fn MapColumnIDs(&mut self, ccolumnids: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgcolumns: *mut usize) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl IColumnsInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColumnsInfo_Impl, const OFFSET: isize>() -> IColumnsInfo_Vtbl {
        unsafe extern "system" fn GetColumnInfo<Identity: ::windows::core::IUnknownImpl, Impl: IColumnsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccolumns: *mut usize, prginfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetColumnInfo(::core::mem::transmute_copy(&pccolumns), ::core::mem::transmute_copy(&prginfo), ::core::mem::transmute_copy(&ppstringsbuffer)).into()
        }
        unsafe extern "system" fn MapColumnIDs<Identity: ::windows::core::IUnknownImpl, Impl: IColumnsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolumnids: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgcolumns: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MapColumnIDs(::core::mem::transmute_copy(&ccolumnids), ::core::mem::transmute_copy(&rgcolumnids), ::core::mem::transmute_copy(&rgcolumns)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetColumnInfo: GetColumnInfo::<Identity, Impl, OFFSET>,
            MapColumnIDs: MapColumnIDs::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColumnsInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait IColumnsInfo2_Impl: Sized + IColumnsInfo_Impl {
    fn GetRestrictedColumnInfo(&mut self, ccolumnidmasks: usize, rgcolumnidmasks: *const super::super::Storage::IndexServer::DBID, dwflags: u32, pccolumns: *mut usize, prgcolumnids: *mut *mut super::super::Storage::IndexServer::DBID, prgcolumninfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl IColumnsInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColumnsInfo2_Impl, const OFFSET: isize>() -> IColumnsInfo2_Vtbl {
        unsafe extern "system" fn GetRestrictedColumnInfo<Identity: ::windows::core::IUnknownImpl, Impl: IColumnsInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolumnidmasks: usize, rgcolumnidmasks: *const super::super::Storage::IndexServer::DBID, dwflags: u32, pccolumns: *mut usize, prgcolumnids: *mut *mut super::super::Storage::IndexServer::DBID, prgcolumninfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRestrictedColumnInfo(::core::mem::transmute_copy(&ccolumnidmasks), ::core::mem::transmute_copy(&rgcolumnidmasks), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pccolumns), ::core::mem::transmute_copy(&prgcolumnids), ::core::mem::transmute_copy(&prgcolumninfo), ::core::mem::transmute_copy(&ppstringsbuffer)).into()
        }
        Self { base: IColumnsInfo_Vtbl::new::<Identity, Impl, OFFSET>(), GetRestrictedColumnInfo: GetRestrictedColumnInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColumnsInfo2 as ::windows::core::Interface>::IID || iid == &<IColumnsInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IColumnsRowset_Impl: Sized {
    fn GetAvailableColumns(&mut self, pcoptcolumns: *mut usize, prgoptcolumns: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
    fn GetColumnsRowset(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, coptcolumns: usize, rgoptcolumns: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppcolrowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IColumnsRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColumnsRowset_Impl, const OFFSET: isize>() -> IColumnsRowset_Vtbl {
        unsafe extern "system" fn GetAvailableColumns<Identity: ::windows::core::IUnknownImpl, Impl: IColumnsRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoptcolumns: *mut usize, prgoptcolumns: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAvailableColumns(::core::mem::transmute_copy(&pcoptcolumns), ::core::mem::transmute_copy(&prgoptcolumns)).into()
        }
        unsafe extern "system" fn GetColumnsRowset<Identity: ::windows::core::IUnknownImpl, Impl: IColumnsRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, coptcolumns: usize, rgoptcolumns: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppcolrowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetColumnsRowset(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&coptcolumns), ::core::mem::transmute_copy(&rgoptcolumns), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&ppcolrowset)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetAvailableColumns: GetAvailableColumns::<Identity, Impl, OFFSET>,
            GetColumnsRowset: GetColumnsRowset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColumnsRowset as ::windows::core::Interface>::IID
    }
}
pub trait ICommand_Impl: Sized {
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn Execute(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID, pparams: *mut DBPARAMS, pcrowsaffected: *mut isize, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetDBSession(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ICommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommand_Impl, const OFFSET: isize>() -> ICommand_Vtbl {
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: ICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Execute<Identity: ::windows::core::IUnknownImpl, Impl: ICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pparams: *mut DBPARAMS, pcrowsaffected: *mut isize, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Execute(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&pcrowsaffected), ::core::mem::transmute_copy(&pprowset)).into()
        }
        unsafe extern "system" fn GetDBSession<Identity: ::windows::core::IUnknownImpl, Impl: ICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDBSession(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Execute: Execute::<Identity, Impl, OFFSET>,
            GetDBSession: GetDBSession::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommand as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICommandCost_Impl: Sized {
    fn GetAccumulatedCost(&mut self, pwszrowsetname: super::super::Foundation::PWSTR, pccostlimits: *mut u32, prgcostlimits: *mut *mut DBCOST) -> ::windows::core::Result<()>;
    fn GetCostEstimate(&mut self, pwszrowsetname: super::super::Foundation::PWSTR, pccostestimates: *mut u32, prgcostestimates: *mut DBCOST) -> ::windows::core::Result<()>;
    fn GetCostGoals(&mut self, pwszrowsetname: super::super::Foundation::PWSTR, pccostgoals: *mut u32, prgcostgoals: *mut DBCOST) -> ::windows::core::Result<()>;
    fn GetCostLimits(&mut self, pwszrowsetname: super::super::Foundation::PWSTR, pccostlimits: *mut u32, prgcostlimits: *mut DBCOST) -> ::windows::core::Result<()>;
    fn SetCostGoals(&mut self, pwszrowsetname: super::super::Foundation::PWSTR, ccostgoals: u32, rgcostgoals: *const DBCOST) -> ::windows::core::Result<()>;
    fn SetCostLimits(&mut self, pwszrowsetname: super::super::Foundation::PWSTR, ccostlimits: u32, prgcostlimits: *mut DBCOST, dwexecutionflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICommandCost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandCost_Impl, const OFFSET: isize>() -> ICommandCost_Vtbl {
        unsafe extern "system" fn GetAccumulatedCost<Identity: ::windows::core::IUnknownImpl, Impl: ICommandCost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszrowsetname: super::super::Foundation::PWSTR, pccostlimits: *mut u32, prgcostlimits: *mut *mut DBCOST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAccumulatedCost(::core::mem::transmute_copy(&pwszrowsetname), ::core::mem::transmute_copy(&pccostlimits), ::core::mem::transmute_copy(&prgcostlimits)).into()
        }
        unsafe extern "system" fn GetCostEstimate<Identity: ::windows::core::IUnknownImpl, Impl: ICommandCost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszrowsetname: super::super::Foundation::PWSTR, pccostestimates: *mut u32, prgcostestimates: *mut DBCOST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCostEstimate(::core::mem::transmute_copy(&pwszrowsetname), ::core::mem::transmute_copy(&pccostestimates), ::core::mem::transmute_copy(&prgcostestimates)).into()
        }
        unsafe extern "system" fn GetCostGoals<Identity: ::windows::core::IUnknownImpl, Impl: ICommandCost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszrowsetname: super::super::Foundation::PWSTR, pccostgoals: *mut u32, prgcostgoals: *mut DBCOST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCostGoals(::core::mem::transmute_copy(&pwszrowsetname), ::core::mem::transmute_copy(&pccostgoals), ::core::mem::transmute_copy(&prgcostgoals)).into()
        }
        unsafe extern "system" fn GetCostLimits<Identity: ::windows::core::IUnknownImpl, Impl: ICommandCost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszrowsetname: super::super::Foundation::PWSTR, pccostlimits: *mut u32, prgcostlimits: *mut DBCOST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCostLimits(::core::mem::transmute_copy(&pwszrowsetname), ::core::mem::transmute_copy(&pccostlimits), ::core::mem::transmute_copy(&prgcostlimits)).into()
        }
        unsafe extern "system" fn SetCostGoals<Identity: ::windows::core::IUnknownImpl, Impl: ICommandCost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszrowsetname: super::super::Foundation::PWSTR, ccostgoals: u32, rgcostgoals: *const DBCOST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCostGoals(::core::mem::transmute_copy(&pwszrowsetname), ::core::mem::transmute_copy(&ccostgoals), ::core::mem::transmute_copy(&rgcostgoals)).into()
        }
        unsafe extern "system" fn SetCostLimits<Identity: ::windows::core::IUnknownImpl, Impl: ICommandCost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszrowsetname: super::super::Foundation::PWSTR, ccostlimits: u32, prgcostlimits: *mut DBCOST, dwexecutionflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCostLimits(::core::mem::transmute_copy(&pwszrowsetname), ::core::mem::transmute_copy(&ccostlimits), ::core::mem::transmute_copy(&prgcostlimits), ::core::mem::transmute_copy(&dwexecutionflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetAccumulatedCost: GetAccumulatedCost::<Identity, Impl, OFFSET>,
            GetCostEstimate: GetCostEstimate::<Identity, Impl, OFFSET>,
            GetCostGoals: GetCostGoals::<Identity, Impl, OFFSET>,
            GetCostLimits: GetCostLimits::<Identity, Impl, OFFSET>,
            SetCostGoals: SetCostGoals::<Identity, Impl, OFFSET>,
            SetCostLimits: SetCostLimits::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandCost as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
pub trait ICommandPersist_Impl: Sized {
    fn DeleteCommand(&mut self, pcommandid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
    fn GetCurrentCommand(&mut self, ppcommandid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
    fn LoadCommand(&mut self, pcommandid: *mut super::super::Storage::IndexServer::DBID, dwflags: u32) -> ::windows::core::Result<()>;
    fn SaveCommand(&mut self, pcommandid: *mut super::super::Storage::IndexServer::DBID, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
impl ICommandPersist_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandPersist_Impl, const OFFSET: isize>() -> ICommandPersist_Vtbl {
        unsafe extern "system" fn DeleteCommand<Identity: ::windows::core::IUnknownImpl, Impl: ICommandPersist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommandid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteCommand(::core::mem::transmute_copy(&pcommandid)).into()
        }
        unsafe extern "system" fn GetCurrentCommand<Identity: ::windows::core::IUnknownImpl, Impl: ICommandPersist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcommandid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCurrentCommand(::core::mem::transmute_copy(&ppcommandid)).into()
        }
        unsafe extern "system" fn LoadCommand<Identity: ::windows::core::IUnknownImpl, Impl: ICommandPersist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommandid: *mut super::super::Storage::IndexServer::DBID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadCommand(::core::mem::transmute_copy(&pcommandid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SaveCommand<Identity: ::windows::core::IUnknownImpl, Impl: ICommandPersist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommandid: *mut super::super::Storage::IndexServer::DBID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveCommand(::core::mem::transmute_copy(&pcommandid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DeleteCommand: DeleteCommand::<Identity, Impl, OFFSET>,
            GetCurrentCommand: GetCurrentCommand::<Identity, Impl, OFFSET>,
            LoadCommand: LoadCommand::<Identity, Impl, OFFSET>,
            SaveCommand: SaveCommand::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandPersist as ::windows::core::Interface>::IID
    }
}
pub trait ICommandPrepare_Impl: Sized {
    fn Prepare(&mut self, cexpectedruns: u32) -> ::windows::core::Result<()>;
    fn Unprepare(&mut self) -> ::windows::core::Result<()>;
}
impl ICommandPrepare_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandPrepare_Impl, const OFFSET: isize>() -> ICommandPrepare_Vtbl {
        unsafe extern "system" fn Prepare<Identity: ::windows::core::IUnknownImpl, Impl: ICommandPrepare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexpectedruns: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Prepare(::core::mem::transmute_copy(&cexpectedruns)).into()
        }
        unsafe extern "system" fn Unprepare<Identity: ::windows::core::IUnknownImpl, Impl: ICommandPrepare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unprepare().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Prepare: Prepare::<Identity, Impl, OFFSET>,
            Unprepare: Unprepare::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandPrepare as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICommandProperties_Impl: Sized {
    fn GetProperties(&mut self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::Result<()>;
    fn SetProperties(&mut self, cpropertysets: u32, rgpropertysets: *const DBPROPSET) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICommandProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandProperties_Impl, const OFFSET: isize>() -> ICommandProperties_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl, Impl: ICommandProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProperties(::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)).into()
        }
        unsafe extern "system" fn SetProperties<Identity: ::windows::core::IUnknownImpl, Impl: ICommandProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *const DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperties(::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            SetProperties: SetProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandProperties as ::windows::core::Interface>::IID
    }
}
pub trait ICommandStream_Impl: Sized {
    fn GetCommandStream(&mut self, piid: *mut ::windows::core::GUID, pguiddialect: *mut ::windows::core::GUID, ppcommandstream: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetCommandStream(&mut self, riid: *const ::windows::core::GUID, rguiddialect: *const ::windows::core::GUID, pcommandstream: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ICommandStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandStream_Impl, const OFFSET: isize>() -> ICommandStream_Vtbl {
        unsafe extern "system" fn GetCommandStream<Identity: ::windows::core::IUnknownImpl, Impl: ICommandStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID, pguiddialect: *mut ::windows::core::GUID, ppcommandstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCommandStream(::core::mem::transmute_copy(&piid), ::core::mem::transmute_copy(&pguiddialect), ::core::mem::transmute_copy(&ppcommandstream)).into()
        }
        unsafe extern "system" fn SetCommandStream<Identity: ::windows::core::IUnknownImpl, Impl: ICommandStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rguiddialect: *const ::windows::core::GUID, pcommandstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCommandStream(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&rguiddialect), ::core::mem::transmute(&pcommandstream)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCommandStream: GetCommandStream::<Identity, Impl, OFFSET>,
            SetCommandStream: SetCommandStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICommandText_Impl: Sized + ICommand_Impl {
    fn GetCommandText(&mut self, pguiddialect: *mut ::windows::core::GUID, ppwszcommand: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetCommandText(&mut self, rguiddialect: *const ::windows::core::GUID, pwszcommand: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICommandText_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandText_Impl, const OFFSET: isize>() -> ICommandText_Vtbl {
        unsafe extern "system" fn GetCommandText<Identity: ::windows::core::IUnknownImpl, Impl: ICommandText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguiddialect: *mut ::windows::core::GUID, ppwszcommand: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCommandText(::core::mem::transmute_copy(&pguiddialect), ::core::mem::transmute_copy(&ppwszcommand)).into()
        }
        unsafe extern "system" fn SetCommandText<Identity: ::windows::core::IUnknownImpl, Impl: ICommandText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguiddialect: *const ::windows::core::GUID, pwszcommand: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCommandText(::core::mem::transmute_copy(&rguiddialect), ::core::mem::transmute_copy(&pwszcommand)).into()
        }
        Self {
            base: ICommand_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCommandText: GetCommandText::<Identity, Impl, OFFSET>,
            SetCommandText: SetCommandText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandText as ::windows::core::Interface>::IID || iid == &<ICommand as ::windows::core::Interface>::IID
    }
}
pub trait ICommandValidate_Impl: Sized {
    fn ValidateCompletely(&mut self) -> ::windows::core::Result<()>;
    fn ValidateSyntax(&mut self) -> ::windows::core::Result<()>;
}
impl ICommandValidate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandValidate_Impl, const OFFSET: isize>() -> ICommandValidate_Vtbl {
        unsafe extern "system" fn ValidateCompletely<Identity: ::windows::core::IUnknownImpl, Impl: ICommandValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ValidateCompletely().into()
        }
        unsafe extern "system" fn ValidateSyntax<Identity: ::windows::core::IUnknownImpl, Impl: ICommandValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ValidateSyntax().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ValidateCompletely: ValidateCompletely::<Identity, Impl, OFFSET>,
            ValidateSyntax: ValidateSyntax::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandValidate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICommandWithParameters_Impl: Sized {
    fn GetParameterInfo(&mut self, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
    fn MapParameterNames(&mut self, cparamnames: usize, rgparamnames: *const super::super::Foundation::PWSTR, rgparamordinals: *mut isize) -> ::windows::core::Result<()>;
    fn SetParameterInfo(&mut self, cparams: usize, rgparamordinals: *const usize, rgparambindinfo: *const DBPARAMBINDINFO) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICommandWithParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandWithParameters_Impl, const OFFSET: isize>() -> ICommandWithParameters_Vtbl {
        unsafe extern "system" fn GetParameterInfo<Identity: ::windows::core::IUnknownImpl, Impl: ICommandWithParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetParameterInfo(::core::mem::transmute_copy(&pcparams), ::core::mem::transmute_copy(&prgparaminfo), ::core::mem::transmute_copy(&ppnamesbuffer)).into()
        }
        unsafe extern "system" fn MapParameterNames<Identity: ::windows::core::IUnknownImpl, Impl: ICommandWithParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cparamnames: usize, rgparamnames: *const super::super::Foundation::PWSTR, rgparamordinals: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MapParameterNames(::core::mem::transmute_copy(&cparamnames), ::core::mem::transmute_copy(&rgparamnames), ::core::mem::transmute_copy(&rgparamordinals)).into()
        }
        unsafe extern "system" fn SetParameterInfo<Identity: ::windows::core::IUnknownImpl, Impl: ICommandWithParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cparams: usize, rgparamordinals: *const usize, rgparambindinfo: *const DBPARAMBINDINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetParameterInfo(::core::mem::transmute_copy(&cparams), ::core::mem::transmute_copy(&rgparamordinals), ::core::mem::transmute_copy(&rgparambindinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetParameterInfo: GetParameterInfo::<Identity, Impl, OFFSET>,
            MapParameterNames: MapParameterNames::<Identity, Impl, OFFSET>,
            SetParameterInfo: SetParameterInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandWithParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
pub trait ICondition_Impl: Sized + super::Com::IPersist_Impl + super::Com::IPersistStream_Impl {
    fn GetConditionType(&mut self) -> ::windows::core::Result<Common::CONDITION_TYPE>;
    fn GetSubConditions(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetComparisonInfo(&mut self, ppszpropertyname: *mut super::super::Foundation::PWSTR, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetValueType(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetValueNormalization(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetInputTerms(&mut self, pppropertyterm: *mut ::core::option::Option<IRichChunk>, ppoperationterm: *mut ::core::option::Option<IRichChunk>, ppvalueterm: *mut ::core::option::Option<IRichChunk>) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<ICondition>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl ICondition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICondition_Impl, const OFFSET: isize>() -> ICondition_Vtbl {
        unsafe extern "system" fn GetConditionType<Identity: ::windows::core::IUnknownImpl, Impl: ICondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnodetype: *mut Common::CONDITION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConditionType() {
                ::core::result::Result::Ok(ok__) => {
                    *pnodetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubConditions<Identity: ::windows::core::IUnknownImpl, Impl: ICondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSubConditions(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetComparisonInfo<Identity: ::windows::core::IUnknownImpl, Impl: ICondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpropertyname: *mut super::super::Foundation::PWSTR, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetComparisonInfo(::core::mem::transmute_copy(&ppszpropertyname), ::core::mem::transmute_copy(&pcop), ::core::mem::transmute_copy(&ppropvar)).into()
        }
        unsafe extern "system" fn GetValueType<Identity: ::windows::core::IUnknownImpl, Impl: ICondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszvaluetypename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetValueType() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszvaluetypename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueNormalization<Identity: ::windows::core::IUnknownImpl, Impl: ICondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsznormalization: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetValueNormalization() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsznormalization = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTerms<Identity: ::windows::core::IUnknownImpl, Impl: ICondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyterm: *mut ::windows::core::RawPtr, ppoperationterm: *mut ::windows::core::RawPtr, ppvalueterm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInputTerms(::core::mem::transmute_copy(&pppropertyterm), ::core::mem::transmute_copy(&ppoperationterm), ::core::mem::transmute_copy(&ppvalueterm)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: ICondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IPersistStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetConditionType: GetConditionType::<Identity, Impl, OFFSET>,
            GetSubConditions: GetSubConditions::<Identity, Impl, OFFSET>,
            GetComparisonInfo: GetComparisonInfo::<Identity, Impl, OFFSET>,
            GetValueType: GetValueType::<Identity, Impl, OFFSET>,
            GetValueNormalization: GetValueNormalization::<Identity, Impl, OFFSET>,
            GetInputTerms: GetInputTerms::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICondition as ::windows::core::Interface>::IID || iid == &<super::Com::IPersist as ::windows::core::Interface>::IID || iid == &<super::Com::IPersistStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ICondition2_Impl: Sized + super::Com::IPersist_Impl + super::Com::IPersistStream_Impl + ICondition_Impl {
    fn GetLocale(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetLeafConditionInfo(&mut self, ppropkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ICondition2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICondition2_Impl, const OFFSET: isize>() -> ICondition2_Vtbl {
        unsafe extern "system" fn GetLocale<Identity: ::windows::core::IUnknownImpl, Impl: ICondition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszlocalename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLocale() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszlocalename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLeafConditionInfo<Identity: ::windows::core::IUnknownImpl, Impl: ICondition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLeafConditionInfo(::core::mem::transmute_copy(&ppropkey), ::core::mem::transmute_copy(&pcop), ::core::mem::transmute_copy(&ppropvar)).into()
        }
        Self {
            base: ICondition_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLocale: GetLocale::<Identity, Impl, OFFSET>,
            GetLeafConditionInfo: GetLeafConditionInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICondition2 as ::windows::core::Interface>::IID || iid == &<super::Com::IPersist as ::windows::core::Interface>::IID || iid == &<super::Com::IPersistStream as ::windows::core::Interface>::IID || iid == &<ICondition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
pub trait IConditionFactory_Impl: Sized {
    fn MakeNot(&mut self, pcsub: &::core::option::Option<ICondition>, fsimplify: super::super::Foundation::BOOL) -> ::windows::core::Result<ICondition>;
    fn MakeAndOr(&mut self, ct: Common::CONDITION_TYPE, peusubs: &::core::option::Option<super::Com::IEnumUnknown>, fsimplify: super::super::Foundation::BOOL) -> ::windows::core::Result<ICondition>;
    fn MakeLeaf(&mut self, pszpropertyname: super::super::Foundation::PWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: super::super::Foundation::PWSTR, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, ppropertynameterm: &::core::option::Option<IRichChunk>, poperationterm: &::core::option::Option<IRichChunk>, pvalueterm: &::core::option::Option<IRichChunk>, fexpand: super::super::Foundation::BOOL) -> ::windows::core::Result<ICondition>;
    fn Resolve(&mut self, pc: &::core::option::Option<ICondition>, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<ICondition>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl IConditionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConditionFactory_Impl, const OFFSET: isize>() -> IConditionFactory_Vtbl {
        unsafe extern "system" fn MakeNot<Identity: ::windows::core::IUnknownImpl, Impl: IConditionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcsub: ::windows::core::RawPtr, fsimplify: super::super::Foundation::BOOL, ppcresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MakeNot(::core::mem::transmute(&pcsub), ::core::mem::transmute_copy(&fsimplify)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeAndOr<Identity: ::windows::core::IUnknownImpl, Impl: IConditionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ct: Common::CONDITION_TYPE, peusubs: ::windows::core::RawPtr, fsimplify: super::super::Foundation::BOOL, ppcresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MakeAndOr(::core::mem::transmute_copy(&ct), ::core::mem::transmute(&peusubs), ::core::mem::transmute_copy(&fsimplify)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeLeaf<Identity: ::windows::core::IUnknownImpl, Impl: IConditionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: super::super::Foundation::PWSTR, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, ppropertynameterm: ::windows::core::RawPtr, poperationterm: ::windows::core::RawPtr, pvalueterm: ::windows::core::RawPtr, fexpand: super::super::Foundation::BOOL, ppcresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MakeLeaf(::core::mem::transmute_copy(&pszpropertyname), ::core::mem::transmute_copy(&cop), ::core::mem::transmute_copy(&pszvaluetype), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute(&ppropertynameterm), ::core::mem::transmute(&poperationterm), ::core::mem::transmute(&pvalueterm), ::core::mem::transmute_copy(&fexpand)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resolve<Identity: ::windows::core::IUnknownImpl, Impl: IConditionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pc: ::windows::core::RawPtr, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME, ppcresolved: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Resolve(::core::mem::transmute(&pc), ::core::mem::transmute_copy(&sqro), ::core::mem::transmute_copy(&pstreferencetime)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcresolved = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            MakeNot: MakeNot::<Identity, Impl, OFFSET>,
            MakeAndOr: MakeAndOr::<Identity, Impl, OFFSET>,
            MakeLeaf: MakeLeaf::<Identity, Impl, OFFSET>,
            Resolve: Resolve::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConditionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IConditionFactory2_Impl: Sized + IConditionFactory_Impl {
    fn CreateTrueFalse(&mut self, fval: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateNegation(&mut self, pcsub: &::core::option::Option<ICondition>, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateCompoundFromObjectArray(&mut self, ct: Common::CONDITION_TYPE, poasubs: &::core::option::Option<super::super::UI::Shell::Common::IObjectArray>, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateCompoundFromArray(&mut self, ct: Common::CONDITION_TYPE, ppcondsubs: *const ::core::option::Option<ICondition>, csubs: u32, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateStringLeaf(&mut self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, pszvalue: super::super::Foundation::PWSTR, pszlocalename: super::super::Foundation::PWSTR, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateIntegerLeaf(&mut self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, lvalue: i32, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateBooleanLeaf(&mut self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, fvalue: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateLeaf(&mut self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, propvar: *const super::Com::StructuredStorage::PROPVARIANT, pszsemantictype: super::super::Foundation::PWSTR, pszlocalename: super::super::Foundation::PWSTR, ppropertynameterm: &::core::option::Option<IRichChunk>, poperationterm: &::core::option::Option<IRichChunk>, pvalueterm: &::core::option::Option<IRichChunk>, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ResolveCondition(&mut self, pc: &::core::option::Option<ICondition>, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IConditionFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConditionFactory2_Impl, const OFFSET: isize>() -> IConditionFactory2_Vtbl {
        unsafe extern "system" fn CreateTrueFalse<Identity: ::windows::core::IUnknownImpl, Impl: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fval: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateTrueFalse(::core::mem::transmute_copy(&fval), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateNegation<Identity: ::windows::core::IUnknownImpl, Impl: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcsub: ::windows::core::RawPtr, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateNegation(::core::mem::transmute(&pcsub), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateCompoundFromObjectArray<Identity: ::windows::core::IUnknownImpl, Impl: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ct: Common::CONDITION_TYPE, poasubs: ::windows::core::RawPtr, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateCompoundFromObjectArray(::core::mem::transmute_copy(&ct), ::core::mem::transmute(&poasubs), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateCompoundFromArray<Identity: ::windows::core::IUnknownImpl, Impl: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ct: Common::CONDITION_TYPE, ppcondsubs: *const ::windows::core::RawPtr, csubs: u32, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateCompoundFromArray(::core::mem::transmute_copy(&ct), ::core::mem::transmute_copy(&ppcondsubs), ::core::mem::transmute_copy(&csubs), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateStringLeaf<Identity: ::windows::core::IUnknownImpl, Impl: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, pszvalue: super::super::Foundation::PWSTR, pszlocalename: super::super::Foundation::PWSTR, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateStringLeaf(::core::mem::transmute_copy(&propkey), ::core::mem::transmute_copy(&cop), ::core::mem::transmute_copy(&pszvalue), ::core::mem::transmute_copy(&pszlocalename), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateIntegerLeaf<Identity: ::windows::core::IUnknownImpl, Impl: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, lvalue: i32, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateIntegerLeaf(::core::mem::transmute_copy(&propkey), ::core::mem::transmute_copy(&cop), ::core::mem::transmute_copy(&lvalue), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateBooleanLeaf<Identity: ::windows::core::IUnknownImpl, Impl: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, fvalue: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateBooleanLeaf(::core::mem::transmute_copy(&propkey), ::core::mem::transmute_copy(&cop), ::core::mem::transmute_copy(&fvalue), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateLeaf<Identity: ::windows::core::IUnknownImpl, Impl: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, propvar: *const super::Com::StructuredStorage::PROPVARIANT, pszsemantictype: super::super::Foundation::PWSTR, pszlocalename: super::super::Foundation::PWSTR, ppropertynameterm: ::windows::core::RawPtr, poperationterm: ::windows::core::RawPtr, pvalueterm: ::windows::core::RawPtr, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateLeaf(::core::mem::transmute_copy(&propkey), ::core::mem::transmute_copy(&cop), ::core::mem::transmute_copy(&propvar), ::core::mem::transmute_copy(&pszsemantictype), ::core::mem::transmute_copy(&pszlocalename), ::core::mem::transmute(&ppropertynameterm), ::core::mem::transmute(&poperationterm), ::core::mem::transmute(&pvalueterm), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn ResolveCondition<Identity: ::windows::core::IUnknownImpl, Impl: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pc: ::windows::core::RawPtr, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResolveCondition(::core::mem::transmute(&pc), ::core::mem::transmute_copy(&sqro), ::core::mem::transmute_copy(&pstreferencetime), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: IConditionFactory_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateTrueFalse: CreateTrueFalse::<Identity, Impl, OFFSET>,
            CreateNegation: CreateNegation::<Identity, Impl, OFFSET>,
            CreateCompoundFromObjectArray: CreateCompoundFromObjectArray::<Identity, Impl, OFFSET>,
            CreateCompoundFromArray: CreateCompoundFromArray::<Identity, Impl, OFFSET>,
            CreateStringLeaf: CreateStringLeaf::<Identity, Impl, OFFSET>,
            CreateIntegerLeaf: CreateIntegerLeaf::<Identity, Impl, OFFSET>,
            CreateBooleanLeaf: CreateBooleanLeaf::<Identity, Impl, OFFSET>,
            CreateLeaf: CreateLeaf::<Identity, Impl, OFFSET>,
            ResolveCondition: ResolveCondition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConditionFactory2 as ::windows::core::Interface>::IID || iid == &<IConditionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
pub trait IConditionGenerator_Impl: Sized {
    fn Initialize(&mut self, pschemaprovider: &::core::option::Option<ISchemaProvider>) -> ::windows::core::Result<()>;
    fn RecognizeNamedEntities(&mut self, pszinputstring: super::super::Foundation::PWSTR, lciduserlocale: u32, ptokencollection: &::core::option::Option<ITokenCollection>, pnamedentities: &::core::option::Option<INamedEntityCollector>) -> ::windows::core::Result<()>;
    fn GenerateForLeaf(&mut self, pconditionfactory: &::core::option::Option<IConditionFactory>, pszpropertyname: super::super::Foundation::PWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, pszvalue2: super::super::Foundation::PWSTR, ppropertynameterm: &::core::option::Option<IRichChunk>, poperationterm: &::core::option::Option<IRichChunk>, pvalueterm: &::core::option::Option<IRichChunk>, automaticwildcard: super::super::Foundation::BOOL, pnostringquery: *mut super::super::Foundation::BOOL, ppqueryexpression: *mut ::core::option::Option<ICondition>) -> ::windows::core::Result<()>;
    fn DefaultPhrase(&mut self, pszvaluetype: super::super::Foundation::PWSTR, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, fuseenglish: super::super::Foundation::BOOL) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl IConditionGenerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConditionGenerator_Impl, const OFFSET: isize>() -> IConditionGenerator_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IConditionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pschemaprovider: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pschemaprovider)).into()
        }
        unsafe extern "system" fn RecognizeNamedEntities<Identity: ::windows::core::IUnknownImpl, Impl: IConditionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszinputstring: super::super::Foundation::PWSTR, lciduserlocale: u32, ptokencollection: ::windows::core::RawPtr, pnamedentities: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RecognizeNamedEntities(::core::mem::transmute_copy(&pszinputstring), ::core::mem::transmute_copy(&lciduserlocale), ::core::mem::transmute(&ptokencollection), ::core::mem::transmute(&pnamedentities)).into()
        }
        unsafe extern "system" fn GenerateForLeaf<Identity: ::windows::core::IUnknownImpl, Impl: IConditionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconditionfactory: ::windows::core::RawPtr, pszpropertyname: super::super::Foundation::PWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, pszvalue2: super::super::Foundation::PWSTR, ppropertynameterm: ::windows::core::RawPtr, poperationterm: ::windows::core::RawPtr, pvalueterm: ::windows::core::RawPtr, automaticwildcard: super::super::Foundation::BOOL, pnostringquery: *mut super::super::Foundation::BOOL, ppqueryexpression: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .GenerateForLeaf(
                    ::core::mem::transmute(&pconditionfactory),
                    ::core::mem::transmute_copy(&pszpropertyname),
                    ::core::mem::transmute_copy(&cop),
                    ::core::mem::transmute_copy(&pszvaluetype),
                    ::core::mem::transmute_copy(&pszvalue),
                    ::core::mem::transmute_copy(&pszvalue2),
                    ::core::mem::transmute(&ppropertynameterm),
                    ::core::mem::transmute(&poperationterm),
                    ::core::mem::transmute(&pvalueterm),
                    ::core::mem::transmute_copy(&automaticwildcard),
                    ::core::mem::transmute_copy(&pnostringquery),
                    ::core::mem::transmute_copy(&ppqueryexpression),
                )
                .into()
        }
        unsafe extern "system" fn DefaultPhrase<Identity: ::windows::core::IUnknownImpl, Impl: IConditionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluetype: super::super::Foundation::PWSTR, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, fuseenglish: super::super::Foundation::BOOL, ppszphrase: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultPhrase(::core::mem::transmute_copy(&pszvaluetype), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&fuseenglish)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszphrase = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            RecognizeNamedEntities: RecognizeNamedEntities::<Identity, Impl, OFFSET>,
            GenerateForLeaf: GenerateForLeaf::<Identity, Impl, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConditionGenerator as ::windows::core::Interface>::IID
    }
}
pub trait IConvertType_Impl: Sized {
    fn CanConvert(&mut self, wfromtype: u16, wtotype: u16, dwconvertflags: u32) -> ::windows::core::Result<()>;
}
impl IConvertType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConvertType_Impl, const OFFSET: isize>() -> IConvertType_Vtbl {
        unsafe extern "system" fn CanConvert<Identity: ::windows::core::IUnknownImpl, Impl: IConvertType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wfromtype: u16, wtotype: u16, dwconvertflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CanConvert(::core::mem::transmute_copy(&wfromtype), ::core::mem::transmute_copy(&wtotype), ::core::mem::transmute_copy(&dwconvertflags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CanConvert: CanConvert::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConvertType as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICreateRow_Impl: Sized {
    fn CreateRow(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, pwszurl: super::super::Foundation::PWSTR, dwbindurlflags: u32, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pauthenticate: &::core::option::Option<super::Com::IAuthenticate>, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppwsznewurl: *mut super::super::Foundation::PWSTR, ppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICreateRow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateRow_Impl, const OFFSET: isize>() -> ICreateRow_Vtbl {
        unsafe extern "system" fn CreateRow<Identity: ::windows::core::IUnknownImpl, Impl: ICreateRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, dwbindurlflags: u32, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pauthenticate: ::windows::core::RawPtr, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppwsznewurl: *mut super::super::Foundation::PWSTR, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateRow(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&dwbindurlflags), ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute(&pauthenticate), ::core::mem::transmute_copy(&pimplsession), ::core::mem::transmute_copy(&pdwbindstatus), ::core::mem::transmute_copy(&ppwsznewurl), ::core::mem::transmute_copy(&ppunk)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateRow: CreateRow::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateRow as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDBAsynchNotify_Impl: Sized {
    fn OnLowResource(&mut self, dwreserved: usize) -> ::windows::core::Result<()>;
    fn OnProgress(&mut self, hchapter: usize, eoperation: u32, ulprogress: usize, ulprogressmax: usize, easynchphase: u32, pwszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn OnStop(&mut self, hchapter: usize, eoperation: u32, hrstatus: ::windows::core::HRESULT, pwszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDBAsynchNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDBAsynchNotify_Impl, const OFFSET: isize>() -> IDBAsynchNotify_Vtbl {
        unsafe extern "system" fn OnLowResource<Identity: ::windows::core::IUnknownImpl, Impl: IDBAsynchNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnLowResource(::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn OnProgress<Identity: ::windows::core::IUnknownImpl, Impl: IDBAsynchNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, eoperation: u32, ulprogress: usize, ulprogressmax: usize, easynchphase: u32, pwszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnProgress(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&eoperation), ::core::mem::transmute_copy(&ulprogress), ::core::mem::transmute_copy(&ulprogressmax), ::core::mem::transmute_copy(&easynchphase), ::core::mem::transmute_copy(&pwszstatustext)).into()
        }
        unsafe extern "system" fn OnStop<Identity: ::windows::core::IUnknownImpl, Impl: IDBAsynchNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, eoperation: u32, hrstatus: ::windows::core::HRESULT, pwszstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnStop(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&eoperation), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&pwszstatustext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnLowResource: OnLowResource::<Identity, Impl, OFFSET>,
            OnProgress: OnProgress::<Identity, Impl, OFFSET>,
            OnStop: OnStop::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBAsynchNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDBAsynchStatus_Impl: Sized {
    fn Abort(&mut self, hchapter: usize, eoperation: u32) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self, hchapter: usize, eoperation: u32, pulprogress: *mut usize, pulprogressmax: *mut usize, peasynchphase: *mut u32, ppwszstatustext: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDBAsynchStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDBAsynchStatus_Impl, const OFFSET: isize>() -> IDBAsynchStatus_Vtbl {
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl, Impl: IDBAsynchStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, eoperation: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Abort(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&eoperation)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDBAsynchStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, eoperation: u32, pulprogress: *mut usize, pulprogressmax: *mut usize, peasynchphase: *mut u32, ppwszstatustext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&eoperation), ::core::mem::transmute_copy(&pulprogress), ::core::mem::transmute_copy(&pulprogressmax), ::core::mem::transmute_copy(&peasynchphase), ::core::mem::transmute_copy(&ppwszstatustext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Abort: Abort::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBAsynchStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDBBinderProperties_Impl: Sized + IDBProperties_Impl {
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDBBinderProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDBBinderProperties_Impl, const OFFSET: isize>() -> IDBBinderProperties_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IDBBinderProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        Self { base: IDBProperties_Vtbl::new::<Identity, Impl, OFFSET>(), Reset: Reset::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBBinderProperties as ::windows::core::Interface>::IID || iid == &<IDBProperties as ::windows::core::Interface>::IID
    }
}
pub trait IDBCreateCommand_Impl: Sized {
    fn CreateCommand(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl IDBCreateCommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDBCreateCommand_Impl, const OFFSET: isize>() -> IDBCreateCommand_Vtbl {
        unsafe extern "system" fn CreateCommand<Identity: ::windows::core::IUnknownImpl, Impl: IDBCreateCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcommand: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateCommand(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcommand = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateCommand: CreateCommand::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBCreateCommand as ::windows::core::Interface>::IID
    }
}
pub trait IDBCreateSession_Impl: Sized {
    fn CreateSession(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl IDBCreateSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDBCreateSession_Impl, const OFFSET: isize>() -> IDBCreateSession_Vtbl {
        unsafe extern "system" fn CreateSession<Identity: ::windows::core::IUnknownImpl, Impl: IDBCreateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppdbsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSession(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdbsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateSession: CreateSession::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBCreateSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDBDataSourceAdmin_Impl: Sized {
    fn CreateDataSource(&mut self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, punkouter: &::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID, ppdbsession: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DestroyDataSource(&mut self) -> ::windows::core::Result<()>;
    fn GetCreationProperties(&mut self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
    fn ModifyDataSource(&mut self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDBDataSourceAdmin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDBDataSourceAdmin_Impl, const OFFSET: isize>() -> IDBDataSourceAdmin_Vtbl {
        unsafe extern "system" fn CreateDataSource<Identity: ::windows::core::IUnknownImpl, Impl: IDBDataSourceAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppdbsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateDataSource(::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppdbsession)).into()
        }
        unsafe extern "system" fn DestroyDataSource<Identity: ::windows::core::IUnknownImpl, Impl: IDBDataSourceAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DestroyDataSource().into()
        }
        unsafe extern "system" fn GetCreationProperties<Identity: ::windows::core::IUnknownImpl, Impl: IDBDataSourceAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCreationProperties(::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertyinfosets), ::core::mem::transmute_copy(&prgpropertyinfosets), ::core::mem::transmute_copy(&ppdescbuffer)).into()
        }
        unsafe extern "system" fn ModifyDataSource<Identity: ::windows::core::IUnknownImpl, Impl: IDBDataSourceAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ModifyDataSource(::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateDataSource: CreateDataSource::<Identity, Impl, OFFSET>,
            DestroyDataSource: DestroyDataSource::<Identity, Impl, OFFSET>,
            GetCreationProperties: GetCreationProperties::<Identity, Impl, OFFSET>,
            ModifyDataSource: ModifyDataSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBDataSourceAdmin as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDBInfo_Impl: Sized {
    fn GetKeywords(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetLiteralInfo(&mut self, cliterals: u32, rgliterals: *const u32, pcliteralinfo: *mut u32, prgliteralinfo: *mut *mut DBLITERALINFO, ppcharbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDBInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDBInfo_Impl, const OFFSET: isize>() -> IDBInfo_Vtbl {
        unsafe extern "system" fn GetKeywords<Identity: ::windows::core::IUnknownImpl, Impl: IDBInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszkeywords: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetKeywords() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszkeywords = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLiteralInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDBInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cliterals: u32, rgliterals: *const u32, pcliteralinfo: *mut u32, prgliteralinfo: *mut *mut DBLITERALINFO, ppcharbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLiteralInfo(::core::mem::transmute_copy(&cliterals), ::core::mem::transmute_copy(&rgliterals), ::core::mem::transmute_copy(&pcliteralinfo), ::core::mem::transmute_copy(&prgliteralinfo), ::core::mem::transmute_copy(&ppcharbuffer)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetKeywords: GetKeywords::<Identity, Impl, OFFSET>,
            GetLiteralInfo: GetLiteralInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBInfo as ::windows::core::Interface>::IID
    }
}
pub trait IDBInitialize_Impl: Sized {
    fn Initialize(&mut self) -> ::windows::core::Result<()>;
    fn Uninitialize(&mut self) -> ::windows::core::Result<()>;
}
impl IDBInitialize_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDBInitialize_Impl, const OFFSET: isize>() -> IDBInitialize_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDBInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize().into()
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows::core::IUnknownImpl, Impl: IDBInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Uninitialize().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBInitialize as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDBPromptInitialize_Impl: Sized {
    fn PromptDataSource(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, csourcetypefilter: u32, rgsourcetypefilter: *const u32, pwszszzproviderfilter: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppdatasource: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn PromptFileName(&mut self, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, pwszinitialdirectory: super::super::Foundation::PWSTR, pwszinitialfile: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDBPromptInitialize_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDBPromptInitialize_Impl, const OFFSET: isize>() -> IDBPromptInitialize_Vtbl {
        unsafe extern "system" fn PromptDataSource<Identity: ::windows::core::IUnknownImpl, Impl: IDBPromptInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, csourcetypefilter: u32, rgsourcetypefilter: *const u32, pwszszzproviderfilter: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PromptDataSource(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwpromptoptions), ::core::mem::transmute_copy(&csourcetypefilter), ::core::mem::transmute_copy(&rgsourcetypefilter), ::core::mem::transmute_copy(&pwszszzproviderfilter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppdatasource)).into()
        }
        unsafe extern "system" fn PromptFileName<Identity: ::windows::core::IUnknownImpl, Impl: IDBPromptInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, pwszinitialdirectory: super::super::Foundation::PWSTR, pwszinitialfile: super::super::Foundation::PWSTR, ppwszselectedfile: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PromptFileName(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwpromptoptions), ::core::mem::transmute_copy(&pwszinitialdirectory), ::core::mem::transmute_copy(&pwszinitialfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszselectedfile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PromptDataSource: PromptDataSource::<Identity, Impl, OFFSET>,
            PromptFileName: PromptFileName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBPromptInitialize as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDBProperties_Impl: Sized {
    fn GetProperties(&mut self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::Result<()>;
    fn GetPropertyInfo(&mut self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
    fn SetProperties(&mut self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDBProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDBProperties_Impl, const OFFSET: isize>() -> IDBProperties_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl, Impl: IDBProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProperties(::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)).into()
        }
        unsafe extern "system" fn GetPropertyInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDBProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropertyInfo(::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertyinfosets), ::core::mem::transmute_copy(&prgpropertyinfosets), ::core::mem::transmute_copy(&ppdescbuffer)).into()
        }
        unsafe extern "system" fn SetProperties<Identity: ::windows::core::IUnknownImpl, Impl: IDBProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperties(::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetPropertyInfo: GetPropertyInfo::<Identity, Impl, OFFSET>,
            SetProperties: SetProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBProperties as ::windows::core::Interface>::IID
    }
}
pub trait IDBSchemaCommand_Impl: Sized {
    fn GetCommand(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, rguidschema: *const ::windows::core::GUID) -> ::windows::core::Result<ICommand>;
    fn GetSchemas(&mut self, pcschemas: *mut u32, prgschemas: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IDBSchemaCommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDBSchemaCommand_Impl, const OFFSET: isize>() -> IDBSchemaCommand_Vtbl {
        unsafe extern "system" fn GetCommand<Identity: ::windows::core::IUnknownImpl, Impl: IDBSchemaCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, rguidschema: *const ::windows::core::GUID, ppcommand: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCommand(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&rguidschema)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcommand = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSchemas<Identity: ::windows::core::IUnknownImpl, Impl: IDBSchemaCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcschemas: *mut u32, prgschemas: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSchemas(::core::mem::transmute_copy(&pcschemas), ::core::mem::transmute_copy(&prgschemas)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCommand: GetCommand::<Identity, Impl, OFFSET>,
            GetSchemas: GetSchemas::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBSchemaCommand as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDBSchemaRowset_Impl: Sized {
    fn GetRowset(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, rguidschema: *const ::windows::core::GUID, crestrictions: u32, rgrestrictions: *const super::Com::VARIANT, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetSchemas(&mut self, pcschemas: *mut u32, prgschemas: *mut *mut ::windows::core::GUID, prgrestrictionsupport: *mut *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDBSchemaRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDBSchemaRowset_Impl, const OFFSET: isize>() -> IDBSchemaRowset_Vtbl {
        unsafe extern "system" fn GetRowset<Identity: ::windows::core::IUnknownImpl, Impl: IDBSchemaRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, rguidschema: *const ::windows::core::GUID, crestrictions: u32, rgrestrictions: *const super::Com::VARIANT, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRowset(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&rguidschema), ::core::mem::transmute_copy(&crestrictions), ::core::mem::transmute_copy(&rgrestrictions), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pprowset)).into()
        }
        unsafe extern "system" fn GetSchemas<Identity: ::windows::core::IUnknownImpl, Impl: IDBSchemaRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcschemas: *mut u32, prgschemas: *mut *mut ::windows::core::GUID, prgrestrictionsupport: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSchemas(::core::mem::transmute_copy(&pcschemas), ::core::mem::transmute_copy(&prgschemas), ::core::mem::transmute_copy(&prgrestrictionsupport)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetRowset: GetRowset::<Identity, Impl, OFFSET>,
            GetSchemas: GetSchemas::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBSchemaRowset as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDCInfo_Impl: Sized {
    fn GetInfo(&mut self, cinfo: u32, rgeinfotype: *const u32, prginfo: *mut *mut DCINFO) -> ::windows::core::Result<()>;
    fn SetInfo(&mut self, cinfo: u32, rginfo: *const DCINFO) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDCInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCInfo_Impl, const OFFSET: isize>() -> IDCInfo_Vtbl {
        unsafe extern "system" fn GetInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDCInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cinfo: u32, rgeinfotype: *const u32, prginfo: *mut *mut DCINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInfo(::core::mem::transmute_copy(&cinfo), ::core::mem::transmute_copy(&rgeinfotype), ::core::mem::transmute_copy(&prginfo)).into()
        }
        unsafe extern "system" fn SetInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDCInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cinfo: u32, rginfo: *const DCINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInfo(::core::mem::transmute_copy(&cinfo), ::core::mem::transmute_copy(&rginfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            SetInfo: SetInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCInfo as ::windows::core::Interface>::IID
    }
}
pub trait IDataConvert_Impl: Sized {
    fn DataConvert(&mut self, wsrctype: u16, wdsttype: u16, cbsrclength: usize, pcbdstlength: *mut usize, psrc: *const ::core::ffi::c_void, pdst: *mut ::core::ffi::c_void, cbdstmaxlength: usize, dbssrcstatus: u32, pdbsstatus: *mut u32, bprecision: u8, bscale: u8, dwflags: u32) -> ::windows::core::Result<()>;
    fn CanConvert(&mut self, wsrctype: u16, wdsttype: u16) -> ::windows::core::Result<()>;
    fn GetConversionSize(&mut self, wsrctype: u16, wdsttype: u16, pcbsrclength: *const usize, pcbdstlength: *mut usize, psrc: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IDataConvert_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataConvert_Impl, const OFFSET: isize>() -> IDataConvert_Vtbl {
        unsafe extern "system" fn DataConvert<Identity: ::windows::core::IUnknownImpl, Impl: IDataConvert_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsrctype: u16, wdsttype: u16, cbsrclength: usize, pcbdstlength: *mut usize, psrc: *const ::core::ffi::c_void, pdst: *mut ::core::ffi::c_void, cbdstmaxlength: usize, dbssrcstatus: u32, pdbsstatus: *mut u32, bprecision: u8, bscale: u8, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .DataConvert(
                    ::core::mem::transmute_copy(&wsrctype),
                    ::core::mem::transmute_copy(&wdsttype),
                    ::core::mem::transmute_copy(&cbsrclength),
                    ::core::mem::transmute_copy(&pcbdstlength),
                    ::core::mem::transmute_copy(&psrc),
                    ::core::mem::transmute_copy(&pdst),
                    ::core::mem::transmute_copy(&cbdstmaxlength),
                    ::core::mem::transmute_copy(&dbssrcstatus),
                    ::core::mem::transmute_copy(&pdbsstatus),
                    ::core::mem::transmute_copy(&bprecision),
                    ::core::mem::transmute_copy(&bscale),
                    ::core::mem::transmute_copy(&dwflags),
                )
                .into()
        }
        unsafe extern "system" fn CanConvert<Identity: ::windows::core::IUnknownImpl, Impl: IDataConvert_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsrctype: u16, wdsttype: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CanConvert(::core::mem::transmute_copy(&wsrctype), ::core::mem::transmute_copy(&wdsttype)).into()
        }
        unsafe extern "system" fn GetConversionSize<Identity: ::windows::core::IUnknownImpl, Impl: IDataConvert_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsrctype: u16, wdsttype: u16, pcbsrclength: *const usize, pcbdstlength: *mut usize, psrc: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConversionSize(::core::mem::transmute_copy(&wsrctype), ::core::mem::transmute_copy(&wdsttype), ::core::mem::transmute_copy(&pcbsrclength), ::core::mem::transmute_copy(&pcbdstlength), ::core::mem::transmute_copy(&psrc)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DataConvert: DataConvert::<Identity, Impl, OFFSET>,
            CanConvert: CanConvert::<Identity, Impl, OFFSET>,
            GetConversionSize: GetConversionSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataConvert as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDataInitialize_Impl: Sized {
    fn GetDataSource(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, dwclsctx: u32, pwszinitializationstring: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppdatasource: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetInitializationString(&mut self, pdatasource: &::core::option::Option<::windows::core::IUnknown>, fincludepassword: u8) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn CreateDBInstance(&mut self, clsidprovider: *const ::windows::core::GUID, punkouter: &::core::option::Option<::windows::core::IUnknown>, dwclsctx: u32, pwszreserved: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn CreateDBInstanceEx(&mut self, clsidprovider: *const ::windows::core::GUID, punkouter: &::core::option::Option<::windows::core::IUnknown>, dwclsctx: u32, pwszreserved: super::super::Foundation::PWSTR, pserverinfo: *const super::Com::COSERVERINFO, cmq: u32, rgmqresults: *mut super::Com::MULTI_QI) -> ::windows::core::Result<()>;
    fn LoadStringFromStorage(&mut self, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn WriteStringToStorage(&mut self, pwszfilename: super::super::Foundation::PWSTR, pwszinitializationstring: super::super::Foundation::PWSTR, dwcreationdisposition: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDataInitialize_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataInitialize_Impl, const OFFSET: isize>() -> IDataInitialize_Vtbl {
        unsafe extern "system" fn GetDataSource<Identity: ::windows::core::IUnknownImpl, Impl: IDataInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, dwclsctx: u32, pwszinitializationstring: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDataSource(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&dwclsctx), ::core::mem::transmute_copy(&pwszinitializationstring), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppdatasource)).into()
        }
        unsafe extern "system" fn GetInitializationString<Identity: ::windows::core::IUnknownImpl, Impl: IDataInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatasource: *mut ::core::ffi::c_void, fincludepassword: u8, ppwszinitstring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInitializationString(::core::mem::transmute(&pdatasource), ::core::mem::transmute_copy(&fincludepassword)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszinitstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDBInstance<Identity: ::windows::core::IUnknownImpl, Impl: IDataInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidprovider: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: u32, pwszreserved: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDBInstance(::core::mem::transmute_copy(&clsidprovider), ::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&dwclsctx), ::core::mem::transmute_copy(&pwszreserved), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdatasource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDBInstanceEx<Identity: ::windows::core::IUnknownImpl, Impl: IDataInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidprovider: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: u32, pwszreserved: super::super::Foundation::PWSTR, pserverinfo: *const super::Com::COSERVERINFO, cmq: u32, rgmqresults: *mut super::Com::MULTI_QI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateDBInstanceEx(::core::mem::transmute_copy(&clsidprovider), ::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&dwclsctx), ::core::mem::transmute_copy(&pwszreserved), ::core::mem::transmute_copy(&pserverinfo), ::core::mem::transmute_copy(&cmq), ::core::mem::transmute_copy(&rgmqresults)).into()
        }
        unsafe extern "system" fn LoadStringFromStorage<Identity: ::windows::core::IUnknownImpl, Impl: IDataInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR, ppwszinitializationstring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoadStringFromStorage(::core::mem::transmute_copy(&pwszfilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszinitializationstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteStringToStorage<Identity: ::windows::core::IUnknownImpl, Impl: IDataInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR, pwszinitializationstring: super::super::Foundation::PWSTR, dwcreationdisposition: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteStringToStorage(::core::mem::transmute_copy(&pwszfilename), ::core::mem::transmute_copy(&pwszinitializationstring), ::core::mem::transmute_copy(&dwcreationdisposition)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDataSource: GetDataSource::<Identity, Impl, OFFSET>,
            GetInitializationString: GetInitializationString::<Identity, Impl, OFFSET>,
            CreateDBInstance: CreateDBInstance::<Identity, Impl, OFFSET>,
            CreateDBInstanceEx: CreateDBInstanceEx::<Identity, Impl, OFFSET>,
            LoadStringFromStorage: LoadStringFromStorage::<Identity, Impl, OFFSET>,
            WriteStringToStorage: WriteStringToStorage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataInitialize as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDataSourceLocator_Impl: Sized + super::Com::IDispatch_Impl {
    fn hWnd(&mut self) -> ::windows::core::Result<i64>;
    fn SethWnd(&mut self, hwndparent: i64) -> ::windows::core::Result<()>;
    fn PromptNew(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn PromptEdit(&mut self, ppadoconnection: *mut ::core::option::Option<super::Com::IDispatch>, pbsuccess: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDataSourceLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataSourceLocator_Impl, const OFFSET: isize>() -> IDataSourceLocator_Vtbl {
        unsafe extern "system" fn hWnd<Identity: ::windows::core::IUnknownImpl, Impl: IDataSourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwndparent: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).hWnd() {
                ::core::result::Result::Ok(ok__) => {
                    *phwndparent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SethWnd<Identity: ::windows::core::IUnknownImpl, Impl: IDataSourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SethWnd(::core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn PromptNew<Identity: ::windows::core::IUnknownImpl, Impl: IDataSourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppadoconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PromptNew() {
                ::core::result::Result::Ok(ok__) => {
                    *ppadoconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PromptEdit<Identity: ::windows::core::IUnknownImpl, Impl: IDataSourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppadoconnection: *mut ::windows::core::RawPtr, pbsuccess: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PromptEdit(::core::mem::transmute_copy(&ppadoconnection), ::core::mem::transmute_copy(&pbsuccess)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            hWnd: hWnd::<Identity, Impl, OFFSET>,
            SethWnd: SethWnd::<Identity, Impl, OFFSET>,
            PromptNew: PromptNew::<Identity, Impl, OFFSET>,
            PromptEdit: PromptEdit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataSourceLocator as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEntity_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Base(&mut self) -> ::windows::core::Result<IEntity>;
    fn Relationships(&mut self, riid: *const ::windows::core::GUID, prelationships: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetRelationship(&mut self, pszrelationname: super::super::Foundation::PWSTR) -> ::windows::core::Result<IRelationship>;
    fn MetaData(&mut self, riid: *const ::windows::core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn NamedEntities(&mut self, riid: *const ::windows::core::GUID, pnamedentities: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetNamedEntity(&mut self, pszvalue: super::super::Foundation::PWSTR) -> ::windows::core::Result<INamedEntity>;
    fn DefaultPhrase(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEntity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEntity_Impl, const OFFSET: isize>() -> IEntity_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Base<Identity: ::windows::core::IUnknownImpl, Impl: IEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbaseentity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Base() {
                ::core::result::Result::Ok(ok__) => {
                    *pbaseentity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Relationships<Identity: ::windows::core::IUnknownImpl, Impl: IEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, prelationships: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Relationships(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&prelationships)).into()
        }
        unsafe extern "system" fn GetRelationship<Identity: ::windows::core::IUnknownImpl, Impl: IEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrelationname: super::super::Foundation::PWSTR, prelationship: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRelationship(::core::mem::transmute_copy(&pszrelationname)) {
                ::core::result::Result::Ok(ok__) => {
                    *prelationship = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MetaData<Identity: ::windows::core::IUnknownImpl, Impl: IEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MetaData(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pmetadata)).into()
        }
        unsafe extern "system" fn NamedEntities<Identity: ::windows::core::IUnknownImpl, Impl: IEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pnamedentities: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NamedEntities(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pnamedentities)).into()
        }
        unsafe extern "system" fn GetNamedEntity<Identity: ::windows::core::IUnknownImpl, Impl: IEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvalue: super::super::Foundation::PWSTR, ppnamedentity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNamedEntity(::core::mem::transmute_copy(&pszvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnamedentity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultPhrase<Identity: ::windows::core::IUnknownImpl, Impl: IEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszphrase: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultPhrase() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszphrase = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Base: Base::<Identity, Impl, OFFSET>,
            Relationships: Relationships::<Identity, Impl, OFFSET>,
            GetRelationship: GetRelationship::<Identity, Impl, OFFSET>,
            MetaData: MetaData::<Identity, Impl, OFFSET>,
            NamedEntities: NamedEntities::<Identity, Impl, OFFSET>,
            GetNamedEntity: GetNamedEntity::<Identity, Impl, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEntity as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumItemProperties_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ITEMPROP, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumItemProperties>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumItemProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumItemProperties_Impl, const OFFSET: isize>() -> IEnumItemProperties_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ITEMPROP, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IEnumItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pncount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumItemProperties as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSearchRoots_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<ISearchRoot>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSearchRoots>;
}
impl IEnumSearchRoots_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSearchRoots_Impl, const OFFSET: isize>() -> IEnumSearchRoots_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSearchRoots_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSearchRoots_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSearchRoots_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSearchRoots_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSearchRoots as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSearchScopeRules_Impl: Sized {
    fn Next(&mut self, celt: u32, pprgelt: *mut ::core::option::Option<ISearchScopeRule>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSearchScopeRules>;
}
impl IEnumSearchScopeRules_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSearchScopeRules_Impl, const OFFSET: isize>() -> IEnumSearchScopeRules_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSearchScopeRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pprgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pprgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSearchScopeRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSearchScopeRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSearchScopeRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSearchScopeRules as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSubscription_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSubscription>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
}
impl IEnumSubscription_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSubscription_Impl, const OFFSET: isize>() -> IEnumSubscription_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pncount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSubscription as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IErrorLookup_Impl: Sized {
    fn GetErrorDescription(&mut self, hrerror: ::windows::core::HRESULT, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, lcid: u32, pbstrsource: *mut super::super::Foundation::BSTR, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetHelpInfo(&mut self, hrerror: ::windows::core::HRESULT, dwlookupid: u32, lcid: u32, pbstrhelpfile: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32) -> ::windows::core::Result<()>;
    fn ReleaseErrors(&mut self, dwdynamicerrorid: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IErrorLookup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IErrorLookup_Impl, const OFFSET: isize>() -> IErrorLookup_Vtbl {
        unsafe extern "system" fn GetErrorDescription<Identity: ::windows::core::IUnknownImpl, Impl: IErrorLookup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, lcid: u32, pbstrsource: *mut super::super::Foundation::BSTR, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetErrorDescription(::core::mem::transmute_copy(&hrerror), ::core::mem::transmute_copy(&dwlookupid), ::core::mem::transmute_copy(&pdispparams), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&pbstrsource), ::core::mem::transmute_copy(&pbstrdescription)).into()
        }
        unsafe extern "system" fn GetHelpInfo<Identity: ::windows::core::IUnknownImpl, Impl: IErrorLookup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, dwlookupid: u32, lcid: u32, pbstrhelpfile: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetHelpInfo(::core::mem::transmute_copy(&hrerror), ::core::mem::transmute_copy(&dwlookupid), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&pbstrhelpfile), ::core::mem::transmute_copy(&pdwhelpcontext)).into()
        }
        unsafe extern "system" fn ReleaseErrors<Identity: ::windows::core::IUnknownImpl, Impl: IErrorLookup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdynamicerrorid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseErrors(::core::mem::transmute_copy(&dwdynamicerrorid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetErrorDescription: GetErrorDescription::<Identity, Impl, OFFSET>,
            GetHelpInfo: GetHelpInfo::<Identity, Impl, OFFSET>,
            ReleaseErrors: ReleaseErrors::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IErrorLookup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IErrorRecords_Impl: Sized {
    fn AddErrorRecord(&mut self, perrorinfo: *const ERRORINFO, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, punkcustomerror: &::core::option::Option<::windows::core::IUnknown>, dwdynamicerrorid: u32) -> ::windows::core::Result<()>;
    fn GetBasicErrorInfo(&mut self, ulrecordnum: u32) -> ::windows::core::Result<ERRORINFO>;
    fn GetCustomErrorObject(&mut self, ulrecordnum: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetErrorInfo(&mut self, ulrecordnum: u32, lcid: u32) -> ::windows::core::Result<super::Com::IErrorInfo>;
    fn GetErrorParameters(&mut self, ulrecordnum: u32) -> ::windows::core::Result<super::Com::DISPPARAMS>;
    fn GetRecordCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IErrorRecords_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IErrorRecords_Impl, const OFFSET: isize>() -> IErrorRecords_Vtbl {
        unsafe extern "system" fn AddErrorRecord<Identity: ::windows::core::IUnknownImpl, Impl: IErrorRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrorinfo: *const ERRORINFO, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, punkcustomerror: *mut ::core::ffi::c_void, dwdynamicerrorid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddErrorRecord(::core::mem::transmute_copy(&perrorinfo), ::core::mem::transmute_copy(&dwlookupid), ::core::mem::transmute_copy(&pdispparams), ::core::mem::transmute(&punkcustomerror), ::core::mem::transmute_copy(&dwdynamicerrorid)).into()
        }
        unsafe extern "system" fn GetBasicErrorInfo<Identity: ::windows::core::IUnknownImpl, Impl: IErrorRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulrecordnum: u32, perrorinfo: *mut ERRORINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBasicErrorInfo(::core::mem::transmute_copy(&ulrecordnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *perrorinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomErrorObject<Identity: ::windows::core::IUnknownImpl, Impl: IErrorRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulrecordnum: u32, riid: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCustomErrorObject(::core::mem::transmute_copy(&ulrecordnum), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorInfo<Identity: ::windows::core::IUnknownImpl, Impl: IErrorRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulrecordnum: u32, lcid: u32, pperrorinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetErrorInfo(::core::mem::transmute_copy(&ulrecordnum), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pperrorinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorParameters<Identity: ::windows::core::IUnknownImpl, Impl: IErrorRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulrecordnum: u32, pdispparams: *mut super::Com::DISPPARAMS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetErrorParameters(::core::mem::transmute_copy(&ulrecordnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdispparams = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecordCount<Identity: ::windows::core::IUnknownImpl, Impl: IErrorRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcrecords: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRecordCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcrecords = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddErrorRecord: AddErrorRecord::<Identity, Impl, OFFSET>,
            GetBasicErrorInfo: GetBasicErrorInfo::<Identity, Impl, OFFSET>,
            GetCustomErrorObject: GetCustomErrorObject::<Identity, Impl, OFFSET>,
            GetErrorInfo: GetErrorInfo::<Identity, Impl, OFFSET>,
            GetErrorParameters: GetErrorParameters::<Identity, Impl, OFFSET>,
            GetRecordCount: GetRecordCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IErrorRecords as ::windows::core::Interface>::IID
    }
}
pub trait IGetDataSource_Impl: Sized {
    fn GetDataSource(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl IGetDataSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetDataSource_Impl, const OFFSET: isize>() -> IGetDataSource_Vtbl {
        unsafe extern "system" fn GetDataSource<Identity: ::windows::core::IUnknownImpl, Impl: IGetDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDataSource(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdatasource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetDataSource: GetDataSource::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetDataSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGetRow_Impl: Sized {
    fn GetRowFromHROW(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, hrow: usize, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetURLFromHROW(&mut self, hrow: usize) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IGetRow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetRow_Impl, const OFFSET: isize>() -> IGetRow_Vtbl {
        unsafe extern "system" fn GetRowFromHROW<Identity: ::windows::core::IUnknownImpl, Impl: IGetRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, hrow: usize, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRowFromHROW(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetURLFromHROW<Identity: ::windows::core::IUnknownImpl, Impl: IGetRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrow: usize, ppwszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetURLFromHROW(::core::mem::transmute_copy(&hrow)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetRowFromHROW: GetRowFromHROW::<Identity, Impl, OFFSET>,
            GetURLFromHROW: GetURLFromHROW::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetRow as ::windows::core::Interface>::IID
    }
}
pub trait IGetSession_Impl: Sized {
    fn GetSession(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl IGetSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetSession_Impl, const OFFSET: isize>() -> IGetSession_Vtbl {
        unsafe extern "system" fn GetSession<Identity: ::windows::core::IUnknownImpl, Impl: IGetSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSession(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetSession: GetSession::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetSession as ::windows::core::Interface>::IID
    }
}
pub trait IGetSourceRow_Impl: Sized {
    fn GetSourceRow(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl IGetSourceRow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetSourceRow_Impl, const OFFSET: isize>() -> IGetSourceRow_Vtbl {
        unsafe extern "system" fn GetSourceRow<Identity: ::windows::core::IUnknownImpl, Impl: IGetSourceRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pprow: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSourceRow(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetSourceRow: GetSourceRow::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetSourceRow as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IIndexDefinition_Impl: Sized {
    fn CreateIndex(&mut self, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, cindexcolumndescs: usize, rgindexcolumndescs: *const DBINDEXCOLUMNDESC, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppindexid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
    fn DropIndex(&mut self, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IIndexDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIndexDefinition_Impl, const OFFSET: isize>() -> IIndexDefinition_Vtbl {
        unsafe extern "system" fn CreateIndex<Identity: ::windows::core::IUnknownImpl, Impl: IIndexDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, cindexcolumndescs: usize, rgindexcolumndescs: *const DBINDEXCOLUMNDESC, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppindexid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateIndex(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pindexid), ::core::mem::transmute_copy(&cindexcolumndescs), ::core::mem::transmute_copy(&rgindexcolumndescs), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&ppindexid)).into()
        }
        unsafe extern "system" fn DropIndex<Identity: ::windows::core::IUnknownImpl, Impl: IIndexDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DropIndex(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pindexid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateIndex: CreateIndex::<Identity, Impl, OFFSET>,
            DropIndex: DropIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIndexDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IInterval_Impl: Sized {
    fn GetLimits(&mut self, pilklower: *mut INTERVAL_LIMIT_KIND, ppropvarlower: *mut super::Com::StructuredStorage::PROPVARIANT, pilkupper: *mut INTERVAL_LIMIT_KIND, ppropvarupper: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IInterval_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInterval_Impl, const OFFSET: isize>() -> IInterval_Vtbl {
        unsafe extern "system" fn GetLimits<Identity: ::windows::core::IUnknownImpl, Impl: IInterval_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pilklower: *mut INTERVAL_LIMIT_KIND, ppropvarlower: *mut super::Com::StructuredStorage::PROPVARIANT, pilkupper: *mut INTERVAL_LIMIT_KIND, ppropvarupper: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLimits(::core::mem::transmute_copy(&pilklower), ::core::mem::transmute_copy(&ppropvarlower), ::core::mem::transmute_copy(&pilkupper), ::core::mem::transmute_copy(&ppropvarupper)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetLimits: GetLimits::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInterval as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ILoadFilter_Impl: Sized {
    fn LoadIFilter(&mut self, pwcspath: super::super::Foundation::PWSTR, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: &::core::option::Option<::windows::core::IUnknown>, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut ::core::option::Option<super::super::Storage::IndexServer::IFilter>) -> ::windows::core::Result<()>;
    fn LoadIFilterFromStorage(&mut self, pstg: &::core::option::Option<super::Com::StructuredStorage::IStorage>, punkouter: &::core::option::Option<::windows::core::IUnknown>, pwcsoverride: super::super::Foundation::PWSTR, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut ::core::option::Option<super::super::Storage::IndexServer::IFilter>) -> ::windows::core::Result<()>;
    fn LoadIFilterFromStream(&mut self, pstm: &::core::option::Option<super::Com::IStream>, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: &::core::option::Option<::windows::core::IUnknown>, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut ::core::option::Option<super::super::Storage::IndexServer::IFilter>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ILoadFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoadFilter_Impl, const OFFSET: isize>() -> ILoadFilter_Vtbl {
        unsafe extern "system" fn LoadIFilter<Identity: ::windows::core::IUnknownImpl, Impl: ILoadFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcspath: super::super::Foundation::PWSTR, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: *mut ::core::ffi::c_void, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadIFilter(::core::mem::transmute_copy(&pwcspath), ::core::mem::transmute_copy(&pfilteredsources), ::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&fusedefault), ::core::mem::transmute_copy(&pfilterclsid), ::core::mem::transmute_copy(&searchdecsize), ::core::mem::transmute_copy(&pwcssearchdesc), ::core::mem::transmute_copy(&ppifilt)).into()
        }
        unsafe extern "system" fn LoadIFilterFromStorage<Identity: ::windows::core::IUnknownImpl, Impl: ILoadFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstg: ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void, pwcsoverride: super::super::Foundation::PWSTR, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadIFilterFromStorage(::core::mem::transmute(&pstg), ::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&pwcsoverride), ::core::mem::transmute_copy(&fusedefault), ::core::mem::transmute_copy(&pfilterclsid), ::core::mem::transmute_copy(&searchdecsize), ::core::mem::transmute_copy(&pwcssearchdesc), ::core::mem::transmute_copy(&ppifilt)).into()
        }
        unsafe extern "system" fn LoadIFilterFromStream<Identity: ::windows::core::IUnknownImpl, Impl: ILoadFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: *mut ::core::ffi::c_void, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadIFilterFromStream(::core::mem::transmute(&pstm), ::core::mem::transmute_copy(&pfilteredsources), ::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&fusedefault), ::core::mem::transmute_copy(&pfilterclsid), ::core::mem::transmute_copy(&searchdecsize), ::core::mem::transmute_copy(&pwcssearchdesc), ::core::mem::transmute_copy(&ppifilt)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            LoadIFilter: LoadIFilter::<Identity, Impl, OFFSET>,
            LoadIFilterFromStorage: LoadIFilterFromStorage::<Identity, Impl, OFFSET>,
            LoadIFilterFromStream: LoadIFilterFromStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoadFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ILoadFilterWithPrivateComActivation_Impl: Sized + ILoadFilter_Impl {
    fn LoadIFilterWithPrivateComActivation(&mut self, filteredsources: *const FILTERED_DATA_SOURCES, usedefault: super::super::Foundation::BOOL, filterclsid: *mut ::windows::core::GUID, isfilterprivatecomactivated: *mut super::super::Foundation::BOOL, filterobj: *mut ::core::option::Option<super::super::Storage::IndexServer::IFilter>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ILoadFilterWithPrivateComActivation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoadFilterWithPrivateComActivation_Impl, const OFFSET: isize>() -> ILoadFilterWithPrivateComActivation_Vtbl {
        unsafe extern "system" fn LoadIFilterWithPrivateComActivation<Identity: ::windows::core::IUnknownImpl, Impl: ILoadFilterWithPrivateComActivation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filteredsources: *const FILTERED_DATA_SOURCES, usedefault: super::super::Foundation::BOOL, filterclsid: *mut ::windows::core::GUID, isfilterprivatecomactivated: *mut super::super::Foundation::BOOL, filterobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadIFilterWithPrivateComActivation(::core::mem::transmute_copy(&filteredsources), ::core::mem::transmute_copy(&usedefault), ::core::mem::transmute_copy(&filterclsid), ::core::mem::transmute_copy(&isfilterprivatecomactivated), ::core::mem::transmute_copy(&filterobj)).into()
        }
        Self {
            base: ILoadFilter_Vtbl::new::<Identity, Impl, OFFSET>(),
            LoadIFilterWithPrivateComActivation: LoadIFilterWithPrivateComActivation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoadFilterWithPrivateComActivation as ::windows::core::Interface>::IID || iid == &<ILoadFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMDDataset_Impl: Sized {
    fn FreeAxisInfo(&mut self, caxes: usize, rgaxisinfo: *mut MDAXISINFO) -> ::windows::core::Result<()>;
    fn GetAxisInfo(&mut self, pcaxes: *mut usize, prgaxisinfo: *mut *mut MDAXISINFO) -> ::windows::core::Result<()>;
    fn GetAxisRowset(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, iaxis: usize, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetCellData(&mut self, haccessor: usize, ulstartcell: usize, ulendcell: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetSpecification(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMDDataset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDDataset_Impl, const OFFSET: isize>() -> IMDDataset_Vtbl {
        unsafe extern "system" fn FreeAxisInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMDDataset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, caxes: usize, rgaxisinfo: *mut MDAXISINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FreeAxisInfo(::core::mem::transmute_copy(&caxes), ::core::mem::transmute_copy(&rgaxisinfo)).into()
        }
        unsafe extern "system" fn GetAxisInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMDDataset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaxes: *mut usize, prgaxisinfo: *mut *mut MDAXISINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAxisInfo(::core::mem::transmute_copy(&pcaxes), ::core::mem::transmute_copy(&prgaxisinfo)).into()
        }
        unsafe extern "system" fn GetAxisRowset<Identity: ::windows::core::IUnknownImpl, Impl: IMDDataset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, iaxis: usize, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAxisRowset(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&iaxis), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pprowset)).into()
        }
        unsafe extern "system" fn GetCellData<Identity: ::windows::core::IUnknownImpl, Impl: IMDDataset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haccessor: usize, ulstartcell: usize, ulendcell: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCellData(::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&ulstartcell), ::core::mem::transmute_copy(&ulendcell), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetSpecification<Identity: ::windows::core::IUnknownImpl, Impl: IMDDataset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppspecification: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSpecification(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppspecification = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            FreeAxisInfo: FreeAxisInfo::<Identity, Impl, OFFSET>,
            GetAxisInfo: GetAxisInfo::<Identity, Impl, OFFSET>,
            GetAxisRowset: GetAxisRowset::<Identity, Impl, OFFSET>,
            GetCellData: GetCellData::<Identity, Impl, OFFSET>,
            GetSpecification: GetSpecification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDDataset as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMDFind_Impl: Sized {
    fn FindCell(&mut self, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *mut super::super::Foundation::PWSTR, pulcellordinal: *mut usize) -> ::windows::core::Result<()>;
    fn FindTuple(&mut self, ulaxisidentifier: u32, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *mut super::super::Foundation::PWSTR, pultupleordinal: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMDFind_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDFind_Impl, const OFFSET: isize>() -> IMDFind_Vtbl {
        unsafe extern "system" fn FindCell<Identity: ::windows::core::IUnknownImpl, Impl: IMDFind_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *mut super::super::Foundation::PWSTR, pulcellordinal: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindCell(::core::mem::transmute_copy(&ulstartingordinal), ::core::mem::transmute_copy(&cmembers), ::core::mem::transmute_copy(&rgpwszmember), ::core::mem::transmute_copy(&pulcellordinal)).into()
        }
        unsafe extern "system" fn FindTuple<Identity: ::windows::core::IUnknownImpl, Impl: IMDFind_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulaxisidentifier: u32, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *mut super::super::Foundation::PWSTR, pultupleordinal: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindTuple(::core::mem::transmute_copy(&ulaxisidentifier), ::core::mem::transmute_copy(&ulstartingordinal), ::core::mem::transmute_copy(&cmembers), ::core::mem::transmute_copy(&rgpwszmember), ::core::mem::transmute_copy(&pultupleordinal)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            FindCell: FindCell::<Identity, Impl, OFFSET>,
            FindTuple: FindTuple::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDFind as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMDRangeRowset_Impl: Sized {
    fn GetRangeRowset(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, ulstartcell: usize, ulendcell: usize, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMDRangeRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDRangeRowset_Impl, const OFFSET: isize>() -> IMDRangeRowset_Vtbl {
        unsafe extern "system" fn GetRangeRowset<Identity: ::windows::core::IUnknownImpl, Impl: IMDRangeRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ulstartcell: usize, ulendcell: usize, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRangeRowset(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&ulstartcell), ::core::mem::transmute_copy(&ulendcell), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pprowset)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetRangeRowset: GetRangeRowset::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDRangeRowset as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMetaData_Impl: Sized {
    fn GetData(&mut self, ppszkey: *mut super::super::Foundation::PWSTR, ppszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMetaData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMetaData_Impl, const OFFSET: isize>() -> IMetaData_Vtbl {
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl, Impl: IMetaData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszkey: *mut super::super::Foundation::PWSTR, ppszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetData(::core::mem::transmute_copy(&ppszkey), ::core::mem::transmute_copy(&ppszvalue)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetData: GetData::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMetaData as ::windows::core::Interface>::IID
    }
}
pub trait IMultipleResults_Impl: Sized {
    fn GetResult(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, lresultflag: isize, riid: *const ::windows::core::GUID, pcrowsaffected: *mut isize, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IMultipleResults_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleResults_Impl, const OFFSET: isize>() -> IMultipleResults_Vtbl {
        unsafe extern "system" fn GetResult<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, lresultflag: isize, riid: *const ::windows::core::GUID, pcrowsaffected: *mut isize, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetResult(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&lresultflag), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pcrowsaffected), ::core::mem::transmute_copy(&pprowset)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetResult: GetResult::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultipleResults as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INamedEntity_Impl: Sized {
    fn GetValue(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn DefaultPhrase(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl INamedEntity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INamedEntity_Impl, const OFFSET: isize>() -> INamedEntity_Vtbl {
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl, Impl: INamedEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetValue() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultPhrase<Identity: ::windows::core::IUnknownImpl, Impl: INamedEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszphrase: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultPhrase() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszphrase = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INamedEntity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INamedEntityCollector_Impl: Sized {
    fn Add(&mut self, beginspan: u32, endspan: u32, beginactual: u32, endactual: u32, ptype: &::core::option::Option<IEntity>, pszvalue: super::super::Foundation::PWSTR, certainty: NAMED_ENTITY_CERTAINTY) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INamedEntityCollector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INamedEntityCollector_Impl, const OFFSET: isize>() -> INamedEntityCollector_Vtbl {
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: INamedEntityCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beginspan: u32, endspan: u32, beginactual: u32, endactual: u32, ptype: ::windows::core::RawPtr, pszvalue: super::super::Foundation::PWSTR, certainty: NAMED_ENTITY_CERTAINTY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&beginspan), ::core::mem::transmute_copy(&endspan), ::core::mem::transmute_copy(&beginactual), ::core::mem::transmute_copy(&endactual), ::core::mem::transmute(&ptype), ::core::mem::transmute_copy(&pszvalue), ::core::mem::transmute_copy(&certainty)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Add: Add::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INamedEntityCollector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
pub trait IObjectAccessControl_Impl: Sized {
    fn GetObjectAccessRights(&mut self, pobject: *mut SEC_OBJECT, pcaccessentries: *mut u32, prgaccessentries: *mut *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> ::windows::core::Result<()>;
    fn GetObjectOwner(&mut self, pobject: *mut SEC_OBJECT, ppowner: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::Result<()>;
    fn IsObjectAccessAllowed(&mut self, pobject: *mut SEC_OBJECT, paccessentry: *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetObjectAccessRights(&mut self, pobject: *mut SEC_OBJECT, caccessentries: u32, prgaccessentries: *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> ::windows::core::Result<()>;
    fn SetObjectOwner(&mut self, pobject: *mut SEC_OBJECT, powner: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
impl IObjectAccessControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectAccessControl_Impl, const OFFSET: isize>() -> IObjectAccessControl_Vtbl {
        unsafe extern "system" fn GetObjectAccessRights<Identity: ::windows::core::IUnknownImpl, Impl: IObjectAccessControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut SEC_OBJECT, pcaccessentries: *mut u32, prgaccessentries: *mut *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObjectAccessRights(::core::mem::transmute_copy(&pobject), ::core::mem::transmute_copy(&pcaccessentries), ::core::mem::transmute_copy(&prgaccessentries)).into()
        }
        unsafe extern "system" fn GetObjectOwner<Identity: ::windows::core::IUnknownImpl, Impl: IObjectAccessControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut SEC_OBJECT, ppowner: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObjectOwner(::core::mem::transmute_copy(&pobject), ::core::mem::transmute_copy(&ppowner)).into()
        }
        unsafe extern "system" fn IsObjectAccessAllowed<Identity: ::windows::core::IUnknownImpl, Impl: IObjectAccessControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut SEC_OBJECT, paccessentry: *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsObjectAccessAllowed(::core::mem::transmute_copy(&pobject), ::core::mem::transmute_copy(&paccessentry), ::core::mem::transmute_copy(&pfresult)).into()
        }
        unsafe extern "system" fn SetObjectAccessRights<Identity: ::windows::core::IUnknownImpl, Impl: IObjectAccessControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut SEC_OBJECT, caccessentries: u32, prgaccessentries: *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetObjectAccessRights(::core::mem::transmute_copy(&pobject), ::core::mem::transmute_copy(&caccessentries), ::core::mem::transmute_copy(&prgaccessentries)).into()
        }
        unsafe extern "system" fn SetObjectOwner<Identity: ::windows::core::IUnknownImpl, Impl: IObjectAccessControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut SEC_OBJECT, powner: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetObjectOwner(::core::mem::transmute_copy(&pobject), ::core::mem::transmute_copy(&powner)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetObjectAccessRights: GetObjectAccessRights::<Identity, Impl, OFFSET>,
            GetObjectOwner: GetObjectOwner::<Identity, Impl, OFFSET>,
            IsObjectAccessAllowed: IsObjectAccessAllowed::<Identity, Impl, OFFSET>,
            SetObjectAccessRights: SetObjectAccessRights::<Identity, Impl, OFFSET>,
            SetObjectOwner: SetObjectOwner::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectAccessControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpLockStatus_Impl: Sized {
    fn IsOplockValid(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsOplockBroken(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetOplockEventHandle(&mut self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOpLockStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpLockStatus_Impl, const OFFSET: isize>() -> IOpLockStatus_Vtbl {
        unsafe extern "system" fn IsOplockValid<Identity: ::windows::core::IUnknownImpl, Impl: IOpLockStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisoplockvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsOplockValid() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisoplockvalid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOplockBroken<Identity: ::windows::core::IUnknownImpl, Impl: IOpLockStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisoplockbroken: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsOplockBroken() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisoplockbroken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOplockEventHandle<Identity: ::windows::core::IUnknownImpl, Impl: IOpLockStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoplockev: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOplockEventHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *phoplockev = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsOplockValid: IsOplockValid::<Identity, Impl, OFFSET>,
            IsOplockBroken: IsOplockBroken::<Identity, Impl, OFFSET>,
            GetOplockEventHandle: GetOplockEventHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpLockStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IOpenRowset_Impl: Sized {
    fn OpenRowset(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IOpenRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpenRowset_Impl, const OFFSET: isize>() -> IOpenRowset_Vtbl {
        unsafe extern "system" fn OpenRowset<Identity: ::windows::core::IUnknownImpl, Impl: IOpenRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenRowset(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pindexid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pprowset)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OpenRowset: OpenRowset::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpenRowset as ::windows::core::Interface>::IID
    }
}
pub trait IParentRowset_Impl: Sized {
    fn GetChildRowset(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, iordinal: usize, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl IParentRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IParentRowset_Impl, const OFFSET: isize>() -> IParentRowset_Vtbl {
        unsafe extern "system" fn GetChildRowset<Identity: ::windows::core::IUnknownImpl, Impl: IParentRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, iordinal: usize, riid: *const ::windows::core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetChildRowset(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&iordinal), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprowset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetChildRowset: GetChildRowset::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IParentRowset as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
pub trait IProtocolHandlerSite_Impl: Sized {
    fn GetFilter(&mut self, pclsidobj: *mut ::windows::core::GUID, pcwszcontenttype: super::super::Foundation::PWSTR, pcwszextension: super::super::Foundation::PWSTR, ppfilter: *mut ::core::option::Option<super::super::Storage::IndexServer::IFilter>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
impl IProtocolHandlerSite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtocolHandlerSite_Impl, const OFFSET: isize>() -> IProtocolHandlerSite_Vtbl {
        unsafe extern "system" fn GetFilter<Identity: ::windows::core::IUnknownImpl, Impl: IProtocolHandlerSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsidobj: *mut ::windows::core::GUID, pcwszcontenttype: super::super::Foundation::PWSTR, pcwszextension: super::super::Foundation::PWSTR, ppfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFilter(::core::mem::transmute_copy(&pclsidobj), ::core::mem::transmute_copy(&pcwszcontenttype), ::core::mem::transmute_copy(&pcwszextension), ::core::mem::transmute_copy(&ppfilter)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetFilter: GetFilter::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProtocolHandlerSite as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideMoniker_Impl: Sized {
    fn GetMoniker(&mut self) -> ::windows::core::Result<super::Com::IMoniker>;
}
#[cfg(feature = "Win32_System_Com")]
impl IProvideMoniker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideMoniker_Impl, const OFFSET: isize>() -> IProvideMoniker_Vtbl {
        unsafe extern "system" fn GetMoniker<Identity: ::windows::core::IUnknownImpl, Impl: IProvideMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimoniker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMoniker() {
                ::core::result::Result::Ok(ok__) => {
                    *ppimoniker = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetMoniker: GetMoniker::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideMoniker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IQueryParser_Impl: Sized {
    fn Parse(&mut self, pszinputstring: super::super::Foundation::PWSTR, pcustomproperties: &::core::option::Option<super::Com::IEnumUnknown>) -> ::windows::core::Result<IQuerySolution>;
    fn SetOption(&mut self, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetOption(&mut self, option: STRUCTURED_QUERY_SINGLE_OPTION) -> ::windows::core::Result<super::Com::StructuredStorage::PROPVARIANT>;
    fn SetMultiOption(&mut self, option: STRUCTURED_QUERY_MULTIOPTION, pszoptionkey: super::super::Foundation::PWSTR, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetSchemaProvider(&mut self) -> ::windows::core::Result<ISchemaProvider>;
    fn RestateToString(&mut self, pcondition: &::core::option::Option<ICondition>, fuseenglish: super::super::Foundation::BOOL) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn ParsePropertyValue(&mut self, pszpropertyname: super::super::Foundation::PWSTR, pszinputstring: super::super::Foundation::PWSTR) -> ::windows::core::Result<IQuerySolution>;
    fn RestatePropertyValueToString(&mut self, pcondition: &::core::option::Option<ICondition>, fuseenglish: super::super::Foundation::BOOL, ppszpropertyname: *mut super::super::Foundation::PWSTR, ppszquerystring: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IQueryParser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQueryParser_Impl, const OFFSET: isize>() -> IQueryParser_Vtbl {
        unsafe extern "system" fn Parse<Identity: ::windows::core::IUnknownImpl, Impl: IQueryParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszinputstring: super::super::Foundation::PWSTR, pcustomproperties: ::windows::core::RawPtr, ppsolution: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Parse(::core::mem::transmute_copy(&pszinputstring), ::core::mem::transmute(&pcustomproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsolution = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOption<Identity: ::windows::core::IUnknownImpl, Impl: IQueryParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOption(::core::mem::transmute_copy(&option), ::core::mem::transmute_copy(&poptionvalue)).into()
        }
        unsafe extern "system" fn GetOption<Identity: ::windows::core::IUnknownImpl, Impl: IQueryParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOption(::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    *poptionvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiOption<Identity: ::windows::core::IUnknownImpl, Impl: IQueryParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: STRUCTURED_QUERY_MULTIOPTION, pszoptionkey: super::super::Foundation::PWSTR, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMultiOption(::core::mem::transmute_copy(&option), ::core::mem::transmute_copy(&pszoptionkey), ::core::mem::transmute_copy(&poptionvalue)).into()
        }
        unsafe extern "system" fn GetSchemaProvider<Identity: ::windows::core::IUnknownImpl, Impl: IQueryParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppschemaprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSchemaProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *ppschemaprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestateToString<Identity: ::windows::core::IUnknownImpl, Impl: IQueryParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcondition: ::windows::core::RawPtr, fuseenglish: super::super::Foundation::BOOL, ppszquerystring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RestateToString(::core::mem::transmute(&pcondition), ::core::mem::transmute_copy(&fuseenglish)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszquerystring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParsePropertyValue<Identity: ::windows::core::IUnknownImpl, Impl: IQueryParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: super::super::Foundation::PWSTR, pszinputstring: super::super::Foundation::PWSTR, ppsolution: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ParsePropertyValue(::core::mem::transmute_copy(&pszpropertyname), ::core::mem::transmute_copy(&pszinputstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsolution = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestatePropertyValueToString<Identity: ::windows::core::IUnknownImpl, Impl: IQueryParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcondition: ::windows::core::RawPtr, fuseenglish: super::super::Foundation::BOOL, ppszpropertyname: *mut super::super::Foundation::PWSTR, ppszquerystring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestatePropertyValueToString(::core::mem::transmute(&pcondition), ::core::mem::transmute_copy(&fuseenglish), ::core::mem::transmute_copy(&ppszpropertyname), ::core::mem::transmute_copy(&ppszquerystring)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Parse: Parse::<Identity, Impl, OFFSET>,
            SetOption: SetOption::<Identity, Impl, OFFSET>,
            GetOption: GetOption::<Identity, Impl, OFFSET>,
            SetMultiOption: SetMultiOption::<Identity, Impl, OFFSET>,
            GetSchemaProvider: GetSchemaProvider::<Identity, Impl, OFFSET>,
            RestateToString: RestateToString::<Identity, Impl, OFFSET>,
            ParsePropertyValue: ParsePropertyValue::<Identity, Impl, OFFSET>,
            RestatePropertyValueToString: RestatePropertyValueToString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQueryParser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IQueryParserManager_Impl: Sized {
    fn CreateLoadedParser(&mut self, pszcatalog: super::super::Foundation::PWSTR, langidforkeywords: u16, riid: *const ::windows::core::GUID, ppqueryparser: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn InitializeOptions(&mut self, funderstandnqs: super::super::Foundation::BOOL, fautowildcard: super::super::Foundation::BOOL, pqueryparser: &::core::option::Option<IQueryParser>) -> ::windows::core::Result<()>;
    fn SetOption(&mut self, option: QUERY_PARSER_MANAGER_OPTION, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IQueryParserManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQueryParserManager_Impl, const OFFSET: isize>() -> IQueryParserManager_Vtbl {
        unsafe extern "system" fn CreateLoadedParser<Identity: ::windows::core::IUnknownImpl, Impl: IQueryParserManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcatalog: super::super::Foundation::PWSTR, langidforkeywords: u16, riid: *const ::windows::core::GUID, ppqueryparser: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateLoadedParser(::core::mem::transmute_copy(&pszcatalog), ::core::mem::transmute_copy(&langidforkeywords), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppqueryparser)).into()
        }
        unsafe extern "system" fn InitializeOptions<Identity: ::windows::core::IUnknownImpl, Impl: IQueryParserManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, funderstandnqs: super::super::Foundation::BOOL, fautowildcard: super::super::Foundation::BOOL, pqueryparser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeOptions(::core::mem::transmute_copy(&funderstandnqs), ::core::mem::transmute_copy(&fautowildcard), ::core::mem::transmute(&pqueryparser)).into()
        }
        unsafe extern "system" fn SetOption<Identity: ::windows::core::IUnknownImpl, Impl: IQueryParserManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: QUERY_PARSER_MANAGER_OPTION, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOption(::core::mem::transmute_copy(&option), ::core::mem::transmute_copy(&poptionvalue)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateLoadedParser: CreateLoadedParser::<Identity, Impl, OFFSET>,
            InitializeOptions: InitializeOptions::<Identity, Impl, OFFSET>,
            SetOption: SetOption::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQueryParserManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
pub trait IQuerySolution_Impl: Sized + IConditionFactory_Impl {
    fn GetQuery(&mut self, ppquerynode: *mut ::core::option::Option<ICondition>, ppmaintype: *mut ::core::option::Option<IEntity>) -> ::windows::core::Result<()>;
    fn GetErrors(&mut self, riid: *const ::windows::core::GUID, ppparseerrors: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetLexicalData(&mut self, ppszinputstring: *mut super::super::Foundation::PWSTR, pptokens: *mut ::core::option::Option<ITokenCollection>, plcid: *mut u32, ppwordbreaker: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl IQuerySolution_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQuerySolution_Impl, const OFFSET: isize>() -> IQuerySolution_Vtbl {
        unsafe extern "system" fn GetQuery<Identity: ::windows::core::IUnknownImpl, Impl: IQuerySolution_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppquerynode: *mut ::windows::core::RawPtr, ppmaintype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetQuery(::core::mem::transmute_copy(&ppquerynode), ::core::mem::transmute_copy(&ppmaintype)).into()
        }
        unsafe extern "system" fn GetErrors<Identity: ::windows::core::IUnknownImpl, Impl: IQuerySolution_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppparseerrors: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetErrors(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppparseerrors)).into()
        }
        unsafe extern "system" fn GetLexicalData<Identity: ::windows::core::IUnknownImpl, Impl: IQuerySolution_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszinputstring: *mut super::super::Foundation::PWSTR, pptokens: *mut ::windows::core::RawPtr, plcid: *mut u32, ppwordbreaker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLexicalData(::core::mem::transmute_copy(&ppszinputstring), ::core::mem::transmute_copy(&pptokens), ::core::mem::transmute_copy(&plcid), ::core::mem::transmute_copy(&ppwordbreaker)).into()
        }
        Self {
            base: IConditionFactory_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetQuery: GetQuery::<Identity, Impl, OFFSET>,
            GetErrors: GetErrors::<Identity, Impl, OFFSET>,
            GetLexicalData: GetLexicalData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQuerySolution as ::windows::core::Interface>::IID || iid == &<IConditionFactory as ::windows::core::Interface>::IID
    }
}
pub trait IReadData_Impl: Sized {
    fn ReadData(&mut self, hchapter: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, haccessor: usize, crows: isize, pcrowsobtained: *mut usize, ppfixeddata: *mut *mut u8, pcbvariabletotal: *mut usize, ppvariabledata: *mut *mut u8) -> ::windows::core::Result<()>;
    fn ReleaseChapter(&mut self, hchapter: usize) -> ::windows::core::Result<()>;
}
impl IReadData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReadData_Impl, const OFFSET: isize>() -> IReadData_Vtbl {
        unsafe extern "system" fn ReadData<Identity: ::windows::core::IUnknownImpl, Impl: IReadData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, haccessor: usize, crows: isize, pcrowsobtained: *mut usize, ppfixeddata: *mut *mut u8, pcbvariabletotal: *mut usize, ppvariabledata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReadData(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark), ::core::mem::transmute_copy(&lrowsoffset), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&pcrowsobtained), ::core::mem::transmute_copy(&ppfixeddata), ::core::mem::transmute_copy(&pcbvariabletotal), ::core::mem::transmute_copy(&ppvariabledata)).into()
        }
        unsafe extern "system" fn ReleaseChapter<Identity: ::windows::core::IUnknownImpl, Impl: IReadData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseChapter(::core::mem::transmute_copy(&hchapter)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ReadData: ReadData::<Identity, Impl, OFFSET>,
            ReleaseChapter: ReleaseChapter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReadData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRegisterProvider_Impl: Sized {
    fn GetURLMapping(&mut self, pwszurl: super::super::Foundation::PWSTR, dwreserved: usize) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetURLMapping(&mut self, pwszurl: super::super::Foundation::PWSTR, dwreserved: usize, rclsidprovider: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn UnregisterProvider(&mut self, pwszurl: super::super::Foundation::PWSTR, dwreserved: usize, rclsidprovider: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRegisterProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegisterProvider_Impl, const OFFSET: isize>() -> IRegisterProvider_Vtbl {
        unsafe extern "system" fn GetURLMapping<Identity: ::windows::core::IUnknownImpl, Impl: IRegisterProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, dwreserved: usize, pclsidprovider: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetURLMapping(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pclsidprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURLMapping<Identity: ::windows::core::IUnknownImpl, Impl: IRegisterProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, dwreserved: usize, rclsidprovider: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetURLMapping(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&rclsidprovider)).into()
        }
        unsafe extern "system" fn UnregisterProvider<Identity: ::windows::core::IUnknownImpl, Impl: IRegisterProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, dwreserved: usize, rclsidprovider: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterProvider(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&rclsidprovider)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetURLMapping: GetURLMapping::<Identity, Impl, OFFSET>,
            SetURLMapping: SetURLMapping::<Identity, Impl, OFFSET>,
            UnregisterProvider: UnregisterProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegisterProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRelationship_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn IsReal(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Destination(&mut self) -> ::windows::core::Result<IEntity>;
    fn MetaData(&mut self, riid: *const ::windows::core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn DefaultPhrase(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRelationship_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRelationship_Impl, const OFFSET: isize>() -> IRelationship_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReal<Identity: ::windows::core::IUnknownImpl, Impl: IRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisreal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsReal() {
                ::core::result::Result::Ok(ok__) => {
                    *pisreal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destination<Identity: ::windows::core::IUnknownImpl, Impl: IRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationentity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Destination() {
                ::core::result::Result::Ok(ok__) => {
                    *pdestinationentity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MetaData<Identity: ::windows::core::IUnknownImpl, Impl: IRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MetaData(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pmetadata)).into()
        }
        unsafe extern "system" fn DefaultPhrase<Identity: ::windows::core::IUnknownImpl, Impl: IRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszphrase: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultPhrase() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszphrase = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            IsReal: IsReal::<Identity, Impl, OFFSET>,
            Destination: Destination::<Identity, Impl, OFFSET>,
            MetaData: MetaData::<Identity, Impl, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRelationship as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IRichChunk_Impl: Sized {
    fn GetData(&mut self, pfirstpos: *mut u32, plength: *mut u32, ppsz: *mut super::super::Foundation::PWSTR, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IRichChunk_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichChunk_Impl, const OFFSET: isize>() -> IRichChunk_Vtbl {
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl, Impl: IRichChunk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfirstpos: *mut u32, plength: *mut u32, ppsz: *mut super::super::Foundation::PWSTR, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetData(::core::mem::transmute_copy(&pfirstpos), ::core::mem::transmute_copy(&plength), ::core::mem::transmute_copy(&ppsz), ::core::mem::transmute_copy(&pvalue)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetData: GetData::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichChunk as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
pub trait IRow_Impl: Sized {
    fn GetColumns(&mut self, ccolumns: usize, rgcolumns: *mut DBCOLUMNACCESS) -> ::windows::core::Result<()>;
    fn GetSourceRowset(&mut self, riid: *const ::windows::core::GUID, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>, phrow: *mut usize) -> ::windows::core::Result<()>;
    fn Open(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, pcolumnid: *const super::super::Storage::IndexServer::DBID, rguidcolumntype: *const ::windows::core::GUID, dwbindflags: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
impl IRow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRow_Impl, const OFFSET: isize>() -> IRow_Vtbl {
        unsafe extern "system" fn GetColumns<Identity: ::windows::core::IUnknownImpl, Impl: IRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolumns: usize, rgcolumns: *mut DBCOLUMNACCESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetColumns(::core::mem::transmute_copy(&ccolumns), ::core::mem::transmute_copy(&rgcolumns)).into()
        }
        unsafe extern "system" fn GetSourceRowset<Identity: ::windows::core::IUnknownImpl, Impl: IRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pprowset: *mut *mut ::core::ffi::c_void, phrow: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSourceRowset(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pprowset), ::core::mem::transmute_copy(&phrow)).into()
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pcolumnid: *const super::super::Storage::IndexServer::DBID, rguidcolumntype: *const ::windows::core::GUID, dwbindflags: u32, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Open(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&pcolumnid), ::core::mem::transmute_copy(&rguidcolumntype), ::core::mem::transmute_copy(&dwbindflags), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetColumns: GetColumns::<Identity, Impl, OFFSET>,
            GetSourceRowset: GetSourceRowset::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRow as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
pub trait IRowChange_Impl: Sized {
    fn SetColumns(&mut self, ccolumns: usize, rgcolumns: *const DBCOLUMNACCESS) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
impl IRowChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowChange_Impl, const OFFSET: isize>() -> IRowChange_Vtbl {
        unsafe extern "system" fn SetColumns<Identity: ::windows::core::IUnknownImpl, Impl: IRowChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolumns: usize, rgcolumns: *const DBCOLUMNACCESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColumns(::core::mem::transmute_copy(&ccolumns), ::core::mem::transmute_copy(&rgcolumns)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetColumns: SetColumns::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowChange as ::windows::core::Interface>::IID
    }
}
pub trait IRowPosition_Impl: Sized {
    fn ClearRowPosition(&mut self) -> ::windows::core::Result<()>;
    fn GetRowPosition(&mut self, phchapter: *mut usize, phrow: *mut usize, pdwpositionflags: *mut u32) -> ::windows::core::Result<()>;
    fn GetRowset(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Initialize(&mut self, prowset: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetRowPosition(&mut self, hchapter: usize, hrow: usize, dwpositionflags: u32) -> ::windows::core::Result<()>;
}
impl IRowPosition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowPosition_Impl, const OFFSET: isize>() -> IRowPosition_Vtbl {
        unsafe extern "system" fn ClearRowPosition<Identity: ::windows::core::IUnknownImpl, Impl: IRowPosition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearRowPosition().into()
        }
        unsafe extern "system" fn GetRowPosition<Identity: ::windows::core::IUnknownImpl, Impl: IRowPosition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phchapter: *mut usize, phrow: *mut usize, pdwpositionflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRowPosition(::core::mem::transmute_copy(&phchapter), ::core::mem::transmute_copy(&phrow), ::core::mem::transmute_copy(&pdwpositionflags)).into()
        }
        unsafe extern "system" fn GetRowset<Identity: ::windows::core::IUnknownImpl, Impl: IRowPosition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRowset(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprowset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IRowPosition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prowset: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&prowset)).into()
        }
        unsafe extern "system" fn SetRowPosition<Identity: ::windows::core::IUnknownImpl, Impl: IRowPosition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, hrow: usize, dwpositionflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRowPosition(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&dwpositionflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ClearRowPosition: ClearRowPosition::<Identity, Impl, OFFSET>,
            GetRowPosition: GetRowPosition::<Identity, Impl, OFFSET>,
            GetRowset: GetRowset::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            SetRowPosition: SetRowPosition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowPosition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRowPositionChange_Impl: Sized {
    fn OnRowPositionChange(&mut self, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRowPositionChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowPositionChange_Impl, const OFFSET: isize>() -> IRowPositionChange_Vtbl {
        unsafe extern "system" fn OnRowPositionChange<Identity: ::windows::core::IUnknownImpl, Impl: IRowPositionChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnRowPositionChange(::core::mem::transmute_copy(&ereason), ::core::mem::transmute_copy(&ephase), ::core::mem::transmute_copy(&fcantdeny)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnRowPositionChange: OnRowPositionChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowPositionChange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait IRowSchemaChange_Impl: Sized + IRowChange_Impl {
    fn DeleteColumns(&mut self, ccolumns: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgdwstatus: *mut u32) -> ::windows::core::Result<()>;
    fn AddColumns(&mut self, ccolumns: usize, rgnewcolumninfo: *const DBCOLUMNINFO, rgcolumns: *mut DBCOLUMNACCESS) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl IRowSchemaChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowSchemaChange_Impl, const OFFSET: isize>() -> IRowSchemaChange_Vtbl {
        unsafe extern "system" fn DeleteColumns<Identity: ::windows::core::IUnknownImpl, Impl: IRowSchemaChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolumns: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteColumns(::core::mem::transmute_copy(&ccolumns), ::core::mem::transmute_copy(&rgcolumnids), ::core::mem::transmute_copy(&rgdwstatus)).into()
        }
        unsafe extern "system" fn AddColumns<Identity: ::windows::core::IUnknownImpl, Impl: IRowSchemaChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolumns: usize, rgnewcolumninfo: *const DBCOLUMNINFO, rgcolumns: *mut DBCOLUMNACCESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddColumns(::core::mem::transmute_copy(&ccolumns), ::core::mem::transmute_copy(&rgnewcolumninfo), ::core::mem::transmute_copy(&rgcolumns)).into()
        }
        Self {
            base: IRowChange_Vtbl::new::<Identity, Impl, OFFSET>(),
            DeleteColumns: DeleteColumns::<Identity, Impl, OFFSET>,
            AddColumns: AddColumns::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowSchemaChange as ::windows::core::Interface>::IID || iid == &<IRowChange as ::windows::core::Interface>::IID
    }
}
pub trait IRowset_Impl: Sized {
    fn AddRefRows(&mut self, crows: usize, rghrows: *const usize, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows::core::Result<()>;
    fn GetData(&mut self, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetNextRows(&mut self, hreserved: usize, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::Result<()>;
    fn ReleaseRows(&mut self, crows: usize, rghrows: *const usize, rgrowoptions: *mut u32, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows::core::Result<()>;
    fn RestartPosition(&mut self, hreserved: usize) -> ::windows::core::Result<()>;
}
impl IRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowset_Impl, const OFFSET: isize>() -> IRowset_Vtbl {
        unsafe extern "system" fn AddRefRows<Identity: ::windows::core::IUnknownImpl, Impl: IRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crows: usize, rghrows: *const usize, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRefRows(::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&rgrefcounts), ::core::mem::transmute_copy(&rgrowstatus)).into()
        }
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl, Impl: IRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetData(::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetNextRows<Identity: ::windows::core::IUnknownImpl, Impl: IRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNextRows(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&lrowsoffset), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&pcrowsobtained), ::core::mem::transmute_copy(&prghrows)).into()
        }
        unsafe extern "system" fn ReleaseRows<Identity: ::windows::core::IUnknownImpl, Impl: IRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crows: usize, rghrows: *const usize, rgrowoptions: *mut u32, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseRows(::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&rgrowoptions), ::core::mem::transmute_copy(&rgrefcounts), ::core::mem::transmute_copy(&rgrowstatus)).into()
        }
        unsafe extern "system" fn RestartPosition<Identity: ::windows::core::IUnknownImpl, Impl: IRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestartPosition(::core::mem::transmute_copy(&hreserved)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddRefRows: AddRefRows::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            GetNextRows: GetNextRows::<Identity, Impl, OFFSET>,
            ReleaseRows: ReleaseRows::<Identity, Impl, OFFSET>,
            RestartPosition: RestartPosition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowset as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRowsetAsynch_Impl: Sized {
    fn RatioFinished(&mut self, puldenominator: *mut usize, pulnumerator: *mut usize, pcrows: *mut usize, pfnewrows: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRowsetAsynch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetAsynch_Impl, const OFFSET: isize>() -> IRowsetAsynch_Vtbl {
        unsafe extern "system" fn RatioFinished<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetAsynch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puldenominator: *mut usize, pulnumerator: *mut usize, pcrows: *mut usize, pfnewrows: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RatioFinished(::core::mem::transmute_copy(&puldenominator), ::core::mem::transmute_copy(&pulnumerator), ::core::mem::transmute_copy(&pcrows), ::core::mem::transmute_copy(&pfnewrows)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetAsynch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Stop().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RatioFinished: RatioFinished::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetAsynch as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetBookmark_Impl: Sized {
    fn PositionOnBookmark(&mut self, hchapter: usize, cbbookmark: usize, pbookmark: *const u8) -> ::windows::core::Result<()>;
}
impl IRowsetBookmark_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetBookmark_Impl, const OFFSET: isize>() -> IRowsetBookmark_Vtbl {
        unsafe extern "system" fn PositionOnBookmark<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetBookmark_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, cbbookmark: usize, pbookmark: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PositionOnBookmark(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), PositionOnBookmark: PositionOnBookmark::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetBookmark as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetChange_Impl: Sized {
    fn DeleteRows(&mut self, hreserved: usize, crows: usize, rghrows: *const usize, rgrowstatus: *mut u32) -> ::windows::core::Result<()>;
    fn SetData(&mut self, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn InsertRow(&mut self, hreserved: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void, phrow: *mut usize) -> ::windows::core::Result<()>;
}
impl IRowsetChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetChange_Impl, const OFFSET: isize>() -> IRowsetChange_Vtbl {
        unsafe extern "system" fn DeleteRows<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, rgrowstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteRows(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&rgrowstatus)).into()
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn InsertRow<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void, phrow: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InsertRow(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&phrow)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DeleteRows: DeleteRows::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
            InsertRow: InsertRow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetChange as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetChangeExtInfo_Impl: Sized {
    fn GetOriginalRow(&mut self, hreserved: usize, hrow: usize, phroworiginal: *mut usize) -> ::windows::core::Result<()>;
    fn GetPendingColumns(&mut self, hreserved: usize, hrow: usize, ccolumnordinals: u32, rgiordinals: *const u32, rgcolumnstatus: *mut u32) -> ::windows::core::Result<()>;
}
impl IRowsetChangeExtInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetChangeExtInfo_Impl, const OFFSET: isize>() -> IRowsetChangeExtInfo_Vtbl {
        unsafe extern "system" fn GetOriginalRow<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetChangeExtInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, hrow: usize, phroworiginal: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOriginalRow(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&phroworiginal)).into()
        }
        unsafe extern "system" fn GetPendingColumns<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetChangeExtInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, hrow: usize, ccolumnordinals: u32, rgiordinals: *const u32, rgcolumnstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPendingColumns(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&ccolumnordinals), ::core::mem::transmute_copy(&rgiordinals), ::core::mem::transmute_copy(&rgcolumnstatus)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetOriginalRow: GetOriginalRow::<Identity, Impl, OFFSET>,
            GetPendingColumns: GetPendingColumns::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetChangeExtInfo as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetChapterMember_Impl: Sized {
    fn IsRowInChapter(&mut self, hchapter: usize, hrow: usize) -> ::windows::core::Result<()>;
}
impl IRowsetChapterMember_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetChapterMember_Impl, const OFFSET: isize>() -> IRowsetChapterMember_Vtbl {
        unsafe extern "system" fn IsRowInChapter<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetChapterMember_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, hrow: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsRowInChapter(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&hrow)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), IsRowInChapter: IsRowInChapter::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetChapterMember as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetCopyRows_Impl: Sized {
    fn CloseSource(&mut self, hsourceid: u16) -> ::windows::core::Result<()>;
    fn CopyByHROWS(&mut self, hsourceid: u16, hreserved: usize, crows: isize, rghrows: *const usize, bflags: u32) -> ::windows::core::Result<()>;
    fn CopyRows(&mut self, hsourceid: u16, hreserved: usize, crows: isize, bflags: u32, pcrowscopied: *mut usize) -> ::windows::core::Result<()>;
    fn DefineSource(&mut self, prowsetsource: &::core::option::Option<IRowset>, ccolids: usize, rgsourcecolumns: *const isize, rgtargetcolumns: *const isize, phsourceid: *mut u16) -> ::windows::core::Result<()>;
}
impl IRowsetCopyRows_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetCopyRows_Impl, const OFFSET: isize>() -> IRowsetCopyRows_Vtbl {
        unsafe extern "system" fn CloseSource<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetCopyRows_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsourceid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CloseSource(::core::mem::transmute_copy(&hsourceid)).into()
        }
        unsafe extern "system" fn CopyByHROWS<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetCopyRows_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsourceid: u16, hreserved: usize, crows: isize, rghrows: *const usize, bflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyByHROWS(::core::mem::transmute_copy(&hsourceid), ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&bflags)).into()
        }
        unsafe extern "system" fn CopyRows<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetCopyRows_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsourceid: u16, hreserved: usize, crows: isize, bflags: u32, pcrowscopied: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyRows(::core::mem::transmute_copy(&hsourceid), ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&bflags), ::core::mem::transmute_copy(&pcrowscopied)).into()
        }
        unsafe extern "system" fn DefineSource<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetCopyRows_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prowsetsource: ::windows::core::RawPtr, ccolids: usize, rgsourcecolumns: *const isize, rgtargetcolumns: *const isize, phsourceid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DefineSource(::core::mem::transmute(&prowsetsource), ::core::mem::transmute_copy(&ccolids), ::core::mem::transmute_copy(&rgsourcecolumns), ::core::mem::transmute_copy(&rgtargetcolumns), ::core::mem::transmute_copy(&phsourceid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CloseSource: CloseSource::<Identity, Impl, OFFSET>,
            CopyByHROWS: CopyByHROWS::<Identity, Impl, OFFSET>,
            CopyRows: CopyRows::<Identity, Impl, OFFSET>,
            DefineSource: DefineSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetCopyRows as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRowsetCurrentIndex_Impl: Sized + IRowsetIndex_Impl {
    fn GetIndex(&mut self, ppindexid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
    fn SetIndex(&mut self, pindexid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRowsetCurrentIndex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetCurrentIndex_Impl, const OFFSET: isize>() -> IRowsetCurrentIndex_Vtbl {
        unsafe extern "system" fn GetIndex<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetCurrentIndex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppindexid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIndex(::core::mem::transmute_copy(&ppindexid)).into()
        }
        unsafe extern "system" fn SetIndex<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetCurrentIndex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIndex(::core::mem::transmute_copy(&pindexid)).into()
        }
        Self {
            base: IRowsetIndex_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
            SetIndex: SetIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetCurrentIndex as ::windows::core::Interface>::IID || iid == &<IRowsetIndex as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IRowsetEvents_Impl: Sized {
    fn OnNewItem(&mut self, itemid: *const super::Com::StructuredStorage::PROPVARIANT, newitemstate: ROWSETEVENT_ITEMSTATE) -> ::windows::core::Result<()>;
    fn OnChangedItem(&mut self, itemid: *const super::Com::StructuredStorage::PROPVARIANT, rowsetitemstate: ROWSETEVENT_ITEMSTATE, changeditemstate: ROWSETEVENT_ITEMSTATE) -> ::windows::core::Result<()>;
    fn OnDeletedItem(&mut self, itemid: *const super::Com::StructuredStorage::PROPVARIANT, deleteditemstate: ROWSETEVENT_ITEMSTATE) -> ::windows::core::Result<()>;
    fn OnRowsetEvent(&mut self, eventtype: ROWSETEVENT_TYPE, eventdata: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IRowsetEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetEvents_Impl, const OFFSET: isize>() -> IRowsetEvents_Vtbl {
        unsafe extern "system" fn OnNewItem<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: *const super::Com::StructuredStorage::PROPVARIANT, newitemstate: ROWSETEVENT_ITEMSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnNewItem(::core::mem::transmute_copy(&itemid), ::core::mem::transmute_copy(&newitemstate)).into()
        }
        unsafe extern "system" fn OnChangedItem<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: *const super::Com::StructuredStorage::PROPVARIANT, rowsetitemstate: ROWSETEVENT_ITEMSTATE, changeditemstate: ROWSETEVENT_ITEMSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnChangedItem(::core::mem::transmute_copy(&itemid), ::core::mem::transmute_copy(&rowsetitemstate), ::core::mem::transmute_copy(&changeditemstate)).into()
        }
        unsafe extern "system" fn OnDeletedItem<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: *const super::Com::StructuredStorage::PROPVARIANT, deleteditemstate: ROWSETEVENT_ITEMSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDeletedItem(::core::mem::transmute_copy(&itemid), ::core::mem::transmute_copy(&deleteditemstate)).into()
        }
        unsafe extern "system" fn OnRowsetEvent<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventtype: ROWSETEVENT_TYPE, eventdata: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnRowsetEvent(::core::mem::transmute_copy(&eventtype), ::core::mem::transmute_copy(&eventdata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnNewItem: OnNewItem::<Identity, Impl, OFFSET>,
            OnChangedItem: OnChangedItem::<Identity, Impl, OFFSET>,
            OnDeletedItem: OnDeletedItem::<Identity, Impl, OFFSET>,
            OnRowsetEvent: OnRowsetEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRowsetFastLoad_Impl: Sized {
    fn InsertRow(&mut self, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Commit(&mut self, fdone: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRowsetFastLoad_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetFastLoad_Impl, const OFFSET: isize>() -> IRowsetFastLoad_Vtbl {
        unsafe extern "system" fn InsertRow<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetFastLoad_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InsertRow(::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetFastLoad_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdone: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit(::core::mem::transmute_copy(&fdone)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            InsertRow: InsertRow::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetFastLoad as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetFind_Impl: Sized {
    fn FindNextRow(&mut self, hchapter: usize, haccessor: usize, pfindvalue: *mut ::core::ffi::c_void, compareop: u32, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::Result<()>;
}
impl IRowsetFind_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetFind_Impl, const OFFSET: isize>() -> IRowsetFind_Vtbl {
        unsafe extern "system" fn FindNextRow<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetFind_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, haccessor: usize, pfindvalue: *mut ::core::ffi::c_void, compareop: u32, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindNextRow(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pfindvalue), ::core::mem::transmute_copy(&compareop), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark), ::core::mem::transmute_copy(&lrowsoffset), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&pcrowsobtained), ::core::mem::transmute_copy(&prghrows)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), FindNextRow: FindNextRow::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetFind as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetIdentity_Impl: Sized {
    fn IsSameRow(&mut self, hthisrow: usize, hthatrow: usize) -> ::windows::core::Result<()>;
}
impl IRowsetIdentity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetIdentity_Impl, const OFFSET: isize>() -> IRowsetIdentity_Vtbl {
        unsafe extern "system" fn IsSameRow<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hthisrow: usize, hthatrow: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsSameRow(::core::mem::transmute_copy(&hthisrow), ::core::mem::transmute_copy(&hthatrow)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), IsSameRow: IsSameRow::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetIdentity as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRowsetIndex_Impl: Sized {
    fn GetIndexInfo(&mut self, pckeycolumns: *mut usize, prgindexcolumndesc: *mut *mut DBINDEXCOLUMNDESC, pcindexpropertysets: *mut u32, prgindexpropertysets: *mut *mut DBPROPSET) -> ::windows::core::Result<()>;
    fn Seek(&mut self, haccessor: usize, ckeyvalues: usize, pdata: *mut ::core::ffi::c_void, dwseekoptions: u32) -> ::windows::core::Result<()>;
    fn SetRange(&mut self, haccessor: usize, cstartkeycolumns: usize, pstartdata: *mut ::core::ffi::c_void, cendkeycolumns: usize, penddata: *mut ::core::ffi::c_void, dwrangeoptions: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRowsetIndex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetIndex_Impl, const OFFSET: isize>() -> IRowsetIndex_Vtbl {
        unsafe extern "system" fn GetIndexInfo<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetIndex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pckeycolumns: *mut usize, prgindexcolumndesc: *mut *mut DBINDEXCOLUMNDESC, pcindexpropertysets: *mut u32, prgindexpropertysets: *mut *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIndexInfo(::core::mem::transmute_copy(&pckeycolumns), ::core::mem::transmute_copy(&prgindexcolumndesc), ::core::mem::transmute_copy(&pcindexpropertysets), ::core::mem::transmute_copy(&prgindexpropertysets)).into()
        }
        unsafe extern "system" fn Seek<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetIndex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haccessor: usize, ckeyvalues: usize, pdata: *mut ::core::ffi::c_void, dwseekoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Seek(::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&ckeyvalues), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwseekoptions)).into()
        }
        unsafe extern "system" fn SetRange<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetIndex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haccessor: usize, cstartkeycolumns: usize, pstartdata: *mut ::core::ffi::c_void, cendkeycolumns: usize, penddata: *mut ::core::ffi::c_void, dwrangeoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRange(::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&cstartkeycolumns), ::core::mem::transmute_copy(&pstartdata), ::core::mem::transmute_copy(&cendkeycolumns), ::core::mem::transmute_copy(&penddata), ::core::mem::transmute_copy(&dwrangeoptions)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetIndexInfo: GetIndexInfo::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            SetRange: SetRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetIndex as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRowsetInfo_Impl: Sized {
    fn GetProperties(&mut self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::Result<()>;
    fn GetReferencedRowset(&mut self, iordinal: usize, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetSpecification(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRowsetInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetInfo_Impl, const OFFSET: isize>() -> IRowsetInfo_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProperties(::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)).into()
        }
        unsafe extern "system" fn GetReferencedRowset<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iordinal: usize, riid: *const ::windows::core::GUID, ppreferencedrowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetReferencedRowset(::core::mem::transmute_copy(&iordinal), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppreferencedrowset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpecification<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppspecification: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSpecification(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppspecification = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetReferencedRowset: GetReferencedRowset::<Identity, Impl, OFFSET>,
            GetSpecification: GetSpecification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetInfo as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetKeys_Impl: Sized {
    fn ListKeys(&mut self, pccolumns: *mut usize, prgcolumns: *mut *mut usize) -> ::windows::core::Result<()>;
}
impl IRowsetKeys_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetKeys_Impl, const OFFSET: isize>() -> IRowsetKeys_Vtbl {
        unsafe extern "system" fn ListKeys<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccolumns: *mut usize, prgcolumns: *mut *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ListKeys(::core::mem::transmute_copy(&pccolumns), ::core::mem::transmute_copy(&prgcolumns)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ListKeys: ListKeys::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetKeys as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetLocate_Impl: Sized + IRowset_Impl {
    fn Compare(&mut self, hreserved: usize, cbbookmark1: usize, pbookmark1: *const u8, cbbookmark2: usize, pbookmark2: *const u8, pcomparison: *mut u32) -> ::windows::core::Result<()>;
    fn GetRowsAt(&mut self, hreserved1: usize, hreserved2: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::Result<()>;
    fn GetRowsByBookmark(&mut self, hreserved: usize, crows: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghrows: *mut usize, rgrowstatus: *mut u32) -> ::windows::core::Result<()>;
    fn Hash(&mut self, hreserved: usize, cbookmarks: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghashedvalues: *mut usize, rgbookmarkstatus: *mut u32) -> ::windows::core::Result<()>;
}
impl IRowsetLocate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetLocate_Impl, const OFFSET: isize>() -> IRowsetLocate_Vtbl {
        unsafe extern "system" fn Compare<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetLocate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, cbbookmark1: usize, pbookmark1: *const u8, cbbookmark2: usize, pbookmark2: *const u8, pcomparison: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Compare(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&cbbookmark1), ::core::mem::transmute_copy(&pbookmark1), ::core::mem::transmute_copy(&cbbookmark2), ::core::mem::transmute_copy(&pbookmark2), ::core::mem::transmute_copy(&pcomparison)).into()
        }
        unsafe extern "system" fn GetRowsAt<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetLocate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved1: usize, hreserved2: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRowsAt(::core::mem::transmute_copy(&hreserved1), ::core::mem::transmute_copy(&hreserved2), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark), ::core::mem::transmute_copy(&lrowsoffset), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&pcrowsobtained), ::core::mem::transmute_copy(&prghrows)).into()
        }
        unsafe extern "system" fn GetRowsByBookmark<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetLocate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghrows: *mut usize, rgrowstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRowsByBookmark(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rgcbbookmarks), ::core::mem::transmute_copy(&rgpbookmarks), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&rgrowstatus)).into()
        }
        unsafe extern "system" fn Hash<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetLocate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, cbookmarks: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghashedvalues: *mut usize, rgbookmarkstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Hash(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&cbookmarks), ::core::mem::transmute_copy(&rgcbbookmarks), ::core::mem::transmute_copy(&rgpbookmarks), ::core::mem::transmute_copy(&rghashedvalues), ::core::mem::transmute_copy(&rgbookmarkstatus)).into()
        }
        Self {
            base: IRowset_Vtbl::new::<Identity, Impl, OFFSET>(),
            Compare: Compare::<Identity, Impl, OFFSET>,
            GetRowsAt: GetRowsAt::<Identity, Impl, OFFSET>,
            GetRowsByBookmark: GetRowsByBookmark::<Identity, Impl, OFFSET>,
            Hash: Hash::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetLocate as ::windows::core::Interface>::IID || iid == &<IRowset as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetNewRowAfter_Impl: Sized {
    fn SetNewDataAfter(&mut self, hchapter: usize, cbbmprevious: u32, pbmprevious: *const u8, haccessor: usize, pdata: *mut u8, phrow: *mut usize) -> ::windows::core::Result<()>;
}
impl IRowsetNewRowAfter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetNewRowAfter_Impl, const OFFSET: isize>() -> IRowsetNewRowAfter_Vtbl {
        unsafe extern "system" fn SetNewDataAfter<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetNewRowAfter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, cbbmprevious: u32, pbmprevious: *const u8, haccessor: usize, pdata: *mut u8, phrow: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNewDataAfter(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&cbbmprevious), ::core::mem::transmute_copy(&pbmprevious), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&phrow)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetNewDataAfter: SetNewDataAfter::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetNewRowAfter as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetNextRowset_Impl: Sized {
    fn GetNextRowset(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl IRowsetNextRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetNextRowset_Impl, const OFFSET: isize>() -> IRowsetNextRowset_Vtbl {
        unsafe extern "system" fn GetNextRowset<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetNextRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppnextrowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNextRowset(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnextrowset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetNextRowset: GetNextRowset::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetNextRowset as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRowsetNotify_Impl: Sized {
    fn OnFieldChange(&mut self, prowset: &::core::option::Option<IRowset>, hrow: usize, ccolumns: usize, rgcolumns: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnRowChange(&mut self, prowset: &::core::option::Option<IRowset>, crows: usize, rghrows: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnRowsetChange(&mut self, prowset: &::core::option::Option<IRowset>, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRowsetNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetNotify_Impl, const OFFSET: isize>() -> IRowsetNotify_Vtbl {
        unsafe extern "system" fn OnFieldChange<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prowset: ::windows::core::RawPtr, hrow: usize, ccolumns: usize, rgcolumns: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnFieldChange(::core::mem::transmute(&prowset), ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&ccolumns), ::core::mem::transmute_copy(&rgcolumns), ::core::mem::transmute_copy(&ereason), ::core::mem::transmute_copy(&ephase), ::core::mem::transmute_copy(&fcantdeny)).into()
        }
        unsafe extern "system" fn OnRowChange<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prowset: ::windows::core::RawPtr, crows: usize, rghrows: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnRowChange(::core::mem::transmute(&prowset), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&ereason), ::core::mem::transmute_copy(&ephase), ::core::mem::transmute_copy(&fcantdeny)).into()
        }
        unsafe extern "system" fn OnRowsetChange<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prowset: ::windows::core::RawPtr, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnRowsetChange(::core::mem::transmute(&prowset), ::core::mem::transmute_copy(&ereason), ::core::mem::transmute_copy(&ephase), ::core::mem::transmute_copy(&fcantdeny)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnFieldChange: OnFieldChange::<Identity, Impl, OFFSET>,
            OnRowChange: OnRowChange::<Identity, Impl, OFFSET>,
            OnRowsetChange: OnRowsetChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetNotify as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetPrioritization_Impl: Sized {
    fn SetScopePriority(&mut self, priority: PRIORITY_LEVEL, scopestatisticseventfrequency: u32) -> ::windows::core::Result<()>;
    fn GetScopePriority(&mut self, priority: *mut PRIORITY_LEVEL, scopestatisticseventfrequency: *mut u32) -> ::windows::core::Result<()>;
    fn GetScopeStatistics(&mut self, indexeddocumentcount: *mut u32, oustandingaddcount: *mut u32, oustandingmodifycount: *mut u32) -> ::windows::core::Result<()>;
}
impl IRowsetPrioritization_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetPrioritization_Impl, const OFFSET: isize>() -> IRowsetPrioritization_Vtbl {
        unsafe extern "system" fn SetScopePriority<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetPrioritization_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: PRIORITY_LEVEL, scopestatisticseventfrequency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScopePriority(::core::mem::transmute_copy(&priority), ::core::mem::transmute_copy(&scopestatisticseventfrequency)).into()
        }
        unsafe extern "system" fn GetScopePriority<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetPrioritization_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: *mut PRIORITY_LEVEL, scopestatisticseventfrequency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetScopePriority(::core::mem::transmute_copy(&priority), ::core::mem::transmute_copy(&scopestatisticseventfrequency)).into()
        }
        unsafe extern "system" fn GetScopeStatistics<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetPrioritization_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexeddocumentcount: *mut u32, oustandingaddcount: *mut u32, oustandingmodifycount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetScopeStatistics(::core::mem::transmute_copy(&indexeddocumentcount), ::core::mem::transmute_copy(&oustandingaddcount), ::core::mem::transmute_copy(&oustandingmodifycount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetScopePriority: SetScopePriority::<Identity, Impl, OFFSET>,
            GetScopePriority: GetScopePriority::<Identity, Impl, OFFSET>,
            GetScopeStatistics: GetScopeStatistics::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetPrioritization as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetQueryStatus_Impl: Sized {
    fn GetStatus(&mut self, pdwstatus: *mut u32) -> ::windows::core::Result<()>;
    fn GetStatusEx(&mut self, pdwstatus: *mut u32, pcfiltereddocuments: *mut u32, pcdocumentstofilter: *mut u32, pdwratiofinisheddenominator: *mut usize, pdwratiofinishednumerator: *mut usize, cbbmk: usize, pbmk: *const u8, pirowbmk: *mut usize, pcrowstotal: *mut usize) -> ::windows::core::Result<()>;
}
impl IRowsetQueryStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetQueryStatus_Impl, const OFFSET: isize>() -> IRowsetQueryStatus_Vtbl {
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetQueryStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&pdwstatus)).into()
        }
        unsafe extern "system" fn GetStatusEx<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetQueryStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32, pcfiltereddocuments: *mut u32, pcdocumentstofilter: *mut u32, pdwratiofinisheddenominator: *mut usize, pdwratiofinishednumerator: *mut usize, cbbmk: usize, pbmk: *const u8, pirowbmk: *mut usize, pcrowstotal: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatusEx(::core::mem::transmute_copy(&pdwstatus), ::core::mem::transmute_copy(&pcfiltereddocuments), ::core::mem::transmute_copy(&pcdocumentstofilter), ::core::mem::transmute_copy(&pdwratiofinisheddenominator), ::core::mem::transmute_copy(&pdwratiofinishednumerator), ::core::mem::transmute_copy(&cbbmk), ::core::mem::transmute_copy(&pbmk), ::core::mem::transmute_copy(&pirowbmk), ::core::mem::transmute_copy(&pcrowstotal)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetStatusEx: GetStatusEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetQueryStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRowsetRefresh_Impl: Sized {
    fn RefreshVisibleData(&mut self, hchapter: usize, crows: usize, rghrows: *const usize, foverwrite: super::super::Foundation::BOOL, pcrowsrefreshed: *mut usize, prghrowsrefreshed: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::Result<()>;
    fn GetLastVisibleData(&mut self, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRowsetRefresh_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetRefresh_Impl, const OFFSET: isize>() -> IRowsetRefresh_Vtbl {
        unsafe extern "system" fn RefreshVisibleData<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetRefresh_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, crows: usize, rghrows: *const usize, foverwrite: super::super::Foundation::BOOL, pcrowsrefreshed: *mut usize, prghrowsrefreshed: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RefreshVisibleData(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&foverwrite), ::core::mem::transmute_copy(&pcrowsrefreshed), ::core::mem::transmute_copy(&prghrowsrefreshed), ::core::mem::transmute_copy(&prgrowstatus)).into()
        }
        unsafe extern "system" fn GetLastVisibleData<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetRefresh_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLastVisibleData(::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RefreshVisibleData: RefreshVisibleData::<Identity, Impl, OFFSET>,
            GetLastVisibleData: GetLastVisibleData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetRefresh as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetResynch_Impl: Sized {
    fn GetVisibleData(&mut self, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ResynchRows(&mut self, crows: usize, rghrows: *const usize, pcrowsresynched: *mut usize, prghrowsresynched: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::Result<()>;
}
impl IRowsetResynch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetResynch_Impl, const OFFSET: isize>() -> IRowsetResynch_Vtbl {
        unsafe extern "system" fn GetVisibleData<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetResynch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVisibleData(::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn ResynchRows<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetResynch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crows: usize, rghrows: *const usize, pcrowsresynched: *mut usize, prghrowsresynched: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResynchRows(::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&pcrowsresynched), ::core::mem::transmute_copy(&prghrowsresynched), ::core::mem::transmute_copy(&prgrowstatus)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetVisibleData: GetVisibleData::<Identity, Impl, OFFSET>,
            ResynchRows: ResynchRows::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetResynch as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetScroll_Impl: Sized + IRowset_Impl + IRowsetLocate_Impl {
    fn GetApproximatePosition(&mut self, hreserved: usize, cbbookmark: usize, pbookmark: *const u8, pulposition: *mut usize, pcrows: *mut usize) -> ::windows::core::Result<()>;
    fn GetRowsAtRatio(&mut self, hreserved1: usize, hreserved2: usize, ulnumerator: usize, uldenominator: usize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::Result<()>;
}
impl IRowsetScroll_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetScroll_Impl, const OFFSET: isize>() -> IRowsetScroll_Vtbl {
        unsafe extern "system" fn GetApproximatePosition<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetScroll_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, cbbookmark: usize, pbookmark: *const u8, pulposition: *mut usize, pcrows: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetApproximatePosition(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark), ::core::mem::transmute_copy(&pulposition), ::core::mem::transmute_copy(&pcrows)).into()
        }
        unsafe extern "system" fn GetRowsAtRatio<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetScroll_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved1: usize, hreserved2: usize, ulnumerator: usize, uldenominator: usize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRowsAtRatio(::core::mem::transmute_copy(&hreserved1), ::core::mem::transmute_copy(&hreserved2), ::core::mem::transmute_copy(&ulnumerator), ::core::mem::transmute_copy(&uldenominator), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&pcrowsobtained), ::core::mem::transmute_copy(&prghrows)).into()
        }
        Self {
            base: IRowsetLocate_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetApproximatePosition: GetApproximatePosition::<Identity, Impl, OFFSET>,
            GetRowsAtRatio: GetRowsAtRatio::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetScroll as ::windows::core::Interface>::IID || iid == &<IRowset as ::windows::core::Interface>::IID || iid == &<IRowsetLocate as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetUpdate_Impl: Sized + IRowsetChange_Impl {
    fn GetOriginalData(&mut self, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetPendingRows(&mut self, hreserved: usize, dwrowstatus: u32, pcpendingrows: *mut usize, prgpendingrows: *mut *mut usize, prgpendingstatus: *mut *mut u32) -> ::windows::core::Result<()>;
    fn GetRowStatus(&mut self, hreserved: usize, crows: usize, rghrows: *const usize, rgpendingstatus: *mut u32) -> ::windows::core::Result<()>;
    fn Undo(&mut self, hreserved: usize, crows: usize, rghrows: *const usize, pcrowsundone: *mut usize, prgrowsundone: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::Result<()>;
    fn Update(&mut self, hreserved: usize, crows: usize, rghrows: *const usize, pcrows: *mut usize, prgrows: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::Result<()>;
}
impl IRowsetUpdate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetUpdate_Impl, const OFFSET: isize>() -> IRowsetUpdate_Vtbl {
        unsafe extern "system" fn GetOriginalData<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOriginalData(::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetPendingRows<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, dwrowstatus: u32, pcpendingrows: *mut usize, prgpendingrows: *mut *mut usize, prgpendingstatus: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPendingRows(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&dwrowstatus), ::core::mem::transmute_copy(&pcpendingrows), ::core::mem::transmute_copy(&prgpendingrows), ::core::mem::transmute_copy(&prgpendingstatus)).into()
        }
        unsafe extern "system" fn GetRowStatus<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, rgpendingstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRowStatus(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&rgpendingstatus)).into()
        }
        unsafe extern "system" fn Undo<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, pcrowsundone: *mut usize, prgrowsundone: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Undo(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&pcrowsundone), ::core::mem::transmute_copy(&prgrowsundone), ::core::mem::transmute_copy(&prgrowstatus)).into()
        }
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, pcrows: *mut usize, prgrows: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Update(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&pcrows), ::core::mem::transmute_copy(&prgrows), ::core::mem::transmute_copy(&prgrowstatus)).into()
        }
        Self {
            base: IRowsetChange_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOriginalData: GetOriginalData::<Identity, Impl, OFFSET>,
            GetPendingRows: GetPendingRows::<Identity, Impl, OFFSET>,
            GetRowStatus: GetRowStatus::<Identity, Impl, OFFSET>,
            Undo: Undo::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetUpdate as ::windows::core::Interface>::IID || iid == &<IRowsetChange as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetView_Impl: Sized {
    fn CreateView(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetView(&mut self, hchapter: usize, riid: *const ::windows::core::GUID, phchaptersource: *mut usize, ppview: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IRowsetView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetView_Impl, const OFFSET: isize>() -> IRowsetView_Vtbl {
        unsafe extern "system" fn CreateView<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateView(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, riid: *const ::windows::core::GUID, phchaptersource: *mut usize, ppview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetView(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&phchaptersource), ::core::mem::transmute_copy(&ppview)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateView: CreateView::<Identity, Impl, OFFSET>,
            GetView: GetView::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetView as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetWatchAll_Impl: Sized {
    fn Acknowledge(&mut self) -> ::windows::core::Result<()>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn StopWatching(&mut self) -> ::windows::core::Result<()>;
}
impl IRowsetWatchAll_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetWatchAll_Impl, const OFFSET: isize>() -> IRowsetWatchAll_Vtbl {
        unsafe extern "system" fn Acknowledge<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetWatchAll_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Acknowledge().into()
        }
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetWatchAll_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn StopWatching<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetWatchAll_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StopWatching().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Acknowledge: Acknowledge::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            StopWatching: StopWatching::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetWatchAll as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetWatchNotify_Impl: Sized {
    fn OnChange(&mut self, prowset: &::core::option::Option<IRowset>, echangereason: u32) -> ::windows::core::Result<()>;
}
impl IRowsetWatchNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetWatchNotify_Impl, const OFFSET: isize>() -> IRowsetWatchNotify_Vtbl {
        unsafe extern "system" fn OnChange<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetWatchNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prowset: ::windows::core::RawPtr, echangereason: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnChange(::core::mem::transmute(&prowset), ::core::mem::transmute_copy(&echangereason)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnChange: OnChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetWatchNotify as ::windows::core::Interface>::IID
    }
}
pub trait IRowsetWatchRegion_Impl: Sized + IRowsetWatchAll_Impl {
    fn CreateWatchRegion(&mut self, dwwatchmode: u32, phregion: *mut usize) -> ::windows::core::Result<()>;
    fn ChangeWatchMode(&mut self, hregion: usize, dwwatchmode: u32) -> ::windows::core::Result<()>;
    fn DeleteWatchRegion(&mut self, hregion: usize) -> ::windows::core::Result<()>;
    fn GetWatchRegionInfo(&mut self, hregion: usize, pdwwatchmode: *mut u32, phchapter: *mut usize, pcbbookmark: *mut usize, ppbookmark: *mut *mut u8, pcrows: *mut isize) -> ::windows::core::Result<()>;
    fn Refresh(&mut self, pcchangesobtained: *mut usize, prgchanges: *mut *mut tagDBROWWATCHRANGE) -> ::windows::core::Result<()>;
    fn ShrinkWatchRegion(&mut self, hregion: usize, hchapter: usize, cbbookmark: usize, pbookmark: *mut u8, crows: isize) -> ::windows::core::Result<()>;
}
impl IRowsetWatchRegion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetWatchRegion_Impl, const OFFSET: isize>() -> IRowsetWatchRegion_Vtbl {
        unsafe extern "system" fn CreateWatchRegion<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetWatchRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwwatchmode: u32, phregion: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateWatchRegion(::core::mem::transmute_copy(&dwwatchmode), ::core::mem::transmute_copy(&phregion)).into()
        }
        unsafe extern "system" fn ChangeWatchMode<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetWatchRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hregion: usize, dwwatchmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangeWatchMode(::core::mem::transmute_copy(&hregion), ::core::mem::transmute_copy(&dwwatchmode)).into()
        }
        unsafe extern "system" fn DeleteWatchRegion<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetWatchRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hregion: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteWatchRegion(::core::mem::transmute_copy(&hregion)).into()
        }
        unsafe extern "system" fn GetWatchRegionInfo<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetWatchRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hregion: usize, pdwwatchmode: *mut u32, phchapter: *mut usize, pcbbookmark: *mut usize, ppbookmark: *mut *mut u8, pcrows: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWatchRegionInfo(::core::mem::transmute_copy(&hregion), ::core::mem::transmute_copy(&pdwwatchmode), ::core::mem::transmute_copy(&phchapter), ::core::mem::transmute_copy(&pcbbookmark), ::core::mem::transmute_copy(&ppbookmark), ::core::mem::transmute_copy(&pcrows)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetWatchRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchangesobtained: *mut usize, prgchanges: *mut *mut tagDBROWWATCHRANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Refresh(::core::mem::transmute_copy(&pcchangesobtained), ::core::mem::transmute_copy(&prgchanges)).into()
        }
        unsafe extern "system" fn ShrinkWatchRegion<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetWatchRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hregion: usize, hchapter: usize, cbbookmark: usize, pbookmark: *mut u8, crows: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShrinkWatchRegion(::core::mem::transmute_copy(&hregion), ::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark), ::core::mem::transmute_copy(&crows)).into()
        }
        Self {
            base: IRowsetWatchAll_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateWatchRegion: CreateWatchRegion::<Identity, Impl, OFFSET>,
            ChangeWatchMode: ChangeWatchMode::<Identity, Impl, OFFSET>,
            DeleteWatchRegion: DeleteWatchRegion::<Identity, Impl, OFFSET>,
            GetWatchRegionInfo: GetWatchRegionInfo::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            ShrinkWatchRegion: ShrinkWatchRegion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetWatchRegion as ::windows::core::Interface>::IID || iid == &<IRowsetWatchAll as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IRowsetWithParameters_Impl: Sized {
    fn GetParameterInfo(&mut self, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
    fn Requery(&mut self, pparams: *mut DBPARAMS, pulerrorparam: *mut u32, phreserved: *mut usize) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRowsetWithParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetWithParameters_Impl, const OFFSET: isize>() -> IRowsetWithParameters_Vtbl {
        unsafe extern "system" fn GetParameterInfo<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetWithParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetParameterInfo(::core::mem::transmute_copy(&pcparams), ::core::mem::transmute_copy(&prgparaminfo), ::core::mem::transmute_copy(&ppnamesbuffer)).into()
        }
        unsafe extern "system" fn Requery<Identity: ::windows::core::IUnknownImpl, Impl: IRowsetWithParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparams: *mut DBPARAMS, pulerrorparam: *mut u32, phreserved: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Requery(::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&pulerrorparam), ::core::mem::transmute_copy(&phreserved)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetParameterInfo: GetParameterInfo::<Identity, Impl, OFFSET>,
            Requery: Requery::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetWithParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISQLErrorInfo_Impl: Sized {
    fn GetSQLInfo(&mut self, pbstrsqlstate: *mut super::super::Foundation::BSTR, plnativeerror: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISQLErrorInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISQLErrorInfo_Impl, const OFFSET: isize>() -> ISQLErrorInfo_Vtbl {
        unsafe extern "system" fn GetSQLInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISQLErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsqlstate: *mut super::super::Foundation::BSTR, plnativeerror: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSQLInfo(::core::mem::transmute_copy(&pbstrsqlstate), ::core::mem::transmute_copy(&plnativeerror)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetSQLInfo: GetSQLInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISQLErrorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISQLGetDiagField_Impl: Sized {
    fn GetDiagField(&mut self, pdiaginfo: *mut KAGGETDIAG) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISQLGetDiagField_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISQLGetDiagField_Impl, const OFFSET: isize>() -> ISQLGetDiagField_Vtbl {
        unsafe extern "system" fn GetDiagField<Identity: ::windows::core::IUnknownImpl, Impl: ISQLGetDiagField_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdiaginfo: *mut KAGGETDIAG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDiagField(::core::mem::transmute_copy(&pdiaginfo)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetDiagField: GetDiagField::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISQLGetDiagField as ::windows::core::Interface>::IID
    }
}
pub trait ISQLRequestDiagFields_Impl: Sized {
    fn RequestDiagFields(&mut self, cdiagfields: u32, rgdiagfields: *const KAGREQDIAG) -> ::windows::core::Result<()>;
}
impl ISQLRequestDiagFields_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISQLRequestDiagFields_Impl, const OFFSET: isize>() -> ISQLRequestDiagFields_Vtbl {
        unsafe extern "system" fn RequestDiagFields<Identity: ::windows::core::IUnknownImpl, Impl: ISQLRequestDiagFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdiagfields: u32, rgdiagfields: *const KAGREQDIAG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RequestDiagFields(::core::mem::transmute_copy(&cdiagfields), ::core::mem::transmute_copy(&rgdiagfields)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), RequestDiagFields: RequestDiagFields::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISQLRequestDiagFields as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISQLServerErrorInfo_Impl: Sized {
    fn GetErrorInfo(&mut self, pperrorinfo: *mut *mut tagSSErrorInfo, ppstringsbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISQLServerErrorInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISQLServerErrorInfo_Impl, const OFFSET: isize>() -> ISQLServerErrorInfo_Vtbl {
        unsafe extern "system" fn GetErrorInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISQLServerErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperrorinfo: *mut *mut tagSSErrorInfo, ppstringsbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetErrorInfo(::core::mem::transmute_copy(&pperrorinfo), ::core::mem::transmute_copy(&ppstringsbuffer)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetErrorInfo: GetErrorInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISQLServerErrorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISchemaLocalizerSupport_Impl: Sized {
    fn Localize(&mut self, pszglobalstring: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISchemaLocalizerSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaLocalizerSupport_Impl, const OFFSET: isize>() -> ISchemaLocalizerSupport_Vtbl {
        unsafe extern "system" fn Localize<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaLocalizerSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszglobalstring: super::super::Foundation::PWSTR, ppszlocalstring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Localize(::core::mem::transmute_copy(&pszglobalstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszlocalstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Localize: Localize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaLocalizerSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
pub trait ISchemaLock_Impl: Sized {
    fn GetSchemaLock(&mut self, ptableid: *mut super::super::Storage::IndexServer::DBID, lmmode: u32, phlockhandle: *mut super::super::Foundation::HANDLE, ptableversion: *mut u64) -> ::windows::core::Result<()>;
    fn ReleaseSchemaLock(&mut self, hlockhandle: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
impl ISchemaLock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaLock_Impl, const OFFSET: isize>() -> ISchemaLock_Vtbl {
        unsafe extern "system" fn GetSchemaLock<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, lmmode: u32, phlockhandle: *mut super::super::Foundation::HANDLE, ptableversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSchemaLock(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&lmmode), ::core::mem::transmute_copy(&phlockhandle), ::core::mem::transmute_copy(&ptableversion)).into()
        }
        unsafe extern "system" fn ReleaseSchemaLock<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hlockhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseSchemaLock(::core::mem::transmute_copy(&hlockhandle)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSchemaLock: GetSchemaLock::<Identity, Impl, OFFSET>,
            ReleaseSchemaLock: ReleaseSchemaLock::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaLock as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISchemaProvider_Impl: Sized {
    fn Entities(&mut self, riid: *const ::windows::core::GUID, pentities: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RootEntity(&mut self) -> ::windows::core::Result<IEntity>;
    fn GetEntity(&mut self, pszentityname: super::super::Foundation::PWSTR) -> ::windows::core::Result<IEntity>;
    fn MetaData(&mut self, riid: *const ::windows::core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Localize(&mut self, lcid: u32, pschemalocalizersupport: &::core::option::Option<ISchemaLocalizerSupport>) -> ::windows::core::Result<()>;
    fn SaveBinary(&mut self, pszschemabinarypath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn LookupAuthoredNamedEntity(&mut self, pentity: &::core::option::Option<IEntity>, pszinputstring: super::super::Foundation::PWSTR, ptokencollection: &::core::option::Option<ITokenCollection>, ctokensbegin: u32, pctokenslength: *mut u32, ppszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISchemaProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaProvider_Impl, const OFFSET: isize>() -> ISchemaProvider_Vtbl {
        unsafe extern "system" fn Entities<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pentities: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Entities(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pentities)).into()
        }
        unsafe extern "system" fn RootEntity<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prootentity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RootEntity() {
                ::core::result::Result::Ok(ok__) => {
                    *prootentity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntity<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszentityname: super::super::Foundation::PWSTR, pentity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEntity(::core::mem::transmute_copy(&pszentityname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pentity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MetaData<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MetaData(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pmetadata)).into()
        }
        unsafe extern "system" fn Localize<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32, pschemalocalizersupport: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Localize(::core::mem::transmute_copy(&lcid), ::core::mem::transmute(&pschemalocalizersupport)).into()
        }
        unsafe extern "system" fn SaveBinary<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszschemabinarypath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveBinary(::core::mem::transmute_copy(&pszschemabinarypath)).into()
        }
        unsafe extern "system" fn LookupAuthoredNamedEntity<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pentity: ::windows::core::RawPtr, pszinputstring: super::super::Foundation::PWSTR, ptokencollection: ::windows::core::RawPtr, ctokensbegin: u32, pctokenslength: *mut u32, ppszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LookupAuthoredNamedEntity(::core::mem::transmute(&pentity), ::core::mem::transmute_copy(&pszinputstring), ::core::mem::transmute(&ptokencollection), ::core::mem::transmute_copy(&ctokensbegin), ::core::mem::transmute_copy(&pctokenslength), ::core::mem::transmute_copy(&ppszvalue)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Entities: Entities::<Identity, Impl, OFFSET>,
            RootEntity: RootEntity::<Identity, Impl, OFFSET>,
            GetEntity: GetEntity::<Identity, Impl, OFFSET>,
            MetaData: MetaData::<Identity, Impl, OFFSET>,
            Localize: Localize::<Identity, Impl, OFFSET>,
            SaveBinary: SaveBinary::<Identity, Impl, OFFSET>,
            LookupAuthoredNamedEntity: LookupAuthoredNamedEntity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IScopedOperations_Impl: Sized + IBindResource_Impl {
    fn Copy(&mut self, crows: usize, rgpwszsourceurls: *const super::super::Foundation::PWSTR, rgpwszdesturls: *const super::super::Foundation::PWSTR, dwcopyflags: u32, pauthenticate: &::core::option::Option<super::Com::IAuthenticate>, rgdwstatus: *mut u32, rgpwsznewurls: *mut super::super::Foundation::PWSTR, ppstringsbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
    fn Move(&mut self, crows: usize, rgpwszsourceurls: *const super::super::Foundation::PWSTR, rgpwszdesturls: *const super::super::Foundation::PWSTR, dwmoveflags: u32, pauthenticate: &::core::option::Option<super::Com::IAuthenticate>, rgdwstatus: *mut u32, rgpwsznewurls: *mut super::super::Foundation::PWSTR, ppstringsbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
    fn Delete(&mut self, crows: usize, rgpwszurls: *const super::super::Foundation::PWSTR, dwdeleteflags: u32, rgdwstatus: *mut u32) -> ::windows::core::Result<()>;
    fn OpenRowset(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IScopedOperations_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScopedOperations_Impl, const OFFSET: isize>() -> IScopedOperations_Vtbl {
        unsafe extern "system" fn Copy<Identity: ::windows::core::IUnknownImpl, Impl: IScopedOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crows: usize, rgpwszsourceurls: *const super::super::Foundation::PWSTR, rgpwszdesturls: *const super::super::Foundation::PWSTR, dwcopyflags: u32, pauthenticate: ::windows::core::RawPtr, rgdwstatus: *mut u32, rgpwsznewurls: *mut super::super::Foundation::PWSTR, ppstringsbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Copy(::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rgpwszsourceurls), ::core::mem::transmute_copy(&rgpwszdesturls), ::core::mem::transmute_copy(&dwcopyflags), ::core::mem::transmute(&pauthenticate), ::core::mem::transmute_copy(&rgdwstatus), ::core::mem::transmute_copy(&rgpwsznewurls), ::core::mem::transmute_copy(&ppstringsbuffer)).into()
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl, Impl: IScopedOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crows: usize, rgpwszsourceurls: *const super::super::Foundation::PWSTR, rgpwszdesturls: *const super::super::Foundation::PWSTR, dwmoveflags: u32, pauthenticate: ::windows::core::RawPtr, rgdwstatus: *mut u32, rgpwsznewurls: *mut super::super::Foundation::PWSTR, ppstringsbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rgpwszsourceurls), ::core::mem::transmute_copy(&rgpwszdesturls), ::core::mem::transmute_copy(&dwmoveflags), ::core::mem::transmute(&pauthenticate), ::core::mem::transmute_copy(&rgdwstatus), ::core::mem::transmute_copy(&rgpwsznewurls), ::core::mem::transmute_copy(&ppstringsbuffer)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IScopedOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crows: usize, rgpwszurls: *const super::super::Foundation::PWSTR, dwdeleteflags: u32, rgdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rgpwszurls), ::core::mem::transmute_copy(&dwdeleteflags), ::core::mem::transmute_copy(&rgdwstatus)).into()
        }
        unsafe extern "system" fn OpenRowset<Identity: ::windows::core::IUnknownImpl, Impl: IScopedOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenRowset(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pindexid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pprowset)).into()
        }
        Self {
            base: IBindResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            Copy: Copy::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            OpenRowset: OpenRowset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScopedOperations as ::windows::core::Interface>::IID || iid == &<IBindResource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISearchCatalogManager_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetParameter(&mut self, pszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<*mut super::Com::StructuredStorage::PROPVARIANT>;
    fn SetParameter(&mut self, pszname: super::super::Foundation::PWSTR, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetCatalogStatus(&mut self, pstatus: *mut CatalogStatus, ppausedreason: *mut CatalogPausedReason) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Reindex(&mut self) -> ::windows::core::Result<()>;
    fn ReindexMatchingURLs(&mut self, pszpattern: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ReindexSearchRoot(&mut self, pszrooturl: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetConnectTimeout(&mut self, dwconnecttimeout: u32) -> ::windows::core::Result<()>;
    fn ConnectTimeout(&mut self) -> ::windows::core::Result<u32>;
    fn SetDataTimeout(&mut self, dwdatatimeout: u32) -> ::windows::core::Result<()>;
    fn DataTimeout(&mut self) -> ::windows::core::Result<u32>;
    fn NumberOfItems(&mut self) -> ::windows::core::Result<i32>;
    fn NumberOfItemsToIndex(&mut self, plincrementalcount: *mut i32, plnotificationqueue: *mut i32, plhighpriorityqueue: *mut i32) -> ::windows::core::Result<()>;
    fn URLBeingIndexed(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetURLIndexingState(&mut self, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<u32>;
    fn GetPersistentItemsChangedSink(&mut self) -> ::windows::core::Result<ISearchPersistentItemsChangedSink>;
    fn RegisterViewForNotification(&mut self, pszview: super::super::Foundation::PWSTR, pviewchangedsink: &::core::option::Option<ISearchViewChangedSink>) -> ::windows::core::Result<u32>;
    fn GetItemsChangedSink(&mut self, pisearchnotifyinlinesite: &::core::option::Option<ISearchNotifyInlineSite>, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void, pguidcatalogresetsignature: *mut ::windows::core::GUID, pguidcheckpointsignature: *mut ::windows::core::GUID, pdwlastcheckpointnumber: *mut u32) -> ::windows::core::Result<()>;
    fn UnregisterViewForNotification(&mut self, dwcookie: u32) -> ::windows::core::Result<()>;
    fn SetExtensionClusion(&mut self, pszextension: super::super::Foundation::PWSTR, fexclude: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn EnumerateExcludedExtensions(&mut self) -> ::windows::core::Result<super::Com::IEnumString>;
    fn GetQueryHelper(&mut self) -> ::windows::core::Result<ISearchQueryHelper>;
    fn SetDiacriticSensitivity(&mut self, fdiacriticsensitive: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DiacriticSensitivity(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetCrawlScopeManager(&mut self) -> ::windows::core::Result<ISearchCrawlScopeManager>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ISearchCatalogManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>() -> ISearchCatalogManager_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParameter<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ppvalue: *mut *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetParameter(::core::mem::transmute_copy(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameter<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetParameter(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetCatalogStatus<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut CatalogStatus, ppausedreason: *mut CatalogPausedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCatalogStatus(::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppausedreason)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Reindex<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reindex().into()
        }
        unsafe extern "system" fn ReindexMatchingURLs<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpattern: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReindexMatchingURLs(::core::mem::transmute_copy(&pszpattern)).into()
        }
        unsafe extern "system" fn ReindexSearchRoot<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrooturl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReindexSearchRoot(::core::mem::transmute_copy(&pszrooturl)).into()
        }
        unsafe extern "system" fn SetConnectTimeout<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnecttimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConnectTimeout(::core::mem::transmute_copy(&dwconnecttimeout)).into()
        }
        unsafe extern "system" fn ConnectTimeout<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwconnecttimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConnectTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwconnecttimeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataTimeout<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdatatimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDataTimeout(::core::mem::transmute_copy(&dwdatatimeout)).into()
        }
        unsafe extern "system" fn DataTimeout<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdatatimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DataTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwdatatimeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfItems<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumberOfItems() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfItemsToIndex<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plincrementalcount: *mut i32, plnotificationqueue: *mut i32, plhighpriorityqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NumberOfItemsToIndex(::core::mem::transmute_copy(&plincrementalcount), ::core::mem::transmute_copy(&plnotificationqueue), ::core::mem::transmute_copy(&plhighpriorityqueue)).into()
        }
        unsafe extern "system" fn URLBeingIndexed<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).URLBeingIndexed() {
                ::core::result::Result::Ok(ok__) => {
                    *pszurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetURLIndexingState<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetURLIndexingState(::core::mem::transmute_copy(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPersistentItemsChangedSink<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppisearchpersistentitemschangedsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPersistentItemsChangedSink() {
                ::core::result::Result::Ok(ok__) => {
                    *ppisearchpersistentitemschangedsink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterViewForNotification<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszview: super::super::Foundation::PWSTR, pviewchangedsink: ::windows::core::RawPtr, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegisterViewForNotification(::core::mem::transmute_copy(&pszview), ::core::mem::transmute(&pviewchangedsink)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsChangedSink<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisearchnotifyinlinesite: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void, pguidcatalogresetsignature: *mut ::windows::core::GUID, pguidcheckpointsignature: *mut ::windows::core::GUID, pdwlastcheckpointnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetItemsChangedSink(::core::mem::transmute(&pisearchnotifyinlinesite), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv), ::core::mem::transmute_copy(&pguidcatalogresetsignature), ::core::mem::transmute_copy(&pguidcheckpointsignature), ::core::mem::transmute_copy(&pdwlastcheckpointnumber)).into()
        }
        unsafe extern "system" fn UnregisterViewForNotification<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterViewForNotification(::core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn SetExtensionClusion<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszextension: super::super::Foundation::PWSTR, fexclude: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExtensionClusion(::core::mem::transmute_copy(&pszextension), ::core::mem::transmute_copy(&fexclude)).into()
        }
        unsafe extern "system" fn EnumerateExcludedExtensions<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextensions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateExcludedExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppextensions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQueryHelper<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsearchqueryhelper: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetQueryHelper() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsearchqueryhelper = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiacriticSensitivity<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdiacriticsensitive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDiacriticSensitivity(::core::mem::transmute_copy(&fdiacriticsensitive)).into()
        }
        unsafe extern "system" fn DiacriticSensitivity<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdiacriticsensitive: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DiacriticSensitivity() {
                ::core::result::Result::Ok(ok__) => {
                    *pfdiacriticsensitive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCrawlScopeManager<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcrawlscopemanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCrawlScopeManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcrawlscopemanager = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            GetParameter: GetParameter::<Identity, Impl, OFFSET>,
            SetParameter: SetParameter::<Identity, Impl, OFFSET>,
            GetCatalogStatus: GetCatalogStatus::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Reindex: Reindex::<Identity, Impl, OFFSET>,
            ReindexMatchingURLs: ReindexMatchingURLs::<Identity, Impl, OFFSET>,
            ReindexSearchRoot: ReindexSearchRoot::<Identity, Impl, OFFSET>,
            SetConnectTimeout: SetConnectTimeout::<Identity, Impl, OFFSET>,
            ConnectTimeout: ConnectTimeout::<Identity, Impl, OFFSET>,
            SetDataTimeout: SetDataTimeout::<Identity, Impl, OFFSET>,
            DataTimeout: DataTimeout::<Identity, Impl, OFFSET>,
            NumberOfItems: NumberOfItems::<Identity, Impl, OFFSET>,
            NumberOfItemsToIndex: NumberOfItemsToIndex::<Identity, Impl, OFFSET>,
            URLBeingIndexed: URLBeingIndexed::<Identity, Impl, OFFSET>,
            GetURLIndexingState: GetURLIndexingState::<Identity, Impl, OFFSET>,
            GetPersistentItemsChangedSink: GetPersistentItemsChangedSink::<Identity, Impl, OFFSET>,
            RegisterViewForNotification: RegisterViewForNotification::<Identity, Impl, OFFSET>,
            GetItemsChangedSink: GetItemsChangedSink::<Identity, Impl, OFFSET>,
            UnregisterViewForNotification: UnregisterViewForNotification::<Identity, Impl, OFFSET>,
            SetExtensionClusion: SetExtensionClusion::<Identity, Impl, OFFSET>,
            EnumerateExcludedExtensions: EnumerateExcludedExtensions::<Identity, Impl, OFFSET>,
            GetQueryHelper: GetQueryHelper::<Identity, Impl, OFFSET>,
            SetDiacriticSensitivity: SetDiacriticSensitivity::<Identity, Impl, OFFSET>,
            DiacriticSensitivity: DiacriticSensitivity::<Identity, Impl, OFFSET>,
            GetCrawlScopeManager: GetCrawlScopeManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchCatalogManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISearchCatalogManager2_Impl: Sized + ISearchCatalogManager_Impl {
    fn PrioritizeMatchingURLs(&mut self, pszpattern: super::super::Foundation::PWSTR, dwprioritizeflags: PRIORITIZE_FLAGS) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ISearchCatalogManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager2_Impl, const OFFSET: isize>() -> ISearchCatalogManager2_Vtbl {
        unsafe extern "system" fn PrioritizeMatchingURLs<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCatalogManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpattern: super::super::Foundation::PWSTR, dwprioritizeflags: PRIORITIZE_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PrioritizeMatchingURLs(::core::mem::transmute_copy(&pszpattern), ::core::mem::transmute_copy(&dwprioritizeflags)).into()
        }
        Self { base: ISearchCatalogManager_Vtbl::new::<Identity, Impl, OFFSET>(), PrioritizeMatchingURLs: PrioritizeMatchingURLs::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchCatalogManager2 as ::windows::core::Interface>::IID || iid == &<ISearchCatalogManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchCrawlScopeManager_Impl: Sized {
    fn AddDefaultScopeRule(&mut self, pszurl: super::super::Foundation::PWSTR, finclude: super::super::Foundation::BOOL, ffollowflags: u32) -> ::windows::core::Result<()>;
    fn AddRoot(&mut self, psearchroot: &::core::option::Option<ISearchRoot>) -> ::windows::core::Result<()>;
    fn RemoveRoot(&mut self, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn EnumerateRoots(&mut self) -> ::windows::core::Result<IEnumSearchRoots>;
    fn AddHierarchicalScope(&mut self, pszurl: super::super::Foundation::PWSTR, finclude: super::super::Foundation::BOOL, fdefault: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn AddUserScopeRule(&mut self, pszurl: super::super::Foundation::PWSTR, finclude: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL, ffollowflags: u32) -> ::windows::core::Result<()>;
    fn RemoveScopeRule(&mut self, pszrule: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn EnumerateScopeRules(&mut self) -> ::windows::core::Result<IEnumSearchScopeRules>;
    fn HasParentScopeRule(&mut self, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn HasChildScopeRule(&mut self, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IncludedInCrawlScope(&mut self, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IncludedInCrawlScopeEx(&mut self, pszurl: super::super::Foundation::PWSTR, pfisincluded: *mut super::super::Foundation::BOOL, preason: *mut CLUSION_REASON) -> ::windows::core::Result<()>;
    fn RevertToDefaultScopes(&mut self) -> ::windows::core::Result<()>;
    fn SaveAll(&mut self) -> ::windows::core::Result<()>;
    fn GetParentScopeVersionId(&mut self, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<i32>;
    fn RemoveDefaultScopeRule(&mut self, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISearchCrawlScopeManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>() -> ISearchCrawlScopeManager_Vtbl {
        unsafe extern "system" fn AddDefaultScopeRule<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, finclude: super::super::Foundation::BOOL, ffollowflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDefaultScopeRule(::core::mem::transmute_copy(&pszurl), ::core::mem::transmute_copy(&finclude), ::core::mem::transmute_copy(&ffollowflags)).into()
        }
        unsafe extern "system" fn AddRoot<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psearchroot: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRoot(::core::mem::transmute(&psearchroot)).into()
        }
        unsafe extern "system" fn RemoveRoot<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveRoot(::core::mem::transmute_copy(&pszurl)).into()
        }
        unsafe extern "system" fn EnumerateRoots<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsearchroots: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateRoots() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsearchroots = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddHierarchicalScope<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, finclude: super::super::Foundation::BOOL, fdefault: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddHierarchicalScope(::core::mem::transmute_copy(&pszurl), ::core::mem::transmute_copy(&finclude), ::core::mem::transmute_copy(&fdefault), ::core::mem::transmute_copy(&foverridechildren)).into()
        }
        unsafe extern "system" fn AddUserScopeRule<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, finclude: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL, ffollowflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddUserScopeRule(::core::mem::transmute_copy(&pszurl), ::core::mem::transmute_copy(&finclude), ::core::mem::transmute_copy(&foverridechildren), ::core::mem::transmute_copy(&ffollowflags)).into()
        }
        unsafe extern "system" fn RemoveScopeRule<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrule: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveScopeRule(::core::mem::transmute_copy(&pszrule)).into()
        }
        unsafe extern "system" fn EnumerateScopeRules<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsearchscoperules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateScopeRules() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsearchscoperules = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasParentScopeRule<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pfhasparentrule: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HasParentScopeRule(::core::mem::transmute_copy(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfhasparentrule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasChildScopeRule<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pfhaschildrule: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HasChildScopeRule(::core::mem::transmute_copy(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfhaschildrule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludedInCrawlScope<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pfisincluded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IncludedInCrawlScope(::core::mem::transmute_copy(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfisincluded = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludedInCrawlScopeEx<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pfisincluded: *mut super::super::Foundation::BOOL, preason: *mut CLUSION_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IncludedInCrawlScopeEx(::core::mem::transmute_copy(&pszurl), ::core::mem::transmute_copy(&pfisincluded), ::core::mem::transmute_copy(&preason)).into()
        }
        unsafe extern "system" fn RevertToDefaultScopes<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RevertToDefaultScopes().into()
        }
        unsafe extern "system" fn SaveAll<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveAll().into()
        }
        unsafe extern "system" fn GetParentScopeVersionId<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, plscopeid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetParentScopeVersionId(::core::mem::transmute_copy(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *plscopeid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDefaultScopeRule<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveDefaultScopeRule(::core::mem::transmute_copy(&pszurl)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddDefaultScopeRule: AddDefaultScopeRule::<Identity, Impl, OFFSET>,
            AddRoot: AddRoot::<Identity, Impl, OFFSET>,
            RemoveRoot: RemoveRoot::<Identity, Impl, OFFSET>,
            EnumerateRoots: EnumerateRoots::<Identity, Impl, OFFSET>,
            AddHierarchicalScope: AddHierarchicalScope::<Identity, Impl, OFFSET>,
            AddUserScopeRule: AddUserScopeRule::<Identity, Impl, OFFSET>,
            RemoveScopeRule: RemoveScopeRule::<Identity, Impl, OFFSET>,
            EnumerateScopeRules: EnumerateScopeRules::<Identity, Impl, OFFSET>,
            HasParentScopeRule: HasParentScopeRule::<Identity, Impl, OFFSET>,
            HasChildScopeRule: HasChildScopeRule::<Identity, Impl, OFFSET>,
            IncludedInCrawlScope: IncludedInCrawlScope::<Identity, Impl, OFFSET>,
            IncludedInCrawlScopeEx: IncludedInCrawlScopeEx::<Identity, Impl, OFFSET>,
            RevertToDefaultScopes: RevertToDefaultScopes::<Identity, Impl, OFFSET>,
            SaveAll: SaveAll::<Identity, Impl, OFFSET>,
            GetParentScopeVersionId: GetParentScopeVersionId::<Identity, Impl, OFFSET>,
            RemoveDefaultScopeRule: RemoveDefaultScopeRule::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchCrawlScopeManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchCrawlScopeManager2_Impl: Sized + ISearchCrawlScopeManager_Impl {
    fn GetVersion(&mut self, plversion: *mut *mut i32, phfilemapping: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISearchCrawlScopeManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager2_Impl, const OFFSET: isize>() -> ISearchCrawlScopeManager2_Vtbl {
        unsafe extern "system" fn GetVersion<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCrawlScopeManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plversion: *mut *mut i32, phfilemapping: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVersion(::core::mem::transmute_copy(&plversion), ::core::mem::transmute_copy(&phfilemapping)).into()
        }
        Self { base: ISearchCrawlScopeManager_Vtbl::new::<Identity, Impl, OFFSET>(), GetVersion: GetVersion::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchCrawlScopeManager2 as ::windows::core::Interface>::IID || iid == &<ISearchCrawlScopeManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISearchItemsChangedSink_Impl: Sized {
    fn StartedMonitoringScope(&mut self, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn StoppedMonitoringScope(&mut self, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn OnItemsChanged(&mut self, dwnumberofchanges: u32, rgdatachangeentries: *const SEARCH_ITEM_CHANGE, rgdwdocids: *mut u32, rghrcompletioncodes: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISearchItemsChangedSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchItemsChangedSink_Impl, const OFFSET: isize>() -> ISearchItemsChangedSink_Vtbl {
        unsafe extern "system" fn StartedMonitoringScope<Identity: ::windows::core::IUnknownImpl, Impl: ISearchItemsChangedSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartedMonitoringScope(::core::mem::transmute_copy(&pszurl)).into()
        }
        unsafe extern "system" fn StoppedMonitoringScope<Identity: ::windows::core::IUnknownImpl, Impl: ISearchItemsChangedSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StoppedMonitoringScope(::core::mem::transmute_copy(&pszurl)).into()
        }
        unsafe extern "system" fn OnItemsChanged<Identity: ::windows::core::IUnknownImpl, Impl: ISearchItemsChangedSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnumberofchanges: u32, rgdatachangeentries: *const SEARCH_ITEM_CHANGE, rgdwdocids: *mut u32, rghrcompletioncodes: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnItemsChanged(::core::mem::transmute_copy(&dwnumberofchanges), ::core::mem::transmute_copy(&rgdatachangeentries), ::core::mem::transmute_copy(&rgdwdocids), ::core::mem::transmute_copy(&rghrcompletioncodes)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            StartedMonitoringScope: StartedMonitoringScope::<Identity, Impl, OFFSET>,
            StoppedMonitoringScope: StoppedMonitoringScope::<Identity, Impl, OFFSET>,
            OnItemsChanged: OnItemsChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchItemsChangedSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchLanguageSupport_Impl: Sized {
    fn SetDiacriticSensitivity(&mut self, fdiacriticsensitive: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetDiacriticSensitivity(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn LoadWordBreaker(&mut self, lcid: u32, riid: *const ::windows::core::GUID, ppwordbreaker: *mut *mut ::core::ffi::c_void, plcidused: *mut u32) -> ::windows::core::Result<()>;
    fn LoadStemmer(&mut self, lcid: u32, riid: *const ::windows::core::GUID, ppstemmer: *mut *mut ::core::ffi::c_void, plcidused: *mut u32) -> ::windows::core::Result<()>;
    fn IsPrefixNormalized(&mut self, pwcsquerytoken: super::super::Foundation::PWSTR, cwcquerytoken: u32, pwcsdocumenttoken: super::super::Foundation::PWSTR, cwcdocumenttoken: u32) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISearchLanguageSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchLanguageSupport_Impl, const OFFSET: isize>() -> ISearchLanguageSupport_Vtbl {
        unsafe extern "system" fn SetDiacriticSensitivity<Identity: ::windows::core::IUnknownImpl, Impl: ISearchLanguageSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdiacriticsensitive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDiacriticSensitivity(::core::mem::transmute_copy(&fdiacriticsensitive)).into()
        }
        unsafe extern "system" fn GetDiacriticSensitivity<Identity: ::windows::core::IUnknownImpl, Impl: ISearchLanguageSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdiacriticsensitive: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDiacriticSensitivity() {
                ::core::result::Result::Ok(ok__) => {
                    *pfdiacriticsensitive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadWordBreaker<Identity: ::windows::core::IUnknownImpl, Impl: ISearchLanguageSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32, riid: *const ::windows::core::GUID, ppwordbreaker: *mut *mut ::core::ffi::c_void, plcidused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadWordBreaker(::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppwordbreaker), ::core::mem::transmute_copy(&plcidused)).into()
        }
        unsafe extern "system" fn LoadStemmer<Identity: ::windows::core::IUnknownImpl, Impl: ISearchLanguageSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32, riid: *const ::windows::core::GUID, ppstemmer: *mut *mut ::core::ffi::c_void, plcidused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadStemmer(::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppstemmer), ::core::mem::transmute_copy(&plcidused)).into()
        }
        unsafe extern "system" fn IsPrefixNormalized<Identity: ::windows::core::IUnknownImpl, Impl: ISearchLanguageSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsquerytoken: super::super::Foundation::PWSTR, cwcquerytoken: u32, pwcsdocumenttoken: super::super::Foundation::PWSTR, cwcdocumenttoken: u32, pulprefixlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsPrefixNormalized(::core::mem::transmute_copy(&pwcsquerytoken), ::core::mem::transmute_copy(&cwcquerytoken), ::core::mem::transmute_copy(&pwcsdocumenttoken), ::core::mem::transmute_copy(&cwcdocumenttoken)) {
                ::core::result::Result::Ok(ok__) => {
                    *pulprefixlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetDiacriticSensitivity: SetDiacriticSensitivity::<Identity, Impl, OFFSET>,
            GetDiacriticSensitivity: GetDiacriticSensitivity::<Identity, Impl, OFFSET>,
            LoadWordBreaker: LoadWordBreaker::<Identity, Impl, OFFSET>,
            LoadStemmer: LoadStemmer::<Identity, Impl, OFFSET>,
            IsPrefixNormalized: IsPrefixNormalized::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchLanguageSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISearchManager_Impl: Sized {
    fn GetIndexerVersionStr(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetIndexerVersion(&mut self, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows::core::Result<()>;
    fn GetParameter(&mut self, pszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<*mut super::Com::StructuredStorage::PROPVARIANT>;
    fn SetParameter(&mut self, pszname: super::super::Foundation::PWSTR, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn ProxyName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn BypassList(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetProxy(&mut self, suseproxy: PROXY_ACCESS, flocalbypassproxy: super::super::Foundation::BOOL, dwportnumber: u32, pszproxyname: super::super::Foundation::PWSTR, pszbypasslist: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetCatalog(&mut self, pszcatalog: super::super::Foundation::PWSTR) -> ::windows::core::Result<ISearchCatalogManager>;
    fn UserAgent(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetUserAgent(&mut self, pszuseragent: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn UseProxy(&mut self) -> ::windows::core::Result<PROXY_ACCESS>;
    fn LocalBypass(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn PortNumber(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ISearchManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchManager_Impl, const OFFSET: isize>() -> ISearchManager_Vtbl {
        unsafe extern "system" fn GetIndexerVersionStr<Identity: ::windows::core::IUnknownImpl, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszversionstring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIndexerVersionStr() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszversionstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndexerVersion<Identity: ::windows::core::IUnknownImpl, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIndexerVersion(::core::mem::transmute_copy(&pdwmajor), ::core::mem::transmute_copy(&pdwminor)).into()
        }
        unsafe extern "system" fn GetParameter<Identity: ::windows::core::IUnknownImpl, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ppvalue: *mut *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetParameter(::core::mem::transmute_copy(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameter<Identity: ::windows::core::IUnknownImpl, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetParameter(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn ProxyName<Identity: ::windows::core::IUnknownImpl, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszproxyname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProxyName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszproxyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BypassList<Identity: ::windows::core::IUnknownImpl, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszbypasslist: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BypassList() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszbypasslist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxy<Identity: ::windows::core::IUnknownImpl, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, suseproxy: PROXY_ACCESS, flocalbypassproxy: super::super::Foundation::BOOL, dwportnumber: u32, pszproxyname: super::super::Foundation::PWSTR, pszbypasslist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProxy(::core::mem::transmute_copy(&suseproxy), ::core::mem::transmute_copy(&flocalbypassproxy), ::core::mem::transmute_copy(&dwportnumber), ::core::mem::transmute_copy(&pszproxyname), ::core::mem::transmute_copy(&pszbypasslist)).into()
        }
        unsafe extern "system" fn GetCatalog<Identity: ::windows::core::IUnknownImpl, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcatalog: super::super::Foundation::PWSTR, ppcatalogmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCatalog(::core::mem::transmute_copy(&pszcatalog)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcatalogmanager = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserAgent<Identity: ::windows::core::IUnknownImpl, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszuseragent: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserAgent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszuseragent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserAgent<Identity: ::windows::core::IUnknownImpl, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszuseragent: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUserAgent(::core::mem::transmute_copy(&pszuseragent)).into()
        }
        unsafe extern "system" fn UseProxy<Identity: ::windows::core::IUnknownImpl, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puseproxy: *mut PROXY_ACCESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UseProxy() {
                ::core::result::Result::Ok(ok__) => {
                    *puseproxy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalBypass<Identity: ::windows::core::IUnknownImpl, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflocalbypass: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalBypass() {
                ::core::result::Result::Ok(ok__) => {
                    *pflocalbypass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PortNumber<Identity: ::windows::core::IUnknownImpl, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwportnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PortNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwportnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetIndexerVersionStr: GetIndexerVersionStr::<Identity, Impl, OFFSET>,
            GetIndexerVersion: GetIndexerVersion::<Identity, Impl, OFFSET>,
            GetParameter: GetParameter::<Identity, Impl, OFFSET>,
            SetParameter: SetParameter::<Identity, Impl, OFFSET>,
            ProxyName: ProxyName::<Identity, Impl, OFFSET>,
            BypassList: BypassList::<Identity, Impl, OFFSET>,
            SetProxy: SetProxy::<Identity, Impl, OFFSET>,
            GetCatalog: GetCatalog::<Identity, Impl, OFFSET>,
            UserAgent: UserAgent::<Identity, Impl, OFFSET>,
            SetUserAgent: SetUserAgent::<Identity, Impl, OFFSET>,
            UseProxy: UseProxy::<Identity, Impl, OFFSET>,
            LocalBypass: LocalBypass::<Identity, Impl, OFFSET>,
            PortNumber: PortNumber::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISearchManager2_Impl: Sized + ISearchManager_Impl {
    fn CreateCatalog(&mut self, pszcatalog: super::super::Foundation::PWSTR) -> ::windows::core::Result<ISearchCatalogManager>;
    fn DeleteCatalog(&mut self, pszcatalog: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ISearchManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchManager2_Impl, const OFFSET: isize>() -> ISearchManager2_Vtbl {
        unsafe extern "system" fn CreateCatalog<Identity: ::windows::core::IUnknownImpl, Impl: ISearchManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcatalog: super::super::Foundation::PWSTR, ppcatalogmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateCatalog(::core::mem::transmute_copy(&pszcatalog)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcatalogmanager = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteCatalog<Identity: ::windows::core::IUnknownImpl, Impl: ISearchManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcatalog: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteCatalog(::core::mem::transmute_copy(&pszcatalog)).into()
        }
        Self {
            base: ISearchManager_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateCatalog: CreateCatalog::<Identity, Impl, OFFSET>,
            DeleteCatalog: DeleteCatalog::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchManager2 as ::windows::core::Interface>::IID || iid == &<ISearchManager as ::windows::core::Interface>::IID
    }
}
pub trait ISearchNotifyInlineSite_Impl: Sized {
    fn OnItemIndexedStatusChange(&mut self, sipstatus: SEARCH_INDEXING_PHASE, dwnumentries: u32, rgitemstatusentries: *const SEARCH_ITEM_INDEXING_STATUS) -> ::windows::core::Result<()>;
    fn OnCatalogStatusChange(&mut self, guidcatalogresetsignature: *const ::windows::core::GUID, guidcheckpointsignature: *const ::windows::core::GUID, dwlastcheckpointnumber: u32) -> ::windows::core::Result<()>;
}
impl ISearchNotifyInlineSite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchNotifyInlineSite_Impl, const OFFSET: isize>() -> ISearchNotifyInlineSite_Vtbl {
        unsafe extern "system" fn OnItemIndexedStatusChange<Identity: ::windows::core::IUnknownImpl, Impl: ISearchNotifyInlineSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sipstatus: SEARCH_INDEXING_PHASE, dwnumentries: u32, rgitemstatusentries: *const SEARCH_ITEM_INDEXING_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnItemIndexedStatusChange(::core::mem::transmute_copy(&sipstatus), ::core::mem::transmute_copy(&dwnumentries), ::core::mem::transmute_copy(&rgitemstatusentries)).into()
        }
        unsafe extern "system" fn OnCatalogStatusChange<Identity: ::windows::core::IUnknownImpl, Impl: ISearchNotifyInlineSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcatalogresetsignature: *const ::windows::core::GUID, guidcheckpointsignature: *const ::windows::core::GUID, dwlastcheckpointnumber: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnCatalogStatusChange(::core::mem::transmute_copy(&guidcatalogresetsignature), ::core::mem::transmute_copy(&guidcheckpointsignature), ::core::mem::transmute_copy(&dwlastcheckpointnumber)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnItemIndexedStatusChange: OnItemIndexedStatusChange::<Identity, Impl, OFFSET>,
            OnCatalogStatusChange: OnCatalogStatusChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchNotifyInlineSite as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchPersistentItemsChangedSink_Impl: Sized {
    fn StartedMonitoringScope(&mut self, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn StoppedMonitoringScope(&mut self, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn OnItemsChanged(&mut self, dwnumberofchanges: u32, datachangeentries: *const SEARCH_ITEM_PERSISTENT_CHANGE, hrcompletioncodes: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISearchPersistentItemsChangedSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchPersistentItemsChangedSink_Impl, const OFFSET: isize>() -> ISearchPersistentItemsChangedSink_Vtbl {
        unsafe extern "system" fn StartedMonitoringScope<Identity: ::windows::core::IUnknownImpl, Impl: ISearchPersistentItemsChangedSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartedMonitoringScope(::core::mem::transmute_copy(&pszurl)).into()
        }
        unsafe extern "system" fn StoppedMonitoringScope<Identity: ::windows::core::IUnknownImpl, Impl: ISearchPersistentItemsChangedSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StoppedMonitoringScope(::core::mem::transmute_copy(&pszurl)).into()
        }
        unsafe extern "system" fn OnItemsChanged<Identity: ::windows::core::IUnknownImpl, Impl: ISearchPersistentItemsChangedSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnumberofchanges: u32, datachangeentries: *const SEARCH_ITEM_PERSISTENT_CHANGE, hrcompletioncodes: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnItemsChanged(::core::mem::transmute_copy(&dwnumberofchanges), ::core::mem::transmute_copy(&datachangeentries), ::core::mem::transmute_copy(&hrcompletioncodes)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            StartedMonitoringScope: StartedMonitoringScope::<Identity, Impl, OFFSET>,
            StoppedMonitoringScope: StoppedMonitoringScope::<Identity, Impl, OFFSET>,
            OnItemsChanged: OnItemsChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchPersistentItemsChangedSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchProtocol_Impl: Sized {
    fn Init(&mut self, ptimeoutinfo: *mut TIMEOUT_INFO, pprotocolhandlersite: &::core::option::Option<IProtocolHandlerSite>, pproxyinfo: *mut PROXY_INFO) -> ::windows::core::Result<()>;
    fn CreateAccessor(&mut self, pcwszurl: super::super::Foundation::PWSTR, pauthenticationinfo: *mut AUTHENTICATION_INFO, pincrementalaccessinfo: *mut INCREMENTAL_ACCESS_INFO, piteminfo: *mut ITEM_INFO, ppaccessor: *mut ::core::option::Option<IUrlAccessor>) -> ::windows::core::Result<()>;
    fn CloseAccessor(&mut self, paccessor: &::core::option::Option<IUrlAccessor>) -> ::windows::core::Result<()>;
    fn ShutDown(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISearchProtocol_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchProtocol_Impl, const OFFSET: isize>() -> ISearchProtocol_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl, Impl: ISearchProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimeoutinfo: *mut TIMEOUT_INFO, pprotocolhandlersite: ::windows::core::RawPtr, pproxyinfo: *mut PROXY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&ptimeoutinfo), ::core::mem::transmute(&pprotocolhandlersite), ::core::mem::transmute_copy(&pproxyinfo)).into()
        }
        unsafe extern "system" fn CreateAccessor<Identity: ::windows::core::IUnknownImpl, Impl: ISearchProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcwszurl: super::super::Foundation::PWSTR, pauthenticationinfo: *mut AUTHENTICATION_INFO, pincrementalaccessinfo: *mut INCREMENTAL_ACCESS_INFO, piteminfo: *mut ITEM_INFO, ppaccessor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateAccessor(::core::mem::transmute_copy(&pcwszurl), ::core::mem::transmute_copy(&pauthenticationinfo), ::core::mem::transmute_copy(&pincrementalaccessinfo), ::core::mem::transmute_copy(&piteminfo), ::core::mem::transmute_copy(&ppaccessor)).into()
        }
        unsafe extern "system" fn CloseAccessor<Identity: ::windows::core::IUnknownImpl, Impl: ISearchProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paccessor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CloseAccessor(::core::mem::transmute(&paccessor)).into()
        }
        unsafe extern "system" fn ShutDown<Identity: ::windows::core::IUnknownImpl, Impl: ISearchProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShutDown().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            CreateAccessor: CreateAccessor::<Identity, Impl, OFFSET>,
            CloseAccessor: CloseAccessor::<Identity, Impl, OFFSET>,
            ShutDown: ShutDown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchProtocol as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISearchProtocol2_Impl: Sized + ISearchProtocol_Impl {
    fn CreateAccessorEx(&mut self, pcwszurl: super::super::Foundation::PWSTR, pauthenticationinfo: *mut AUTHENTICATION_INFO, pincrementalaccessinfo: *mut INCREMENTAL_ACCESS_INFO, piteminfo: *mut ITEM_INFO, puserdata: *const super::Com::BLOB, ppaccessor: *mut ::core::option::Option<IUrlAccessor>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISearchProtocol2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchProtocol2_Impl, const OFFSET: isize>() -> ISearchProtocol2_Vtbl {
        unsafe extern "system" fn CreateAccessorEx<Identity: ::windows::core::IUnknownImpl, Impl: ISearchProtocol2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcwszurl: super::super::Foundation::PWSTR, pauthenticationinfo: *mut AUTHENTICATION_INFO, pincrementalaccessinfo: *mut INCREMENTAL_ACCESS_INFO, piteminfo: *mut ITEM_INFO, puserdata: *const super::Com::BLOB, ppaccessor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateAccessorEx(::core::mem::transmute_copy(&pcwszurl), ::core::mem::transmute_copy(&pauthenticationinfo), ::core::mem::transmute_copy(&pincrementalaccessinfo), ::core::mem::transmute_copy(&piteminfo), ::core::mem::transmute_copy(&puserdata), ::core::mem::transmute_copy(&ppaccessor)).into()
        }
        Self { base: ISearchProtocol_Vtbl::new::<Identity, Impl, OFFSET>(), CreateAccessorEx: CreateAccessorEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchProtocol2 as ::windows::core::Interface>::IID || iid == &<ISearchProtocol as ::windows::core::Interface>::IID
    }
}
pub trait ISearchProtocolThreadContext_Impl: Sized {
    fn ThreadInit(&mut self) -> ::windows::core::Result<()>;
    fn ThreadShutdown(&mut self) -> ::windows::core::Result<()>;
    fn ThreadIdle(&mut self, dwtimeelaspedsincelastcallinms: u32) -> ::windows::core::Result<()>;
}
impl ISearchProtocolThreadContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchProtocolThreadContext_Impl, const OFFSET: isize>() -> ISearchProtocolThreadContext_Vtbl {
        unsafe extern "system" fn ThreadInit<Identity: ::windows::core::IUnknownImpl, Impl: ISearchProtocolThreadContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ThreadInit().into()
        }
        unsafe extern "system" fn ThreadShutdown<Identity: ::windows::core::IUnknownImpl, Impl: ISearchProtocolThreadContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ThreadShutdown().into()
        }
        unsafe extern "system" fn ThreadIdle<Identity: ::windows::core::IUnknownImpl, Impl: ISearchProtocolThreadContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtimeelaspedsincelastcallinms: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ThreadIdle(::core::mem::transmute_copy(&dwtimeelaspedsincelastcallinms)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ThreadInit: ThreadInit::<Identity, Impl, OFFSET>,
            ThreadShutdown: ThreadShutdown::<Identity, Impl, OFFSET>,
            ThreadIdle: ThreadIdle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchProtocolThreadContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISearchQueryHelper_Impl: Sized {
    fn ConnectionString(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetQueryContentLocale(&mut self, lcid: u32) -> ::windows::core::Result<()>;
    fn QueryContentLocale(&mut self) -> ::windows::core::Result<u32>;
    fn SetQueryKeywordLocale(&mut self, lcid: u32) -> ::windows::core::Result<()>;
    fn QueryKeywordLocale(&mut self) -> ::windows::core::Result<u32>;
    fn SetQueryTermExpansion(&mut self, expandterms: SEARCH_TERM_EXPANSION) -> ::windows::core::Result<()>;
    fn QueryTermExpansion(&mut self) -> ::windows::core::Result<SEARCH_TERM_EXPANSION>;
    fn SetQuerySyntax(&mut self, querysyntax: SEARCH_QUERY_SYNTAX) -> ::windows::core::Result<()>;
    fn QuerySyntax(&mut self) -> ::windows::core::Result<SEARCH_QUERY_SYNTAX>;
    fn SetQueryContentProperties(&mut self, pszcontentproperties: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn QueryContentProperties(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetQuerySelectColumns(&mut self, pszselectcolumns: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn QuerySelectColumns(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetQueryWhereRestrictions(&mut self, pszrestrictions: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn QueryWhereRestrictions(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetQuerySorting(&mut self, pszsorting: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn QuerySorting(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GenerateSQLFromUserQuery(&mut self, pszquery: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn WriteProperties(&mut self, itemid: i32, dwnumberofcolumns: u32, pcolumns: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalues: *const SEARCH_COLUMN_PROPERTIES, pftgathermodifiedtime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn SetQueryMaxResults(&mut self, cmaxresults: i32) -> ::windows::core::Result<()>;
    fn QueryMaxResults(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISearchQueryHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>() -> ISearchQueryHelper_Vtbl {
        unsafe extern "system" fn ConnectionString<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconnectionstring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConnectionString() {
                ::core::result::Result::Ok(ok__) => {
                    *pszconnectionstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryContentLocale<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetQueryContentLocale(::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn QueryContentLocale<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryContentLocale() {
                ::core::result::Result::Ok(ok__) => {
                    *plcid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryKeywordLocale<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetQueryKeywordLocale(::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn QueryKeywordLocale<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryKeywordLocale() {
                ::core::result::Result::Ok(ok__) => {
                    *plcid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryTermExpansion<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expandterms: SEARCH_TERM_EXPANSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetQueryTermExpansion(::core::mem::transmute_copy(&expandterms)).into()
        }
        unsafe extern "system" fn QueryTermExpansion<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexpandterms: *mut SEARCH_TERM_EXPANSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryTermExpansion() {
                ::core::result::Result::Ok(ok__) => {
                    *pexpandterms = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuerySyntax<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querysyntax: SEARCH_QUERY_SYNTAX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetQuerySyntax(::core::mem::transmute_copy(&querysyntax)).into()
        }
        unsafe extern "system" fn QuerySyntax<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquerysyntax: *mut SEARCH_QUERY_SYNTAX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QuerySyntax() {
                ::core::result::Result::Ok(ok__) => {
                    *pquerysyntax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryContentProperties<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcontentproperties: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetQueryContentProperties(::core::mem::transmute_copy(&pszcontentproperties)).into()
        }
        unsafe extern "system" fn QueryContentProperties<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcontentproperties: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryContentProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcontentproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuerySelectColumns<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszselectcolumns: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetQuerySelectColumns(::core::mem::transmute_copy(&pszselectcolumns)).into()
        }
        unsafe extern "system" fn QuerySelectColumns<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszselectcolumns: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QuerySelectColumns() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszselectcolumns = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryWhereRestrictions<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrestrictions: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetQueryWhereRestrictions(::core::mem::transmute_copy(&pszrestrictions)).into()
        }
        unsafe extern "system" fn QueryWhereRestrictions<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszrestrictions: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryWhereRestrictions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszrestrictions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuerySorting<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsorting: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetQuerySorting(::core::mem::transmute_copy(&pszsorting)).into()
        }
        unsafe extern "system" fn QuerySorting<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszsorting: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QuerySorting() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszsorting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateSQLFromUserQuery<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszquery: super::super::Foundation::PWSTR, ppszsql: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GenerateSQLFromUserQuery(::core::mem::transmute_copy(&pszquery)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszsql = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteProperties<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: i32, dwnumberofcolumns: u32, pcolumns: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalues: *const SEARCH_COLUMN_PROPERTIES, pftgathermodifiedtime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteProperties(::core::mem::transmute_copy(&itemid), ::core::mem::transmute_copy(&dwnumberofcolumns), ::core::mem::transmute_copy(&pcolumns), ::core::mem::transmute_copy(&pvalues), ::core::mem::transmute_copy(&pftgathermodifiedtime)).into()
        }
        unsafe extern "system" fn SetQueryMaxResults<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmaxresults: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetQueryMaxResults(::core::mem::transmute_copy(&cmaxresults)).into()
        }
        unsafe extern "system" fn QueryMaxResults<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcmaxresults: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryMaxResults() {
                ::core::result::Result::Ok(ok__) => {
                    *pcmaxresults = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ConnectionString: ConnectionString::<Identity, Impl, OFFSET>,
            SetQueryContentLocale: SetQueryContentLocale::<Identity, Impl, OFFSET>,
            QueryContentLocale: QueryContentLocale::<Identity, Impl, OFFSET>,
            SetQueryKeywordLocale: SetQueryKeywordLocale::<Identity, Impl, OFFSET>,
            QueryKeywordLocale: QueryKeywordLocale::<Identity, Impl, OFFSET>,
            SetQueryTermExpansion: SetQueryTermExpansion::<Identity, Impl, OFFSET>,
            QueryTermExpansion: QueryTermExpansion::<Identity, Impl, OFFSET>,
            SetQuerySyntax: SetQuerySyntax::<Identity, Impl, OFFSET>,
            QuerySyntax: QuerySyntax::<Identity, Impl, OFFSET>,
            SetQueryContentProperties: SetQueryContentProperties::<Identity, Impl, OFFSET>,
            QueryContentProperties: QueryContentProperties::<Identity, Impl, OFFSET>,
            SetQuerySelectColumns: SetQuerySelectColumns::<Identity, Impl, OFFSET>,
            QuerySelectColumns: QuerySelectColumns::<Identity, Impl, OFFSET>,
            SetQueryWhereRestrictions: SetQueryWhereRestrictions::<Identity, Impl, OFFSET>,
            QueryWhereRestrictions: QueryWhereRestrictions::<Identity, Impl, OFFSET>,
            SetQuerySorting: SetQuerySorting::<Identity, Impl, OFFSET>,
            QuerySorting: QuerySorting::<Identity, Impl, OFFSET>,
            GenerateSQLFromUserQuery: GenerateSQLFromUserQuery::<Identity, Impl, OFFSET>,
            WriteProperties: WriteProperties::<Identity, Impl, OFFSET>,
            SetQueryMaxResults: SetQueryMaxResults::<Identity, Impl, OFFSET>,
            QueryMaxResults: QueryMaxResults::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchQueryHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait ISearchQueryHits_Impl: Sized {
    fn Init(&mut self, pflt: &::core::option::Option<super::super::Storage::IndexServer::IFilter>, ulflags: u32) -> i32;
    fn NextHitMoniker(&mut self, pcmnk: *mut u32, papmnk: *mut *mut ::core::option::Option<super::Com::IMoniker>) -> i32;
    fn NextHitOffset(&mut self, pcregion: *mut u32, paregion: *mut *mut super::super::Storage::IndexServer::FILTERREGION) -> i32;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ISearchQueryHits_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHits_Impl, const OFFSET: isize>() -> ISearchQueryHits_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflt: ::windows::core::RawPtr, ulflags: u32) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Init(::core::mem::transmute(&pflt), ::core::mem::transmute_copy(&ulflags))
        }
        unsafe extern "system" fn NextHitMoniker<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcmnk: *mut u32, papmnk: *mut *mut ::windows::core::RawPtr) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NextHitMoniker(::core::mem::transmute_copy(&pcmnk), ::core::mem::transmute_copy(&papmnk))
        }
        unsafe extern "system" fn NextHitOffset<Identity: ::windows::core::IUnknownImpl, Impl: ISearchQueryHits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcregion: *mut u32, paregion: *mut *mut super::super::Storage::IndexServer::FILTERREGION) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NextHitOffset(::core::mem::transmute_copy(&pcregion), ::core::mem::transmute_copy(&paregion))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            NextHitMoniker: NextHitMoniker::<Identity, Impl, OFFSET>,
            NextHitOffset: NextHitOffset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchQueryHits as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchRoot_Impl: Sized {
    fn SetSchedule(&mut self, psztaskarg: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Schedule(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetRootURL(&mut self, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RootURL(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetIsHierarchical(&mut self, fishierarchical: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsHierarchical(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetProvidesNotifications(&mut self, fprovidesnotifications: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ProvidesNotifications(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetUseNotificationsOnly(&mut self, fusenotificationsonly: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn UseNotificationsOnly(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnumerationDepth(&mut self, dwdepth: u32) -> ::windows::core::Result<()>;
    fn EnumerationDepth(&mut self) -> ::windows::core::Result<u32>;
    fn SetHostDepth(&mut self, dwdepth: u32) -> ::windows::core::Result<()>;
    fn HostDepth(&mut self) -> ::windows::core::Result<u32>;
    fn SetFollowDirectories(&mut self, ffollowdirectories: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn FollowDirectories(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetAuthenticationType(&mut self, authtype: AUTH_TYPE) -> ::windows::core::Result<()>;
    fn AuthenticationType(&mut self) -> ::windows::core::Result<AUTH_TYPE>;
    fn SetUser(&mut self, pszuser: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn User(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetPassword(&mut self, pszpassword: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Password(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISearchRoot_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>() -> ISearchRoot_Vtbl {
        unsafe extern "system" fn SetSchedule<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztaskarg: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSchedule(::core::mem::transmute_copy(&psztaskarg)).into()
        }
        unsafe extern "system" fn Schedule<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztaskarg: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Schedule() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsztaskarg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRootURL<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRootURL(::core::mem::transmute_copy(&pszurl)).into()
        }
        unsafe extern "system" fn RootURL<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RootURL() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHierarchical<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fishierarchical: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIsHierarchical(::core::mem::transmute_copy(&fishierarchical)).into()
        }
        unsafe extern "system" fn IsHierarchical<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfishierarchical: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsHierarchical() {
                ::core::result::Result::Ok(ok__) => {
                    *pfishierarchical = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProvidesNotifications<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fprovidesnotifications: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProvidesNotifications(::core::mem::transmute_copy(&fprovidesnotifications)).into()
        }
        unsafe extern "system" fn ProvidesNotifications<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprovidesnotifications: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProvidesNotifications() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprovidesnotifications = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseNotificationsOnly<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fusenotificationsonly: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUseNotificationsOnly(::core::mem::transmute_copy(&fusenotificationsonly)).into()
        }
        unsafe extern "system" fn UseNotificationsOnly<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfusenotificationsonly: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UseNotificationsOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *pfusenotificationsonly = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnumerationDepth<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdepth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnumerationDepth(::core::mem::transmute_copy(&dwdepth)).into()
        }
        unsafe extern "system" fn EnumerationDepth<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdepth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerationDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwdepth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostDepth<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdepth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHostDepth(::core::mem::transmute_copy(&dwdepth)).into()
        }
        unsafe extern "system" fn HostDepth<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdepth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HostDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwdepth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFollowDirectories<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffollowdirectories: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFollowDirectories(::core::mem::transmute_copy(&ffollowdirectories)).into()
        }
        unsafe extern "system" fn FollowDirectories<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffollowdirectories: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FollowDirectories() {
                ::core::result::Result::Ok(ok__) => {
                    *pffollowdirectories = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationType<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authtype: AUTH_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuthenticationType(::core::mem::transmute_copy(&authtype)).into()
        }
        unsafe extern "system" fn AuthenticationType<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthtype: *mut AUTH_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AuthenticationType() {
                ::core::result::Result::Ok(ok__) => {
                    *pauthtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUser<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszuser: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUser(::core::mem::transmute_copy(&pszuser)).into()
        }
        unsafe extern "system" fn User<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszuser: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszuser = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPassword<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPassword(::core::mem::transmute_copy(&pszpassword)).into()
        }
        unsafe extern "system" fn Password<Identity: ::windows::core::IUnknownImpl, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Password() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpassword = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetSchedule: SetSchedule::<Identity, Impl, OFFSET>,
            Schedule: Schedule::<Identity, Impl, OFFSET>,
            SetRootURL: SetRootURL::<Identity, Impl, OFFSET>,
            RootURL: RootURL::<Identity, Impl, OFFSET>,
            SetIsHierarchical: SetIsHierarchical::<Identity, Impl, OFFSET>,
            IsHierarchical: IsHierarchical::<Identity, Impl, OFFSET>,
            SetProvidesNotifications: SetProvidesNotifications::<Identity, Impl, OFFSET>,
            ProvidesNotifications: ProvidesNotifications::<Identity, Impl, OFFSET>,
            SetUseNotificationsOnly: SetUseNotificationsOnly::<Identity, Impl, OFFSET>,
            UseNotificationsOnly: UseNotificationsOnly::<Identity, Impl, OFFSET>,
            SetEnumerationDepth: SetEnumerationDepth::<Identity, Impl, OFFSET>,
            EnumerationDepth: EnumerationDepth::<Identity, Impl, OFFSET>,
            SetHostDepth: SetHostDepth::<Identity, Impl, OFFSET>,
            HostDepth: HostDepth::<Identity, Impl, OFFSET>,
            SetFollowDirectories: SetFollowDirectories::<Identity, Impl, OFFSET>,
            FollowDirectories: FollowDirectories::<Identity, Impl, OFFSET>,
            SetAuthenticationType: SetAuthenticationType::<Identity, Impl, OFFSET>,
            AuthenticationType: AuthenticationType::<Identity, Impl, OFFSET>,
            SetUser: SetUser::<Identity, Impl, OFFSET>,
            User: User::<Identity, Impl, OFFSET>,
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
            Password: Password::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchRoot as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchScopeRule_Impl: Sized {
    fn PatternOrURL(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn IsIncluded(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsDefault(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn FollowFlags(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISearchScopeRule_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchScopeRule_Impl, const OFFSET: isize>() -> ISearchScopeRule_Vtbl {
        unsafe extern "system" fn PatternOrURL<Identity: ::windows::core::IUnknownImpl, Impl: ISearchScopeRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpatternorurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PatternOrURL() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpatternorurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIncluded<Identity: ::windows::core::IUnknownImpl, Impl: ISearchScopeRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisincluded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsIncluded() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisincluded = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDefault<Identity: ::windows::core::IUnknownImpl, Impl: ISearchScopeRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisdefault: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisdefault = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FollowFlags<Identity: ::windows::core::IUnknownImpl, Impl: ISearchScopeRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfollowflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FollowFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pfollowflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PatternOrURL: PatternOrURL::<Identity, Impl, OFFSET>,
            IsIncluded: IsIncluded::<Identity, Impl, OFFSET>,
            IsDefault: IsDefault::<Identity, Impl, OFFSET>,
            FollowFlags: FollowFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchScopeRule as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISearchViewChangedSink_Impl: Sized {
    fn OnChange(&mut self, pdwdocid: *const i32, pchange: *const SEARCH_ITEM_CHANGE, pfinview: *const super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISearchViewChangedSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchViewChangedSink_Impl, const OFFSET: isize>() -> ISearchViewChangedSink_Vtbl {
        unsafe extern "system" fn OnChange<Identity: ::windows::core::IUnknownImpl, Impl: ISearchViewChangedSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdocid: *const i32, pchange: *const SEARCH_ITEM_CHANGE, pfinview: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnChange(::core::mem::transmute_copy(&pdwdocid), ::core::mem::transmute_copy(&pchange), ::core::mem::transmute_copy(&pfinview)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnChange: OnChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchViewChangedSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization"))]
pub trait ISecurityInfo_Impl: Sized {
    fn GetCurrentTrustee(&mut self, pptrustee: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::Result<()>;
    fn GetObjectTypes(&mut self, cobjecttypes: *mut u32, rgobjecttypes: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetPermissions(&mut self, objecttype: &::windows::core::GUID, ppermissions: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization"))]
impl ISecurityInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInfo_Impl, const OFFSET: isize>() -> ISecurityInfo_Vtbl {
        unsafe extern "system" fn GetCurrentTrustee<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptrustee: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCurrentTrustee(::core::mem::transmute_copy(&pptrustee)).into()
        }
        unsafe extern "system" fn GetObjectTypes<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cobjecttypes: *mut u32, rgobjecttypes: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObjectTypes(::core::mem::transmute_copy(&cobjecttypes), ::core::mem::transmute_copy(&rgobjecttypes)).into()
        }
        unsafe extern "system" fn GetPermissions<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objecttype: ::windows::core::GUID, ppermissions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPermissions(::core::mem::transmute_copy(&objecttype), ::core::mem::transmute_copy(&ppermissions)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCurrentTrustee: GetCurrentTrustee::<Identity, Impl, OFFSET>,
            GetObjectTypes: GetObjectTypes::<Identity, Impl, OFFSET>,
            GetPermissions: GetPermissions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityInfo as ::windows::core::Interface>::IID
    }
}
pub trait IService_Impl: Sized {
    fn InvokeService(&mut self, punkinner: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IService_Impl, const OFFSET: isize>() -> IService_Vtbl {
        unsafe extern "system" fn InvokeService<Identity: ::windows::core::IUnknownImpl, Impl: IService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkinner: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvokeService(::core::mem::transmute(&punkinner)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), InvokeService: InvokeService::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISessionProperties_Impl: Sized {
    fn GetProperties(&mut self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::Result<()>;
    fn SetProperties(&mut self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISessionProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISessionProperties_Impl, const OFFSET: isize>() -> ISessionProperties_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl, Impl: ISessionProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProperties(::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)).into()
        }
        unsafe extern "system" fn SetProperties<Identity: ::windows::core::IUnknownImpl, Impl: ISessionProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperties(::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            SetProperties: SetProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISessionProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISimpleCommandCreator_Impl: Sized {
    fn CreateICommand(&mut self, ppiunknown: *mut ::core::option::Option<::windows::core::IUnknown>, pouterunk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn VerifyCatalog(&mut self, pwszmachine: super::super::Foundation::PWSTR, pwszcatalogname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetDefaultCatalog(&mut self, pwszcatalogname: super::super::Foundation::PWSTR, cwcin: u32, pcwcout: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISimpleCommandCreator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleCommandCreator_Impl, const OFFSET: isize>() -> ISimpleCommandCreator_Vtbl {
        unsafe extern "system" fn CreateICommand<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleCommandCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiunknown: *mut *mut ::core::ffi::c_void, pouterunk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateICommand(::core::mem::transmute_copy(&ppiunknown), ::core::mem::transmute(&pouterunk)).into()
        }
        unsafe extern "system" fn VerifyCatalog<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleCommandCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszmachine: super::super::Foundation::PWSTR, pwszcatalogname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VerifyCatalog(::core::mem::transmute_copy(&pwszmachine), ::core::mem::transmute_copy(&pwszcatalogname)).into()
        }
        unsafe extern "system" fn GetDefaultCatalog<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleCommandCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcatalogname: super::super::Foundation::PWSTR, cwcin: u32, pcwcout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDefaultCatalog(::core::mem::transmute_copy(&pwszcatalogname), ::core::mem::transmute_copy(&cwcin), ::core::mem::transmute_copy(&pcwcout)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateICommand: CreateICommand::<Identity, Impl, OFFSET>,
            VerifyCatalog: VerifyCatalog::<Identity, Impl, OFFSET>,
            GetDefaultCatalog: GetDefaultCatalog::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleCommandCreator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISourcesRowset_Impl: Sized {
    fn GetSourcesRowset(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID, cpropertysets: u32, rgproperties: *mut DBPROPSET, ppsourcesrowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISourcesRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISourcesRowset_Impl, const OFFSET: isize>() -> ISourcesRowset_Vtbl {
        unsafe extern "system" fn GetSourcesRowset<Identity: ::windows::core::IUnknownImpl, Impl: ISourcesRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, cpropertysets: u32, rgproperties: *mut DBPROPSET, ppsourcesrowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSourcesRowset(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgproperties), ::core::mem::transmute_copy(&ppsourcesrowset)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetSourcesRowset: GetSourcesRowset::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISourcesRowset as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IStemmer_Impl: Sized {
    fn Init(&mut self, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GenerateWordForms(&mut self, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32, pstemsink: &::core::option::Option<IWordFormSink>) -> ::windows::core::Result<()>;
    fn GetLicenseToUse(&mut self, ppwcslicense: *const *const u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IStemmer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStemmer_Impl, const OFFSET: isize>() -> IStemmer_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl, Impl: IStemmer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&ulmaxtokensize), ::core::mem::transmute_copy(&pflicense)).into()
        }
        unsafe extern "system" fn GenerateWordForms<Identity: ::windows::core::IUnknownImpl, Impl: IStemmer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32, pstemsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateWordForms(::core::mem::transmute_copy(&pwcinbuf), ::core::mem::transmute_copy(&cwc), ::core::mem::transmute(&pstemsink)).into()
        }
        unsafe extern "system" fn GetLicenseToUse<Identity: ::windows::core::IUnknownImpl, Impl: IStemmer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwcslicense: *const *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLicenseToUse(::core::mem::transmute_copy(&ppwcslicense)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            GenerateWordForms: GenerateWordForms::<Identity, Impl, OFFSET>,
            GetLicenseToUse: GetLicenseToUse::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStemmer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISubscriptionItem_Impl: Sized {
    fn GetCookie(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetSubscriptionItemInfo(&mut self) -> ::windows::core::Result<SUBSCRIPTIONITEMINFO>;
    fn SetSubscriptionItemInfo(&mut self, psubscriptioniteminfo: *const SUBSCRIPTIONITEMINFO) -> ::windows::core::Result<()>;
    fn ReadProperties(&mut self, ncount: u32, rgwszname: *const super::super::Foundation::PWSTR, rgvalue: *mut super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn WriteProperties(&mut self, ncount: u32, rgwszname: *const super::super::Foundation::PWSTR, rgvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn EnumProperties(&mut self) -> ::windows::core::Result<IEnumItemProperties>;
    fn NotifyChanged(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISubscriptionItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionItem_Impl, const OFFSET: isize>() -> ISubscriptionItem_Vtbl {
        unsafe extern "system" fn GetCookie<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcookie: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCookie() {
                ::core::result::Result::Ok(ok__) => {
                    *pcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriptionItemInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubscriptioniteminfo: *mut SUBSCRIPTIONITEMINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSubscriptionItemInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *psubscriptioniteminfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriptionItemInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubscriptioniteminfo: *const SUBSCRIPTIONITEMINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSubscriptionItemInfo(::core::mem::transmute_copy(&psubscriptioniteminfo)).into()
        }
        unsafe extern "system" fn ReadProperties<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncount: u32, rgwszname: *const super::super::Foundation::PWSTR, rgvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReadProperties(::core::mem::transmute_copy(&ncount), ::core::mem::transmute_copy(&rgwszname), ::core::mem::transmute_copy(&rgvalue)).into()
        }
        unsafe extern "system" fn WriteProperties<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncount: u32, rgwszname: *const super::super::Foundation::PWSTR, rgvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteProperties(::core::mem::transmute_copy(&ncount), ::core::mem::transmute_copy(&rgwszname), ::core::mem::transmute_copy(&rgvalue)).into()
        }
        unsafe extern "system" fn EnumProperties<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumitemproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumitemproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyChanged<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyChanged().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCookie: GetCookie::<Identity, Impl, OFFSET>,
            GetSubscriptionItemInfo: GetSubscriptionItemInfo::<Identity, Impl, OFFSET>,
            SetSubscriptionItemInfo: SetSubscriptionItemInfo::<Identity, Impl, OFFSET>,
            ReadProperties: ReadProperties::<Identity, Impl, OFFSET>,
            WriteProperties: WriteProperties::<Identity, Impl, OFFSET>,
            EnumProperties: EnumProperties::<Identity, Impl, OFFSET>,
            NotifyChanged: NotifyChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISubscriptionItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISubscriptionMgr_Impl: Sized {
    fn DeleteSubscription(&mut self, pwszurl: super::super::Foundation::PWSTR, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn UpdateSubscription(&mut self, pwszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn UpdateAll(&mut self) -> ::windows::core::Result<()>;
    fn IsSubscribed(&mut self, pwszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetSubscriptionInfo(&mut self, pwszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<SUBSCRIPTIONINFO>;
    fn GetDefaultInfo(&mut self, subtype: SUBSCRIPTIONTYPE) -> ::windows::core::Result<SUBSCRIPTIONINFO>;
    fn ShowSubscriptionProperties(&mut self, pwszurl: super::super::Foundation::PWSTR, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn CreateSubscription(&mut self, hwnd: super::super::Foundation::HWND, pwszurl: super::super::Foundation::PWSTR, pwszfriendlyname: super::super::Foundation::PWSTR, dwflags: u32, substype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISubscriptionMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionMgr_Impl, const OFFSET: isize>() -> ISubscriptionMgr_Vtbl {
        unsafe extern "system" fn DeleteSubscription<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteSubscription(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn UpdateSubscription<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateSubscription(::core::mem::transmute_copy(&pwszurl)).into()
        }
        unsafe extern "system" fn UpdateAll<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateAll().into()
        }
        unsafe extern "system" fn IsSubscribed<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pfsubscribed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsSubscribed(::core::mem::transmute_copy(&pwszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfsubscribed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriptionInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSubscriptionInfo(::core::mem::transmute_copy(&pwszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *pinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subtype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDefaultInfo(::core::mem::transmute_copy(&subtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowSubscriptionProperties<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowSubscriptionProperties(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn CreateSubscription<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pwszurl: super::super::Foundation::PWSTR, pwszfriendlyname: super::super::Foundation::PWSTR, dwflags: u32, substype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateSubscription(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pwszfriendlyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&substype), ::core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DeleteSubscription: DeleteSubscription::<Identity, Impl, OFFSET>,
            UpdateSubscription: UpdateSubscription::<Identity, Impl, OFFSET>,
            UpdateAll: UpdateAll::<Identity, Impl, OFFSET>,
            IsSubscribed: IsSubscribed::<Identity, Impl, OFFSET>,
            GetSubscriptionInfo: GetSubscriptionInfo::<Identity, Impl, OFFSET>,
            GetDefaultInfo: GetDefaultInfo::<Identity, Impl, OFFSET>,
            ShowSubscriptionProperties: ShowSubscriptionProperties::<Identity, Impl, OFFSET>,
            CreateSubscription: CreateSubscription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISubscriptionMgr as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISubscriptionMgr2_Impl: Sized + ISubscriptionMgr_Impl {
    fn GetItemFromURL(&mut self, pwszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<ISubscriptionItem>;
    fn GetItemFromCookie(&mut self, psubscriptioncookie: *const ::windows::core::GUID) -> ::windows::core::Result<ISubscriptionItem>;
    fn GetSubscriptionRunState(&mut self, dwnumcookies: u32, pcookies: *const ::windows::core::GUID, pdwrunstate: *mut u32) -> ::windows::core::Result<()>;
    fn EnumSubscriptions(&mut self, dwflags: u32) -> ::windows::core::Result<IEnumSubscription>;
    fn UpdateItems(&mut self, dwflags: u32, dwnumcookies: u32, pcookies: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn AbortItems(&mut self, dwnumcookies: u32, pcookies: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn AbortAll(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISubscriptionMgr2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionMgr2_Impl, const OFFSET: isize>() -> ISubscriptionMgr2_Vtbl {
        unsafe extern "system" fn GetItemFromURL<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, ppsubscriptionitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItemFromURL(::core::mem::transmute_copy(&pwszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubscriptionitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemFromCookie<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubscriptioncookie: *const ::windows::core::GUID, ppsubscriptionitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItemFromCookie(::core::mem::transmute_copy(&psubscriptioncookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubscriptionitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriptionRunState<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnumcookies: u32, pcookies: *const ::windows::core::GUID, pdwrunstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSubscriptionRunState(::core::mem::transmute_copy(&dwnumcookies), ::core::mem::transmute_copy(&pcookies), ::core::mem::transmute_copy(&pdwrunstate)).into()
        }
        unsafe extern "system" fn EnumSubscriptions<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenumsubscriptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumSubscriptions(::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumsubscriptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateItems<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwnumcookies: u32, pcookies: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateItems(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwnumcookies), ::core::mem::transmute_copy(&pcookies)).into()
        }
        unsafe extern "system" fn AbortItems<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnumcookies: u32, pcookies: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AbortItems(::core::mem::transmute_copy(&dwnumcookies), ::core::mem::transmute_copy(&pcookies)).into()
        }
        unsafe extern "system" fn AbortAll<Identity: ::windows::core::IUnknownImpl, Impl: ISubscriptionMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AbortAll().into()
        }
        Self {
            base: ISubscriptionMgr_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetItemFromURL: GetItemFromURL::<Identity, Impl, OFFSET>,
            GetItemFromCookie: GetItemFromCookie::<Identity, Impl, OFFSET>,
            GetSubscriptionRunState: GetSubscriptionRunState::<Identity, Impl, OFFSET>,
            EnumSubscriptions: EnumSubscriptions::<Identity, Impl, OFFSET>,
            UpdateItems: UpdateItems::<Identity, Impl, OFFSET>,
            AbortItems: AbortItems::<Identity, Impl, OFFSET>,
            AbortAll: AbortAll::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISubscriptionMgr2 as ::windows::core::Interface>::IID || iid == &<ISubscriptionMgr as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITableCreation_Impl: Sized + ITableDefinition_Impl {
    fn GetTableDefinition(&mut self, ptableid: *const super::super::Storage::IndexServer::DBID, pccolumndescs: *mut usize, prgcolumndescs: *mut *mut DBCOLUMNDESC, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET, pcconstraintdescs: *mut u32, prgconstraintdescs: *mut *mut DBCONSTRAINTDESC, ppwszstringbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITableCreation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableCreation_Impl, const OFFSET: isize>() -> ITableCreation_Vtbl {
        unsafe extern "system" fn GetTableDefinition<Identity: ::windows::core::IUnknownImpl, Impl: ITableCreation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pccolumndescs: *mut usize, prgcolumndescs: *mut *mut DBCOLUMNDESC, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET, pcconstraintdescs: *mut u32, prgconstraintdescs: *mut *mut DBCONSTRAINTDESC, ppwszstringbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTableDefinition(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pccolumndescs), ::core::mem::transmute_copy(&prgcolumndescs), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets), ::core::mem::transmute_copy(&pcconstraintdescs), ::core::mem::transmute_copy(&prgconstraintdescs), ::core::mem::transmute_copy(&ppwszstringbuffer)).into()
        }
        Self { base: ITableDefinition_Vtbl::new::<Identity, Impl, OFFSET>(), GetTableDefinition: GetTableDefinition::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITableCreation as ::windows::core::Interface>::IID || iid == &<ITableDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITableDefinition_Impl: Sized {
    fn CreateTable(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, ptableid: *const super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *const DBCOLUMNDESC, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DropTable(&mut self, ptableid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
    fn AddColumn(&mut self, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumndesc: *const DBCOLUMNDESC) -> ::windows::core::Result<*mut super::super::Storage::IndexServer::DBID>;
    fn DropColumn(&mut self, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumnid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITableDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableDefinition_Impl, const OFFSET: isize>() -> ITableDefinition_Vtbl {
        unsafe extern "system" fn CreateTable<Identity: ::windows::core::IUnknownImpl, Impl: ITableDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *const DBCOLUMNDESC, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateTable(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&ccolumndescs), ::core::mem::transmute_copy(&rgcolumndescs), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pptableid), ::core::mem::transmute_copy(&pprowset)).into()
        }
        unsafe extern "system" fn DropTable<Identity: ::windows::core::IUnknownImpl, Impl: ITableDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DropTable(::core::mem::transmute_copy(&ptableid)).into()
        }
        unsafe extern "system" fn AddColumn<Identity: ::windows::core::IUnknownImpl, Impl: ITableDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumndesc: *const DBCOLUMNDESC, ppcolumnid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddColumn(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pcolumndesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolumnid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropColumn<Identity: ::windows::core::IUnknownImpl, Impl: ITableDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumnid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DropColumn(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pcolumnid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateTable: CreateTable::<Identity, Impl, OFFSET>,
            DropTable: DropTable::<Identity, Impl, OFFSET>,
            AddColumn: AddColumn::<Identity, Impl, OFFSET>,
            DropColumn: DropColumn::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITableDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITableDefinitionWithConstraints_Impl: Sized + ITableDefinition_Impl + ITableCreation_Impl {
    fn AddConstraint(&mut self, ptableid: *mut super::super::Storage::IndexServer::DBID, pconstraintdesc: *mut DBCONSTRAINTDESC) -> ::windows::core::Result<()>;
    fn CreateTableWithConstraints(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, ptableid: *mut super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *mut DBCOLUMNDESC, cconstraintdescs: u32, rgconstraintdescs: *mut DBCONSTRAINTDESC, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DropConstraint(&mut self, ptableid: *mut super::super::Storage::IndexServer::DBID, pconstraintid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITableDefinitionWithConstraints_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableDefinitionWithConstraints_Impl, const OFFSET: isize>() -> ITableDefinitionWithConstraints_Vtbl {
        unsafe extern "system" fn AddConstraint<Identity: ::windows::core::IUnknownImpl, Impl: ITableDefinitionWithConstraints_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, pconstraintdesc: *mut DBCONSTRAINTDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddConstraint(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pconstraintdesc)).into()
        }
        unsafe extern "system" fn CreateTableWithConstraints<Identity: ::windows::core::IUnknownImpl, Impl: ITableDefinitionWithConstraints_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *mut DBCOLUMNDESC, cconstraintdescs: u32, rgconstraintdescs: *mut DBCONSTRAINTDESC, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .CreateTableWithConstraints(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&ccolumndescs), ::core::mem::transmute_copy(&rgcolumndescs), ::core::mem::transmute_copy(&cconstraintdescs), ::core::mem::transmute_copy(&rgconstraintdescs), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pptableid), ::core::mem::transmute_copy(&pprowset))
                .into()
        }
        unsafe extern "system" fn DropConstraint<Identity: ::windows::core::IUnknownImpl, Impl: ITableDefinitionWithConstraints_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, pconstraintid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DropConstraint(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pconstraintid)).into()
        }
        Self {
            base: ITableCreation_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddConstraint: AddConstraint::<Identity, Impl, OFFSET>,
            CreateTableWithConstraints: CreateTableWithConstraints::<Identity, Impl, OFFSET>,
            DropConstraint: DropConstraint::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITableDefinitionWithConstraints as ::windows::core::Interface>::IID || iid == &<ITableDefinition as ::windows::core::Interface>::IID || iid == &<ITableCreation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
pub trait ITableRename_Impl: Sized {
    fn RenameColumn(&mut self, ptableid: *mut super::super::Storage::IndexServer::DBID, poldcolumnid: *mut super::super::Storage::IndexServer::DBID, pnewcolumnid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
    fn RenameTable(&mut self, poldtableid: *mut super::super::Storage::IndexServer::DBID, poldindexid: *mut super::super::Storage::IndexServer::DBID, pnewtableid: *mut super::super::Storage::IndexServer::DBID, pnewindexid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
impl ITableRename_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableRename_Impl, const OFFSET: isize>() -> ITableRename_Vtbl {
        unsafe extern "system" fn RenameColumn<Identity: ::windows::core::IUnknownImpl, Impl: ITableRename_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, poldcolumnid: *mut super::super::Storage::IndexServer::DBID, pnewcolumnid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RenameColumn(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&poldcolumnid), ::core::mem::transmute_copy(&pnewcolumnid)).into()
        }
        unsafe extern "system" fn RenameTable<Identity: ::windows::core::IUnknownImpl, Impl: ITableRename_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poldtableid: *mut super::super::Storage::IndexServer::DBID, poldindexid: *mut super::super::Storage::IndexServer::DBID, pnewtableid: *mut super::super::Storage::IndexServer::DBID, pnewindexid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RenameTable(::core::mem::transmute_copy(&poldtableid), ::core::mem::transmute_copy(&poldindexid), ::core::mem::transmute_copy(&pnewtableid), ::core::mem::transmute_copy(&pnewindexid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RenameColumn: RenameColumn::<Identity, Impl, OFFSET>,
            RenameTable: RenameTable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITableRename as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITokenCollection_Impl: Sized {
    fn NumberOfTokens(&mut self, pcount: *const u32) -> ::windows::core::Result<()>;
    fn GetToken(&mut self, i: u32, pbegin: *mut u32, plength: *mut u32, ppsz: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITokenCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITokenCollection_Impl, const OFFSET: isize>() -> ITokenCollection_Vtbl {
        unsafe extern "system" fn NumberOfTokens<Identity: ::windows::core::IUnknownImpl, Impl: ITokenCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NumberOfTokens(::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn GetToken<Identity: ::windows::core::IUnknownImpl, Impl: ITokenCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: u32, pbegin: *mut u32, plength: *mut u32, ppsz: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetToken(::core::mem::transmute_copy(&i), ::core::mem::transmute_copy(&pbegin), ::core::mem::transmute_copy(&plength), ::core::mem::transmute_copy(&ppsz)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            NumberOfTokens: NumberOfTokens::<Identity, Impl, OFFSET>,
            GetToken: GetToken::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITokenCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ITransactionJoin_Impl: Sized {
    fn GetOptionsObject(&mut self) -> ::windows::core::Result<super::DistributedTransactionCoordinator::ITransactionOptions>;
    fn JoinTransaction(&mut self, punktransactioncoord: &::core::option::Option<::windows::core::IUnknown>, isolevel: i32, isoflags: u32, potheroptions: &::core::option::Option<super::DistributedTransactionCoordinator::ITransactionOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ITransactionJoin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionJoin_Impl, const OFFSET: isize>() -> ITransactionJoin_Vtbl {
        unsafe extern "system" fn GetOptionsObject<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionJoin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOptionsObject() {
                ::core::result::Result::Ok(ok__) => {
                    *ppoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JoinTransaction<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionJoin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punktransactioncoord: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, potheroptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).JoinTransaction(::core::mem::transmute(&punktransactioncoord), ::core::mem::transmute_copy(&isolevel), ::core::mem::transmute_copy(&isoflags), ::core::mem::transmute(&potheroptions)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetOptionsObject: GetOptionsObject::<Identity, Impl, OFFSET>,
            JoinTransaction: JoinTransaction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionJoin as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
pub trait ITransactionLocal_Impl: Sized + super::DistributedTransactionCoordinator::ITransaction_Impl {
    fn GetOptionsObject(&mut self) -> ::windows::core::Result<super::DistributedTransactionCoordinator::ITransactionOptions>;
    fn StartTransaction(&mut self, isolevel: i32, isoflags: u32, potheroptions: &::core::option::Option<super::DistributedTransactionCoordinator::ITransactionOptions>) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
impl ITransactionLocal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionLocal_Impl, const OFFSET: isize>() -> ITransactionLocal_Vtbl {
        unsafe extern "system" fn GetOptionsObject<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionLocal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOptionsObject() {
                ::core::result::Result::Ok(ok__) => {
                    *ppoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTransaction<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionLocal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, potheroptions: ::windows::core::RawPtr, pultransactionlevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartTransaction(::core::mem::transmute_copy(&isolevel), ::core::mem::transmute_copy(&isoflags), ::core::mem::transmute(&potheroptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *pultransactionlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::DistributedTransactionCoordinator::ITransaction_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOptionsObject: GetOptionsObject::<Identity, Impl, OFFSET>,
            StartTransaction: StartTransaction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionLocal as ::windows::core::Interface>::IID || iid == &<super::DistributedTransactionCoordinator::ITransaction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ITransactionObject_Impl: Sized {
    fn GetTransactionObject(&mut self, ultransactionlevel: u32) -> ::windows::core::Result<super::DistributedTransactionCoordinator::ITransaction>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ITransactionObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionObject_Impl, const OFFSET: isize>() -> ITransactionObject_Vtbl {
        unsafe extern "system" fn GetTransactionObject<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultransactionlevel: u32, pptransactionobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTransactionObject(::core::mem::transmute_copy(&ultransactionlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptransactionobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetTransactionObject: GetTransactionObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITrusteeAdmin_Impl: Sized {
    fn CompareTrustees(&mut self, ptrustee1: *mut super::super::Security::Authorization::TRUSTEE_W, ptrustee2: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::Result<()>;
    fn CreateTrustee(&mut self, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::Result<()>;
    fn DeleteTrustee(&mut self, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::Result<()>;
    fn SetTrusteeProperties(&mut self, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::Result<()>;
    fn GetTrusteeProperties(&mut self, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITrusteeAdmin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITrusteeAdmin_Impl, const OFFSET: isize>() -> ITrusteeAdmin_Vtbl {
        unsafe extern "system" fn CompareTrustees<Identity: ::windows::core::IUnknownImpl, Impl: ITrusteeAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrustee1: *mut super::super::Security::Authorization::TRUSTEE_W, ptrustee2: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CompareTrustees(::core::mem::transmute_copy(&ptrustee1), ::core::mem::transmute_copy(&ptrustee2)).into()
        }
        unsafe extern "system" fn CreateTrustee<Identity: ::windows::core::IUnknownImpl, Impl: ITrusteeAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateTrustee(::core::mem::transmute_copy(&ptrustee), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into()
        }
        unsafe extern "system" fn DeleteTrustee<Identity: ::windows::core::IUnknownImpl, Impl: ITrusteeAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteTrustee(::core::mem::transmute_copy(&ptrustee)).into()
        }
        unsafe extern "system" fn SetTrusteeProperties<Identity: ::windows::core::IUnknownImpl, Impl: ITrusteeAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTrusteeProperties(::core::mem::transmute_copy(&ptrustee), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into()
        }
        unsafe extern "system" fn GetTrusteeProperties<Identity: ::windows::core::IUnknownImpl, Impl: ITrusteeAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTrusteeProperties(::core::mem::transmute_copy(&ptrustee), ::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CompareTrustees: CompareTrustees::<Identity, Impl, OFFSET>,
            CreateTrustee: CreateTrustee::<Identity, Impl, OFFSET>,
            DeleteTrustee: DeleteTrustee::<Identity, Impl, OFFSET>,
            SetTrusteeProperties: SetTrusteeProperties::<Identity, Impl, OFFSET>,
            GetTrusteeProperties: GetTrusteeProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITrusteeAdmin as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization"))]
pub trait ITrusteeGroupAdmin_Impl: Sized {
    fn AddMember(&mut self, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::Result<()>;
    fn DeleteMember(&mut self, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::Result<()>;
    fn IsMember(&mut self, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pfstatus: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetMembers(&mut self, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pcmembers: *mut u32, prgmembers: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::Result<()>;
    fn GetMemberships(&mut self, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pcmemberships: *mut u32, prgmemberships: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization"))]
impl ITrusteeGroupAdmin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITrusteeGroupAdmin_Impl, const OFFSET: isize>() -> ITrusteeGroupAdmin_Vtbl {
        unsafe extern "system" fn AddMember<Identity: ::windows::core::IUnknownImpl, Impl: ITrusteeGroupAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddMember(::core::mem::transmute_copy(&pmembershiptrustee), ::core::mem::transmute_copy(&pmembertrustee)).into()
        }
        unsafe extern "system" fn DeleteMember<Identity: ::windows::core::IUnknownImpl, Impl: ITrusteeGroupAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteMember(::core::mem::transmute_copy(&pmembershiptrustee), ::core::mem::transmute_copy(&pmembertrustee)).into()
        }
        unsafe extern "system" fn IsMember<Identity: ::windows::core::IUnknownImpl, Impl: ITrusteeGroupAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pfstatus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsMember(::core::mem::transmute_copy(&pmembershiptrustee), ::core::mem::transmute_copy(&pmembertrustee), ::core::mem::transmute_copy(&pfstatus)).into()
        }
        unsafe extern "system" fn GetMembers<Identity: ::windows::core::IUnknownImpl, Impl: ITrusteeGroupAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pcmembers: *mut u32, prgmembers: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMembers(::core::mem::transmute_copy(&pmembershiptrustee), ::core::mem::transmute_copy(&pcmembers), ::core::mem::transmute_copy(&prgmembers)).into()
        }
        unsafe extern "system" fn GetMemberships<Identity: ::windows::core::IUnknownImpl, Impl: ITrusteeGroupAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pcmemberships: *mut u32, prgmemberships: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMemberships(::core::mem::transmute_copy(&ptrustee), ::core::mem::transmute_copy(&pcmemberships), ::core::mem::transmute_copy(&prgmemberships)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddMember: AddMember::<Identity, Impl, OFFSET>,
            DeleteMember: DeleteMember::<Identity, Impl, OFFSET>,
            IsMember: IsMember::<Identity, Impl, OFFSET>,
            GetMembers: GetMembers::<Identity, Impl, OFFSET>,
            GetMemberships: GetMemberships::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITrusteeGroupAdmin as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUMS_Impl: Sized {
    fn SqlUmsSuspend(&mut self, ticks: u32);
    fn SqlUmsYield(&mut self, ticks: u32);
    fn SqlUmsSwitchPremptive(&mut self);
    fn SqlUmsSwitchNonPremptive(&mut self);
    fn SqlUmsFIsPremptive(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl IUMS_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUMS_Impl, const OFFSET: isize>() -> IUMS_Vtbl {
        unsafe extern "system" fn SqlUmsSuspend<Identity: ::windows::core::IUnknownImpl, Impl: IUMS_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ticks: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SqlUmsSuspend(::core::mem::transmute_copy(&ticks))
        }
        unsafe extern "system" fn SqlUmsYield<Identity: ::windows::core::IUnknownImpl, Impl: IUMS_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ticks: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SqlUmsYield(::core::mem::transmute_copy(&ticks))
        }
        unsafe extern "system" fn SqlUmsSwitchPremptive<Identity: ::windows::core::IUnknownImpl, Impl: IUMS_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SqlUmsSwitchPremptive()
        }
        unsafe extern "system" fn SqlUmsSwitchNonPremptive<Identity: ::windows::core::IUnknownImpl, Impl: IUMS_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SqlUmsSwitchNonPremptive()
        }
        unsafe extern "system" fn SqlUmsFIsPremptive<Identity: ::windows::core::IUnknownImpl, Impl: IUMS_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SqlUmsFIsPremptive()
        }
        Self {
            SqlUmsSuspend: SqlUmsSuspend::<Identity, Impl, OFFSET>,
            SqlUmsYield: SqlUmsYield::<Identity, Impl, OFFSET>,
            SqlUmsSwitchPremptive: SqlUmsSwitchPremptive::<Identity, Impl, OFFSET>,
            SqlUmsSwitchNonPremptive: SqlUmsSwitchNonPremptive::<Identity, Impl, OFFSET>,
            SqlUmsFIsPremptive: SqlUmsFIsPremptive::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUMS as ::windows::core::Interface>::IID
    }
}
pub trait IUMSInitialize_Impl: Sized {
    fn Initialize(&mut self, pums: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IUMSInitialize_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUMSInitialize_Impl, const OFFSET: isize>() -> IUMSInitialize_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IUMSInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pums: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pums)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUMSInitialize as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IUrlAccessor_Impl: Sized {
    fn AddRequestParameter(&mut self, pspec: *const super::Com::StructuredStorage::PROPSPEC, pvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetDocFormat(&mut self, wszdocformat: super::super::Foundation::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()>;
    fn GetCLSID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetHost(&mut self, wszhost: super::super::Foundation::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()>;
    fn IsDirectory(&mut self) -> ::windows::core::Result<()>;
    fn GetSize(&mut self) -> ::windows::core::Result<u64>;
    fn GetLastModified(&mut self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn GetFileName(&mut self, wszfilename: super::super::Foundation::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()>;
    fn GetSecurityDescriptor(&mut self, psd: *mut u8, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()>;
    fn GetRedirectedURL(&mut self, wszredirectedurl: super::super::Foundation::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()>;
    fn GetSecurityProvider(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BindToStream(&mut self) -> ::windows::core::Result<super::Com::IStream>;
    fn BindToFilter(&mut self) -> ::windows::core::Result<super::super::Storage::IndexServer::IFilter>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IUrlAccessor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor_Impl, const OFFSET: isize>() -> IUrlAccessor_Vtbl {
        unsafe extern "system" fn AddRequestParameter<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pspec: *const super::Com::StructuredStorage::PROPSPEC, pvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRequestParameter(::core::mem::transmute_copy(&pspec), ::core::mem::transmute_copy(&pvar)).into()
        }
        unsafe extern "system" fn GetDocFormat<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszdocformat: super::super::Foundation::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDocFormat(::core::mem::transmute_copy(&wszdocformat), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetCLSID<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHost<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszhost: super::super::Foundation::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetHost(::core::mem::transmute_copy(&wszhost), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn IsDirectory<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsDirectory().into()
        }
        unsafe extern "system" fn GetSize<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pllsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastModified<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftlastmodified: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLastModified() {
                ::core::result::Result::Ok(ok__) => {
                    *pftlastmodified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileName<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszfilename: super::super::Foundation::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFileName(::core::mem::transmute_copy(&wszfilename), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psd: *mut u8, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSecurityDescriptor(::core::mem::transmute_copy(&psd), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetRedirectedURL<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszredirectedurl: super::super::Foundation::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRedirectedURL(::core::mem::transmute_copy(&wszredirectedurl), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetSecurityProvider<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pspclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSecurityProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *pspclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToStream<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BindToStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToFilter<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BindToFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddRequestParameter: AddRequestParameter::<Identity, Impl, OFFSET>,
            GetDocFormat: GetDocFormat::<Identity, Impl, OFFSET>,
            GetCLSID: GetCLSID::<Identity, Impl, OFFSET>,
            GetHost: GetHost::<Identity, Impl, OFFSET>,
            IsDirectory: IsDirectory::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetLastModified: GetLastModified::<Identity, Impl, OFFSET>,
            GetFileName: GetFileName::<Identity, Impl, OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, Impl, OFFSET>,
            GetRedirectedURL: GetRedirectedURL::<Identity, Impl, OFFSET>,
            GetSecurityProvider: GetSecurityProvider::<Identity, Impl, OFFSET>,
            BindToStream: BindToStream::<Identity, Impl, OFFSET>,
            BindToFilter: BindToFilter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUrlAccessor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IUrlAccessor2_Impl: Sized + IUrlAccessor_Impl {
    fn GetDisplayUrl(&mut self, wszdocurl: super::super::Foundation::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()>;
    fn IsDocument(&mut self) -> ::windows::core::Result<()>;
    fn GetCodePage(&mut self, wszcodepage: super::super::Foundation::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IUrlAccessor2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor2_Impl, const OFFSET: isize>() -> IUrlAccessor2_Vtbl {
        unsafe extern "system" fn GetDisplayUrl<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszdocurl: super::super::Foundation::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplayUrl(::core::mem::transmute_copy(&wszdocurl), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn IsDocument<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsDocument().into()
        }
        unsafe extern "system" fn GetCodePage<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszcodepage: super::super::Foundation::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCodePage(::core::mem::transmute_copy(&wszcodepage), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        Self {
            base: IUrlAccessor_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDisplayUrl: GetDisplayUrl::<Identity, Impl, OFFSET>,
            IsDocument: IsDocument::<Identity, Impl, OFFSET>,
            GetCodePage: GetCodePage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUrlAccessor2 as ::windows::core::Interface>::IID || iid == &<IUrlAccessor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IUrlAccessor3_Impl: Sized + IUrlAccessor_Impl + IUrlAccessor2_Impl {
    fn GetImpersonationSidBlobs(&mut self, pcwszurl: super::super::Foundation::PWSTR, pcsidcount: *mut u32, ppsidblobs: *mut *mut super::Com::BLOB) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IUrlAccessor3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor3_Impl, const OFFSET: isize>() -> IUrlAccessor3_Vtbl {
        unsafe extern "system" fn GetImpersonationSidBlobs<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcwszurl: super::super::Foundation::PWSTR, pcsidcount: *mut u32, ppsidblobs: *mut *mut super::Com::BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetImpersonationSidBlobs(::core::mem::transmute_copy(&pcwszurl), ::core::mem::transmute_copy(&pcsidcount), ::core::mem::transmute_copy(&ppsidblobs)).into()
        }
        Self { base: IUrlAccessor2_Vtbl::new::<Identity, Impl, OFFSET>(), GetImpersonationSidBlobs: GetImpersonationSidBlobs::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUrlAccessor3 as ::windows::core::Interface>::IID || iid == &<IUrlAccessor as ::windows::core::Interface>::IID || iid == &<IUrlAccessor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IUrlAccessor4_Impl: Sized + IUrlAccessor_Impl + IUrlAccessor2_Impl + IUrlAccessor3_Impl {
    fn ShouldIndexItemContent(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ShouldIndexProperty(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IUrlAccessor4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor4_Impl, const OFFSET: isize>() -> IUrlAccessor4_Vtbl {
        unsafe extern "system" fn ShouldIndexItemContent<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfindexcontent: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShouldIndexItemContent() {
                ::core::result::Result::Ok(ok__) => {
                    *pfindexcontent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldIndexProperty<Identity: ::windows::core::IUnknownImpl, Impl: IUrlAccessor4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfindexproperty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShouldIndexProperty(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfindexproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUrlAccessor3_Vtbl::new::<Identity, Impl, OFFSET>(),
            ShouldIndexItemContent: ShouldIndexItemContent::<Identity, Impl, OFFSET>,
            ShouldIndexProperty: ShouldIndexProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUrlAccessor4 as ::windows::core::Interface>::IID || iid == &<IUrlAccessor as ::windows::core::Interface>::IID || iid == &<IUrlAccessor2 as ::windows::core::Interface>::IID || iid == &<IUrlAccessor3 as ::windows::core::Interface>::IID
    }
}
pub trait IViewChapter_Impl: Sized {
    fn GetSpecification(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn OpenViewChapter(&mut self, hsource: usize) -> ::windows::core::Result<usize>;
}
impl IViewChapter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewChapter_Impl, const OFFSET: isize>() -> IViewChapter_Vtbl {
        unsafe extern "system" fn GetSpecification<Identity: ::windows::core::IUnknownImpl, Impl: IViewChapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSpecification(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprowset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenViewChapter<Identity: ::windows::core::IUnknownImpl, Impl: IViewChapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsource: usize, phviewchapter: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenViewChapter(::core::mem::transmute_copy(&hsource)) {
                ::core::result::Result::Ok(ok__) => {
                    *phviewchapter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSpecification: GetSpecification::<Identity, Impl, OFFSET>,
            OpenViewChapter: OpenViewChapter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewChapter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IViewFilter_Impl: Sized {
    fn GetFilter(&mut self, haccessor: usize, pcrows: *mut usize, pcompareops: *mut *mut u32, pcriteriadata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetFilterBindings(&mut self, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> ::windows::core::Result<()>;
    fn SetFilter(&mut self, haccessor: usize, crows: usize, compareops: *const u32, pcriteriadata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IViewFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewFilter_Impl, const OFFSET: isize>() -> IViewFilter_Vtbl {
        unsafe extern "system" fn GetFilter<Identity: ::windows::core::IUnknownImpl, Impl: IViewFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haccessor: usize, pcrows: *mut usize, pcompareops: *mut *mut u32, pcriteriadata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFilter(::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pcrows), ::core::mem::transmute_copy(&pcompareops), ::core::mem::transmute_copy(&pcriteriadata)).into()
        }
        unsafe extern "system" fn GetFilterBindings<Identity: ::windows::core::IUnknownImpl, Impl: IViewFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFilterBindings(::core::mem::transmute_copy(&pcbindings), ::core::mem::transmute_copy(&prgbindings)).into()
        }
        unsafe extern "system" fn SetFilter<Identity: ::windows::core::IUnknownImpl, Impl: IViewFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haccessor: usize, crows: usize, compareops: *const u32, pcriteriadata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFilter(::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&compareops), ::core::mem::transmute_copy(&pcriteriadata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetFilter: GetFilter::<Identity, Impl, OFFSET>,
            GetFilterBindings: GetFilterBindings::<Identity, Impl, OFFSET>,
            SetFilter: SetFilter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewFilter as ::windows::core::Interface>::IID
    }
}
pub trait IViewRowset_Impl: Sized {
    fn GetSpecification(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn OpenViewRowset(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl IViewRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewRowset_Impl, const OFFSET: isize>() -> IViewRowset_Vtbl {
        unsafe extern "system" fn GetSpecification<Identity: ::windows::core::IUnknownImpl, Impl: IViewRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSpecification(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenViewRowset<Identity: ::windows::core::IUnknownImpl, Impl: IViewRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenViewRowset(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprowset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSpecification: GetSpecification::<Identity, Impl, OFFSET>,
            OpenViewRowset: OpenViewRowset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewRowset as ::windows::core::Interface>::IID
    }
}
pub trait IViewSort_Impl: Sized {
    fn GetSortOrder(&mut self, pcvalues: *mut usize, prgcolumns: *mut *mut usize, prgorders: *mut *mut u32) -> ::windows::core::Result<()>;
    fn SetSortOrder(&mut self, cvalues: usize, rgcolumns: *const usize, rgorders: *const u32) -> ::windows::core::Result<()>;
}
impl IViewSort_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewSort_Impl, const OFFSET: isize>() -> IViewSort_Vtbl {
        unsafe extern "system" fn GetSortOrder<Identity: ::windows::core::IUnknownImpl, Impl: IViewSort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcvalues: *mut usize, prgcolumns: *mut *mut usize, prgorders: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSortOrder(::core::mem::transmute_copy(&pcvalues), ::core::mem::transmute_copy(&prgcolumns), ::core::mem::transmute_copy(&prgorders)).into()
        }
        unsafe extern "system" fn SetSortOrder<Identity: ::windows::core::IUnknownImpl, Impl: IViewSort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cvalues: usize, rgcolumns: *const usize, rgorders: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSortOrder(::core::mem::transmute_copy(&cvalues), ::core::mem::transmute_copy(&rgcolumns), ::core::mem::transmute_copy(&rgorders)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSortOrder: GetSortOrder::<Identity, Impl, OFFSET>,
            SetSortOrder: SetSortOrder::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewSort as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
pub trait IWordBreaker_Impl: Sized {
    fn Init(&mut self, fquery: super::super::Foundation::BOOL, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn BreakText(&mut self, ptextsource: *mut TEXT_SOURCE, pwordsink: &::core::option::Option<IWordSink>, pphrasesink: &::core::option::Option<super::super::Storage::IndexServer::IPhraseSink>) -> ::windows::core::Result<()>;
    fn ComposePhrase(&mut self, pwcnoun: super::super::Foundation::PWSTR, cwcnoun: u32, pwcmodifier: super::super::Foundation::PWSTR, cwcmodifier: u32, ulattachmenttype: u32, pwcphrase: super::super::Foundation::PWSTR, pcwcphrase: *mut u32) -> ::windows::core::Result<()>;
    fn GetLicenseToUse(&mut self, ppwcslicense: *const *const u16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
impl IWordBreaker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWordBreaker_Impl, const OFFSET: isize>() -> IWordBreaker_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl, Impl: IWordBreaker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fquery: super::super::Foundation::BOOL, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&fquery), ::core::mem::transmute_copy(&ulmaxtokensize), ::core::mem::transmute_copy(&pflicense)).into()
        }
        unsafe extern "system" fn BreakText<Identity: ::windows::core::IUnknownImpl, Impl: IWordBreaker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptextsource: *mut TEXT_SOURCE, pwordsink: ::windows::core::RawPtr, pphrasesink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BreakText(::core::mem::transmute_copy(&ptextsource), ::core::mem::transmute(&pwordsink), ::core::mem::transmute(&pphrasesink)).into()
        }
        unsafe extern "system" fn ComposePhrase<Identity: ::windows::core::IUnknownImpl, Impl: IWordBreaker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcnoun: super::super::Foundation::PWSTR, cwcnoun: u32, pwcmodifier: super::super::Foundation::PWSTR, cwcmodifier: u32, ulattachmenttype: u32, pwcphrase: super::super::Foundation::PWSTR, pcwcphrase: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ComposePhrase(::core::mem::transmute_copy(&pwcnoun), ::core::mem::transmute_copy(&cwcnoun), ::core::mem::transmute_copy(&pwcmodifier), ::core::mem::transmute_copy(&cwcmodifier), ::core::mem::transmute_copy(&ulattachmenttype), ::core::mem::transmute_copy(&pwcphrase), ::core::mem::transmute_copy(&pcwcphrase)).into()
        }
        unsafe extern "system" fn GetLicenseToUse<Identity: ::windows::core::IUnknownImpl, Impl: IWordBreaker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwcslicense: *const *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLicenseToUse(::core::mem::transmute_copy(&ppwcslicense)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            BreakText: BreakText::<Identity, Impl, OFFSET>,
            ComposePhrase: ComposePhrase::<Identity, Impl, OFFSET>,
            GetLicenseToUse: GetLicenseToUse::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWordBreaker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWordFormSink_Impl: Sized {
    fn PutAltWord(&mut self, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32) -> ::windows::core::Result<()>;
    fn PutWord(&mut self, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWordFormSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWordFormSink_Impl, const OFFSET: isize>() -> IWordFormSink_Vtbl {
        unsafe extern "system" fn PutAltWord<Identity: ::windows::core::IUnknownImpl, Impl: IWordFormSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PutAltWord(::core::mem::transmute_copy(&pwcinbuf), ::core::mem::transmute_copy(&cwc)).into()
        }
        unsafe extern "system" fn PutWord<Identity: ::windows::core::IUnknownImpl, Impl: IWordFormSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PutWord(::core::mem::transmute_copy(&pwcinbuf), ::core::mem::transmute_copy(&cwc)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PutAltWord: PutAltWord::<Identity, Impl, OFFSET>,
            PutWord: PutWord::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWordFormSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
pub trait IWordSink_Impl: Sized {
    fn PutWord(&mut self, cwc: u32, pwcinbuf: super::super::Foundation::PWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> ::windows::core::Result<()>;
    fn PutAltWord(&mut self, cwc: u32, pwcinbuf: super::super::Foundation::PWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> ::windows::core::Result<()>;
    fn StartAltPhrase(&mut self) -> ::windows::core::Result<()>;
    fn EndAltPhrase(&mut self) -> ::windows::core::Result<()>;
    fn PutBreak(&mut self, breaktype: super::super::Storage::IndexServer::WORDREP_BREAK_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
impl IWordSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWordSink_Impl, const OFFSET: isize>() -> IWordSink_Vtbl {
        unsafe extern "system" fn PutWord<Identity: ::windows::core::IUnknownImpl, Impl: IWordSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cwc: u32, pwcinbuf: super::super::Foundation::PWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PutWord(::core::mem::transmute_copy(&cwc), ::core::mem::transmute_copy(&pwcinbuf), ::core::mem::transmute_copy(&cwcsrclen), ::core::mem::transmute_copy(&cwcsrcpos)).into()
        }
        unsafe extern "system" fn PutAltWord<Identity: ::windows::core::IUnknownImpl, Impl: IWordSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cwc: u32, pwcinbuf: super::super::Foundation::PWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PutAltWord(::core::mem::transmute_copy(&cwc), ::core::mem::transmute_copy(&pwcinbuf), ::core::mem::transmute_copy(&cwcsrclen), ::core::mem::transmute_copy(&cwcsrcpos)).into()
        }
        unsafe extern "system" fn StartAltPhrase<Identity: ::windows::core::IUnknownImpl, Impl: IWordSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartAltPhrase().into()
        }
        unsafe extern "system" fn EndAltPhrase<Identity: ::windows::core::IUnknownImpl, Impl: IWordSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndAltPhrase().into()
        }
        unsafe extern "system" fn PutBreak<Identity: ::windows::core::IUnknownImpl, Impl: IWordSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, breaktype: super::super::Storage::IndexServer::WORDREP_BREAK_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PutBreak(::core::mem::transmute_copy(&breaktype)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PutWord: PutWord::<Identity, Impl, OFFSET>,
            PutAltWord: PutAltWord::<Identity, Impl, OFFSET>,
            StartAltPhrase: StartAltPhrase::<Identity, Impl, OFFSET>,
            EndAltPhrase: EndAltPhrase::<Identity, Impl, OFFSET>,
            PutBreak: PutBreak::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWordSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait OLEDBSimpleProvider_Impl: Sized {
    fn getRowCount(&mut self) -> ::windows::core::Result<isize>;
    fn getColumnCount(&mut self) -> ::windows::core::Result<isize>;
    fn getRWStatus(&mut self, irow: isize, icolumn: isize) -> ::windows::core::Result<OSPRW>;
    fn getVariant(&mut self, irow: isize, icolumn: isize, format: OSPFORMAT) -> ::windows::core::Result<super::Com::VARIANT>;
    fn setVariant(&mut self, irow: isize, icolumn: isize, format: OSPFORMAT, var: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn getLocale(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn deleteRows(&mut self, irow: isize, crows: isize) -> ::windows::core::Result<isize>;
    fn insertRows(&mut self, irow: isize, crows: isize) -> ::windows::core::Result<isize>;
    fn find(&mut self, irowstart: isize, icolumn: isize, val: &super::Com::VARIANT, findflags: OSPFIND, comptype: OSPCOMP) -> ::windows::core::Result<isize>;
    fn addOLEDBSimpleProviderListener(&mut self, pospilistener: &::core::option::Option<OLEDBSimpleProviderListener>) -> ::windows::core::Result<()>;
    fn removeOLEDBSimpleProviderListener(&mut self, pospilistener: &::core::option::Option<OLEDBSimpleProviderListener>) -> ::windows::core::Result<()>;
    fn isAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn getEstimatedRows(&mut self) -> ::windows::core::Result<isize>;
    fn stopTransfer(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl OLEDBSimpleProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>() -> OLEDBSimpleProvider_Vtbl {
        unsafe extern "system" fn getRowCount<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcrows: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getRowCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcrows = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getColumnCount<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccolumns: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getColumnCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pccolumns = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getRWStatus<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize, prwstatus: *mut OSPRW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getRWStatus(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&icolumn)) {
                ::core::result::Result::Ok(ok__) => {
                    *prwstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getVariant<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize, format: OSPFORMAT, pvar: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getVariant(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&icolumn), ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setVariant<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize, format: OSPFORMAT, var: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setVariant(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&icolumn), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&var)).into()
        }
        unsafe extern "system" fn getLocale<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlocale: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getLocale() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlocale = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn deleteRows<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize, pcrowsdeleted: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).deleteRows(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcrowsdeleted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn insertRows<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize, pcrowsinserted: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).insertRows(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcrowsinserted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn find<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irowstart: isize, icolumn: isize, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>, findflags: OSPFIND, comptype: OSPCOMP, pirowfound: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).find(::core::mem::transmute_copy(&irowstart), ::core::mem::transmute_copy(&icolumn), ::core::mem::transmute_copy(&val), ::core::mem::transmute_copy(&findflags), ::core::mem::transmute_copy(&comptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pirowfound = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addOLEDBSimpleProviderListener<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pospilistener: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).addOLEDBSimpleProviderListener(::core::mem::transmute(&pospilistener)).into()
        }
        unsafe extern "system" fn removeOLEDBSimpleProviderListener<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pospilistener: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).removeOLEDBSimpleProviderListener(::core::mem::transmute(&pospilistener)).into()
        }
        unsafe extern "system" fn isAsync<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbasynch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).isAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *pbasynch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getEstimatedRows<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirows: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getEstimatedRows() {
                ::core::result::Result::Ok(ok__) => {
                    *pirows = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn stopTransfer<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).stopTransfer().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            getRowCount: getRowCount::<Identity, Impl, OFFSET>,
            getColumnCount: getColumnCount::<Identity, Impl, OFFSET>,
            getRWStatus: getRWStatus::<Identity, Impl, OFFSET>,
            getVariant: getVariant::<Identity, Impl, OFFSET>,
            setVariant: setVariant::<Identity, Impl, OFFSET>,
            getLocale: getLocale::<Identity, Impl, OFFSET>,
            deleteRows: deleteRows::<Identity, Impl, OFFSET>,
            insertRows: insertRows::<Identity, Impl, OFFSET>,
            find: find::<Identity, Impl, OFFSET>,
            addOLEDBSimpleProviderListener: addOLEDBSimpleProviderListener::<Identity, Impl, OFFSET>,
            removeOLEDBSimpleProviderListener: removeOLEDBSimpleProviderListener::<Identity, Impl, OFFSET>,
            isAsync: isAsync::<Identity, Impl, OFFSET>,
            getEstimatedRows: getEstimatedRows::<Identity, Impl, OFFSET>,
            stopTransfer: stopTransfer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<OLEDBSimpleProvider as ::windows::core::Interface>::IID
    }
}
pub trait OLEDBSimpleProviderListener_Impl: Sized {
    fn aboutToChangeCell(&mut self, irow: isize, icolumn: isize) -> ::windows::core::Result<()>;
    fn cellChanged(&mut self, irow: isize, icolumn: isize) -> ::windows::core::Result<()>;
    fn aboutToDeleteRows(&mut self, irow: isize, crows: isize) -> ::windows::core::Result<()>;
    fn deletedRows(&mut self, irow: isize, crows: isize) -> ::windows::core::Result<()>;
    fn aboutToInsertRows(&mut self, irow: isize, crows: isize) -> ::windows::core::Result<()>;
    fn insertedRows(&mut self, irow: isize, crows: isize) -> ::windows::core::Result<()>;
    fn rowsAvailable(&mut self, irow: isize, crows: isize) -> ::windows::core::Result<()>;
    fn transferComplete(&mut self, xfer: OSPXFER) -> ::windows::core::Result<()>;
}
impl OLEDBSimpleProviderListener_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: isize>() -> OLEDBSimpleProviderListener_Vtbl {
        unsafe extern "system" fn aboutToChangeCell<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).aboutToChangeCell(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&icolumn)).into()
        }
        unsafe extern "system" fn cellChanged<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).cellChanged(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&icolumn)).into()
        }
        unsafe extern "system" fn aboutToDeleteRows<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).aboutToDeleteRows(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)).into()
        }
        unsafe extern "system" fn deletedRows<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).deletedRows(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)).into()
        }
        unsafe extern "system" fn aboutToInsertRows<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).aboutToInsertRows(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)).into()
        }
        unsafe extern "system" fn insertedRows<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).insertedRows(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)).into()
        }
        unsafe extern "system" fn rowsAvailable<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).rowsAvailable(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)).into()
        }
        unsafe extern "system" fn transferComplete<Identity: ::windows::core::IUnknownImpl, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xfer: OSPXFER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).transferComplete(::core::mem::transmute_copy(&xfer)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            aboutToChangeCell: aboutToChangeCell::<Identity, Impl, OFFSET>,
            cellChanged: cellChanged::<Identity, Impl, OFFSET>,
            aboutToDeleteRows: aboutToDeleteRows::<Identity, Impl, OFFSET>,
            deletedRows: deletedRows::<Identity, Impl, OFFSET>,
            aboutToInsertRows: aboutToInsertRows::<Identity, Impl, OFFSET>,
            insertedRows: insertedRows::<Identity, Impl, OFFSET>,
            rowsAvailable: rowsAvailable::<Identity, Impl, OFFSET>,
            transferComplete: transferComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<OLEDBSimpleProviderListener as ::windows::core::Interface>::IID
    }
}
