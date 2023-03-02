#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait DataSource_Impl: Sized {
    fn getDataMember(&self, bstrdm: *const u16, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn getDataMemberName(&self, lindex: i32) -> ::windows::core::Result<*mut u16>;
    fn getDataMemberCount(&self) -> ::windows::core::Result<i32>;
    fn addDataSourceListener(&self, pdsl: ::core::option::Option<&DataSourceListener>) -> ::windows::core::Result<()>;
    fn removeDataSourceListener(&self, pdsl: ::core::option::Option<&DataSourceListener>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for DataSource {}
impl DataSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: DataSource_Impl, const OFFSET: isize>() -> DataSource_Vtbl {
        unsafe extern "system" fn getDataMember<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: DataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdm: *const u16, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getDataMember(::core::mem::transmute_copy(&bstrdm), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getDataMemberName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: DataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrdm: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getDataMemberName(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdm, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getDataMemberCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: DataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getDataMemberCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addDataSourceListener<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: DataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsl: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addDataSourceListener(::windows::core::from_raw_borrowed(&pdsl)).into()
        }
        unsafe extern "system" fn removeDataSourceListener<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: DataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsl: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeDataSourceListener(::windows::core::from_raw_borrowed(&pdsl)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            getDataMember: getDataMember::<Identity, Impl, OFFSET>,
            getDataMemberName: getDataMemberName::<Identity, Impl, OFFSET>,
            getDataMemberCount: getDataMemberCount::<Identity, Impl, OFFSET>,
            addDataSourceListener: addDataSourceListener::<Identity, Impl, OFFSET>,
            removeDataSourceListener: removeDataSourceListener::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DataSource as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait DataSourceListener_Impl: Sized {
    fn dataMemberChanged(&self, bstrdm: *const u16) -> ::windows::core::Result<()>;
    fn dataMemberAdded(&self, bstrdm: *const u16) -> ::windows::core::Result<()>;
    fn dataMemberRemoved(&self, bstrdm: *const u16) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for DataSourceListener {}
impl DataSourceListener_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: DataSourceListener_Impl, const OFFSET: isize>() -> DataSourceListener_Vtbl {
        unsafe extern "system" fn dataMemberChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: DataSourceListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdm: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.dataMemberChanged(::core::mem::transmute_copy(&bstrdm)).into()
        }
        unsafe extern "system" fn dataMemberAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: DataSourceListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdm: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.dataMemberAdded(::core::mem::transmute_copy(&bstrdm)).into()
        }
        unsafe extern "system" fn dataMemberRemoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: DataSourceListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdm: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.dataMemberRemoved(::core::mem::transmute_copy(&bstrdm)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            dataMemberChanged: dataMemberChanged::<Identity, Impl, OFFSET>,
            dataMemberAdded: dataMemberAdded::<Identity, Impl, OFFSET>,
            dataMemberRemoved: dataMemberRemoved::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DataSourceListener as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DataSourceObject_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for DataSourceObject {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DataSourceObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: DataSourceObject_Impl, const OFFSET: isize>() -> DataSourceObject_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DataSourceObject as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessor_Impl: Sized {
    fn AddRefAccessor(&self, haccessor: HACCESSOR, pcrefcount: *mut u32) -> ::windows::core::Result<()>;
    fn CreateAccessor(&self, dwaccessorflags: u32, cbindings: usize, rgbindings: *const DBBINDING, cbrowsize: usize, phaccessor: *mut HACCESSOR, rgstatus: *mut u32) -> ::windows::core::Result<()>;
    fn GetBindings(&self, haccessor: HACCESSOR, pdwaccessorflags: *mut u32, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> ::windows::core::Result<()>;
    fn ReleaseAccessor(&self, haccessor: HACCESSOR, pcrefcount: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAccessor {}
#[cfg(feature = "Win32_System_Com")]
impl IAccessor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccessor_Impl, const OFFSET: isize>() -> IAccessor_Vtbl {
        unsafe extern "system" fn AddRefAccessor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haccessor: HACCESSOR, pcrefcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRefAccessor(::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pcrefcount)).into()
        }
        unsafe extern "system" fn CreateAccessor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaccessorflags: u32, cbindings: usize, rgbindings: *const DBBINDING, cbrowsize: usize, phaccessor: *mut HACCESSOR, rgstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateAccessor(::core::mem::transmute_copy(&dwaccessorflags), ::core::mem::transmute_copy(&cbindings), ::core::mem::transmute_copy(&rgbindings), ::core::mem::transmute_copy(&cbrowsize), ::core::mem::transmute_copy(&phaccessor), ::core::mem::transmute_copy(&rgstatus)).into()
        }
        unsafe extern "system" fn GetBindings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haccessor: HACCESSOR, pdwaccessorflags: *mut u32, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBindings(::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdwaccessorflags), ::core::mem::transmute_copy(&pcbindings), ::core::mem::transmute_copy(&prgbindings)).into()
        }
        unsafe extern "system" fn ReleaseAccessor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haccessor: HACCESSOR, pcrefcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseAccessor(::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pcrefcount)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRefAccessor: AddRefAccessor::<Identity, Impl, OFFSET>,
            CreateAccessor: CreateAccessor::<Identity, Impl, OFFSET>,
            GetBindings: GetBindings::<Identity, Impl, OFFSET>,
            ReleaseAccessor: ReleaseAccessor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessor as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAlterIndex_Impl: Sized {
    fn AlterIndex(&self, ptableid: *mut super::super::Storage::IndexServer::DBID, pindexid: *mut super::super::Storage::IndexServer::DBID, pnewindexid: *mut super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAlterIndex {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAlterIndex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAlterIndex_Impl, const OFFSET: isize>() -> IAlterIndex_Vtbl {
        unsafe extern "system" fn AlterIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAlterIndex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, pindexid: *mut super::super::Storage::IndexServer::DBID, pnewindexid: *mut super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AlterIndex(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pindexid), ::core::mem::transmute_copy(&pnewindexid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AlterIndex: AlterIndex::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAlterIndex as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAlterTable_Impl: Sized {
    fn AlterColumn(&self, ptableid: *mut super::super::Storage::IndexServer::DBID, pcolumnid: *mut super::super::Storage::IndexServer::DBID, dwcolumndescflags: u32, pcolumndesc: *mut DBCOLUMNDESC) -> ::windows::core::Result<()>;
    fn AlterTable(&self, ptableid: *mut super::super::Storage::IndexServer::DBID, pnewtableid: *mut super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAlterTable {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAlterTable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAlterTable_Impl, const OFFSET: isize>() -> IAlterTable_Vtbl {
        unsafe extern "system" fn AlterColumn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAlterTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, pcolumnid: *mut super::super::Storage::IndexServer::DBID, dwcolumndescflags: u32, pcolumndesc: *mut DBCOLUMNDESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AlterColumn(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pcolumnid), ::core::mem::transmute_copy(&dwcolumndescflags), ::core::mem::transmute_copy(&pcolumndesc)).into()
        }
        unsafe extern "system" fn AlterTable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAlterTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, pnewtableid: *mut super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AlterTable(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pnewtableid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AlterColumn: AlterColumn::<Identity, Impl, OFFSET>,
            AlterTable: AlterTable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAlterTable as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IBindResource_Impl: Sized {
    fn Bind(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, pwszurl: &::windows::core::PCWSTR, dwbindurlflags: u32, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pauthenticate: ::core::option::Option<&super::Com::IAuthenticate>, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IBindResource {}
#[cfg(feature = "Win32_System_Com")]
impl IBindResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindResource_Impl, const OFFSET: isize>() -> IBindResource_Vtbl {
        unsafe extern "system" fn Bind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, dwbindurlflags: u32, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pauthenticate: *mut ::core::ffi::c_void, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Bind(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&dwbindurlflags), ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&riid), ::windows::core::from_raw_borrowed(&pauthenticate), ::core::mem::transmute_copy(&pimplsession), ::core::mem::transmute_copy(&pdwbindstatus), ::core::mem::transmute_copy(&ppunk)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Bind: Bind::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindResource as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IChapteredRowset_Impl: Sized {
    fn AddRefChapter(&self, hchapter: usize, pcrefcount: *mut u32) -> ::windows::core::Result<()>;
    fn ReleaseChapter(&self, hchapter: usize, pcrefcount: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IChapteredRowset {}
impl IChapteredRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChapteredRowset_Impl, const OFFSET: isize>() -> IChapteredRowset_Vtbl {
        unsafe extern "system" fn AddRefChapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChapteredRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, pcrefcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRefChapter(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&pcrefcount)).into()
        }
        unsafe extern "system" fn ReleaseChapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChapteredRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, pcrefcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseChapter(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&pcrefcount)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRefChapter: AddRefChapter::<Identity, Impl, OFFSET>,
            ReleaseChapter: ReleaseChapter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChapteredRowset as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IColumnMapper_Impl: Sized {
    fn GetPropInfoFromName(&self, wcspropname: &::windows::core::PCWSTR, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows::core::Result<()>;
    fn GetPropInfoFromId(&self, ppropid: *const super::super::Storage::IndexServer::DBID, pwcsname: *mut *mut u16, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows::core::Result<()>;
    fn EnumPropInfo(&self, ientry: u32, pwcsname: *const *const u16, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows::core::Result<()>;
    fn IsMapUpToDate(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::windows::core::RuntimeName for IColumnMapper {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IColumnMapper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IColumnMapper_Impl, const OFFSET: isize>() -> IColumnMapper_Vtbl {
        unsafe extern "system" fn GetPropInfoFromName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IColumnMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wcspropname: ::windows::core::PCWSTR, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropInfoFromName(::core::mem::transmute(&wcspropname), ::core::mem::transmute_copy(&pppropid), ::core::mem::transmute_copy(&pproptype), ::core::mem::transmute_copy(&puiwidth)).into()
        }
        unsafe extern "system" fn GetPropInfoFromId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IColumnMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropid: *const super::super::Storage::IndexServer::DBID, pwcsname: *mut *mut u16, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropInfoFromId(::core::mem::transmute_copy(&ppropid), ::core::mem::transmute_copy(&pwcsname), ::core::mem::transmute_copy(&pproptype), ::core::mem::transmute_copy(&puiwidth)).into()
        }
        unsafe extern "system" fn EnumPropInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IColumnMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ientry: u32, pwcsname: *const *const u16, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumPropInfo(::core::mem::transmute_copy(&ientry), ::core::mem::transmute_copy(&pwcsname), ::core::mem::transmute_copy(&pppropid), ::core::mem::transmute_copy(&pproptype), ::core::mem::transmute_copy(&puiwidth)).into()
        }
        unsafe extern "system" fn IsMapUpToDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IColumnMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsMapUpToDate().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropInfoFromName: GetPropInfoFromName::<Identity, Impl, OFFSET>,
            GetPropInfoFromId: GetPropInfoFromId::<Identity, Impl, OFFSET>,
            EnumPropInfo: EnumPropInfo::<Identity, Impl, OFFSET>,
            IsMapUpToDate: IsMapUpToDate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColumnMapper as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IColumnMapperCreator_Impl: Sized {
    fn GetColumnMapper(&self, wcsmachinename: &::windows::core::PCWSTR, wcscatalogname: &::windows::core::PCWSTR) -> ::windows::core::Result<IColumnMapper>;
}
impl ::windows::core::RuntimeName for IColumnMapperCreator {}
impl IColumnMapperCreator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IColumnMapperCreator_Impl, const OFFSET: isize>() -> IColumnMapperCreator_Vtbl {
        unsafe extern "system" fn GetColumnMapper<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IColumnMapperCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wcsmachinename: ::windows::core::PCWSTR, wcscatalogname: ::windows::core::PCWSTR, ppcolumnmapper: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColumnMapper(::core::mem::transmute(&wcsmachinename), ::core::mem::transmute(&wcscatalogname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolumnmapper, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetColumnMapper: GetColumnMapper::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColumnMapperCreator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait IColumnsInfo_Impl: Sized {
    fn GetColumnInfo(&self, pccolumns: *mut usize, prginfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
    fn MapColumnIDs(&self, ccolumnids: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgcolumns: *mut usize) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IColumnsInfo {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl IColumnsInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IColumnsInfo_Impl, const OFFSET: isize>() -> IColumnsInfo_Vtbl {
        unsafe extern "system" fn GetColumnInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IColumnsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccolumns: *mut usize, prginfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColumnInfo(::core::mem::transmute_copy(&pccolumns), ::core::mem::transmute_copy(&prginfo), ::core::mem::transmute_copy(&ppstringsbuffer)).into()
        }
        unsafe extern "system" fn MapColumnIDs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IColumnsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolumnids: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgcolumns: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MapColumnIDs(::core::mem::transmute_copy(&ccolumnids), ::core::mem::transmute_copy(&rgcolumnids), ::core::mem::transmute_copy(&rgcolumns)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetColumnInfo: GetColumnInfo::<Identity, Impl, OFFSET>,
            MapColumnIDs: MapColumnIDs::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColumnsInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait IColumnsInfo2_Impl: Sized + IColumnsInfo_Impl {
    fn GetRestrictedColumnInfo(&self, ccolumnidmasks: usize, rgcolumnidmasks: *const super::super::Storage::IndexServer::DBID, dwflags: u32, pccolumns: *mut usize, prgcolumnids: *mut *mut super::super::Storage::IndexServer::DBID, prgcolumninfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IColumnsInfo2 {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl IColumnsInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IColumnsInfo2_Impl, const OFFSET: isize>() -> IColumnsInfo2_Vtbl {
        unsafe extern "system" fn GetRestrictedColumnInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IColumnsInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolumnidmasks: usize, rgcolumnidmasks: *const super::super::Storage::IndexServer::DBID, dwflags: u32, pccolumns: *mut usize, prgcolumnids: *mut *mut super::super::Storage::IndexServer::DBID, prgcolumninfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRestrictedColumnInfo(::core::mem::transmute_copy(&ccolumnidmasks), ::core::mem::transmute_copy(&rgcolumnidmasks), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pccolumns), ::core::mem::transmute_copy(&prgcolumnids), ::core::mem::transmute_copy(&prgcolumninfo), ::core::mem::transmute_copy(&ppstringsbuffer)).into()
        }
        Self { base__: IColumnsInfo_Vtbl::new::<Identity, Impl, OFFSET>(), GetRestrictedColumnInfo: GetRestrictedColumnInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColumnsInfo2 as ::windows::core::ComInterface>::IID || iid == &<IColumnsInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IColumnsRowset_Impl: Sized {
    fn GetAvailableColumns(&self, pcoptcolumns: *mut usize, prgoptcolumns: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
    fn GetColumnsRowset(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, coptcolumns: usize, rgoptcolumns: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppcolrowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IColumnsRowset {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IColumnsRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IColumnsRowset_Impl, const OFFSET: isize>() -> IColumnsRowset_Vtbl {
        unsafe extern "system" fn GetAvailableColumns<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IColumnsRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoptcolumns: *mut usize, prgoptcolumns: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAvailableColumns(::core::mem::transmute_copy(&pcoptcolumns), ::core::mem::transmute_copy(&prgoptcolumns)).into()
        }
        unsafe extern "system" fn GetColumnsRowset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IColumnsRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, coptcolumns: usize, rgoptcolumns: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppcolrowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColumnsRowset(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&coptcolumns), ::core::mem::transmute_copy(&rgoptcolumns), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&ppcolrowset)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAvailableColumns: GetAvailableColumns::<Identity, Impl, OFFSET>,
            GetColumnsRowset: GetColumnsRowset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColumnsRowset as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait ICommand_Impl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn Execute(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, riid: *const ::windows::core::GUID, pparams: *mut DBPARAMS, pcrowsaffected: *mut isize, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetDBSession(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ::windows::core::RuntimeName for ICommand {}
impl ICommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommand_Impl, const OFFSET: isize>() -> ICommand_Vtbl {
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        unsafe extern "system" fn Execute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pparams: *mut DBPARAMS, pcrowsaffected: *mut isize, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Execute(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&pcrowsaffected), ::core::mem::transmute_copy(&pprowset)).into()
        }
        unsafe extern "system" fn GetDBSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDBSession(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Execute: Execute::<Identity, Impl, OFFSET>,
            GetDBSession: GetDBSession::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommand as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait ICommandCost_Impl: Sized {
    fn GetAccumulatedCost(&self, pwszrowsetname: &::windows::core::PCWSTR, pccostlimits: *mut u32, prgcostlimits: *mut *mut DBCOST) -> ::windows::core::Result<()>;
    fn GetCostEstimate(&self, pwszrowsetname: &::windows::core::PCWSTR, pccostestimates: *mut u32, prgcostestimates: *mut DBCOST) -> ::windows::core::Result<()>;
    fn GetCostGoals(&self, pwszrowsetname: &::windows::core::PCWSTR, pccostgoals: *mut u32, prgcostgoals: *mut DBCOST) -> ::windows::core::Result<()>;
    fn GetCostLimits(&self, pwszrowsetname: &::windows::core::PCWSTR, pccostlimits: *mut u32, prgcostlimits: *mut DBCOST) -> ::windows::core::Result<()>;
    fn SetCostGoals(&self, pwszrowsetname: &::windows::core::PCWSTR, ccostgoals: u32, rgcostgoals: *const DBCOST) -> ::windows::core::Result<()>;
    fn SetCostLimits(&self, pwszrowsetname: &::windows::core::PCWSTR, ccostlimits: u32, prgcostlimits: *mut DBCOST, dwexecutionflags: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICommandCost {}
impl ICommandCost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandCost_Impl, const OFFSET: isize>() -> ICommandCost_Vtbl {
        unsafe extern "system" fn GetAccumulatedCost<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandCost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszrowsetname: ::windows::core::PCWSTR, pccostlimits: *mut u32, prgcostlimits: *mut *mut DBCOST) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAccumulatedCost(::core::mem::transmute(&pwszrowsetname), ::core::mem::transmute_copy(&pccostlimits), ::core::mem::transmute_copy(&prgcostlimits)).into()
        }
        unsafe extern "system" fn GetCostEstimate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandCost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszrowsetname: ::windows::core::PCWSTR, pccostestimates: *mut u32, prgcostestimates: *mut DBCOST) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCostEstimate(::core::mem::transmute(&pwszrowsetname), ::core::mem::transmute_copy(&pccostestimates), ::core::mem::transmute_copy(&prgcostestimates)).into()
        }
        unsafe extern "system" fn GetCostGoals<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandCost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszrowsetname: ::windows::core::PCWSTR, pccostgoals: *mut u32, prgcostgoals: *mut DBCOST) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCostGoals(::core::mem::transmute(&pwszrowsetname), ::core::mem::transmute_copy(&pccostgoals), ::core::mem::transmute_copy(&prgcostgoals)).into()
        }
        unsafe extern "system" fn GetCostLimits<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandCost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszrowsetname: ::windows::core::PCWSTR, pccostlimits: *mut u32, prgcostlimits: *mut DBCOST) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCostLimits(::core::mem::transmute(&pwszrowsetname), ::core::mem::transmute_copy(&pccostlimits), ::core::mem::transmute_copy(&prgcostlimits)).into()
        }
        unsafe extern "system" fn SetCostGoals<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandCost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszrowsetname: ::windows::core::PCWSTR, ccostgoals: u32, rgcostgoals: *const DBCOST) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCostGoals(::core::mem::transmute(&pwszrowsetname), ::core::mem::transmute_copy(&ccostgoals), ::core::mem::transmute_copy(&rgcostgoals)).into()
        }
        unsafe extern "system" fn SetCostLimits<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandCost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszrowsetname: ::windows::core::PCWSTR, ccostlimits: u32, prgcostlimits: *mut DBCOST, dwexecutionflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCostLimits(::core::mem::transmute(&pwszrowsetname), ::core::mem::transmute_copy(&ccostlimits), ::core::mem::transmute_copy(&prgcostlimits), ::core::mem::transmute_copy(&dwexecutionflags)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAccumulatedCost: GetAccumulatedCost::<Identity, Impl, OFFSET>,
            GetCostEstimate: GetCostEstimate::<Identity, Impl, OFFSET>,
            GetCostGoals: GetCostGoals::<Identity, Impl, OFFSET>,
            GetCostLimits: GetCostLimits::<Identity, Impl, OFFSET>,
            SetCostGoals: SetCostGoals::<Identity, Impl, OFFSET>,
            SetCostLimits: SetCostLimits::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandCost as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait ICommandPersist_Impl: Sized {
    fn DeleteCommand(&self, pcommandid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
    fn GetCurrentCommand(&self, ppcommandid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
    fn LoadCommand(&self, pcommandid: *mut super::super::Storage::IndexServer::DBID, dwflags: u32) -> ::windows::core::Result<()>;
    fn SaveCommand(&self, pcommandid: *mut super::super::Storage::IndexServer::DBID, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::windows::core::RuntimeName for ICommandPersist {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ICommandPersist_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandPersist_Impl, const OFFSET: isize>() -> ICommandPersist_Vtbl {
        unsafe extern "system" fn DeleteCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandPersist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommandid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteCommand(::core::mem::transmute_copy(&pcommandid)).into()
        }
        unsafe extern "system" fn GetCurrentCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandPersist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcommandid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentCommand(::core::mem::transmute_copy(&ppcommandid)).into()
        }
        unsafe extern "system" fn LoadCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandPersist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommandid: *mut super::super::Storage::IndexServer::DBID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadCommand(::core::mem::transmute_copy(&pcommandid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SaveCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandPersist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommandid: *mut super::super::Storage::IndexServer::DBID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveCommand(::core::mem::transmute_copy(&pcommandid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeleteCommand: DeleteCommand::<Identity, Impl, OFFSET>,
            GetCurrentCommand: GetCurrentCommand::<Identity, Impl, OFFSET>,
            LoadCommand: LoadCommand::<Identity, Impl, OFFSET>,
            SaveCommand: SaveCommand::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandPersist as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait ICommandPrepare_Impl: Sized {
    fn Prepare(&self, cexpectedruns: u32) -> ::windows::core::Result<()>;
    fn Unprepare(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICommandPrepare {}
impl ICommandPrepare_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandPrepare_Impl, const OFFSET: isize>() -> ICommandPrepare_Vtbl {
        unsafe extern "system" fn Prepare<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandPrepare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexpectedruns: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Prepare(::core::mem::transmute_copy(&cexpectedruns)).into()
        }
        unsafe extern "system" fn Unprepare<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandPrepare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unprepare().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Prepare: Prepare::<Identity, Impl, OFFSET>,
            Unprepare: Unprepare::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandPrepare as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICommandProperties_Impl: Sized {
    fn GetProperties(&self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::Result<()>;
    fn SetProperties(&self, cpropertysets: u32, rgpropertysets: *const DBPROPSET) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ICommandProperties {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICommandProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandProperties_Impl, const OFFSET: isize>() -> ICommandProperties_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProperties(::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)).into()
        }
        unsafe extern "system" fn SetProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *const DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperties(::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            SetProperties: SetProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandProperties as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait ICommandStream_Impl: Sized {
    fn GetCommandStream(&self, piid: *mut ::windows::core::GUID, pguiddialect: *mut ::windows::core::GUID, ppcommandstream: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetCommandStream(&self, riid: *const ::windows::core::GUID, rguiddialect: *const ::windows::core::GUID, pcommandstream: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICommandStream {}
impl ICommandStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandStream_Impl, const OFFSET: isize>() -> ICommandStream_Vtbl {
        unsafe extern "system" fn GetCommandStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID, pguiddialect: *mut ::windows::core::GUID, ppcommandstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCommandStream(::core::mem::transmute_copy(&piid), ::core::mem::transmute_copy(&pguiddialect), ::core::mem::transmute_copy(&ppcommandstream)).into()
        }
        unsafe extern "system" fn SetCommandStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rguiddialect: *const ::windows::core::GUID, pcommandstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCommandStream(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&rguiddialect), ::windows::core::from_raw_borrowed(&pcommandstream)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCommandStream: GetCommandStream::<Identity, Impl, OFFSET>,
            SetCommandStream: SetCommandStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandStream as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait ICommandText_Impl: Sized + ICommand_Impl {
    fn GetCommandText(&self, pguiddialect: *mut ::windows::core::GUID, ppwszcommand: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn SetCommandText(&self, rguiddialect: *const ::windows::core::GUID, pwszcommand: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICommandText {}
impl ICommandText_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandText_Impl, const OFFSET: isize>() -> ICommandText_Vtbl {
        unsafe extern "system" fn GetCommandText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguiddialect: *mut ::windows::core::GUID, ppwszcommand: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCommandText(::core::mem::transmute_copy(&pguiddialect), ::core::mem::transmute_copy(&ppwszcommand)).into()
        }
        unsafe extern "system" fn SetCommandText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguiddialect: *const ::windows::core::GUID, pwszcommand: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCommandText(::core::mem::transmute_copy(&rguiddialect), ::core::mem::transmute(&pwszcommand)).into()
        }
        Self {
            base__: ICommand_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCommandText: GetCommandText::<Identity, Impl, OFFSET>,
            SetCommandText: SetCommandText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandText as ::windows::core::ComInterface>::IID || iid == &<ICommand as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait ICommandValidate_Impl: Sized {
    fn ValidateCompletely(&self) -> ::windows::core::Result<()>;
    fn ValidateSyntax(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICommandValidate {}
impl ICommandValidate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandValidate_Impl, const OFFSET: isize>() -> ICommandValidate_Vtbl {
        unsafe extern "system" fn ValidateCompletely<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ValidateCompletely().into()
        }
        unsafe extern "system" fn ValidateSyntax<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ValidateSyntax().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ValidateCompletely: ValidateCompletely::<Identity, Impl, OFFSET>,
            ValidateSyntax: ValidateSyntax::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandValidate as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait ICommandWithParameters_Impl: Sized {
    fn GetParameterInfo(&self, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
    fn MapParameterNames(&self, cparamnames: usize, rgparamnames: *const ::windows::core::PCWSTR, rgparamordinals: *mut isize) -> ::windows::core::Result<()>;
    fn SetParameterInfo(&self, cparams: usize, rgparamordinals: *const usize, rgparambindinfo: *const DBPARAMBINDINFO) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ICommandWithParameters {}
#[cfg(feature = "Win32_System_Com")]
impl ICommandWithParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandWithParameters_Impl, const OFFSET: isize>() -> ICommandWithParameters_Vtbl {
        unsafe extern "system" fn GetParameterInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandWithParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetParameterInfo(::core::mem::transmute_copy(&pcparams), ::core::mem::transmute_copy(&prgparaminfo), ::core::mem::transmute_copy(&ppnamesbuffer)).into()
        }
        unsafe extern "system" fn MapParameterNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandWithParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cparamnames: usize, rgparamnames: *const ::windows::core::PCWSTR, rgparamordinals: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MapParameterNames(::core::mem::transmute_copy(&cparamnames), ::core::mem::transmute_copy(&rgparamnames), ::core::mem::transmute_copy(&rgparamordinals)).into()
        }
        unsafe extern "system" fn SetParameterInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommandWithParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cparams: usize, rgparamordinals: *const usize, rgparambindinfo: *const DBPARAMBINDINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParameterInfo(::core::mem::transmute_copy(&cparams), ::core::mem::transmute_copy(&rgparamordinals), ::core::mem::transmute_copy(&rgparambindinfo)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetParameterInfo: GetParameterInfo::<Identity, Impl, OFFSET>,
            MapParameterNames: MapParameterNames::<Identity, Impl, OFFSET>,
            SetParameterInfo: SetParameterInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandWithParameters as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
pub trait ICondition_Impl: Sized + super::Com::IPersistStream_Impl {
    fn GetConditionType(&self) -> ::windows::core::Result<Common::CONDITION_TYPE>;
    fn GetSubConditions(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetComparisonInfo(&self, ppszpropertyname: *mut ::windows::core::PWSTR, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetValueType(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetValueNormalization(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetInputTerms(&self, pppropertyterm: *mut ::core::option::Option<IRichChunk>, ppoperationterm: *mut ::core::option::Option<IRichChunk>, ppvalueterm: *mut ::core::option::Option<IRichChunk>) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<ICondition>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl ::windows::core::RuntimeName for ICondition {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl ICondition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICondition_Impl, const OFFSET: isize>() -> ICondition_Vtbl {
        unsafe extern "system" fn GetConditionType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnodetype: *mut Common::CONDITION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConditionType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnodetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubConditions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSubConditions(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetComparisonInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpropertyname: *mut ::windows::core::PWSTR, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetComparisonInfo(::core::mem::transmute_copy(&ppszpropertyname), ::core::mem::transmute_copy(&pcop), ::core::mem::transmute_copy(&ppropvar)).into()
        }
        unsafe extern "system" fn GetValueType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszvaluetypename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValueType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszvaluetypename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueNormalization<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsznormalization: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValueNormalization() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsznormalization, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTerms<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyterm: *mut *mut ::core::ffi::c_void, ppoperationterm: *mut *mut ::core::ffi::c_void, ppvalueterm: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInputTerms(::core::mem::transmute_copy(&pppropertyterm), ::core::mem::transmute_copy(&ppoperationterm), ::core::mem::transmute_copy(&ppvalueterm)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IPersistStream_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<ICondition as ::windows::core::ComInterface>::IID || iid == &<super::Com::IPersist as ::windows::core::ComInterface>::IID || iid == &<super::Com::IPersistStream as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ICondition2_Impl: Sized + ICondition_Impl {
    fn GetLocale(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetLeafConditionInfo(&self, ppropkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows::core::RuntimeName for ICondition2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ICondition2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICondition2_Impl, const OFFSET: isize>() -> ICondition2_Vtbl {
        unsafe extern "system" fn GetLocale<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICondition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszlocalename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLocale() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszlocalename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLeafConditionInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICondition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLeafConditionInfo(::core::mem::transmute_copy(&ppropkey), ::core::mem::transmute_copy(&pcop), ::core::mem::transmute_copy(&ppropvar)).into()
        }
        Self {
            base__: ICondition_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLocale: GetLocale::<Identity, Impl, OFFSET>,
            GetLeafConditionInfo: GetLeafConditionInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICondition2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IPersist as ::windows::core::ComInterface>::IID || iid == &<super::Com::IPersistStream as ::windows::core::ComInterface>::IID || iid == &<ICondition as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
pub trait IConditionFactory_Impl: Sized {
    fn MakeNot(&self, pcsub: ::core::option::Option<&ICondition>, fsimplify: super::super::Foundation::BOOL) -> ::windows::core::Result<ICondition>;
    fn MakeAndOr(&self, ct: Common::CONDITION_TYPE, peusubs: ::core::option::Option<&super::Com::IEnumUnknown>, fsimplify: super::super::Foundation::BOOL) -> ::windows::core::Result<ICondition>;
    fn MakeLeaf(&self, pszpropertyname: &::windows::core::PCWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: &::windows::core::PCWSTR, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, ppropertynameterm: ::core::option::Option<&IRichChunk>, poperationterm: ::core::option::Option<&IRichChunk>, pvalueterm: ::core::option::Option<&IRichChunk>, fexpand: super::super::Foundation::BOOL) -> ::windows::core::Result<ICondition>;
    fn Resolve(&self, pc: ::core::option::Option<&ICondition>, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<ICondition>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl ::windows::core::RuntimeName for IConditionFactory {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl IConditionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionFactory_Impl, const OFFSET: isize>() -> IConditionFactory_Vtbl {
        unsafe extern "system" fn MakeNot<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcsub: *mut ::core::ffi::c_void, fsimplify: super::super::Foundation::BOOL, ppcresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MakeNot(::windows::core::from_raw_borrowed(&pcsub), ::core::mem::transmute_copy(&fsimplify)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeAndOr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ct: Common::CONDITION_TYPE, peusubs: *mut ::core::ffi::c_void, fsimplify: super::super::Foundation::BOOL, ppcresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MakeAndOr(::core::mem::transmute_copy(&ct), ::windows::core::from_raw_borrowed(&peusubs), ::core::mem::transmute_copy(&fsimplify)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeLeaf<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PCWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: ::windows::core::PCWSTR, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, ppropertynameterm: *mut ::core::ffi::c_void, poperationterm: *mut ::core::ffi::c_void, pvalueterm: *mut ::core::ffi::c_void, fexpand: super::super::Foundation::BOOL, ppcresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MakeLeaf(::core::mem::transmute(&pszpropertyname), ::core::mem::transmute_copy(&cop), ::core::mem::transmute(&pszvaluetype), ::core::mem::transmute_copy(&ppropvar), ::windows::core::from_raw_borrowed(&ppropertynameterm), ::windows::core::from_raw_borrowed(&poperationterm), ::windows::core::from_raw_borrowed(&pvalueterm), ::core::mem::transmute_copy(&fexpand)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resolve<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pc: *mut ::core::ffi::c_void, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME, ppcresolved: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Resolve(::windows::core::from_raw_borrowed(&pc), ::core::mem::transmute_copy(&sqro), ::core::mem::transmute_copy(&pstreferencetime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcresolved, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MakeNot: MakeNot::<Identity, Impl, OFFSET>,
            MakeAndOr: MakeAndOr::<Identity, Impl, OFFSET>,
            MakeLeaf: MakeLeaf::<Identity, Impl, OFFSET>,
            Resolve: Resolve::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConditionFactory as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"Win32_UI_Shell_Common\"`, `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IConditionFactory2_Impl: Sized + IConditionFactory_Impl {
    fn CreateTrueFalse(&self, fval: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateNegation(&self, pcsub: ::core::option::Option<&ICondition>, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateCompoundFromObjectArray(&self, ct: Common::CONDITION_TYPE, poasubs: ::core::option::Option<&super::super::UI::Shell::Common::IObjectArray>, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateCompoundFromArray(&self, ct: Common::CONDITION_TYPE, ppcondsubs: *const ::core::option::Option<ICondition>, csubs: u32, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateStringLeaf(&self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, pszvalue: &::windows::core::PCWSTR, pszlocalename: &::windows::core::PCWSTR, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateIntegerLeaf(&self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, lvalue: i32, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateBooleanLeaf(&self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, fvalue: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateLeaf(&self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, propvar: *const super::Com::StructuredStorage::PROPVARIANT, pszsemantictype: &::windows::core::PCWSTR, pszlocalename: &::windows::core::PCWSTR, ppropertynameterm: ::core::option::Option<&IRichChunk>, poperationterm: ::core::option::Option<&IRichChunk>, pvalueterm: ::core::option::Option<&IRichChunk>, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ResolveCondition(&self, pc: ::core::option::Option<&ICondition>, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows::core::RuntimeName for IConditionFactory2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IConditionFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: isize>() -> IConditionFactory2_Vtbl {
        unsafe extern "system" fn CreateTrueFalse<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fval: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateTrueFalse(::core::mem::transmute_copy(&fval), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateNegation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcsub: *mut ::core::ffi::c_void, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateNegation(::windows::core::from_raw_borrowed(&pcsub), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateCompoundFromObjectArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ct: Common::CONDITION_TYPE, poasubs: *mut ::core::ffi::c_void, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateCompoundFromObjectArray(::core::mem::transmute_copy(&ct), ::windows::core::from_raw_borrowed(&poasubs), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateCompoundFromArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ct: Common::CONDITION_TYPE, ppcondsubs: *const *mut ::core::ffi::c_void, csubs: u32, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateCompoundFromArray(::core::mem::transmute_copy(&ct), ::core::mem::transmute_copy(&ppcondsubs), ::core::mem::transmute_copy(&csubs), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateStringLeaf<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, pszvalue: ::windows::core::PCWSTR, pszlocalename: ::windows::core::PCWSTR, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateStringLeaf(::core::mem::transmute_copy(&propkey), ::core::mem::transmute_copy(&cop), ::core::mem::transmute(&pszvalue), ::core::mem::transmute(&pszlocalename), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateIntegerLeaf<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, lvalue: i32, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateIntegerLeaf(::core::mem::transmute_copy(&propkey), ::core::mem::transmute_copy(&cop), ::core::mem::transmute_copy(&lvalue), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateBooleanLeaf<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, fvalue: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateBooleanLeaf(::core::mem::transmute_copy(&propkey), ::core::mem::transmute_copy(&cop), ::core::mem::transmute_copy(&fvalue), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateLeaf<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, propvar: *const super::Com::StructuredStorage::PROPVARIANT, pszsemantictype: ::windows::core::PCWSTR, pszlocalename: ::windows::core::PCWSTR, ppropertynameterm: *mut ::core::ffi::c_void, poperationterm: *mut ::core::ffi::c_void, pvalueterm: *mut ::core::ffi::c_void, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateLeaf(::core::mem::transmute_copy(&propkey), ::core::mem::transmute_copy(&cop), ::core::mem::transmute_copy(&propvar), ::core::mem::transmute(&pszsemantictype), ::core::mem::transmute(&pszlocalename), ::windows::core::from_raw_borrowed(&ppropertynameterm), ::windows::core::from_raw_borrowed(&poperationterm), ::windows::core::from_raw_borrowed(&pvalueterm), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv))
                .into()
        }
        unsafe extern "system" fn ResolveCondition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pc: *mut ::core::ffi::c_void, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResolveCondition(::windows::core::from_raw_borrowed(&pc), ::core::mem::transmute_copy(&sqro), ::core::mem::transmute_copy(&pstreferencetime), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: IConditionFactory_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IConditionFactory2 as ::windows::core::ComInterface>::IID || iid == &<IConditionFactory as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
pub trait IConditionGenerator_Impl: Sized {
    fn Initialize(&self, pschemaprovider: ::core::option::Option<&ISchemaProvider>) -> ::windows::core::Result<()>;
    fn RecognizeNamedEntities(&self, pszinputstring: &::windows::core::PCWSTR, lciduserlocale: u32, ptokencollection: ::core::option::Option<&ITokenCollection>, pnamedentities: ::core::option::Option<&INamedEntityCollector>) -> ::windows::core::Result<()>;
    fn GenerateForLeaf(&self, pconditionfactory: ::core::option::Option<&IConditionFactory>, pszpropertyname: &::windows::core::PCWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: &::windows::core::PCWSTR, pszvalue: &::windows::core::PCWSTR, pszvalue2: &::windows::core::PCWSTR, ppropertynameterm: ::core::option::Option<&IRichChunk>, poperationterm: ::core::option::Option<&IRichChunk>, pvalueterm: ::core::option::Option<&IRichChunk>, automaticwildcard: super::super::Foundation::BOOL, pnostringquery: *mut super::super::Foundation::BOOL, ppqueryexpression: *mut ::core::option::Option<ICondition>) -> ::windows::core::Result<()>;
    fn DefaultPhrase(&self, pszvaluetype: &::windows::core::PCWSTR, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, fuseenglish: super::super::Foundation::BOOL, ppszphrase: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl ::windows::core::RuntimeName for IConditionGenerator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl IConditionGenerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionGenerator_Impl, const OFFSET: isize>() -> IConditionGenerator_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pschemaprovider: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows::core::from_raw_borrowed(&pschemaprovider)).into()
        }
        unsafe extern "system" fn RecognizeNamedEntities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszinputstring: ::windows::core::PCWSTR, lciduserlocale: u32, ptokencollection: *mut ::core::ffi::c_void, pnamedentities: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RecognizeNamedEntities(::core::mem::transmute(&pszinputstring), ::core::mem::transmute_copy(&lciduserlocale), ::windows::core::from_raw_borrowed(&ptokencollection), ::windows::core::from_raw_borrowed(&pnamedentities)).into()
        }
        unsafe extern "system" fn GenerateForLeaf<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconditionfactory: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PCWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: ::windows::core::PCWSTR, pszvalue: ::windows::core::PCWSTR, pszvalue2: ::windows::core::PCWSTR, ppropertynameterm: *mut ::core::ffi::c_void, poperationterm: *mut ::core::ffi::c_void, pvalueterm: *mut ::core::ffi::c_void, automaticwildcard: super::super::Foundation::BOOL, pnostringquery: *mut super::super::Foundation::BOOL, ppqueryexpression: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GenerateForLeaf(
                ::windows::core::from_raw_borrowed(&pconditionfactory),
                ::core::mem::transmute(&pszpropertyname),
                ::core::mem::transmute_copy(&cop),
                ::core::mem::transmute(&pszvaluetype),
                ::core::mem::transmute(&pszvalue),
                ::core::mem::transmute(&pszvalue2),
                ::windows::core::from_raw_borrowed(&ppropertynameterm),
                ::windows::core::from_raw_borrowed(&poperationterm),
                ::windows::core::from_raw_borrowed(&pvalueterm),
                ::core::mem::transmute_copy(&automaticwildcard),
                ::core::mem::transmute_copy(&pnostringquery),
                ::core::mem::transmute_copy(&ppqueryexpression),
            )
            .into()
        }
        unsafe extern "system" fn DefaultPhrase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConditionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluetype: ::windows::core::PCWSTR, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, fuseenglish: super::super::Foundation::BOOL, ppszphrase: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DefaultPhrase(::core::mem::transmute(&pszvaluetype), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&fuseenglish), ::core::mem::transmute_copy(&ppszphrase)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            RecognizeNamedEntities: RecognizeNamedEntities::<Identity, Impl, OFFSET>,
            GenerateForLeaf: GenerateForLeaf::<Identity, Impl, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConditionGenerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IConvertType_Impl: Sized {
    fn CanConvert(&self, wfromtype: u16, wtotype: u16, dwconvertflags: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IConvertType {}
impl IConvertType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConvertType_Impl, const OFFSET: isize>() -> IConvertType_Vtbl {
        unsafe extern "system" fn CanConvert<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConvertType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wfromtype: u16, wtotype: u16, dwconvertflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CanConvert(::core::mem::transmute_copy(&wfromtype), ::core::mem::transmute_copy(&wtotype), ::core::mem::transmute_copy(&dwconvertflags)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CanConvert: CanConvert::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConvertType as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait ICreateRow_Impl: Sized {
    fn CreateRow(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, pwszurl: &::windows::core::PCWSTR, dwbindurlflags: u32, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pauthenticate: ::core::option::Option<&super::Com::IAuthenticate>, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppwsznewurl: *mut ::windows::core::PWSTR, ppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ICreateRow {}
#[cfg(feature = "Win32_System_Com")]
impl ICreateRow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICreateRow_Impl, const OFFSET: isize>() -> ICreateRow_Vtbl {
        unsafe extern "system" fn CreateRow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICreateRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, dwbindurlflags: u32, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pauthenticate: *mut ::core::ffi::c_void, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppwsznewurl: *mut ::windows::core::PWSTR, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateRow(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&dwbindurlflags), ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&riid), ::windows::core::from_raw_borrowed(&pauthenticate), ::core::mem::transmute_copy(&pimplsession), ::core::mem::transmute_copy(&pdwbindstatus), ::core::mem::transmute_copy(&ppwsznewurl), ::core::mem::transmute_copy(&ppunk)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateRow: CreateRow::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateRow as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IDBAsynchNotify_Impl: Sized {
    fn OnLowResource(&self, dwreserved: usize) -> ::windows::core::Result<()>;
    fn OnProgress(&self, hchapter: usize, eoperation: u32, ulprogress: usize, ulprogressmax: usize, easynchphase: u32, pwszstatustext: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn OnStop(&self, hchapter: usize, eoperation: u32, hrstatus: ::windows::core::HRESULT, pwszstatustext: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDBAsynchNotify {}
impl IDBAsynchNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBAsynchNotify_Impl, const OFFSET: isize>() -> IDBAsynchNotify_Vtbl {
        unsafe extern "system" fn OnLowResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBAsynchNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLowResource(::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn OnProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBAsynchNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, eoperation: u32, ulprogress: usize, ulprogressmax: usize, easynchphase: u32, pwszstatustext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnProgress(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&eoperation), ::core::mem::transmute_copy(&ulprogress), ::core::mem::transmute_copy(&ulprogressmax), ::core::mem::transmute_copy(&easynchphase), ::core::mem::transmute(&pwszstatustext)).into()
        }
        unsafe extern "system" fn OnStop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBAsynchNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, eoperation: u32, hrstatus: ::windows::core::HRESULT, pwszstatustext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStop(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&eoperation), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute(&pwszstatustext)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnLowResource: OnLowResource::<Identity, Impl, OFFSET>,
            OnProgress: OnProgress::<Identity, Impl, OFFSET>,
            OnStop: OnStop::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBAsynchNotify as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IDBAsynchStatus_Impl: Sized {
    fn Abort(&self, hchapter: usize, eoperation: u32) -> ::windows::core::Result<()>;
    fn GetStatus(&self, hchapter: usize, eoperation: u32, pulprogress: *mut usize, pulprogressmax: *mut usize, peasynchphase: *mut u32, ppwszstatustext: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDBAsynchStatus {}
impl IDBAsynchStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBAsynchStatus_Impl, const OFFSET: isize>() -> IDBAsynchStatus_Vtbl {
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBAsynchStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, eoperation: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&eoperation)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBAsynchStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, eoperation: u32, pulprogress: *mut usize, pulprogressmax: *mut usize, peasynchphase: *mut u32, ppwszstatustext: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatus(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&eoperation), ::core::mem::transmute_copy(&pulprogress), ::core::mem::transmute_copy(&pulprogressmax), ::core::mem::transmute_copy(&peasynchphase), ::core::mem::transmute_copy(&ppwszstatustext)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Abort: Abort::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBAsynchStatus as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDBBinderProperties_Impl: Sized + IDBProperties_Impl {
    fn Reset(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IDBBinderProperties {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDBBinderProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBBinderProperties_Impl, const OFFSET: isize>() -> IDBBinderProperties_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBBinderProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        Self { base__: IDBProperties_Vtbl::new::<Identity, Impl, OFFSET>(), Reset: Reset::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBBinderProperties as ::windows::core::ComInterface>::IID || iid == &<IDBProperties as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IDBCreateCommand_Impl: Sized {
    fn CreateCommand(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ::windows::core::RuntimeName for IDBCreateCommand {}
impl IDBCreateCommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBCreateCommand_Impl, const OFFSET: isize>() -> IDBCreateCommand_Vtbl {
        unsafe extern "system" fn CreateCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBCreateCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcommand: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCommand(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcommand, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateCommand: CreateCommand::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBCreateCommand as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IDBCreateSession_Impl: Sized {
    fn CreateSession(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ::windows::core::RuntimeName for IDBCreateSession {}
impl IDBCreateSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBCreateSession_Impl, const OFFSET: isize>() -> IDBCreateSession_Vtbl {
        unsafe extern "system" fn CreateSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBCreateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppdbsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSession(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdbsession, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateSession: CreateSession::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBCreateSession as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDBDataSourceAdmin_Impl: Sized {
    fn CreateDataSource(&self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, punkouter: ::core::option::Option<&::windows::core::IUnknown>, riid: *const ::windows::core::GUID, ppdbsession: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DestroyDataSource(&self) -> ::windows::core::Result<()>;
    fn GetCreationProperties(&self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
    fn ModifyDataSource(&self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IDBDataSourceAdmin {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDBDataSourceAdmin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBDataSourceAdmin_Impl, const OFFSET: isize>() -> IDBDataSourceAdmin_Vtbl {
        unsafe extern "system" fn CreateDataSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBDataSourceAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppdbsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateDataSource(::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppdbsession)).into()
        }
        unsafe extern "system" fn DestroyDataSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBDataSourceAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DestroyDataSource().into()
        }
        unsafe extern "system" fn GetCreationProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBDataSourceAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCreationProperties(::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertyinfosets), ::core::mem::transmute_copy(&prgpropertyinfosets), ::core::mem::transmute_copy(&ppdescbuffer)).into()
        }
        unsafe extern "system" fn ModifyDataSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBDataSourceAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModifyDataSource(::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDataSource: CreateDataSource::<Identity, Impl, OFFSET>,
            DestroyDataSource: DestroyDataSource::<Identity, Impl, OFFSET>,
            GetCreationProperties: GetCreationProperties::<Identity, Impl, OFFSET>,
            ModifyDataSource: ModifyDataSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBDataSourceAdmin as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDBInfo_Impl: Sized {
    fn GetKeywords(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetLiteralInfo(&self, cliterals: u32, rgliterals: *const u32, pcliteralinfo: *mut u32, prgliteralinfo: *mut *mut DBLITERALINFO, ppcharbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDBInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IDBInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBInfo_Impl, const OFFSET: isize>() -> IDBInfo_Vtbl {
        unsafe extern "system" fn GetKeywords<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszkeywords: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetKeywords() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszkeywords, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLiteralInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cliterals: u32, rgliterals: *const u32, pcliteralinfo: *mut u32, prgliteralinfo: *mut *mut DBLITERALINFO, ppcharbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLiteralInfo(::core::mem::transmute_copy(&cliterals), ::core::mem::transmute_copy(&rgliterals), ::core::mem::transmute_copy(&pcliteralinfo), ::core::mem::transmute_copy(&prgliteralinfo), ::core::mem::transmute_copy(&ppcharbuffer)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetKeywords: GetKeywords::<Identity, Impl, OFFSET>,
            GetLiteralInfo: GetLiteralInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IDBInitialize_Impl: Sized {
    fn Initialize(&self) -> ::windows::core::Result<()>;
    fn Uninitialize(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDBInitialize {}
impl IDBInitialize_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBInitialize_Impl, const OFFSET: isize>() -> IDBInitialize_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize().into()
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Uninitialize().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBInitialize as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDBPromptInitialize_Impl: Sized {
    fn PromptDataSource(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, csourcetypefilter: u32, rgsourcetypefilter: *const u32, pwszszzproviderfilter: &::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppdatasource: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn PromptFileName(&self, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, pwszinitialdirectory: &::windows::core::PCWSTR, pwszinitialfile: &::windows::core::PCWSTR) -> ::windows::core::Result<::windows::core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDBPromptInitialize {}
#[cfg(feature = "Win32_Foundation")]
impl IDBPromptInitialize_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBPromptInitialize_Impl, const OFFSET: isize>() -> IDBPromptInitialize_Vtbl {
        unsafe extern "system" fn PromptDataSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBPromptInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, csourcetypefilter: u32, rgsourcetypefilter: *const u32, pwszszzproviderfilter: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PromptDataSource(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwpromptoptions), ::core::mem::transmute_copy(&csourcetypefilter), ::core::mem::transmute_copy(&rgsourcetypefilter), ::core::mem::transmute(&pwszszzproviderfilter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppdatasource)).into()
        }
        unsafe extern "system" fn PromptFileName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBPromptInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, pwszinitialdirectory: ::windows::core::PCWSTR, pwszinitialfile: ::windows::core::PCWSTR, ppwszselectedfile: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PromptFileName(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwpromptoptions), ::core::mem::transmute(&pwszinitialdirectory), ::core::mem::transmute(&pwszinitialfile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszselectedfile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PromptDataSource: PromptDataSource::<Identity, Impl, OFFSET>,
            PromptFileName: PromptFileName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBPromptInitialize as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDBProperties_Impl: Sized {
    fn GetProperties(&self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::Result<()>;
    fn GetPropertyInfo(&self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
    fn SetProperties(&self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IDBProperties {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDBProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBProperties_Impl, const OFFSET: isize>() -> IDBProperties_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProperties(::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)).into()
        }
        unsafe extern "system" fn GetPropertyInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyInfo(::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertyinfosets), ::core::mem::transmute_copy(&prgpropertyinfosets), ::core::mem::transmute_copy(&ppdescbuffer)).into()
        }
        unsafe extern "system" fn SetProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperties(::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetPropertyInfo: GetPropertyInfo::<Identity, Impl, OFFSET>,
            SetProperties: SetProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBProperties as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IDBSchemaCommand_Impl: Sized {
    fn GetCommand(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, rguidschema: *const ::windows::core::GUID) -> ::windows::core::Result<ICommand>;
    fn GetSchemas(&self, pcschemas: *mut u32, prgschemas: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDBSchemaCommand {}
impl IDBSchemaCommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBSchemaCommand_Impl, const OFFSET: isize>() -> IDBSchemaCommand_Vtbl {
        unsafe extern "system" fn GetCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBSchemaCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, rguidschema: *const ::windows::core::GUID, ppcommand: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCommand(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&rguidschema)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcommand, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSchemas<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBSchemaCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcschemas: *mut u32, prgschemas: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSchemas(::core::mem::transmute_copy(&pcschemas), ::core::mem::transmute_copy(&prgschemas)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCommand: GetCommand::<Identity, Impl, OFFSET>,
            GetSchemas: GetSchemas::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBSchemaCommand as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDBSchemaRowset_Impl: Sized {
    fn GetRowset(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, rguidschema: *const ::windows::core::GUID, crestrictions: u32, rgrestrictions: *const super::Com::VARIANT, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetSchemas(&self, pcschemas: *mut u32, prgschemas: *mut *mut ::windows::core::GUID, prgrestrictionsupport: *mut *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IDBSchemaRowset {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDBSchemaRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBSchemaRowset_Impl, const OFFSET: isize>() -> IDBSchemaRowset_Vtbl {
        unsafe extern "system" fn GetRowset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBSchemaRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, rguidschema: *const ::windows::core::GUID, crestrictions: u32, rgrestrictions: *const super::Com::VARIANT, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRowset(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&rguidschema), ::core::mem::transmute_copy(&crestrictions), ::core::mem::transmute_copy(&rgrestrictions), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pprowset)).into()
        }
        unsafe extern "system" fn GetSchemas<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDBSchemaRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcschemas: *mut u32, prgschemas: *mut *mut ::windows::core::GUID, prgrestrictionsupport: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSchemas(::core::mem::transmute_copy(&pcschemas), ::core::mem::transmute_copy(&prgschemas), ::core::mem::transmute_copy(&prgrestrictionsupport)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRowset: GetRowset::<Identity, Impl, OFFSET>,
            GetSchemas: GetSchemas::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDBSchemaRowset as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDCInfo_Impl: Sized {
    fn GetInfo(&self, cinfo: u32, rgeinfotype: *const u32, prginfo: *mut *mut DCINFO) -> ::windows::core::Result<()>;
    fn SetInfo(&self, cinfo: u32, rginfo: *const DCINFO) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IDCInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDCInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDCInfo_Impl, const OFFSET: isize>() -> IDCInfo_Vtbl {
        unsafe extern "system" fn GetInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDCInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cinfo: u32, rgeinfotype: *const u32, prginfo: *mut *mut DCINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInfo(::core::mem::transmute_copy(&cinfo), ::core::mem::transmute_copy(&rgeinfotype), ::core::mem::transmute_copy(&prginfo)).into()
        }
        unsafe extern "system" fn SetInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDCInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cinfo: u32, rginfo: *const DCINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInfo(::core::mem::transmute_copy(&cinfo), ::core::mem::transmute_copy(&rginfo)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            SetInfo: SetInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IDataConvert_Impl: Sized {
    fn DataConvert(&self, wsrctype: u16, wdsttype: u16, cbsrclength: usize, pcbdstlength: *mut usize, psrc: *const ::core::ffi::c_void, pdst: *mut ::core::ffi::c_void, cbdstmaxlength: usize, dbssrcstatus: u32, pdbsstatus: *mut u32, bprecision: u8, bscale: u8, dwflags: u32) -> ::windows::core::Result<()>;
    fn CanConvert(&self, wsrctype: u16, wdsttype: u16) -> ::windows::core::Result<()>;
    fn GetConversionSize(&self, wsrctype: u16, wdsttype: u16, pcbsrclength: *const usize, pcbdstlength: *mut usize, psrc: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDataConvert {}
impl IDataConvert_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataConvert_Impl, const OFFSET: isize>() -> IDataConvert_Vtbl {
        unsafe extern "system" fn DataConvert<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataConvert_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsrctype: u16, wdsttype: u16, cbsrclength: usize, pcbdstlength: *mut usize, psrc: *const ::core::ffi::c_void, pdst: *mut ::core::ffi::c_void, cbdstmaxlength: usize, dbssrcstatus: u32, pdbsstatus: *mut u32, bprecision: u8, bscale: u8, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DataConvert(
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
        unsafe extern "system" fn CanConvert<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataConvert_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsrctype: u16, wdsttype: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CanConvert(::core::mem::transmute_copy(&wsrctype), ::core::mem::transmute_copy(&wdsttype)).into()
        }
        unsafe extern "system" fn GetConversionSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataConvert_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsrctype: u16, wdsttype: u16, pcbsrclength: *const usize, pcbdstlength: *mut usize, psrc: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConversionSize(::core::mem::transmute_copy(&wsrctype), ::core::mem::transmute_copy(&wdsttype), ::core::mem::transmute_copy(&pcbsrclength), ::core::mem::transmute_copy(&pcbdstlength), ::core::mem::transmute_copy(&psrc)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DataConvert: DataConvert::<Identity, Impl, OFFSET>,
            CanConvert: CanConvert::<Identity, Impl, OFFSET>,
            GetConversionSize: GetConversionSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataConvert as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDataInitialize_Impl: Sized {
    fn GetDataSource(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, dwclsctx: u32, pwszinitializationstring: &::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppdatasource: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetInitializationString(&self, pdatasource: ::core::option::Option<&::windows::core::IUnknown>, fincludepassword: u8) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn CreateDBInstance(&self, clsidprovider: *const ::windows::core::GUID, punkouter: ::core::option::Option<&::windows::core::IUnknown>, dwclsctx: u32, pwszreserved: &::windows::core::PCWSTR, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn CreateDBInstanceEx(&self, clsidprovider: *const ::windows::core::GUID, punkouter: ::core::option::Option<&::windows::core::IUnknown>, dwclsctx: u32, pwszreserved: &::windows::core::PCWSTR, pserverinfo: *const super::Com::COSERVERINFO, cmq: u32, rgmqresults: *mut super::Com::MULTI_QI) -> ::windows::core::Result<()>;
    fn LoadStringFromStorage(&self, pwszfilename: &::windows::core::PCWSTR) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn WriteStringToStorage(&self, pwszfilename: &::windows::core::PCWSTR, pwszinitializationstring: &::windows::core::PCWSTR, dwcreationdisposition: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDataInitialize {}
#[cfg(feature = "Win32_System_Com")]
impl IDataInitialize_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataInitialize_Impl, const OFFSET: isize>() -> IDataInitialize_Vtbl {
        unsafe extern "system" fn GetDataSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, dwclsctx: u32, pwszinitializationstring: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDataSource(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&dwclsctx), ::core::mem::transmute(&pwszinitializationstring), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppdatasource)).into()
        }
        unsafe extern "system" fn GetInitializationString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatasource: *mut ::core::ffi::c_void, fincludepassword: u8, ppwszinitstring: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInitializationString(::windows::core::from_raw_borrowed(&pdatasource), ::core::mem::transmute_copy(&fincludepassword)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszinitstring, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDBInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidprovider: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: u32, pwszreserved: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDBInstance(::core::mem::transmute_copy(&clsidprovider), ::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&dwclsctx), ::core::mem::transmute(&pwszreserved), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdatasource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDBInstanceEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidprovider: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: u32, pwszreserved: ::windows::core::PCWSTR, pserverinfo: *const super::Com::COSERVERINFO, cmq: u32, rgmqresults: *mut super::Com::MULTI_QI) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateDBInstanceEx(::core::mem::transmute_copy(&clsidprovider), ::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&dwclsctx), ::core::mem::transmute(&pwszreserved), ::core::mem::transmute_copy(&pserverinfo), ::core::mem::transmute_copy(&cmq), ::core::mem::transmute_copy(&rgmqresults)).into()
        }
        unsafe extern "system" fn LoadStringFromStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR, ppwszinitializationstring: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadStringFromStorage(::core::mem::transmute(&pwszfilename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszinitializationstring, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteStringToStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR, pwszinitializationstring: ::windows::core::PCWSTR, dwcreationdisposition: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteStringToStorage(::core::mem::transmute(&pwszfilename), ::core::mem::transmute(&pwszinitializationstring), ::core::mem::transmute_copy(&dwcreationdisposition)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDataSource: GetDataSource::<Identity, Impl, OFFSET>,
            GetInitializationString: GetInitializationString::<Identity, Impl, OFFSET>,
            CreateDBInstance: CreateDBInstance::<Identity, Impl, OFFSET>,
            CreateDBInstanceEx: CreateDBInstanceEx::<Identity, Impl, OFFSET>,
            LoadStringFromStorage: LoadStringFromStorage::<Identity, Impl, OFFSET>,
            WriteStringToStorage: WriteStringToStorage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataInitialize as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDataSourceLocator_Impl: Sized + super::Com::IDispatch_Impl {
    fn hWnd(&self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn SethWnd(&self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn PromptNew(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn PromptEdit(&self, ppadoconnection: *mut ::core::option::Option<super::Com::IDispatch>, pbsuccess: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IDataSourceLocator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDataSourceLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataSourceLocator_Impl, const OFFSET: isize>() -> IDataSourceLocator_Vtbl {
        unsafe extern "system" fn hWnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataSourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwndparent: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hWnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwndparent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SethWnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataSourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SethWnd(::core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn PromptNew<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataSourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppadoconnection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PromptNew() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppadoconnection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PromptEdit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataSourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppadoconnection: *mut *mut ::core::ffi::c_void, pbsuccess: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PromptEdit(::core::mem::transmute_copy(&ppadoconnection), ::core::mem::transmute_copy(&pbsuccess)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            hWnd: hWnd::<Identity, Impl, OFFSET>,
            SethWnd: SethWnd::<Identity, Impl, OFFSET>,
            PromptNew: PromptNew::<Identity, Impl, OFFSET>,
            PromptEdit: PromptEdit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataSourceLocator as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IEntity_Impl: Sized {
    fn Name(&self, ppszname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn Base(&self) -> ::windows::core::Result<IEntity>;
    fn Relationships(&self, riid: *const ::windows::core::GUID, prelationships: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetRelationship(&self, pszrelationname: &::windows::core::PCWSTR) -> ::windows::core::Result<IRelationship>;
    fn MetaData(&self, riid: *const ::windows::core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn NamedEntities(&self, riid: *const ::windows::core::GUID, pnamedentities: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetNamedEntity(&self, pszvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<INamedEntity>;
    fn DefaultPhrase(&self, ppszphrase: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IEntity {}
impl IEntity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEntity_Impl, const OFFSET: isize>() -> IEntity_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Name(::core::mem::transmute_copy(&ppszname)).into()
        }
        unsafe extern "system" fn Base<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbaseentity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Base() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbaseentity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Relationships<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, prelationships: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Relationships(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&prelationships)).into()
        }
        unsafe extern "system" fn GetRelationship<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrelationname: ::windows::core::PCWSTR, prelationship: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRelationship(::core::mem::transmute(&pszrelationname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prelationship, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MetaData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MetaData(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pmetadata)).into()
        }
        unsafe extern "system" fn NamedEntities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pnamedentities: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NamedEntities(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pnamedentities)).into()
        }
        unsafe extern "system" fn GetNamedEntity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvalue: ::windows::core::PCWSTR, ppnamedentity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNamedEntity(::core::mem::transmute(&pszvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnamedentity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultPhrase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszphrase: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DefaultPhrase(::core::mem::transmute_copy(&ppszphrase)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IEntity as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumItemProperties_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ITEMPROP, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumItemProperties>;
    fn GetCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IEnumItemProperties {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumItemProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumItemProperties_Impl, const OFFSET: isize>() -> IEnumItemProperties_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ITEMPROP, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pncount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumItemProperties as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IEnumSearchRoots_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<ISearchRoot>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumSearchRoots>;
}
impl ::windows::core::RuntimeName for IEnumSearchRoots {}
impl IEnumSearchRoots_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSearchRoots_Impl, const OFFSET: isize>() -> IEnumSearchRoots_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSearchRoots_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSearchRoots_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSearchRoots_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSearchRoots_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSearchRoots as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IEnumSearchScopeRules_Impl: Sized {
    fn Next(&self, celt: u32, pprgelt: *mut ::core::option::Option<ISearchScopeRule>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumSearchScopeRules>;
}
impl ::windows::core::RuntimeName for IEnumSearchScopeRules {}
impl IEnumSearchScopeRules_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSearchScopeRules_Impl, const OFFSET: isize>() -> IEnumSearchScopeRules_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSearchScopeRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pprgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pprgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSearchScopeRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSearchScopeRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSearchScopeRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSearchScopeRules as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IEnumSubscription_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumSubscription>;
    fn GetCount(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IEnumSubscription {}
impl IEnumSubscription_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSubscription_Impl, const OFFSET: isize>() -> IEnumSubscription_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pncount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSubscription as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IErrorLookup_Impl: Sized {
    fn GetErrorDescription(&self, hrerror: ::windows::core::HRESULT, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, lcid: u32, pbstrsource: *mut ::windows::core::BSTR, pbstrdescription: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetHelpInfo(&self, hrerror: ::windows::core::HRESULT, dwlookupid: u32, lcid: u32, pbstrhelpfile: *mut ::windows::core::BSTR, pdwhelpcontext: *mut u32) -> ::windows::core::Result<()>;
    fn ReleaseErrors(&self, dwdynamicerrorid: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IErrorLookup {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IErrorLookup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorLookup_Impl, const OFFSET: isize>() -> IErrorLookup_Vtbl {
        unsafe extern "system" fn GetErrorDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorLookup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, lcid: u32, pbstrsource: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetErrorDescription(::core::mem::transmute_copy(&hrerror), ::core::mem::transmute_copy(&dwlookupid), ::core::mem::transmute_copy(&pdispparams), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&pbstrsource), ::core::mem::transmute_copy(&pbstrdescription)).into()
        }
        unsafe extern "system" fn GetHelpInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorLookup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, dwlookupid: u32, lcid: u32, pbstrhelpfile: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pdwhelpcontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHelpInfo(::core::mem::transmute_copy(&hrerror), ::core::mem::transmute_copy(&dwlookupid), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&pbstrhelpfile), ::core::mem::transmute_copy(&pdwhelpcontext)).into()
        }
        unsafe extern "system" fn ReleaseErrors<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorLookup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdynamicerrorid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseErrors(::core::mem::transmute_copy(&dwdynamicerrorid)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetErrorDescription: GetErrorDescription::<Identity, Impl, OFFSET>,
            GetHelpInfo: GetHelpInfo::<Identity, Impl, OFFSET>,
            ReleaseErrors: ReleaseErrors::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IErrorLookup as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IErrorRecords_Impl: Sized {
    fn AddErrorRecord(&self, perrorinfo: *const ERRORINFO, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, punkcustomerror: ::core::option::Option<&::windows::core::IUnknown>, dwdynamicerrorid: u32) -> ::windows::core::Result<()>;
    fn GetBasicErrorInfo(&self, ulrecordnum: u32, perrorinfo: *mut ERRORINFO) -> ::windows::core::Result<()>;
    fn GetCustomErrorObject(&self, ulrecordnum: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetErrorInfo(&self, ulrecordnum: u32, lcid: u32) -> ::windows::core::Result<super::Com::IErrorInfo>;
    fn GetErrorParameters(&self, ulrecordnum: u32) -> ::windows::core::Result<super::Com::DISPPARAMS>;
    fn GetRecordCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IErrorRecords {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IErrorRecords_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorRecords_Impl, const OFFSET: isize>() -> IErrorRecords_Vtbl {
        unsafe extern "system" fn AddErrorRecord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrorinfo: *const ERRORINFO, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, punkcustomerror: *mut ::core::ffi::c_void, dwdynamicerrorid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddErrorRecord(::core::mem::transmute_copy(&perrorinfo), ::core::mem::transmute_copy(&dwlookupid), ::core::mem::transmute_copy(&pdispparams), ::windows::core::from_raw_borrowed(&punkcustomerror), ::core::mem::transmute_copy(&dwdynamicerrorid)).into()
        }
        unsafe extern "system" fn GetBasicErrorInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulrecordnum: u32, perrorinfo: *mut ERRORINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBasicErrorInfo(::core::mem::transmute_copy(&ulrecordnum), ::core::mem::transmute_copy(&perrorinfo)).into()
        }
        unsafe extern "system" fn GetCustomErrorObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulrecordnum: u32, riid: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCustomErrorObject(::core::mem::transmute_copy(&ulrecordnum), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulrecordnum: u32, lcid: u32, pperrorinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetErrorInfo(::core::mem::transmute_copy(&ulrecordnum), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pperrorinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulrecordnum: u32, pdispparams: *mut super::Com::DISPPARAMS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetErrorParameters(::core::mem::transmute_copy(&ulrecordnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdispparams, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecordCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcrecords: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRecordCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcrecords, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddErrorRecord: AddErrorRecord::<Identity, Impl, OFFSET>,
            GetBasicErrorInfo: GetBasicErrorInfo::<Identity, Impl, OFFSET>,
            GetCustomErrorObject: GetCustomErrorObject::<Identity, Impl, OFFSET>,
            GetErrorInfo: GetErrorInfo::<Identity, Impl, OFFSET>,
            GetErrorParameters: GetErrorParameters::<Identity, Impl, OFFSET>,
            GetRecordCount: GetRecordCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IErrorRecords as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IGetDataSource_Impl: Sized {
    fn GetDataSource(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ::windows::core::RuntimeName for IGetDataSource {}
impl IGetDataSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetDataSource_Impl, const OFFSET: isize>() -> IGetDataSource_Vtbl {
        unsafe extern "system" fn GetDataSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDataSource(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdatasource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDataSource: GetDataSource::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetDataSource as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IGetRow_Impl: Sized {
    fn GetRowFromHROW(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, hrow: usize, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetURLFromHROW(&self, hrow: usize) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl ::windows::core::RuntimeName for IGetRow {}
impl IGetRow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetRow_Impl, const OFFSET: isize>() -> IGetRow_Vtbl {
        unsafe extern "system" fn GetRowFromHROW<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, hrow: usize, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRowFromHROW(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetURLFromHROW<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrow: usize, ppwszurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetURLFromHROW(::core::mem::transmute_copy(&hrow)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRowFromHROW: GetRowFromHROW::<Identity, Impl, OFFSET>,
            GetURLFromHROW: GetURLFromHROW::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetRow as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IGetSession_Impl: Sized {
    fn GetSession(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ::windows::core::RuntimeName for IGetSession {}
impl IGetSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetSession_Impl, const OFFSET: isize>() -> IGetSession_Vtbl {
        unsafe extern "system" fn GetSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSession(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSession: GetSession::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetSession as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IGetSourceRow_Impl: Sized {
    fn GetSourceRow(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ::windows::core::RuntimeName for IGetSourceRow {}
impl IGetSourceRow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetSourceRow_Impl, const OFFSET: isize>() -> IGetSourceRow_Vtbl {
        unsafe extern "system" fn GetSourceRow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetSourceRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pprow: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSourceRow(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSourceRow: GetSourceRow::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetSourceRow as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IIndexDefinition_Impl: Sized {
    fn CreateIndex(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, cindexcolumndescs: usize, rgindexcolumndescs: *const DBINDEXCOLUMNDESC, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppindexid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
    fn DropIndex(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IIndexDefinition {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IIndexDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIndexDefinition_Impl, const OFFSET: isize>() -> IIndexDefinition_Vtbl {
        unsafe extern "system" fn CreateIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIndexDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, cindexcolumndescs: usize, rgindexcolumndescs: *const DBINDEXCOLUMNDESC, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppindexid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateIndex(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pindexid), ::core::mem::transmute_copy(&cindexcolumndescs), ::core::mem::transmute_copy(&rgindexcolumndescs), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&ppindexid)).into()
        }
        unsafe extern "system" fn DropIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIndexDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DropIndex(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pindexid)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateIndex: CreateIndex::<Identity, Impl, OFFSET>,
            DropIndex: DropIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIndexDefinition as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IInterval_Impl: Sized {
    fn GetLimits(&self, pilklower: *mut INTERVAL_LIMIT_KIND, ppropvarlower: *mut super::Com::StructuredStorage::PROPVARIANT, pilkupper: *mut INTERVAL_LIMIT_KIND, ppropvarupper: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IInterval {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IInterval_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInterval_Impl, const OFFSET: isize>() -> IInterval_Vtbl {
        unsafe extern "system" fn GetLimits<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInterval_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pilklower: *mut INTERVAL_LIMIT_KIND, ppropvarlower: *mut super::Com::StructuredStorage::PROPVARIANT, pilkupper: *mut INTERVAL_LIMIT_KIND, ppropvarupper: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLimits(::core::mem::transmute_copy(&pilklower), ::core::mem::transmute_copy(&ppropvarlower), ::core::mem::transmute_copy(&pilkupper), ::core::mem::transmute_copy(&ppropvarupper)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetLimits: GetLimits::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInterval as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ILoadFilter_Impl: Sized {
    fn LoadIFilter(&self, pwcspath: &::windows::core::PCWSTR, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: ::core::option::Option<&::windows::core::IUnknown>, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut ::core::option::Option<super::super::Storage::IndexServer::IFilter>) -> ::windows::core::Result<()>;
    fn LoadIFilterFromStorage(&self, pstg: ::core::option::Option<&super::Com::StructuredStorage::IStorage>, punkouter: ::core::option::Option<&::windows::core::IUnknown>, pwcsoverride: &::windows::core::PCWSTR, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut ::core::option::Option<super::super::Storage::IndexServer::IFilter>) -> ::windows::core::Result<()>;
    fn LoadIFilterFromStream(&self, pstm: ::core::option::Option<&super::Com::IStream>, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: ::core::option::Option<&::windows::core::IUnknown>, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut ::core::option::Option<super::super::Storage::IndexServer::IFilter>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for ILoadFilter {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ILoadFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoadFilter_Impl, const OFFSET: isize>() -> ILoadFilter_Vtbl {
        unsafe extern "system" fn LoadIFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoadFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcspath: ::windows::core::PCWSTR, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: *mut ::core::ffi::c_void, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadIFilter(::core::mem::transmute(&pwcspath), ::core::mem::transmute_copy(&pfilteredsources), ::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&fusedefault), ::core::mem::transmute_copy(&pfilterclsid), ::core::mem::transmute_copy(&searchdecsize), ::core::mem::transmute_copy(&pwcssearchdesc), ::core::mem::transmute_copy(&ppifilt)).into()
        }
        unsafe extern "system" fn LoadIFilterFromStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoadFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstg: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pwcsoverride: ::windows::core::PCWSTR, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadIFilterFromStorage(::windows::core::from_raw_borrowed(&pstg), ::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute(&pwcsoverride), ::core::mem::transmute_copy(&fusedefault), ::core::mem::transmute_copy(&pfilterclsid), ::core::mem::transmute_copy(&searchdecsize), ::core::mem::transmute_copy(&pwcssearchdesc), ::core::mem::transmute_copy(&ppifilt)).into()
        }
        unsafe extern "system" fn LoadIFilterFromStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoadFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: *mut ::core::ffi::c_void, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadIFilterFromStream(::windows::core::from_raw_borrowed(&pstm), ::core::mem::transmute_copy(&pfilteredsources), ::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&fusedefault), ::core::mem::transmute_copy(&pfilterclsid), ::core::mem::transmute_copy(&searchdecsize), ::core::mem::transmute_copy(&pwcssearchdesc), ::core::mem::transmute_copy(&ppifilt)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadIFilter: LoadIFilter::<Identity, Impl, OFFSET>,
            LoadIFilterFromStorage: LoadIFilterFromStorage::<Identity, Impl, OFFSET>,
            LoadIFilterFromStream: LoadIFilterFromStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoadFilter as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ILoadFilterWithPrivateComActivation_Impl: Sized + ILoadFilter_Impl {
    fn LoadIFilterWithPrivateComActivation(&self, filteredsources: *const FILTERED_DATA_SOURCES, usedefault: super::super::Foundation::BOOL, filterclsid: *mut ::windows::core::GUID, isfilterprivatecomactivated: *mut super::super::Foundation::BOOL, filterobj: *mut ::core::option::Option<super::super::Storage::IndexServer::IFilter>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for ILoadFilterWithPrivateComActivation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ILoadFilterWithPrivateComActivation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoadFilterWithPrivateComActivation_Impl, const OFFSET: isize>() -> ILoadFilterWithPrivateComActivation_Vtbl {
        unsafe extern "system" fn LoadIFilterWithPrivateComActivation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoadFilterWithPrivateComActivation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filteredsources: *const FILTERED_DATA_SOURCES, usedefault: super::super::Foundation::BOOL, filterclsid: *mut ::windows::core::GUID, isfilterprivatecomactivated: *mut super::super::Foundation::BOOL, filterobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadIFilterWithPrivateComActivation(::core::mem::transmute_copy(&filteredsources), ::core::mem::transmute_copy(&usedefault), ::core::mem::transmute_copy(&filterclsid), ::core::mem::transmute_copy(&isfilterprivatecomactivated), ::core::mem::transmute_copy(&filterobj)).into()
        }
        Self {
            base__: ILoadFilter_Vtbl::new::<Identity, Impl, OFFSET>(),
            LoadIFilterWithPrivateComActivation: LoadIFilterWithPrivateComActivation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoadFilterWithPrivateComActivation as ::windows::core::ComInterface>::IID || iid == &<ILoadFilter as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMDDataset_Impl: Sized {
    fn FreeAxisInfo(&self, caxes: usize, rgaxisinfo: *mut MDAXISINFO) -> ::windows::core::Result<()>;
    fn GetAxisInfo(&self, pcaxes: *mut usize, prgaxisinfo: *mut *mut MDAXISINFO) -> ::windows::core::Result<()>;
    fn GetAxisRowset(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, iaxis: usize, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetCellData(&self, haccessor: HACCESSOR, ulstartcell: usize, ulendcell: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetSpecification(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMDDataset {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMDDataset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDDataset_Impl, const OFFSET: isize>() -> IMDDataset_Vtbl {
        unsafe extern "system" fn FreeAxisInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDDataset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, caxes: usize, rgaxisinfo: *mut MDAXISINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreeAxisInfo(::core::mem::transmute_copy(&caxes), ::core::mem::transmute_copy(&rgaxisinfo)).into()
        }
        unsafe extern "system" fn GetAxisInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDDataset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaxes: *mut usize, prgaxisinfo: *mut *mut MDAXISINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAxisInfo(::core::mem::transmute_copy(&pcaxes), ::core::mem::transmute_copy(&prgaxisinfo)).into()
        }
        unsafe extern "system" fn GetAxisRowset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDDataset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, iaxis: usize, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAxisRowset(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&iaxis), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pprowset)).into()
        }
        unsafe extern "system" fn GetCellData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDDataset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haccessor: HACCESSOR, ulstartcell: usize, ulendcell: usize, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCellData(::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&ulstartcell), ::core::mem::transmute_copy(&ulendcell), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetSpecification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDDataset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppspecification: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSpecification(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppspecification, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FreeAxisInfo: FreeAxisInfo::<Identity, Impl, OFFSET>,
            GetAxisInfo: GetAxisInfo::<Identity, Impl, OFFSET>,
            GetAxisRowset: GetAxisRowset::<Identity, Impl, OFFSET>,
            GetCellData: GetCellData::<Identity, Impl, OFFSET>,
            GetSpecification: GetSpecification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDDataset as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IMDFind_Impl: Sized {
    fn FindCell(&self, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *mut ::windows::core::PWSTR, pulcellordinal: *mut usize) -> ::windows::core::Result<()>;
    fn FindTuple(&self, ulaxisidentifier: u32, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *mut ::windows::core::PWSTR, pultupleordinal: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMDFind {}
impl IMDFind_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDFind_Impl, const OFFSET: isize>() -> IMDFind_Vtbl {
        unsafe extern "system" fn FindCell<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDFind_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *mut ::windows::core::PWSTR, pulcellordinal: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindCell(::core::mem::transmute_copy(&ulstartingordinal), ::core::mem::transmute_copy(&cmembers), ::core::mem::transmute_copy(&rgpwszmember), ::core::mem::transmute_copy(&pulcellordinal)).into()
        }
        unsafe extern "system" fn FindTuple<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDFind_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulaxisidentifier: u32, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *mut ::windows::core::PWSTR, pultupleordinal: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindTuple(::core::mem::transmute_copy(&ulaxisidentifier), ::core::mem::transmute_copy(&ulstartingordinal), ::core::mem::transmute_copy(&cmembers), ::core::mem::transmute_copy(&rgpwszmember), ::core::mem::transmute_copy(&pultupleordinal)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindCell: FindCell::<Identity, Impl, OFFSET>,
            FindTuple: FindTuple::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDFind as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMDRangeRowset_Impl: Sized {
    fn GetRangeRowset(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, ulstartcell: usize, ulendcell: usize, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMDRangeRowset {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMDRangeRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDRangeRowset_Impl, const OFFSET: isize>() -> IMDRangeRowset_Vtbl {
        unsafe extern "system" fn GetRangeRowset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDRangeRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ulstartcell: usize, ulendcell: usize, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRangeRowset(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&ulstartcell), ::core::mem::transmute_copy(&ulendcell), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pprowset)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRangeRowset: GetRangeRowset::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDRangeRowset as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IMetaData_Impl: Sized {
    fn GetData(&self, ppszkey: *mut ::windows::core::PWSTR, ppszvalue: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMetaData {}
impl IMetaData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMetaData_Impl, const OFFSET: isize>() -> IMetaData_Vtbl {
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMetaData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszkey: *mut ::windows::core::PWSTR, ppszvalue: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetData(::core::mem::transmute_copy(&ppszkey), ::core::mem::transmute_copy(&ppszvalue)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetData: GetData::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMetaData as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IMultipleResults_Impl: Sized {
    fn GetResult(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, lresultflag: isize, riid: *const ::windows::core::GUID, pcrowsaffected: *mut isize, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMultipleResults {}
impl IMultipleResults_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMultipleResults_Impl, const OFFSET: isize>() -> IMultipleResults_Vtbl {
        unsafe extern "system" fn GetResult<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMultipleResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, lresultflag: isize, riid: *const ::windows::core::GUID, pcrowsaffected: *mut isize, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetResult(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&lresultflag), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pcrowsaffected), ::core::mem::transmute_copy(&pprowset)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetResult: GetResult::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultipleResults as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait INamedEntity_Impl: Sized {
    fn GetValue(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn DefaultPhrase(&self, ppszphrase: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INamedEntity {}
impl INamedEntity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INamedEntity_Impl, const OFFSET: isize>() -> INamedEntity_Vtbl {
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INamedEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszvalue: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultPhrase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INamedEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszphrase: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DefaultPhrase(::core::mem::transmute_copy(&ppszphrase)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INamedEntity as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait INamedEntityCollector_Impl: Sized {
    fn Add(&self, beginspan: u32, endspan: u32, beginactual: u32, endactual: u32, ptype: ::core::option::Option<&IEntity>, pszvalue: &::windows::core::PCWSTR, certainty: NAMED_ENTITY_CERTAINTY) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INamedEntityCollector {}
impl INamedEntityCollector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INamedEntityCollector_Impl, const OFFSET: isize>() -> INamedEntityCollector_Vtbl {
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INamedEntityCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beginspan: u32, endspan: u32, beginactual: u32, endactual: u32, ptype: *mut ::core::ffi::c_void, pszvalue: ::windows::core::PCWSTR, certainty: NAMED_ENTITY_CERTAINTY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute_copy(&beginspan), ::core::mem::transmute_copy(&endspan), ::core::mem::transmute_copy(&beginactual), ::core::mem::transmute_copy(&endactual), ::windows::core::from_raw_borrowed(&ptype), ::core::mem::transmute(&pszvalue), ::core::mem::transmute_copy(&certainty)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Add: Add::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INamedEntityCollector as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authorization\"`, `\"Win32_Storage_IndexServer\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
pub trait IObjectAccessControl_Impl: Sized {
    fn GetObjectAccessRights(&self, pobject: *mut SEC_OBJECT, pcaccessentries: *mut u32, prgaccessentries: *mut *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> ::windows::core::Result<()>;
    fn GetObjectOwner(&self, pobject: *mut SEC_OBJECT, ppowner: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::Result<()>;
    fn IsObjectAccessAllowed(&self, pobject: *mut SEC_OBJECT, paccessentry: *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetObjectAccessRights(&self, pobject: *mut SEC_OBJECT, caccessentries: u32, prgaccessentries: *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> ::windows::core::Result<()>;
    fn SetObjectOwner(&self, pobject: *mut SEC_OBJECT, powner: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
impl ::windows::core::RuntimeName for IObjectAccessControl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
impl IObjectAccessControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectAccessControl_Impl, const OFFSET: isize>() -> IObjectAccessControl_Vtbl {
        unsafe extern "system" fn GetObjectAccessRights<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectAccessControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut SEC_OBJECT, pcaccessentries: *mut u32, prgaccessentries: *mut *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectAccessRights(::core::mem::transmute_copy(&pobject), ::core::mem::transmute_copy(&pcaccessentries), ::core::mem::transmute_copy(&prgaccessentries)).into()
        }
        unsafe extern "system" fn GetObjectOwner<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectAccessControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut SEC_OBJECT, ppowner: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectOwner(::core::mem::transmute_copy(&pobject), ::core::mem::transmute_copy(&ppowner)).into()
        }
        unsafe extern "system" fn IsObjectAccessAllowed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectAccessControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut SEC_OBJECT, paccessentry: *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsObjectAccessAllowed(::core::mem::transmute_copy(&pobject), ::core::mem::transmute_copy(&paccessentry), ::core::mem::transmute_copy(&pfresult)).into()
        }
        unsafe extern "system" fn SetObjectAccessRights<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectAccessControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut SEC_OBJECT, caccessentries: u32, prgaccessentries: *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetObjectAccessRights(::core::mem::transmute_copy(&pobject), ::core::mem::transmute_copy(&caccessentries), ::core::mem::transmute_copy(&prgaccessentries)).into()
        }
        unsafe extern "system" fn SetObjectOwner<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectAccessControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut SEC_OBJECT, powner: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetObjectOwner(::core::mem::transmute_copy(&pobject), ::core::mem::transmute_copy(&powner)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetObjectAccessRights: GetObjectAccessRights::<Identity, Impl, OFFSET>,
            GetObjectOwner: GetObjectOwner::<Identity, Impl, OFFSET>,
            IsObjectAccessAllowed: IsObjectAccessAllowed::<Identity, Impl, OFFSET>,
            SetObjectAccessRights: SetObjectAccessRights::<Identity, Impl, OFFSET>,
            SetObjectOwner: SetObjectOwner::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectAccessControl as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOpLockStatus_Impl: Sized {
    fn IsOplockValid(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsOplockBroken(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetOplockEventHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IOpLockStatus {}
#[cfg(feature = "Win32_Foundation")]
impl IOpLockStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpLockStatus_Impl, const OFFSET: isize>() -> IOpLockStatus_Vtbl {
        unsafe extern "system" fn IsOplockValid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpLockStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisoplockvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOplockValid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisoplockvalid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOplockBroken<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpLockStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisoplockbroken: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOplockBroken() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisoplockbroken, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOplockEventHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpLockStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoplockev: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOplockEventHandle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phoplockev, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsOplockValid: IsOplockValid::<Identity, Impl, OFFSET>,
            IsOplockBroken: IsOplockBroken::<Identity, Impl, OFFSET>,
            GetOplockEventHandle: GetOplockEventHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpLockStatus as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IOpenRowset_Impl: Sized {
    fn OpenRowset(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IOpenRowset {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IOpenRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpenRowset_Impl, const OFFSET: isize>() -> IOpenRowset_Vtbl {
        unsafe extern "system" fn OpenRowset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpenRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenRowset(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pindexid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pprowset)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OpenRowset: OpenRowset::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpenRowset as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IParentRowset_Impl: Sized {
    fn GetChildRowset(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, iordinal: usize, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ::windows::core::RuntimeName for IParentRowset {}
impl IParentRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IParentRowset_Impl, const OFFSET: isize>() -> IParentRowset_Vtbl {
        unsafe extern "system" fn GetChildRowset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IParentRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, iordinal: usize, riid: *const ::windows::core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChildRowset(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&iordinal), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprowset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetChildRowset: GetChildRowset::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IParentRowset as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IProtocolHandlerSite_Impl: Sized {
    fn GetFilter(&self, pclsidobj: *mut ::windows::core::GUID, pcwszcontenttype: &::windows::core::PCWSTR, pcwszextension: &::windows::core::PCWSTR, ppfilter: *mut ::core::option::Option<super::super::Storage::IndexServer::IFilter>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::windows::core::RuntimeName for IProtocolHandlerSite {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IProtocolHandlerSite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProtocolHandlerSite_Impl, const OFFSET: isize>() -> IProtocolHandlerSite_Vtbl {
        unsafe extern "system" fn GetFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProtocolHandlerSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsidobj: *mut ::windows::core::GUID, pcwszcontenttype: ::windows::core::PCWSTR, pcwszextension: ::windows::core::PCWSTR, ppfilter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFilter(::core::mem::transmute_copy(&pclsidobj), ::core::mem::transmute(&pcwszcontenttype), ::core::mem::transmute(&pcwszextension), ::core::mem::transmute_copy(&ppfilter)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetFilter: GetFilter::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProtocolHandlerSite as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideMoniker_Impl: Sized {
    fn GetMoniker(&self) -> ::windows::core::Result<super::Com::IMoniker>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IProvideMoniker {}
#[cfg(feature = "Win32_System_Com")]
impl IProvideMoniker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvideMoniker_Impl, const OFFSET: isize>() -> IProvideMoniker_Vtbl {
        unsafe extern "system" fn GetMoniker<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvideMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimoniker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMoniker() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimoniker, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMoniker: GetMoniker::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideMoniker as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IQueryParser_Impl: Sized {
    fn Parse(&self, pszinputstring: &::windows::core::PCWSTR, pcustomproperties: ::core::option::Option<&super::Com::IEnumUnknown>) -> ::windows::core::Result<IQuerySolution>;
    fn SetOption(&self, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetOption(&self, option: STRUCTURED_QUERY_SINGLE_OPTION) -> ::windows::core::Result<super::Com::StructuredStorage::PROPVARIANT>;
    fn SetMultiOption(&self, option: STRUCTURED_QUERY_MULTIOPTION, pszoptionkey: &::windows::core::PCWSTR, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetSchemaProvider(&self) -> ::windows::core::Result<ISchemaProvider>;
    fn RestateToString(&self, pcondition: ::core::option::Option<&ICondition>, fuseenglish: super::super::Foundation::BOOL) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn ParsePropertyValue(&self, pszpropertyname: &::windows::core::PCWSTR, pszinputstring: &::windows::core::PCWSTR) -> ::windows::core::Result<IQuerySolution>;
    fn RestatePropertyValueToString(&self, pcondition: ::core::option::Option<&ICondition>, fuseenglish: super::super::Foundation::BOOL, ppszpropertyname: *mut ::windows::core::PWSTR, ppszquerystring: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IQueryParser {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IQueryParser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryParser_Impl, const OFFSET: isize>() -> IQueryParser_Vtbl {
        unsafe extern "system" fn Parse<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszinputstring: ::windows::core::PCWSTR, pcustomproperties: *mut ::core::ffi::c_void, ppsolution: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Parse(::core::mem::transmute(&pszinputstring), ::windows::core::from_raw_borrowed(&pcustomproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsolution, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOption(::core::mem::transmute_copy(&option), ::core::mem::transmute_copy(&poptionvalue)).into()
        }
        unsafe extern "system" fn GetOption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOption(::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(poptionvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiOption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: STRUCTURED_QUERY_MULTIOPTION, pszoptionkey: ::windows::core::PCWSTR, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMultiOption(::core::mem::transmute_copy(&option), ::core::mem::transmute(&pszoptionkey), ::core::mem::transmute_copy(&poptionvalue)).into()
        }
        unsafe extern "system" fn GetSchemaProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppschemaprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSchemaProvider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppschemaprovider, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestateToString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcondition: *mut ::core::ffi::c_void, fuseenglish: super::super::Foundation::BOOL, ppszquerystring: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RestateToString(::windows::core::from_raw_borrowed(&pcondition), ::core::mem::transmute_copy(&fuseenglish)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszquerystring, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParsePropertyValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows::core::PCWSTR, pszinputstring: ::windows::core::PCWSTR, ppsolution: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ParsePropertyValue(::core::mem::transmute(&pszpropertyname), ::core::mem::transmute(&pszinputstring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsolution, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestatePropertyValueToString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcondition: *mut ::core::ffi::c_void, fuseenglish: super::super::Foundation::BOOL, ppszpropertyname: *mut ::windows::core::PWSTR, ppszquerystring: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestatePropertyValueToString(::windows::core::from_raw_borrowed(&pcondition), ::core::mem::transmute_copy(&fuseenglish), ::core::mem::transmute_copy(&ppszpropertyname), ::core::mem::transmute_copy(&ppszquerystring)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IQueryParser as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IQueryParserManager_Impl: Sized {
    fn CreateLoadedParser(&self, pszcatalog: &::windows::core::PCWSTR, langidforkeywords: u16, riid: *const ::windows::core::GUID, ppqueryparser: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn InitializeOptions(&self, funderstandnqs: super::super::Foundation::BOOL, fautowildcard: super::super::Foundation::BOOL, pqueryparser: ::core::option::Option<&IQueryParser>) -> ::windows::core::Result<()>;
    fn SetOption(&self, option: QUERY_PARSER_MANAGER_OPTION, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IQueryParserManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IQueryParserManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryParserManager_Impl, const OFFSET: isize>() -> IQueryParserManager_Vtbl {
        unsafe extern "system" fn CreateLoadedParser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryParserManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcatalog: ::windows::core::PCWSTR, langidforkeywords: u16, riid: *const ::windows::core::GUID, ppqueryparser: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateLoadedParser(::core::mem::transmute(&pszcatalog), ::core::mem::transmute_copy(&langidforkeywords), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppqueryparser)).into()
        }
        unsafe extern "system" fn InitializeOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryParserManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, funderstandnqs: super::super::Foundation::BOOL, fautowildcard: super::super::Foundation::BOOL, pqueryparser: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeOptions(::core::mem::transmute_copy(&funderstandnqs), ::core::mem::transmute_copy(&fautowildcard), ::windows::core::from_raw_borrowed(&pqueryparser)).into()
        }
        unsafe extern "system" fn SetOption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryParserManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: QUERY_PARSER_MANAGER_OPTION, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOption(::core::mem::transmute_copy(&option), ::core::mem::transmute_copy(&poptionvalue)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateLoadedParser: CreateLoadedParser::<Identity, Impl, OFFSET>,
            InitializeOptions: InitializeOptions::<Identity, Impl, OFFSET>,
            SetOption: SetOption::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQueryParserManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
pub trait IQuerySolution_Impl: Sized + IConditionFactory_Impl {
    fn GetQuery(&self, ppquerynode: *mut ::core::option::Option<ICondition>, ppmaintype: *mut ::core::option::Option<IEntity>) -> ::windows::core::Result<()>;
    fn GetErrors(&self, riid: *const ::windows::core::GUID, ppparseerrors: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetLexicalData(&self, ppszinputstring: *mut ::windows::core::PWSTR, pptokens: *mut ::core::option::Option<ITokenCollection>, plcid: *mut u32, ppwordbreaker: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl ::windows::core::RuntimeName for IQuerySolution {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl IQuerySolution_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQuerySolution_Impl, const OFFSET: isize>() -> IQuerySolution_Vtbl {
        unsafe extern "system" fn GetQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQuerySolution_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppquerynode: *mut *mut ::core::ffi::c_void, ppmaintype: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetQuery(::core::mem::transmute_copy(&ppquerynode), ::core::mem::transmute_copy(&ppmaintype)).into()
        }
        unsafe extern "system" fn GetErrors<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQuerySolution_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppparseerrors: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetErrors(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppparseerrors)).into()
        }
        unsafe extern "system" fn GetLexicalData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQuerySolution_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszinputstring: *mut ::windows::core::PWSTR, pptokens: *mut *mut ::core::ffi::c_void, plcid: *mut u32, ppwordbreaker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLexicalData(::core::mem::transmute_copy(&ppszinputstring), ::core::mem::transmute_copy(&pptokens), ::core::mem::transmute_copy(&plcid), ::core::mem::transmute_copy(&ppwordbreaker)).into()
        }
        Self {
            base__: IConditionFactory_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetQuery: GetQuery::<Identity, Impl, OFFSET>,
            GetErrors: GetErrors::<Identity, Impl, OFFSET>,
            GetLexicalData: GetLexicalData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQuerySolution as ::windows::core::ComInterface>::IID || iid == &<IConditionFactory as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IReadData_Impl: Sized {
    fn ReadData(&self, hchapter: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, haccessor: HACCESSOR, crows: isize, pcrowsobtained: *mut usize, ppfixeddata: *mut *mut u8, pcbvariabletotal: *mut usize, ppvariabledata: *mut *mut u8) -> ::windows::core::Result<()>;
    fn ReleaseChapter(&self, hchapter: usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IReadData {}
impl IReadData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReadData_Impl, const OFFSET: isize>() -> IReadData_Vtbl {
        unsafe extern "system" fn ReadData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReadData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, haccessor: HACCESSOR, crows: isize, pcrowsobtained: *mut usize, ppfixeddata: *mut *mut u8, pcbvariabletotal: *mut usize, ppvariabledata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadData(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark), ::core::mem::transmute_copy(&lrowsoffset), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&pcrowsobtained), ::core::mem::transmute_copy(&ppfixeddata), ::core::mem::transmute_copy(&pcbvariabletotal), ::core::mem::transmute_copy(&ppvariabledata)).into()
        }
        unsafe extern "system" fn ReleaseChapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReadData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseChapter(::core::mem::transmute_copy(&hchapter)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadData: ReadData::<Identity, Impl, OFFSET>,
            ReleaseChapter: ReleaseChapter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReadData as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRegisterProvider_Impl: Sized {
    fn GetURLMapping(&self, pwszurl: &::windows::core::PCWSTR, dwreserved: usize) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetURLMapping(&self, pwszurl: &::windows::core::PCWSTR, dwreserved: usize, rclsidprovider: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn UnregisterProvider(&self, pwszurl: &::windows::core::PCWSTR, dwreserved: usize, rclsidprovider: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRegisterProvider {}
impl IRegisterProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisterProvider_Impl, const OFFSET: isize>() -> IRegisterProvider_Vtbl {
        unsafe extern "system" fn GetURLMapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisterProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, dwreserved: usize, pclsidprovider: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetURLMapping(::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsidprovider, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURLMapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisterProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, dwreserved: usize, rclsidprovider: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetURLMapping(::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&rclsidprovider)).into()
        }
        unsafe extern "system" fn UnregisterProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisterProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, dwreserved: usize, rclsidprovider: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterProvider(::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&rclsidprovider)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetURLMapping: GetURLMapping::<Identity, Impl, OFFSET>,
            SetURLMapping: SetURLMapping::<Identity, Impl, OFFSET>,
            UnregisterProvider: UnregisterProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegisterProvider as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRelationship_Impl: Sized {
    fn Name(&self, ppszname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn IsReal(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Destination(&self) -> ::windows::core::Result<IEntity>;
    fn MetaData(&self, riid: *const ::windows::core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn DefaultPhrase(&self, ppszphrase: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRelationship {}
#[cfg(feature = "Win32_Foundation")]
impl IRelationship_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRelationship_Impl, const OFFSET: isize>() -> IRelationship_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Name(::core::mem::transmute_copy(&ppszname)).into()
        }
        unsafe extern "system" fn IsReal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisreal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsReal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisreal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destination<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationentity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Destination() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdestinationentity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MetaData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MetaData(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pmetadata)).into()
        }
        unsafe extern "system" fn DefaultPhrase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszphrase: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DefaultPhrase(::core::mem::transmute_copy(&ppszphrase)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            IsReal: IsReal::<Identity, Impl, OFFSET>,
            Destination: Destination::<Identity, Impl, OFFSET>,
            MetaData: MetaData::<Identity, Impl, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRelationship as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IRichChunk_Impl: Sized {
    fn GetData(&self, pfirstpos: *mut u32, plength: *mut u32, ppsz: *mut ::windows::core::PWSTR, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IRichChunk {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IRichChunk_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRichChunk_Impl, const OFFSET: isize>() -> IRichChunk_Vtbl {
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRichChunk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfirstpos: *mut u32, plength: *mut u32, ppsz: *mut ::windows::core::PWSTR, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetData(::core::mem::transmute_copy(&pfirstpos), ::core::mem::transmute_copy(&plength), ::core::mem::transmute_copy(&ppsz), ::core::mem::transmute_copy(&pvalue)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetData: GetData::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichChunk as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IRow_Impl: Sized {
    fn GetColumns(&self, ccolumns: usize, rgcolumns: *mut DBCOLUMNACCESS) -> ::windows::core::Result<()>;
    fn GetSourceRowset(&self, riid: *const ::windows::core::GUID, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>, phrow: *mut usize) -> ::windows::core::Result<()>;
    fn Open(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, pcolumnid: *const super::super::Storage::IndexServer::DBID, rguidcolumntype: *const ::windows::core::GUID, dwbindflags: u32, riid: *const ::windows::core::GUID, ppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::windows::core::RuntimeName for IRow {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IRow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRow_Impl, const OFFSET: isize>() -> IRow_Vtbl {
        unsafe extern "system" fn GetColumns<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolumns: usize, rgcolumns: *mut DBCOLUMNACCESS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColumns(::core::mem::transmute_copy(&ccolumns), ::core::mem::transmute_copy(&rgcolumns)).into()
        }
        unsafe extern "system" fn GetSourceRowset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pprowset: *mut *mut ::core::ffi::c_void, phrow: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSourceRowset(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pprowset), ::core::mem::transmute_copy(&phrow)).into()
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pcolumnid: *const super::super::Storage::IndexServer::DBID, rguidcolumntype: *const ::windows::core::GUID, dwbindflags: u32, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&pcolumnid), ::core::mem::transmute_copy(&rguidcolumntype), ::core::mem::transmute_copy(&dwbindflags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetColumns: GetColumns::<Identity, Impl, OFFSET>,
            GetSourceRowset: GetSourceRowset::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRow as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IRowChange_Impl: Sized {
    fn SetColumns(&self, ccolumns: usize, rgcolumns: *const DBCOLUMNACCESS) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::windows::core::RuntimeName for IRowChange {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IRowChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowChange_Impl, const OFFSET: isize>() -> IRowChange_Vtbl {
        unsafe extern "system" fn SetColumns<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolumns: usize, rgcolumns: *const DBCOLUMNACCESS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColumns(::core::mem::transmute_copy(&ccolumns), ::core::mem::transmute_copy(&rgcolumns)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetColumns: SetColumns::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowChange as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowPosition_Impl: Sized {
    fn ClearRowPosition(&self) -> ::windows::core::Result<()>;
    fn GetRowPosition(&self, phchapter: *mut usize, phrow: *mut usize, pdwpositionflags: *mut u32) -> ::windows::core::Result<()>;
    fn GetRowset(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Initialize(&self, prowset: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetRowPosition(&self, hchapter: usize, hrow: usize, dwpositionflags: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowPosition {}
impl IRowPosition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowPosition_Impl, const OFFSET: isize>() -> IRowPosition_Vtbl {
        unsafe extern "system" fn ClearRowPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowPosition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearRowPosition().into()
        }
        unsafe extern "system" fn GetRowPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowPosition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phchapter: *mut usize, phrow: *mut usize, pdwpositionflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRowPosition(::core::mem::transmute_copy(&phchapter), ::core::mem::transmute_copy(&phrow), ::core::mem::transmute_copy(&pdwpositionflags)).into()
        }
        unsafe extern "system" fn GetRowset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowPosition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRowset(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprowset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowPosition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prowset: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows::core::from_raw_borrowed(&prowset)).into()
        }
        unsafe extern "system" fn SetRowPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowPosition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, hrow: usize, dwpositionflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRowPosition(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&dwpositionflags)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ClearRowPosition: ClearRowPosition::<Identity, Impl, OFFSET>,
            GetRowPosition: GetRowPosition::<Identity, Impl, OFFSET>,
            GetRowset: GetRowset::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            SetRowPosition: SetRowPosition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowPosition as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRowPositionChange_Impl: Sized {
    fn OnRowPositionChange(&self, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRowPositionChange {}
#[cfg(feature = "Win32_Foundation")]
impl IRowPositionChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowPositionChange_Impl, const OFFSET: isize>() -> IRowPositionChange_Vtbl {
        unsafe extern "system" fn OnRowPositionChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowPositionChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnRowPositionChange(::core::mem::transmute_copy(&ereason), ::core::mem::transmute_copy(&ephase), ::core::mem::transmute_copy(&fcantdeny)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnRowPositionChange: OnRowPositionChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowPositionChange as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait IRowSchemaChange_Impl: Sized + IRowChange_Impl {
    fn DeleteColumns(&self, ccolumns: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgdwstatus: *mut u32) -> ::windows::core::Result<()>;
    fn AddColumns(&self, ccolumns: usize, rgnewcolumninfo: *const DBCOLUMNINFO, rgcolumns: *mut DBCOLUMNACCESS) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IRowSchemaChange {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl IRowSchemaChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowSchemaChange_Impl, const OFFSET: isize>() -> IRowSchemaChange_Vtbl {
        unsafe extern "system" fn DeleteColumns<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowSchemaChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolumns: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteColumns(::core::mem::transmute_copy(&ccolumns), ::core::mem::transmute_copy(&rgcolumnids), ::core::mem::transmute_copy(&rgdwstatus)).into()
        }
        unsafe extern "system" fn AddColumns<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowSchemaChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolumns: usize, rgnewcolumninfo: *const DBCOLUMNINFO, rgcolumns: *mut DBCOLUMNACCESS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddColumns(::core::mem::transmute_copy(&ccolumns), ::core::mem::transmute_copy(&rgnewcolumninfo), ::core::mem::transmute_copy(&rgcolumns)).into()
        }
        Self {
            base__: IRowChange_Vtbl::new::<Identity, Impl, OFFSET>(),
            DeleteColumns: DeleteColumns::<Identity, Impl, OFFSET>,
            AddColumns: AddColumns::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowSchemaChange as ::windows::core::ComInterface>::IID || iid == &<IRowChange as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowset_Impl: Sized {
    fn AddRefRows(&self, crows: usize, rghrows: *const usize, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows::core::Result<()>;
    fn GetData(&self, hrow: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetNextRows(&self, hreserved: usize, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::Result<()>;
    fn ReleaseRows(&self, crows: usize, rghrows: *const usize, rgrowoptions: *mut u32, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows::core::Result<()>;
    fn RestartPosition(&self, hreserved: usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowset {}
impl IRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowset_Impl, const OFFSET: isize>() -> IRowset_Vtbl {
        unsafe extern "system" fn AddRefRows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crows: usize, rghrows: *const usize, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRefRows(::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&rgrefcounts), ::core::mem::transmute_copy(&rgrowstatus)).into()
        }
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetData(::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetNextRows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNextRows(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&lrowsoffset), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&pcrowsobtained), ::core::mem::transmute_copy(&prghrows)).into()
        }
        unsafe extern "system" fn ReleaseRows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crows: usize, rghrows: *const usize, rgrowoptions: *mut u32, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseRows(::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&rgrowoptions), ::core::mem::transmute_copy(&rgrefcounts), ::core::mem::transmute_copy(&rgrowstatus)).into()
        }
        unsafe extern "system" fn RestartPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestartPosition(::core::mem::transmute_copy(&hreserved)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRefRows: AddRefRows::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            GetNextRows: GetNextRows::<Identity, Impl, OFFSET>,
            ReleaseRows: ReleaseRows::<Identity, Impl, OFFSET>,
            RestartPosition: RestartPosition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowset as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRowsetAsynch_Impl: Sized {
    fn RatioFinished(&self, puldenominator: *mut usize, pulnumerator: *mut usize, pcrows: *mut usize, pfnewrows: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRowsetAsynch {}
#[cfg(feature = "Win32_Foundation")]
impl IRowsetAsynch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetAsynch_Impl, const OFFSET: isize>() -> IRowsetAsynch_Vtbl {
        unsafe extern "system" fn RatioFinished<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetAsynch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puldenominator: *mut usize, pulnumerator: *mut usize, pcrows: *mut usize, pfnewrows: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RatioFinished(::core::mem::transmute_copy(&puldenominator), ::core::mem::transmute_copy(&pulnumerator), ::core::mem::transmute_copy(&pcrows), ::core::mem::transmute_copy(&pfnewrows)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetAsynch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RatioFinished: RatioFinished::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetAsynch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetBookmark_Impl: Sized {
    fn PositionOnBookmark(&self, hchapter: usize, cbbookmark: usize, pbookmark: *const u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetBookmark {}
impl IRowsetBookmark_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetBookmark_Impl, const OFFSET: isize>() -> IRowsetBookmark_Vtbl {
        unsafe extern "system" fn PositionOnBookmark<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetBookmark_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, cbbookmark: usize, pbookmark: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PositionOnBookmark(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), PositionOnBookmark: PositionOnBookmark::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetBookmark as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetChange_Impl: Sized {
    fn DeleteRows(&self, hreserved: usize, crows: usize, rghrows: *const usize, rgrowstatus: *mut u32) -> ::windows::core::Result<()>;
    fn SetData(&self, hrow: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn InsertRow(&self, hreserved: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void, phrow: *mut usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetChange {}
impl IRowsetChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetChange_Impl, const OFFSET: isize>() -> IRowsetChange_Vtbl {
        unsafe extern "system" fn DeleteRows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, rgrowstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRows(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&rgrowstatus)).into()
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetData(::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn InsertRow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void, phrow: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertRow(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&phrow)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeleteRows: DeleteRows::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
            InsertRow: InsertRow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetChange as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetChangeExtInfo_Impl: Sized {
    fn GetOriginalRow(&self, hreserved: usize, hrow: usize, phroworiginal: *mut usize) -> ::windows::core::Result<()>;
    fn GetPendingColumns(&self, hreserved: usize, hrow: usize, ccolumnordinals: u32, rgiordinals: *const u32, rgcolumnstatus: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetChangeExtInfo {}
impl IRowsetChangeExtInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetChangeExtInfo_Impl, const OFFSET: isize>() -> IRowsetChangeExtInfo_Vtbl {
        unsafe extern "system" fn GetOriginalRow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetChangeExtInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, hrow: usize, phroworiginal: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOriginalRow(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&phroworiginal)).into()
        }
        unsafe extern "system" fn GetPendingColumns<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetChangeExtInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, hrow: usize, ccolumnordinals: u32, rgiordinals: *const u32, rgcolumnstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPendingColumns(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&ccolumnordinals), ::core::mem::transmute_copy(&rgiordinals), ::core::mem::transmute_copy(&rgcolumnstatus)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOriginalRow: GetOriginalRow::<Identity, Impl, OFFSET>,
            GetPendingColumns: GetPendingColumns::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetChangeExtInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetChapterMember_Impl: Sized {
    fn IsRowInChapter(&self, hchapter: usize, hrow: usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetChapterMember {}
impl IRowsetChapterMember_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetChapterMember_Impl, const OFFSET: isize>() -> IRowsetChapterMember_Vtbl {
        unsafe extern "system" fn IsRowInChapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetChapterMember_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, hrow: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsRowInChapter(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&hrow)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsRowInChapter: IsRowInChapter::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetChapterMember as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetCopyRows_Impl: Sized {
    fn CloseSource(&self, hsourceid: u16) -> ::windows::core::Result<()>;
    fn CopyByHROWS(&self, hsourceid: u16, hreserved: usize, crows: isize, rghrows: *const usize, bflags: u32) -> ::windows::core::Result<()>;
    fn CopyRows(&self, hsourceid: u16, hreserved: usize, crows: isize, bflags: u32, pcrowscopied: *mut usize) -> ::windows::core::Result<()>;
    fn DefineSource(&self, prowsetsource: ::core::option::Option<&IRowset>, ccolids: usize, rgsourcecolumns: *const isize, rgtargetcolumns: *const isize, phsourceid: *mut u16) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetCopyRows {}
impl IRowsetCopyRows_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetCopyRows_Impl, const OFFSET: isize>() -> IRowsetCopyRows_Vtbl {
        unsafe extern "system" fn CloseSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetCopyRows_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsourceid: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseSource(::core::mem::transmute_copy(&hsourceid)).into()
        }
        unsafe extern "system" fn CopyByHROWS<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetCopyRows_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsourceid: u16, hreserved: usize, crows: isize, rghrows: *const usize, bflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyByHROWS(::core::mem::transmute_copy(&hsourceid), ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&bflags)).into()
        }
        unsafe extern "system" fn CopyRows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetCopyRows_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsourceid: u16, hreserved: usize, crows: isize, bflags: u32, pcrowscopied: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyRows(::core::mem::transmute_copy(&hsourceid), ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&bflags), ::core::mem::transmute_copy(&pcrowscopied)).into()
        }
        unsafe extern "system" fn DefineSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetCopyRows_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prowsetsource: *mut ::core::ffi::c_void, ccolids: usize, rgsourcecolumns: *const isize, rgtargetcolumns: *const isize, phsourceid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DefineSource(::windows::core::from_raw_borrowed(&prowsetsource), ::core::mem::transmute_copy(&ccolids), ::core::mem::transmute_copy(&rgsourcecolumns), ::core::mem::transmute_copy(&rgtargetcolumns), ::core::mem::transmute_copy(&phsourceid)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CloseSource: CloseSource::<Identity, Impl, OFFSET>,
            CopyByHROWS: CopyByHROWS::<Identity, Impl, OFFSET>,
            CopyRows: CopyRows::<Identity, Impl, OFFSET>,
            DefineSource: DefineSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetCopyRows as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRowsetCurrentIndex_Impl: Sized + IRowsetIndex_Impl {
    fn GetIndex(&self, ppindexid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
    fn SetIndex(&self, pindexid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRowsetCurrentIndex {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRowsetCurrentIndex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetCurrentIndex_Impl, const OFFSET: isize>() -> IRowsetCurrentIndex_Vtbl {
        unsafe extern "system" fn GetIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetCurrentIndex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppindexid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIndex(::core::mem::transmute_copy(&ppindexid)).into()
        }
        unsafe extern "system" fn SetIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetCurrentIndex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIndex(::core::mem::transmute_copy(&pindexid)).into()
        }
        Self {
            base__: IRowsetIndex_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
            SetIndex: SetIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetCurrentIndex as ::windows::core::ComInterface>::IID || iid == &<IRowsetIndex as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IRowsetEvents_Impl: Sized {
    fn OnNewItem(&self, itemid: *const super::Com::StructuredStorage::PROPVARIANT, newitemstate: ROWSETEVENT_ITEMSTATE) -> ::windows::core::Result<()>;
    fn OnChangedItem(&self, itemid: *const super::Com::StructuredStorage::PROPVARIANT, rowsetitemstate: ROWSETEVENT_ITEMSTATE, changeditemstate: ROWSETEVENT_ITEMSTATE) -> ::windows::core::Result<()>;
    fn OnDeletedItem(&self, itemid: *const super::Com::StructuredStorage::PROPVARIANT, deleteditemstate: ROWSETEVENT_ITEMSTATE) -> ::windows::core::Result<()>;
    fn OnRowsetEvent(&self, eventtype: ROWSETEVENT_TYPE, eventdata: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IRowsetEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IRowsetEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetEvents_Impl, const OFFSET: isize>() -> IRowsetEvents_Vtbl {
        unsafe extern "system" fn OnNewItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: *const super::Com::StructuredStorage::PROPVARIANT, newitemstate: ROWSETEVENT_ITEMSTATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnNewItem(::core::mem::transmute_copy(&itemid), ::core::mem::transmute_copy(&newitemstate)).into()
        }
        unsafe extern "system" fn OnChangedItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: *const super::Com::StructuredStorage::PROPVARIANT, rowsetitemstate: ROWSETEVENT_ITEMSTATE, changeditemstate: ROWSETEVENT_ITEMSTATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnChangedItem(::core::mem::transmute_copy(&itemid), ::core::mem::transmute_copy(&rowsetitemstate), ::core::mem::transmute_copy(&changeditemstate)).into()
        }
        unsafe extern "system" fn OnDeletedItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: *const super::Com::StructuredStorage::PROPVARIANT, deleteditemstate: ROWSETEVENT_ITEMSTATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDeletedItem(::core::mem::transmute_copy(&itemid), ::core::mem::transmute_copy(&deleteditemstate)).into()
        }
        unsafe extern "system" fn OnRowsetEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventtype: ROWSETEVENT_TYPE, eventdata: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnRowsetEvent(::core::mem::transmute_copy(&eventtype), ::core::mem::transmute_copy(&eventdata)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnNewItem: OnNewItem::<Identity, Impl, OFFSET>,
            OnChangedItem: OnChangedItem::<Identity, Impl, OFFSET>,
            OnDeletedItem: OnDeletedItem::<Identity, Impl, OFFSET>,
            OnRowsetEvent: OnRowsetEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRowsetFastLoad_Impl: Sized {
    fn InsertRow(&self, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Commit(&self, fdone: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRowsetFastLoad {}
#[cfg(feature = "Win32_Foundation")]
impl IRowsetFastLoad_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetFastLoad_Impl, const OFFSET: isize>() -> IRowsetFastLoad_Vtbl {
        unsafe extern "system" fn InsertRow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetFastLoad_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertRow(::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetFastLoad_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdone: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit(::core::mem::transmute_copy(&fdone)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InsertRow: InsertRow::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetFastLoad as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetFind_Impl: Sized {
    fn FindNextRow(&self, hchapter: usize, haccessor: HACCESSOR, pfindvalue: *mut ::core::ffi::c_void, compareop: u32, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetFind {}
impl IRowsetFind_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetFind_Impl, const OFFSET: isize>() -> IRowsetFind_Vtbl {
        unsafe extern "system" fn FindNextRow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetFind_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, haccessor: HACCESSOR, pfindvalue: *mut ::core::ffi::c_void, compareop: u32, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindNextRow(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pfindvalue), ::core::mem::transmute_copy(&compareop), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark), ::core::mem::transmute_copy(&lrowsoffset), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&pcrowsobtained), ::core::mem::transmute_copy(&prghrows)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FindNextRow: FindNextRow::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetFind as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetIdentity_Impl: Sized {
    fn IsSameRow(&self, hthisrow: usize, hthatrow: usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetIdentity {}
impl IRowsetIdentity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetIdentity_Impl, const OFFSET: isize>() -> IRowsetIdentity_Vtbl {
        unsafe extern "system" fn IsSameRow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hthisrow: usize, hthatrow: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsSameRow(::core::mem::transmute_copy(&hthisrow), ::core::mem::transmute_copy(&hthatrow)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsSameRow: IsSameRow::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetIdentity as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRowsetIndex_Impl: Sized {
    fn GetIndexInfo(&self, pckeycolumns: *mut usize, prgindexcolumndesc: *mut *mut DBINDEXCOLUMNDESC, pcindexpropertysets: *mut u32, prgindexpropertysets: *mut *mut DBPROPSET) -> ::windows::core::Result<()>;
    fn Seek(&self, haccessor: HACCESSOR, ckeyvalues: usize, pdata: *mut ::core::ffi::c_void, dwseekoptions: u32) -> ::windows::core::Result<()>;
    fn SetRange(&self, haccessor: HACCESSOR, cstartkeycolumns: usize, pstartdata: *mut ::core::ffi::c_void, cendkeycolumns: usize, penddata: *mut ::core::ffi::c_void, dwrangeoptions: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRowsetIndex {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRowsetIndex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetIndex_Impl, const OFFSET: isize>() -> IRowsetIndex_Vtbl {
        unsafe extern "system" fn GetIndexInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetIndex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pckeycolumns: *mut usize, prgindexcolumndesc: *mut *mut DBINDEXCOLUMNDESC, pcindexpropertysets: *mut u32, prgindexpropertysets: *mut *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIndexInfo(::core::mem::transmute_copy(&pckeycolumns), ::core::mem::transmute_copy(&prgindexcolumndesc), ::core::mem::transmute_copy(&pcindexpropertysets), ::core::mem::transmute_copy(&prgindexpropertysets)).into()
        }
        unsafe extern "system" fn Seek<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetIndex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haccessor: HACCESSOR, ckeyvalues: usize, pdata: *mut ::core::ffi::c_void, dwseekoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Seek(::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&ckeyvalues), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwseekoptions)).into()
        }
        unsafe extern "system" fn SetRange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetIndex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haccessor: HACCESSOR, cstartkeycolumns: usize, pstartdata: *mut ::core::ffi::c_void, cendkeycolumns: usize, penddata: *mut ::core::ffi::c_void, dwrangeoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRange(::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&cstartkeycolumns), ::core::mem::transmute_copy(&pstartdata), ::core::mem::transmute_copy(&cendkeycolumns), ::core::mem::transmute_copy(&penddata), ::core::mem::transmute_copy(&dwrangeoptions)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIndexInfo: GetIndexInfo::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            SetRange: SetRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetIndex as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRowsetInfo_Impl: Sized {
    fn GetProperties(&self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::Result<()>;
    fn GetReferencedRowset(&self, iordinal: usize, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetSpecification(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRowsetInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRowsetInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetInfo_Impl, const OFFSET: isize>() -> IRowsetInfo_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProperties(::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)).into()
        }
        unsafe extern "system" fn GetReferencedRowset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iordinal: usize, riid: *const ::windows::core::GUID, ppreferencedrowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReferencedRowset(::core::mem::transmute_copy(&iordinal), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreferencedrowset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpecification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppspecification: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSpecification(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppspecification, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetReferencedRowset: GetReferencedRowset::<Identity, Impl, OFFSET>,
            GetSpecification: GetSpecification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetKeys_Impl: Sized {
    fn ListKeys(&self, pccolumns: *mut usize, prgcolumns: *mut *mut usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetKeys {}
impl IRowsetKeys_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetKeys_Impl, const OFFSET: isize>() -> IRowsetKeys_Vtbl {
        unsafe extern "system" fn ListKeys<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccolumns: *mut usize, prgcolumns: *mut *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ListKeys(::core::mem::transmute_copy(&pccolumns), ::core::mem::transmute_copy(&prgcolumns)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ListKeys: ListKeys::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetKeys as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetLocate_Impl: Sized + IRowset_Impl {
    fn Compare(&self, hreserved: usize, cbbookmark1: usize, pbookmark1: *const u8, cbbookmark2: usize, pbookmark2: *const u8, pcomparison: *mut u32) -> ::windows::core::Result<()>;
    fn GetRowsAt(&self, hreserved1: usize, hreserved2: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::Result<()>;
    fn GetRowsByBookmark(&self, hreserved: usize, crows: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghrows: *mut usize, rgrowstatus: *mut u32) -> ::windows::core::Result<()>;
    fn Hash(&self, hreserved: usize, cbookmarks: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghashedvalues: *mut usize, rgbookmarkstatus: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetLocate {}
impl IRowsetLocate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetLocate_Impl, const OFFSET: isize>() -> IRowsetLocate_Vtbl {
        unsafe extern "system" fn Compare<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetLocate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, cbbookmark1: usize, pbookmark1: *const u8, cbbookmark2: usize, pbookmark2: *const u8, pcomparison: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Compare(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&cbbookmark1), ::core::mem::transmute_copy(&pbookmark1), ::core::mem::transmute_copy(&cbbookmark2), ::core::mem::transmute_copy(&pbookmark2), ::core::mem::transmute_copy(&pcomparison)).into()
        }
        unsafe extern "system" fn GetRowsAt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetLocate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved1: usize, hreserved2: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRowsAt(::core::mem::transmute_copy(&hreserved1), ::core::mem::transmute_copy(&hreserved2), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark), ::core::mem::transmute_copy(&lrowsoffset), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&pcrowsobtained), ::core::mem::transmute_copy(&prghrows)).into()
        }
        unsafe extern "system" fn GetRowsByBookmark<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetLocate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghrows: *mut usize, rgrowstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRowsByBookmark(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rgcbbookmarks), ::core::mem::transmute_copy(&rgpbookmarks), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&rgrowstatus)).into()
        }
        unsafe extern "system" fn Hash<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetLocate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, cbookmarks: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghashedvalues: *mut usize, rgbookmarkstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Hash(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&cbookmarks), ::core::mem::transmute_copy(&rgcbbookmarks), ::core::mem::transmute_copy(&rgpbookmarks), ::core::mem::transmute_copy(&rghashedvalues), ::core::mem::transmute_copy(&rgbookmarkstatus)).into()
        }
        Self {
            base__: IRowset_Vtbl::new::<Identity, Impl, OFFSET>(),
            Compare: Compare::<Identity, Impl, OFFSET>,
            GetRowsAt: GetRowsAt::<Identity, Impl, OFFSET>,
            GetRowsByBookmark: GetRowsByBookmark::<Identity, Impl, OFFSET>,
            Hash: Hash::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetLocate as ::windows::core::ComInterface>::IID || iid == &<IRowset as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetNewRowAfter_Impl: Sized {
    fn SetNewDataAfter(&self, hchapter: usize, cbbmprevious: u32, pbmprevious: *const u8, haccessor: HACCESSOR, pdata: *mut u8, phrow: *mut usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetNewRowAfter {}
impl IRowsetNewRowAfter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetNewRowAfter_Impl, const OFFSET: isize>() -> IRowsetNewRowAfter_Vtbl {
        unsafe extern "system" fn SetNewDataAfter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetNewRowAfter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, cbbmprevious: u32, pbmprevious: *const u8, haccessor: HACCESSOR, pdata: *mut u8, phrow: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNewDataAfter(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&cbbmprevious), ::core::mem::transmute_copy(&pbmprevious), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&phrow)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetNewDataAfter: SetNewDataAfter::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetNewRowAfter as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetNextRowset_Impl: Sized {
    fn GetNextRowset(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ::windows::core::RuntimeName for IRowsetNextRowset {}
impl IRowsetNextRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetNextRowset_Impl, const OFFSET: isize>() -> IRowsetNextRowset_Vtbl {
        unsafe extern "system" fn GetNextRowset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetNextRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppnextrowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNextRowset(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnextrowset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetNextRowset: GetNextRowset::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetNextRowset as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRowsetNotify_Impl: Sized {
    fn OnFieldChange(&self, prowset: ::core::option::Option<&IRowset>, hrow: usize, ccolumns: usize, rgcolumns: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnRowChange(&self, prowset: ::core::option::Option<&IRowset>, crows: usize, rghrows: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnRowsetChange(&self, prowset: ::core::option::Option<&IRowset>, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRowsetNotify {}
#[cfg(feature = "Win32_Foundation")]
impl IRowsetNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetNotify_Impl, const OFFSET: isize>() -> IRowsetNotify_Vtbl {
        unsafe extern "system" fn OnFieldChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prowset: *mut ::core::ffi::c_void, hrow: usize, ccolumns: usize, rgcolumns: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnFieldChange(::windows::core::from_raw_borrowed(&prowset), ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&ccolumns), ::core::mem::transmute_copy(&rgcolumns), ::core::mem::transmute_copy(&ereason), ::core::mem::transmute_copy(&ephase), ::core::mem::transmute_copy(&fcantdeny)).into()
        }
        unsafe extern "system" fn OnRowChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prowset: *mut ::core::ffi::c_void, crows: usize, rghrows: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnRowChange(::windows::core::from_raw_borrowed(&prowset), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&ereason), ::core::mem::transmute_copy(&ephase), ::core::mem::transmute_copy(&fcantdeny)).into()
        }
        unsafe extern "system" fn OnRowsetChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prowset: *mut ::core::ffi::c_void, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnRowsetChange(::windows::core::from_raw_borrowed(&prowset), ::core::mem::transmute_copy(&ereason), ::core::mem::transmute_copy(&ephase), ::core::mem::transmute_copy(&fcantdeny)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnFieldChange: OnFieldChange::<Identity, Impl, OFFSET>,
            OnRowChange: OnRowChange::<Identity, Impl, OFFSET>,
            OnRowsetChange: OnRowsetChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetNotify as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetPrioritization_Impl: Sized {
    fn SetScopePriority(&self, priority: PRIORITY_LEVEL, scopestatisticseventfrequency: u32) -> ::windows::core::Result<()>;
    fn GetScopePriority(&self, priority: *mut PRIORITY_LEVEL, scopestatisticseventfrequency: *mut u32) -> ::windows::core::Result<()>;
    fn GetScopeStatistics(&self, indexeddocumentcount: *mut u32, oustandingaddcount: *mut u32, oustandingmodifycount: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetPrioritization {}
impl IRowsetPrioritization_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetPrioritization_Impl, const OFFSET: isize>() -> IRowsetPrioritization_Vtbl {
        unsafe extern "system" fn SetScopePriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetPrioritization_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: PRIORITY_LEVEL, scopestatisticseventfrequency: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScopePriority(::core::mem::transmute_copy(&priority), ::core::mem::transmute_copy(&scopestatisticseventfrequency)).into()
        }
        unsafe extern "system" fn GetScopePriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetPrioritization_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: *mut PRIORITY_LEVEL, scopestatisticseventfrequency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScopePriority(::core::mem::transmute_copy(&priority), ::core::mem::transmute_copy(&scopestatisticseventfrequency)).into()
        }
        unsafe extern "system" fn GetScopeStatistics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetPrioritization_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexeddocumentcount: *mut u32, oustandingaddcount: *mut u32, oustandingmodifycount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScopeStatistics(::core::mem::transmute_copy(&indexeddocumentcount), ::core::mem::transmute_copy(&oustandingaddcount), ::core::mem::transmute_copy(&oustandingmodifycount)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetScopePriority: SetScopePriority::<Identity, Impl, OFFSET>,
            GetScopePriority: GetScopePriority::<Identity, Impl, OFFSET>,
            GetScopeStatistics: GetScopeStatistics::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetPrioritization as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetQueryStatus_Impl: Sized {
    fn GetStatus(&self, pdwstatus: *mut u32) -> ::windows::core::Result<()>;
    fn GetStatusEx(&self, pdwstatus: *mut u32, pcfiltereddocuments: *mut u32, pcdocumentstofilter: *mut u32, pdwratiofinisheddenominator: *mut usize, pdwratiofinishednumerator: *mut usize, cbbmk: usize, pbmk: *const u8, pirowbmk: *mut usize, pcrowstotal: *mut usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetQueryStatus {}
impl IRowsetQueryStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetQueryStatus_Impl, const OFFSET: isize>() -> IRowsetQueryStatus_Vtbl {
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetQueryStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatus(::core::mem::transmute_copy(&pdwstatus)).into()
        }
        unsafe extern "system" fn GetStatusEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetQueryStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32, pcfiltereddocuments: *mut u32, pcdocumentstofilter: *mut u32, pdwratiofinisheddenominator: *mut usize, pdwratiofinishednumerator: *mut usize, cbbmk: usize, pbmk: *const u8, pirowbmk: *mut usize, pcrowstotal: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatusEx(::core::mem::transmute_copy(&pdwstatus), ::core::mem::transmute_copy(&pcfiltereddocuments), ::core::mem::transmute_copy(&pcdocumentstofilter), ::core::mem::transmute_copy(&pdwratiofinisheddenominator), ::core::mem::transmute_copy(&pdwratiofinishednumerator), ::core::mem::transmute_copy(&cbbmk), ::core::mem::transmute_copy(&pbmk), ::core::mem::transmute_copy(&pirowbmk), ::core::mem::transmute_copy(&pcrowstotal)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetStatusEx: GetStatusEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetQueryStatus as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRowsetRefresh_Impl: Sized {
    fn RefreshVisibleData(&self, hchapter: usize, crows: usize, rghrows: *const usize, foverwrite: super::super::Foundation::BOOL, pcrowsrefreshed: *mut usize, prghrowsrefreshed: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::Result<()>;
    fn GetLastVisibleData(&self, hrow: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRowsetRefresh {}
#[cfg(feature = "Win32_Foundation")]
impl IRowsetRefresh_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetRefresh_Impl, const OFFSET: isize>() -> IRowsetRefresh_Vtbl {
        unsafe extern "system" fn RefreshVisibleData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetRefresh_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, crows: usize, rghrows: *const usize, foverwrite: super::super::Foundation::BOOL, pcrowsrefreshed: *mut usize, prghrowsrefreshed: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RefreshVisibleData(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&foverwrite), ::core::mem::transmute_copy(&pcrowsrefreshed), ::core::mem::transmute_copy(&prghrowsrefreshed), ::core::mem::transmute_copy(&prgrowstatus)).into()
        }
        unsafe extern "system" fn GetLastVisibleData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetRefresh_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLastVisibleData(::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RefreshVisibleData: RefreshVisibleData::<Identity, Impl, OFFSET>,
            GetLastVisibleData: GetLastVisibleData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetRefresh as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetResynch_Impl: Sized {
    fn GetVisibleData(&self, hrow: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ResynchRows(&self, crows: usize, rghrows: *const usize, pcrowsresynched: *mut usize, prghrowsresynched: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetResynch {}
impl IRowsetResynch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetResynch_Impl, const OFFSET: isize>() -> IRowsetResynch_Vtbl {
        unsafe extern "system" fn GetVisibleData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetResynch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVisibleData(::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn ResynchRows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetResynch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crows: usize, rghrows: *const usize, pcrowsresynched: *mut usize, prghrowsresynched: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResynchRows(::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&pcrowsresynched), ::core::mem::transmute_copy(&prghrowsresynched), ::core::mem::transmute_copy(&prgrowstatus)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVisibleData: GetVisibleData::<Identity, Impl, OFFSET>,
            ResynchRows: ResynchRows::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetResynch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetScroll_Impl: Sized + IRowsetLocate_Impl {
    fn GetApproximatePosition(&self, hreserved: usize, cbbookmark: usize, pbookmark: *const u8, pulposition: *mut usize, pcrows: *mut usize) -> ::windows::core::Result<()>;
    fn GetRowsAtRatio(&self, hreserved1: usize, hreserved2: usize, ulnumerator: usize, uldenominator: usize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetScroll {}
impl IRowsetScroll_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetScroll_Impl, const OFFSET: isize>() -> IRowsetScroll_Vtbl {
        unsafe extern "system" fn GetApproximatePosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetScroll_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, cbbookmark: usize, pbookmark: *const u8, pulposition: *mut usize, pcrows: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetApproximatePosition(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark), ::core::mem::transmute_copy(&pulposition), ::core::mem::transmute_copy(&pcrows)).into()
        }
        unsafe extern "system" fn GetRowsAtRatio<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetScroll_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved1: usize, hreserved2: usize, ulnumerator: usize, uldenominator: usize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRowsAtRatio(::core::mem::transmute_copy(&hreserved1), ::core::mem::transmute_copy(&hreserved2), ::core::mem::transmute_copy(&ulnumerator), ::core::mem::transmute_copy(&uldenominator), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&pcrowsobtained), ::core::mem::transmute_copy(&prghrows)).into()
        }
        Self {
            base__: IRowsetLocate_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetApproximatePosition: GetApproximatePosition::<Identity, Impl, OFFSET>,
            GetRowsAtRatio: GetRowsAtRatio::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetScroll as ::windows::core::ComInterface>::IID || iid == &<IRowset as ::windows::core::ComInterface>::IID || iid == &<IRowsetLocate as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetUpdate_Impl: Sized + IRowsetChange_Impl {
    fn GetOriginalData(&self, hrow: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetPendingRows(&self, hreserved: usize, dwrowstatus: u32, pcpendingrows: *mut usize, prgpendingrows: *mut *mut usize, prgpendingstatus: *mut *mut u32) -> ::windows::core::Result<()>;
    fn GetRowStatus(&self, hreserved: usize, crows: usize, rghrows: *const usize, rgpendingstatus: *mut u32) -> ::windows::core::Result<()>;
    fn Undo(&self, hreserved: usize, crows: usize, rghrows: *const usize, pcrowsundone: *mut usize, prgrowsundone: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::Result<()>;
    fn Update(&self, hreserved: usize, crows: usize, rghrows: *const usize, pcrows: *mut usize, prgrows: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetUpdate {}
impl IRowsetUpdate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetUpdate_Impl, const OFFSET: isize>() -> IRowsetUpdate_Vtbl {
        unsafe extern "system" fn GetOriginalData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOriginalData(::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetPendingRows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, dwrowstatus: u32, pcpendingrows: *mut usize, prgpendingrows: *mut *mut usize, prgpendingstatus: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPendingRows(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&dwrowstatus), ::core::mem::transmute_copy(&pcpendingrows), ::core::mem::transmute_copy(&prgpendingrows), ::core::mem::transmute_copy(&prgpendingstatus)).into()
        }
        unsafe extern "system" fn GetRowStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, rgpendingstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRowStatus(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&rgpendingstatus)).into()
        }
        unsafe extern "system" fn Undo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, pcrowsundone: *mut usize, prgrowsundone: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Undo(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&pcrowsundone), ::core::mem::transmute_copy(&prgrowsundone), ::core::mem::transmute_copy(&prgrowstatus)).into()
        }
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, pcrows: *mut usize, prgrows: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Update(::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&pcrows), ::core::mem::transmute_copy(&prgrows), ::core::mem::transmute_copy(&prgrowstatus)).into()
        }
        Self {
            base__: IRowsetChange_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOriginalData: GetOriginalData::<Identity, Impl, OFFSET>,
            GetPendingRows: GetPendingRows::<Identity, Impl, OFFSET>,
            GetRowStatus: GetRowStatus::<Identity, Impl, OFFSET>,
            Undo: Undo::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetUpdate as ::windows::core::ComInterface>::IID || iid == &<IRowsetChange as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetView_Impl: Sized {
    fn CreateView(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetView(&self, hchapter: usize, riid: *const ::windows::core::GUID, phchaptersource: *mut usize, ppview: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetView {}
impl IRowsetView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetView_Impl, const OFFSET: isize>() -> IRowsetView_Vtbl {
        unsafe extern "system" fn CreateView<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateView(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppview, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hchapter: usize, riid: *const ::windows::core::GUID, phchaptersource: *mut usize, ppview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetView(::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&phchaptersource), ::core::mem::transmute_copy(&ppview)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateView: CreateView::<Identity, Impl, OFFSET>,
            GetView: GetView::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetView as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetWatchAll_Impl: Sized {
    fn Acknowledge(&self) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn StopWatching(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetWatchAll {}
impl IRowsetWatchAll_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetWatchAll_Impl, const OFFSET: isize>() -> IRowsetWatchAll_Vtbl {
        unsafe extern "system" fn Acknowledge<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetWatchAll_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Acknowledge().into()
        }
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetWatchAll_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start().into()
        }
        unsafe extern "system" fn StopWatching<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetWatchAll_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopWatching().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Acknowledge: Acknowledge::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            StopWatching: StopWatching::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetWatchAll as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetWatchNotify_Impl: Sized {
    fn OnChange(&self, prowset: ::core::option::Option<&IRowset>, echangereason: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetWatchNotify {}
impl IRowsetWatchNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetWatchNotify_Impl, const OFFSET: isize>() -> IRowsetWatchNotify_Vtbl {
        unsafe extern "system" fn OnChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetWatchNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prowset: *mut ::core::ffi::c_void, echangereason: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnChange(::windows::core::from_raw_borrowed(&prowset), ::core::mem::transmute_copy(&echangereason)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnChange: OnChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetWatchNotify as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IRowsetWatchRegion_Impl: Sized + IRowsetWatchAll_Impl {
    fn CreateWatchRegion(&self, dwwatchmode: u32, phregion: *mut usize) -> ::windows::core::Result<()>;
    fn ChangeWatchMode(&self, hregion: usize, dwwatchmode: u32) -> ::windows::core::Result<()>;
    fn DeleteWatchRegion(&self, hregion: usize) -> ::windows::core::Result<()>;
    fn GetWatchRegionInfo(&self, hregion: usize, pdwwatchmode: *mut u32, phchapter: *mut usize, pcbbookmark: *mut usize, ppbookmark: *mut *mut u8, pcrows: *mut isize) -> ::windows::core::Result<()>;
    fn Refresh(&self, pcchangesobtained: *mut usize, prgchanges: *mut *mut DBROWWATCHCHANGE) -> ::windows::core::Result<()>;
    fn ShrinkWatchRegion(&self, hregion: usize, hchapter: usize, cbbookmark: usize, pbookmark: *mut u8, crows: isize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRowsetWatchRegion {}
impl IRowsetWatchRegion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetWatchRegion_Impl, const OFFSET: isize>() -> IRowsetWatchRegion_Vtbl {
        unsafe extern "system" fn CreateWatchRegion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetWatchRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwwatchmode: u32, phregion: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateWatchRegion(::core::mem::transmute_copy(&dwwatchmode), ::core::mem::transmute_copy(&phregion)).into()
        }
        unsafe extern "system" fn ChangeWatchMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetWatchRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hregion: usize, dwwatchmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChangeWatchMode(::core::mem::transmute_copy(&hregion), ::core::mem::transmute_copy(&dwwatchmode)).into()
        }
        unsafe extern "system" fn DeleteWatchRegion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetWatchRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hregion: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteWatchRegion(::core::mem::transmute_copy(&hregion)).into()
        }
        unsafe extern "system" fn GetWatchRegionInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetWatchRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hregion: usize, pdwwatchmode: *mut u32, phchapter: *mut usize, pcbbookmark: *mut usize, ppbookmark: *mut *mut u8, pcrows: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWatchRegionInfo(::core::mem::transmute_copy(&hregion), ::core::mem::transmute_copy(&pdwwatchmode), ::core::mem::transmute_copy(&phchapter), ::core::mem::transmute_copy(&pcbbookmark), ::core::mem::transmute_copy(&ppbookmark), ::core::mem::transmute_copy(&pcrows)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetWatchRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchangesobtained: *mut usize, prgchanges: *mut *mut DBROWWATCHCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh(::core::mem::transmute_copy(&pcchangesobtained), ::core::mem::transmute_copy(&prgchanges)).into()
        }
        unsafe extern "system" fn ShrinkWatchRegion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetWatchRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hregion: usize, hchapter: usize, cbbookmark: usize, pbookmark: *mut u8, crows: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShrinkWatchRegion(::core::mem::transmute_copy(&hregion), ::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark), ::core::mem::transmute_copy(&crows)).into()
        }
        Self {
            base__: IRowsetWatchAll_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateWatchRegion: CreateWatchRegion::<Identity, Impl, OFFSET>,
            ChangeWatchMode: ChangeWatchMode::<Identity, Impl, OFFSET>,
            DeleteWatchRegion: DeleteWatchRegion::<Identity, Impl, OFFSET>,
            GetWatchRegionInfo: GetWatchRegionInfo::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            ShrinkWatchRegion: ShrinkWatchRegion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetWatchRegion as ::windows::core::ComInterface>::IID || iid == &<IRowsetWatchAll as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IRowsetWithParameters_Impl: Sized {
    fn GetParameterInfo(&self, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
    fn Requery(&self, pparams: *mut DBPARAMS, pulerrorparam: *mut u32, phreserved: *mut usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRowsetWithParameters {}
#[cfg(feature = "Win32_System_Com")]
impl IRowsetWithParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetWithParameters_Impl, const OFFSET: isize>() -> IRowsetWithParameters_Vtbl {
        unsafe extern "system" fn GetParameterInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetWithParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetParameterInfo(::core::mem::transmute_copy(&pcparams), ::core::mem::transmute_copy(&prgparaminfo), ::core::mem::transmute_copy(&ppnamesbuffer)).into()
        }
        unsafe extern "system" fn Requery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRowsetWithParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparams: *mut DBPARAMS, pulerrorparam: *mut u32, phreserved: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Requery(::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&pulerrorparam), ::core::mem::transmute_copy(&phreserved)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetParameterInfo: GetParameterInfo::<Identity, Impl, OFFSET>,
            Requery: Requery::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRowsetWithParameters as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait ISQLErrorInfo_Impl: Sized {
    fn GetSQLInfo(&self, pbstrsqlstate: *mut ::windows::core::BSTR, plnativeerror: *mut i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISQLErrorInfo {}
impl ISQLErrorInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISQLErrorInfo_Impl, const OFFSET: isize>() -> ISQLErrorInfo_Vtbl {
        unsafe extern "system" fn GetSQLInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISQLErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsqlstate: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, plnativeerror: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSQLInfo(::core::mem::transmute_copy(&pbstrsqlstate), ::core::mem::transmute_copy(&plnativeerror)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSQLInfo: GetSQLInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISQLErrorInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISQLGetDiagField_Impl: Sized {
    fn GetDiagField(&self, pdiaginfo: *mut KAGGETDIAG) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISQLGetDiagField {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISQLGetDiagField_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISQLGetDiagField_Impl, const OFFSET: isize>() -> ISQLGetDiagField_Vtbl {
        unsafe extern "system" fn GetDiagField<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISQLGetDiagField_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdiaginfo: *mut KAGGETDIAG) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDiagField(::core::mem::transmute_copy(&pdiaginfo)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDiagField: GetDiagField::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISQLGetDiagField as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait ISQLRequestDiagFields_Impl: Sized {
    fn RequestDiagFields(&self, cdiagfields: u32, rgdiagfields: *const KAGREQDIAG) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISQLRequestDiagFields {}
#[cfg(feature = "Win32_System_Com")]
impl ISQLRequestDiagFields_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISQLRequestDiagFields_Impl, const OFFSET: isize>() -> ISQLRequestDiagFields_Vtbl {
        unsafe extern "system" fn RequestDiagFields<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISQLRequestDiagFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdiagfields: u32, rgdiagfields: *const KAGREQDIAG) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestDiagFields(::core::mem::transmute_copy(&cdiagfields), ::core::mem::transmute_copy(&rgdiagfields)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RequestDiagFields: RequestDiagFields::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISQLRequestDiagFields as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait ISQLServerErrorInfo_Impl: Sized {
    fn GetErrorInfo(&self, pperrorinfo: *mut *mut SSERRORINFO, ppstringsbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISQLServerErrorInfo {}
impl ISQLServerErrorInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISQLServerErrorInfo_Impl, const OFFSET: isize>() -> ISQLServerErrorInfo_Vtbl {
        unsafe extern "system" fn GetErrorInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISQLServerErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperrorinfo: *mut *mut SSERRORINFO, ppstringsbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetErrorInfo(::core::mem::transmute_copy(&pperrorinfo), ::core::mem::transmute_copy(&ppstringsbuffer)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetErrorInfo: GetErrorInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISQLServerErrorInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait ISchemaLocalizerSupport_Impl: Sized {
    fn Localize(&self, pszglobalstring: &::windows::core::PCWSTR) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl ::windows::core::RuntimeName for ISchemaLocalizerSupport {}
impl ISchemaLocalizerSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaLocalizerSupport_Impl, const OFFSET: isize>() -> ISchemaLocalizerSupport_Vtbl {
        unsafe extern "system" fn Localize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaLocalizerSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszglobalstring: ::windows::core::PCWSTR, ppszlocalstring: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Localize(::core::mem::transmute(&pszglobalstring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszlocalstring, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Localize: Localize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaLocalizerSupport as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
pub trait ISchemaLock_Impl: Sized {
    fn GetSchemaLock(&self, ptableid: *mut super::super::Storage::IndexServer::DBID, lmmode: u32, phlockhandle: *mut super::super::Foundation::HANDLE, ptableversion: *mut u64) -> ::windows::core::Result<()>;
    fn ReleaseSchemaLock(&self, hlockhandle: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
impl ::windows::core::RuntimeName for ISchemaLock {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
impl ISchemaLock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaLock_Impl, const OFFSET: isize>() -> ISchemaLock_Vtbl {
        unsafe extern "system" fn GetSchemaLock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, lmmode: u32, phlockhandle: *mut super::super::Foundation::HANDLE, ptableversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSchemaLock(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&lmmode), ::core::mem::transmute_copy(&phlockhandle), ::core::mem::transmute_copy(&ptableversion)).into()
        }
        unsafe extern "system" fn ReleaseSchemaLock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hlockhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseSchemaLock(::core::mem::transmute_copy(&hlockhandle)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSchemaLock: GetSchemaLock::<Identity, Impl, OFFSET>,
            ReleaseSchemaLock: ReleaseSchemaLock::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaLock as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait ISchemaProvider_Impl: Sized {
    fn Entities(&self, riid: *const ::windows::core::GUID, pentities: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RootEntity(&self) -> ::windows::core::Result<IEntity>;
    fn GetEntity(&self, pszentityname: &::windows::core::PCWSTR) -> ::windows::core::Result<IEntity>;
    fn MetaData(&self, riid: *const ::windows::core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Localize(&self, lcid: u32, pschemalocalizersupport: ::core::option::Option<&ISchemaLocalizerSupport>) -> ::windows::core::Result<()>;
    fn SaveBinary(&self, pszschemabinarypath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn LookupAuthoredNamedEntity(&self, pentity: ::core::option::Option<&IEntity>, pszinputstring: &::windows::core::PCWSTR, ptokencollection: ::core::option::Option<&ITokenCollection>, ctokensbegin: u32, pctokenslength: *mut u32, ppszvalue: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISchemaProvider {}
impl ISchemaProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaProvider_Impl, const OFFSET: isize>() -> ISchemaProvider_Vtbl {
        unsafe extern "system" fn Entities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pentities: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Entities(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pentities)).into()
        }
        unsafe extern "system" fn RootEntity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prootentity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RootEntity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prootentity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszentityname: ::windows::core::PCWSTR, pentity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEntity(::core::mem::transmute(&pszentityname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pentity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MetaData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MetaData(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pmetadata)).into()
        }
        unsafe extern "system" fn Localize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32, pschemalocalizersupport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Localize(::core::mem::transmute_copy(&lcid), ::windows::core::from_raw_borrowed(&pschemalocalizersupport)).into()
        }
        unsafe extern "system" fn SaveBinary<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszschemabinarypath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveBinary(::core::mem::transmute(&pszschemabinarypath)).into()
        }
        unsafe extern "system" fn LookupAuthoredNamedEntity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pentity: *mut ::core::ffi::c_void, pszinputstring: ::windows::core::PCWSTR, ptokencollection: *mut ::core::ffi::c_void, ctokensbegin: u32, pctokenslength: *mut u32, ppszvalue: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LookupAuthoredNamedEntity(::windows::core::from_raw_borrowed(&pentity), ::core::mem::transmute(&pszinputstring), ::windows::core::from_raw_borrowed(&ptokencollection), ::core::mem::transmute_copy(&ctokensbegin), ::core::mem::transmute_copy(&pctokenslength), ::core::mem::transmute_copy(&ppszvalue)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<ISchemaProvider as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IScopedOperations_Impl: Sized + IBindResource_Impl {
    fn Copy(&self, crows: usize, rgpwszsourceurls: *const ::windows::core::PCWSTR, rgpwszdesturls: *const ::windows::core::PCWSTR, dwcopyflags: u32, pauthenticate: ::core::option::Option<&super::Com::IAuthenticate>, rgdwstatus: *mut u32, rgpwsznewurls: *mut ::windows::core::PWSTR, ppstringsbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
    fn Move(&self, crows: usize, rgpwszsourceurls: *const ::windows::core::PCWSTR, rgpwszdesturls: *const ::windows::core::PCWSTR, dwmoveflags: u32, pauthenticate: ::core::option::Option<&super::Com::IAuthenticate>, rgdwstatus: *mut u32, rgpwsznewurls: *mut ::windows::core::PWSTR, ppstringsbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
    fn Delete(&self, crows: usize, rgpwszurls: *const ::windows::core::PCWSTR, dwdeleteflags: u32, rgdwstatus: *mut u32) -> ::windows::core::Result<()>;
    fn OpenRowset(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IScopedOperations {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IScopedOperations_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScopedOperations_Impl, const OFFSET: isize>() -> IScopedOperations_Vtbl {
        unsafe extern "system" fn Copy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScopedOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crows: usize, rgpwszsourceurls: *const ::windows::core::PCWSTR, rgpwszdesturls: *const ::windows::core::PCWSTR, dwcopyflags: u32, pauthenticate: *mut ::core::ffi::c_void, rgdwstatus: *mut u32, rgpwsznewurls: *mut ::windows::core::PWSTR, ppstringsbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Copy(::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rgpwszsourceurls), ::core::mem::transmute_copy(&rgpwszdesturls), ::core::mem::transmute_copy(&dwcopyflags), ::windows::core::from_raw_borrowed(&pauthenticate), ::core::mem::transmute_copy(&rgdwstatus), ::core::mem::transmute_copy(&rgpwsznewurls), ::core::mem::transmute_copy(&ppstringsbuffer)).into()
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScopedOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crows: usize, rgpwszsourceurls: *const ::windows::core::PCWSTR, rgpwszdesturls: *const ::windows::core::PCWSTR, dwmoveflags: u32, pauthenticate: *mut ::core::ffi::c_void, rgdwstatus: *mut u32, rgpwsznewurls: *mut ::windows::core::PWSTR, ppstringsbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Move(::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rgpwszsourceurls), ::core::mem::transmute_copy(&rgpwszdesturls), ::core::mem::transmute_copy(&dwmoveflags), ::windows::core::from_raw_borrowed(&pauthenticate), ::core::mem::transmute_copy(&rgdwstatus), ::core::mem::transmute_copy(&rgpwsznewurls), ::core::mem::transmute_copy(&ppstringsbuffer)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScopedOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crows: usize, rgpwszurls: *const ::windows::core::PCWSTR, dwdeleteflags: u32, rgdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rgpwszurls), ::core::mem::transmute_copy(&dwdeleteflags), ::core::mem::transmute_copy(&rgdwstatus)).into()
        }
        unsafe extern "system" fn OpenRowset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScopedOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenRowset(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pindexid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pprowset)).into()
        }
        Self {
            base__: IBindResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            Copy: Copy::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            OpenRowset: OpenRowset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScopedOperations as ::windows::core::ComInterface>::IID || iid == &<IBindResource as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISearchCatalogManager_Impl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetParameter(&self, pszname: &::windows::core::PCWSTR) -> ::windows::core::Result<*mut super::Com::StructuredStorage::PROPVARIANT>;
    fn SetParameter(&self, pszname: &::windows::core::PCWSTR, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetCatalogStatus(&self, pstatus: *mut CatalogStatus, ppausedreason: *mut CatalogPausedReason) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Reindex(&self) -> ::windows::core::Result<()>;
    fn ReindexMatchingURLs(&self, pszpattern: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn ReindexSearchRoot(&self, pszrooturl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetConnectTimeout(&self, dwconnecttimeout: u32) -> ::windows::core::Result<()>;
    fn ConnectTimeout(&self) -> ::windows::core::Result<u32>;
    fn SetDataTimeout(&self, dwdatatimeout: u32) -> ::windows::core::Result<()>;
    fn DataTimeout(&self) -> ::windows::core::Result<u32>;
    fn NumberOfItems(&self) -> ::windows::core::Result<i32>;
    fn NumberOfItemsToIndex(&self, plincrementalcount: *mut i32, plnotificationqueue: *mut i32, plhighpriorityqueue: *mut i32) -> ::windows::core::Result<()>;
    fn URLBeingIndexed(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetURLIndexingState(&self, pszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<u32>;
    fn GetPersistentItemsChangedSink(&self) -> ::windows::core::Result<ISearchPersistentItemsChangedSink>;
    fn RegisterViewForNotification(&self, pszview: &::windows::core::PCWSTR, pviewchangedsink: ::core::option::Option<&ISearchViewChangedSink>) -> ::windows::core::Result<u32>;
    fn GetItemsChangedSink(&self, pisearchnotifyinlinesite: ::core::option::Option<&ISearchNotifyInlineSite>, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void, pguidcatalogresetsignature: *mut ::windows::core::GUID, pguidcheckpointsignature: *mut ::windows::core::GUID, pdwlastcheckpointnumber: *mut u32) -> ::windows::core::Result<()>;
    fn UnregisterViewForNotification(&self, dwcookie: u32) -> ::windows::core::Result<()>;
    fn SetExtensionClusion(&self, pszextension: &::windows::core::PCWSTR, fexclude: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn EnumerateExcludedExtensions(&self) -> ::windows::core::Result<super::Com::IEnumString>;
    fn GetQueryHelper(&self) -> ::windows::core::Result<ISearchQueryHelper>;
    fn SetDiacriticSensitivity(&self, fdiacriticsensitive: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DiacriticSensitivity(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetCrawlScopeManager(&self) -> ::windows::core::Result<ISearchCrawlScopeManager>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for ISearchCatalogManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ISearchCatalogManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>() -> ISearchCatalogManager_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParameter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, ppvalue: *mut *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParameter(::core::mem::transmute(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParameter(::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetCatalogStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut CatalogStatus, ppausedreason: *mut CatalogPausedReason) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCatalogStatus(::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppausedreason)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Reindex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reindex().into()
        }
        unsafe extern "system" fn ReindexMatchingURLs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpattern: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReindexMatchingURLs(::core::mem::transmute(&pszpattern)).into()
        }
        unsafe extern "system" fn ReindexSearchRoot<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrooturl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReindexSearchRoot(::core::mem::transmute(&pszrooturl)).into()
        }
        unsafe extern "system" fn SetConnectTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnecttimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConnectTimeout(::core::mem::transmute_copy(&dwconnecttimeout)).into()
        }
        unsafe extern "system" fn ConnectTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwconnecttimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConnectTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwconnecttimeout, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdatatimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDataTimeout(::core::mem::transmute_copy(&dwdatatimeout)).into()
        }
        unsafe extern "system" fn DataTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdatatimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DataTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwdatatimeout, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfItems<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NumberOfItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfItemsToIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plincrementalcount: *mut i32, plnotificationqueue: *mut i32, plhighpriorityqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NumberOfItemsToIndex(::core::mem::transmute_copy(&plincrementalcount), ::core::mem::transmute_copy(&plnotificationqueue), ::core::mem::transmute_copy(&plhighpriorityqueue)).into()
        }
        unsafe extern "system" fn URLBeingIndexed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.URLBeingIndexed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetURLIndexingState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetURLIndexingState(::core::mem::transmute(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPersistentItemsChangedSink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppisearchpersistentitemschangedsink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPersistentItemsChangedSink() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppisearchpersistentitemschangedsink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterViewForNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszview: ::windows::core::PCWSTR, pviewchangedsink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterViewForNotification(::core::mem::transmute(&pszview), ::windows::core::from_raw_borrowed(&pviewchangedsink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsChangedSink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisearchnotifyinlinesite: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void, pguidcatalogresetsignature: *mut ::windows::core::GUID, pguidcheckpointsignature: *mut ::windows::core::GUID, pdwlastcheckpointnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetItemsChangedSink(::windows::core::from_raw_borrowed(&pisearchnotifyinlinesite), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv), ::core::mem::transmute_copy(&pguidcatalogresetsignature), ::core::mem::transmute_copy(&pguidcheckpointsignature), ::core::mem::transmute_copy(&pdwlastcheckpointnumber)).into()
        }
        unsafe extern "system" fn UnregisterViewForNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterViewForNotification(::core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn SetExtensionClusion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszextension: ::windows::core::PCWSTR, fexclude: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExtensionClusion(::core::mem::transmute(&pszextension), ::core::mem::transmute_copy(&fexclude)).into()
        }
        unsafe extern "system" fn EnumerateExcludedExtensions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextensions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateExcludedExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppextensions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQueryHelper<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsearchqueryhelper: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetQueryHelper() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsearchqueryhelper, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiacriticSensitivity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdiacriticsensitive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDiacriticSensitivity(::core::mem::transmute_copy(&fdiacriticsensitive)).into()
        }
        unsafe extern "system" fn DiacriticSensitivity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdiacriticsensitive: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DiacriticSensitivity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfdiacriticsensitive, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCrawlScopeManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcrawlscopemanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCrawlScopeManager() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcrawlscopemanager, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<ISearchCatalogManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISearchCatalogManager2_Impl: Sized + ISearchCatalogManager_Impl {
    fn PrioritizeMatchingURLs(&self, pszpattern: &::windows::core::PCWSTR, dwprioritizeflags: PRIORITIZE_FLAGS) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for ISearchCatalogManager2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ISearchCatalogManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager2_Impl, const OFFSET: isize>() -> ISearchCatalogManager2_Vtbl {
        unsafe extern "system" fn PrioritizeMatchingURLs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCatalogManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpattern: ::windows::core::PCWSTR, dwprioritizeflags: PRIORITIZE_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrioritizeMatchingURLs(::core::mem::transmute(&pszpattern), ::core::mem::transmute_copy(&dwprioritizeflags)).into()
        }
        Self { base__: ISearchCatalogManager_Vtbl::new::<Identity, Impl, OFFSET>(), PrioritizeMatchingURLs: PrioritizeMatchingURLs::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchCatalogManager2 as ::windows::core::ComInterface>::IID || iid == &<ISearchCatalogManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchCrawlScopeManager_Impl: Sized {
    fn AddDefaultScopeRule(&self, pszurl: &::windows::core::PCWSTR, finclude: super::super::Foundation::BOOL, ffollowflags: u32) -> ::windows::core::Result<()>;
    fn AddRoot(&self, psearchroot: ::core::option::Option<&ISearchRoot>) -> ::windows::core::Result<()>;
    fn RemoveRoot(&self, pszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn EnumerateRoots(&self) -> ::windows::core::Result<IEnumSearchRoots>;
    fn AddHierarchicalScope(&self, pszurl: &::windows::core::PCWSTR, finclude: super::super::Foundation::BOOL, fdefault: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn AddUserScopeRule(&self, pszurl: &::windows::core::PCWSTR, finclude: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL, ffollowflags: u32) -> ::windows::core::Result<()>;
    fn RemoveScopeRule(&self, pszrule: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn EnumerateScopeRules(&self) -> ::windows::core::Result<IEnumSearchScopeRules>;
    fn HasParentScopeRule(&self, pszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn HasChildScopeRule(&self, pszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IncludedInCrawlScope(&self, pszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IncludedInCrawlScopeEx(&self, pszurl: &::windows::core::PCWSTR, pfisincluded: *mut super::super::Foundation::BOOL, preason: *mut CLUSION_REASON) -> ::windows::core::Result<()>;
    fn RevertToDefaultScopes(&self) -> ::windows::core::Result<()>;
    fn SaveAll(&self) -> ::windows::core::Result<()>;
    fn GetParentScopeVersionId(&self, pszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<i32>;
    fn RemoveDefaultScopeRule(&self, pszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISearchCrawlScopeManager {}
#[cfg(feature = "Win32_Foundation")]
impl ISearchCrawlScopeManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>() -> ISearchCrawlScopeManager_Vtbl {
        unsafe extern "system" fn AddDefaultScopeRule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR, finclude: super::super::Foundation::BOOL, ffollowflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDefaultScopeRule(::core::mem::transmute(&pszurl), ::core::mem::transmute_copy(&finclude), ::core::mem::transmute_copy(&ffollowflags)).into()
        }
        unsafe extern "system" fn AddRoot<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psearchroot: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRoot(::windows::core::from_raw_borrowed(&psearchroot)).into()
        }
        unsafe extern "system" fn RemoveRoot<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveRoot(::core::mem::transmute(&pszurl)).into()
        }
        unsafe extern "system" fn EnumerateRoots<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsearchroots: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateRoots() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsearchroots, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddHierarchicalScope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR, finclude: super::super::Foundation::BOOL, fdefault: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddHierarchicalScope(::core::mem::transmute(&pszurl), ::core::mem::transmute_copy(&finclude), ::core::mem::transmute_copy(&fdefault), ::core::mem::transmute_copy(&foverridechildren)).into()
        }
        unsafe extern "system" fn AddUserScopeRule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR, finclude: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL, ffollowflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddUserScopeRule(::core::mem::transmute(&pszurl), ::core::mem::transmute_copy(&finclude), ::core::mem::transmute_copy(&foverridechildren), ::core::mem::transmute_copy(&ffollowflags)).into()
        }
        unsafe extern "system" fn RemoveScopeRule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrule: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveScopeRule(::core::mem::transmute(&pszrule)).into()
        }
        unsafe extern "system" fn EnumerateScopeRules<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsearchscoperules: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateScopeRules() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsearchscoperules, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasParentScopeRule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR, pfhasparentrule: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasParentScopeRule(::core::mem::transmute(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfhasparentrule, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasChildScopeRule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR, pfhaschildrule: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasChildScopeRule(::core::mem::transmute(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfhaschildrule, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludedInCrawlScope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR, pfisincluded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IncludedInCrawlScope(::core::mem::transmute(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisincluded, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludedInCrawlScopeEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR, pfisincluded: *mut super::super::Foundation::BOOL, preason: *mut CLUSION_REASON) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IncludedInCrawlScopeEx(::core::mem::transmute(&pszurl), ::core::mem::transmute_copy(&pfisincluded), ::core::mem::transmute_copy(&preason)).into()
        }
        unsafe extern "system" fn RevertToDefaultScopes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RevertToDefaultScopes().into()
        }
        unsafe extern "system" fn SaveAll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveAll().into()
        }
        unsafe extern "system" fn GetParentScopeVersionId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR, plscopeid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParentScopeVersionId(::core::mem::transmute(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plscopeid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDefaultScopeRule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveDefaultScopeRule(::core::mem::transmute(&pszurl)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<ISearchCrawlScopeManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchCrawlScopeManager2_Impl: Sized + ISearchCrawlScopeManager_Impl {
    fn GetVersion(&self, plversion: *mut *mut i32, phfilemapping: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISearchCrawlScopeManager2 {}
#[cfg(feature = "Win32_Foundation")]
impl ISearchCrawlScopeManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager2_Impl, const OFFSET: isize>() -> ISearchCrawlScopeManager2_Vtbl {
        unsafe extern "system" fn GetVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchCrawlScopeManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plversion: *mut *mut i32, phfilemapping: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVersion(::core::mem::transmute_copy(&plversion), ::core::mem::transmute_copy(&phfilemapping)).into()
        }
        Self { base__: ISearchCrawlScopeManager_Vtbl::new::<Identity, Impl, OFFSET>(), GetVersion: GetVersion::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchCrawlScopeManager2 as ::windows::core::ComInterface>::IID || iid == &<ISearchCrawlScopeManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchItemsChangedSink_Impl: Sized {
    fn StartedMonitoringScope(&self, pszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn StoppedMonitoringScope(&self, pszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn OnItemsChanged(&self, dwnumberofchanges: u32, rgdatachangeentries: *const SEARCH_ITEM_CHANGE, rgdwdocids: *mut u32, rghrcompletioncodes: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISearchItemsChangedSink {}
#[cfg(feature = "Win32_System_Com")]
impl ISearchItemsChangedSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchItemsChangedSink_Impl, const OFFSET: isize>() -> ISearchItemsChangedSink_Vtbl {
        unsafe extern "system" fn StartedMonitoringScope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchItemsChangedSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartedMonitoringScope(::core::mem::transmute(&pszurl)).into()
        }
        unsafe extern "system" fn StoppedMonitoringScope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchItemsChangedSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StoppedMonitoringScope(::core::mem::transmute(&pszurl)).into()
        }
        unsafe extern "system" fn OnItemsChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchItemsChangedSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnumberofchanges: u32, rgdatachangeentries: *const SEARCH_ITEM_CHANGE, rgdwdocids: *mut u32, rghrcompletioncodes: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnItemsChanged(::core::mem::transmute_copy(&dwnumberofchanges), ::core::mem::transmute_copy(&rgdatachangeentries), ::core::mem::transmute_copy(&rgdwdocids), ::core::mem::transmute_copy(&rghrcompletioncodes)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartedMonitoringScope: StartedMonitoringScope::<Identity, Impl, OFFSET>,
            StoppedMonitoringScope: StoppedMonitoringScope::<Identity, Impl, OFFSET>,
            OnItemsChanged: OnItemsChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchItemsChangedSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchLanguageSupport_Impl: Sized {
    fn SetDiacriticSensitivity(&self, fdiacriticsensitive: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetDiacriticSensitivity(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn LoadWordBreaker(&self, lcid: u32, riid: *const ::windows::core::GUID, ppwordbreaker: *mut *mut ::core::ffi::c_void, plcidused: *mut u32) -> ::windows::core::Result<()>;
    fn LoadStemmer(&self, lcid: u32, riid: *const ::windows::core::GUID, ppstemmer: *mut *mut ::core::ffi::c_void, plcidused: *mut u32) -> ::windows::core::Result<()>;
    fn IsPrefixNormalized(&self, pwcsquerytoken: &::windows::core::PCWSTR, cwcquerytoken: u32, pwcsdocumenttoken: &::windows::core::PCWSTR, cwcdocumenttoken: u32) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISearchLanguageSupport {}
#[cfg(feature = "Win32_Foundation")]
impl ISearchLanguageSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchLanguageSupport_Impl, const OFFSET: isize>() -> ISearchLanguageSupport_Vtbl {
        unsafe extern "system" fn SetDiacriticSensitivity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchLanguageSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdiacriticsensitive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDiacriticSensitivity(::core::mem::transmute_copy(&fdiacriticsensitive)).into()
        }
        unsafe extern "system" fn GetDiacriticSensitivity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchLanguageSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdiacriticsensitive: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDiacriticSensitivity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfdiacriticsensitive, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadWordBreaker<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchLanguageSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32, riid: *const ::windows::core::GUID, ppwordbreaker: *mut *mut ::core::ffi::c_void, plcidused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadWordBreaker(::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppwordbreaker), ::core::mem::transmute_copy(&plcidused)).into()
        }
        unsafe extern "system" fn LoadStemmer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchLanguageSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32, riid: *const ::windows::core::GUID, ppstemmer: *mut *mut ::core::ffi::c_void, plcidused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadStemmer(::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppstemmer), ::core::mem::transmute_copy(&plcidused)).into()
        }
        unsafe extern "system" fn IsPrefixNormalized<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchLanguageSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsquerytoken: ::windows::core::PCWSTR, cwcquerytoken: u32, pwcsdocumenttoken: ::windows::core::PCWSTR, cwcdocumenttoken: u32, pulprefixlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPrefixNormalized(::core::mem::transmute(&pwcsquerytoken), ::core::mem::transmute_copy(&cwcquerytoken), ::core::mem::transmute(&pwcsdocumenttoken), ::core::mem::transmute_copy(&cwcdocumenttoken)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulprefixlength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDiacriticSensitivity: SetDiacriticSensitivity::<Identity, Impl, OFFSET>,
            GetDiacriticSensitivity: GetDiacriticSensitivity::<Identity, Impl, OFFSET>,
            LoadWordBreaker: LoadWordBreaker::<Identity, Impl, OFFSET>,
            LoadStemmer: LoadStemmer::<Identity, Impl, OFFSET>,
            IsPrefixNormalized: IsPrefixNormalized::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchLanguageSupport as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISearchManager_Impl: Sized {
    fn GetIndexerVersionStr(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetIndexerVersion(&self, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows::core::Result<()>;
    fn GetParameter(&self, pszname: &::windows::core::PCWSTR) -> ::windows::core::Result<*mut super::Com::StructuredStorage::PROPVARIANT>;
    fn SetParameter(&self, pszname: &::windows::core::PCWSTR, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn ProxyName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn BypassList(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetProxy(&self, suseproxy: PROXY_ACCESS, flocalbypassproxy: super::super::Foundation::BOOL, dwportnumber: u32, pszproxyname: &::windows::core::PCWSTR, pszbypasslist: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetCatalog(&self, pszcatalog: &::windows::core::PCWSTR) -> ::windows::core::Result<ISearchCatalogManager>;
    fn UserAgent(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetUserAgent(&self, pszuseragent: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn UseProxy(&self) -> ::windows::core::Result<PROXY_ACCESS>;
    fn LocalBypass(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn PortNumber(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for ISearchManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ISearchManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: isize>() -> ISearchManager_Vtbl {
        unsafe extern "system" fn GetIndexerVersionStr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszversionstring: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIndexerVersionStr() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszversionstring, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndexerVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIndexerVersion(::core::mem::transmute_copy(&pdwmajor), ::core::mem::transmute_copy(&pdwminor)).into()
        }
        unsafe extern "system" fn GetParameter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, ppvalue: *mut *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParameter(::core::mem::transmute(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParameter(::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn ProxyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszproxyname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProxyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszproxyname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BypassList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszbypasslist: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BypassList() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszbypasslist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, suseproxy: PROXY_ACCESS, flocalbypassproxy: super::super::Foundation::BOOL, dwportnumber: u32, pszproxyname: ::windows::core::PCWSTR, pszbypasslist: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProxy(::core::mem::transmute_copy(&suseproxy), ::core::mem::transmute_copy(&flocalbypassproxy), ::core::mem::transmute_copy(&dwportnumber), ::core::mem::transmute(&pszproxyname), ::core::mem::transmute(&pszbypasslist)).into()
        }
        unsafe extern "system" fn GetCatalog<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcatalog: ::windows::core::PCWSTR, ppcatalogmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCatalog(::core::mem::transmute(&pszcatalog)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogmanager, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserAgent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszuseragent: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserAgent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszuseragent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserAgent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszuseragent: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUserAgent(::core::mem::transmute(&pszuseragent)).into()
        }
        unsafe extern "system" fn UseProxy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puseproxy: *mut PROXY_ACCESS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UseProxy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puseproxy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalBypass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflocalbypass: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LocalBypass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflocalbypass, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PortNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwportnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PortNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwportnumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<ISearchManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISearchManager2_Impl: Sized + ISearchManager_Impl {
    fn CreateCatalog(&self, pszcatalog: &::windows::core::PCWSTR) -> ::windows::core::Result<ISearchCatalogManager>;
    fn DeleteCatalog(&self, pszcatalog: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for ISearchManager2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ISearchManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchManager2_Impl, const OFFSET: isize>() -> ISearchManager2_Vtbl {
        unsafe extern "system" fn CreateCatalog<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcatalog: ::windows::core::PCWSTR, ppcatalogmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCatalog(::core::mem::transmute(&pszcatalog)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogmanager, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteCatalog<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcatalog: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteCatalog(::core::mem::transmute(&pszcatalog)).into()
        }
        Self {
            base__: ISearchManager_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateCatalog: CreateCatalog::<Identity, Impl, OFFSET>,
            DeleteCatalog: DeleteCatalog::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchManager2 as ::windows::core::ComInterface>::IID || iid == &<ISearchManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait ISearchNotifyInlineSite_Impl: Sized {
    fn OnItemIndexedStatusChange(&self, sipstatus: SEARCH_INDEXING_PHASE, dwnumentries: u32, rgitemstatusentries: *const SEARCH_ITEM_INDEXING_STATUS) -> ::windows::core::Result<()>;
    fn OnCatalogStatusChange(&self, guidcatalogresetsignature: *const ::windows::core::GUID, guidcheckpointsignature: *const ::windows::core::GUID, dwlastcheckpointnumber: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISearchNotifyInlineSite {}
impl ISearchNotifyInlineSite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchNotifyInlineSite_Impl, const OFFSET: isize>() -> ISearchNotifyInlineSite_Vtbl {
        unsafe extern "system" fn OnItemIndexedStatusChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchNotifyInlineSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sipstatus: SEARCH_INDEXING_PHASE, dwnumentries: u32, rgitemstatusentries: *const SEARCH_ITEM_INDEXING_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnItemIndexedStatusChange(::core::mem::transmute_copy(&sipstatus), ::core::mem::transmute_copy(&dwnumentries), ::core::mem::transmute_copy(&rgitemstatusentries)).into()
        }
        unsafe extern "system" fn OnCatalogStatusChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchNotifyInlineSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcatalogresetsignature: *const ::windows::core::GUID, guidcheckpointsignature: *const ::windows::core::GUID, dwlastcheckpointnumber: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCatalogStatusChange(::core::mem::transmute_copy(&guidcatalogresetsignature), ::core::mem::transmute_copy(&guidcheckpointsignature), ::core::mem::transmute_copy(&dwlastcheckpointnumber)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnItemIndexedStatusChange: OnItemIndexedStatusChange::<Identity, Impl, OFFSET>,
            OnCatalogStatusChange: OnCatalogStatusChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchNotifyInlineSite as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait ISearchPersistentItemsChangedSink_Impl: Sized {
    fn StartedMonitoringScope(&self, pszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn StoppedMonitoringScope(&self, pszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn OnItemsChanged(&self, dwnumberofchanges: u32, datachangeentries: *const SEARCH_ITEM_PERSISTENT_CHANGE, hrcompletioncodes: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISearchPersistentItemsChangedSink {}
impl ISearchPersistentItemsChangedSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchPersistentItemsChangedSink_Impl, const OFFSET: isize>() -> ISearchPersistentItemsChangedSink_Vtbl {
        unsafe extern "system" fn StartedMonitoringScope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchPersistentItemsChangedSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartedMonitoringScope(::core::mem::transmute(&pszurl)).into()
        }
        unsafe extern "system" fn StoppedMonitoringScope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchPersistentItemsChangedSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StoppedMonitoringScope(::core::mem::transmute(&pszurl)).into()
        }
        unsafe extern "system" fn OnItemsChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchPersistentItemsChangedSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnumberofchanges: u32, datachangeentries: *const SEARCH_ITEM_PERSISTENT_CHANGE, hrcompletioncodes: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnItemsChanged(::core::mem::transmute_copy(&dwnumberofchanges), ::core::mem::transmute_copy(&datachangeentries), ::core::mem::transmute_copy(&hrcompletioncodes)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartedMonitoringScope: StartedMonitoringScope::<Identity, Impl, OFFSET>,
            StoppedMonitoringScope: StoppedMonitoringScope::<Identity, Impl, OFFSET>,
            OnItemsChanged: OnItemsChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchPersistentItemsChangedSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchProtocol_Impl: Sized {
    fn Init(&self, ptimeoutinfo: *mut TIMEOUT_INFO, pprotocolhandlersite: ::core::option::Option<&IProtocolHandlerSite>, pproxyinfo: *mut PROXY_INFO) -> ::windows::core::Result<()>;
    fn CreateAccessor(&self, pcwszurl: &::windows::core::PCWSTR, pauthenticationinfo: *mut AUTHENTICATION_INFO, pincrementalaccessinfo: *mut INCREMENTAL_ACCESS_INFO, piteminfo: *mut ITEM_INFO, ppaccessor: *mut ::core::option::Option<IUrlAccessor>) -> ::windows::core::Result<()>;
    fn CloseAccessor(&self, paccessor: ::core::option::Option<&IUrlAccessor>) -> ::windows::core::Result<()>;
    fn ShutDown(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISearchProtocol {}
#[cfg(feature = "Win32_Foundation")]
impl ISearchProtocol_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchProtocol_Impl, const OFFSET: isize>() -> ISearchProtocol_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimeoutinfo: *mut TIMEOUT_INFO, pprotocolhandlersite: *mut ::core::ffi::c_void, pproxyinfo: *mut PROXY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Init(::core::mem::transmute_copy(&ptimeoutinfo), ::windows::core::from_raw_borrowed(&pprotocolhandlersite), ::core::mem::transmute_copy(&pproxyinfo)).into()
        }
        unsafe extern "system" fn CreateAccessor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcwszurl: ::windows::core::PCWSTR, pauthenticationinfo: *mut AUTHENTICATION_INFO, pincrementalaccessinfo: *mut INCREMENTAL_ACCESS_INFO, piteminfo: *mut ITEM_INFO, ppaccessor: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateAccessor(::core::mem::transmute(&pcwszurl), ::core::mem::transmute_copy(&pauthenticationinfo), ::core::mem::transmute_copy(&pincrementalaccessinfo), ::core::mem::transmute_copy(&piteminfo), ::core::mem::transmute_copy(&ppaccessor)).into()
        }
        unsafe extern "system" fn CloseAccessor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paccessor: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseAccessor(::windows::core::from_raw_borrowed(&paccessor)).into()
        }
        unsafe extern "system" fn ShutDown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShutDown().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            CreateAccessor: CreateAccessor::<Identity, Impl, OFFSET>,
            CloseAccessor: CloseAccessor::<Identity, Impl, OFFSET>,
            ShutDown: ShutDown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchProtocol as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISearchProtocol2_Impl: Sized + ISearchProtocol_Impl {
    fn CreateAccessorEx(&self, pcwszurl: &::windows::core::PCWSTR, pauthenticationinfo: *mut AUTHENTICATION_INFO, pincrementalaccessinfo: *mut INCREMENTAL_ACCESS_INFO, piteminfo: *mut ITEM_INFO, puserdata: *const super::Com::BLOB, ppaccessor: *mut ::core::option::Option<IUrlAccessor>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for ISearchProtocol2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISearchProtocol2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchProtocol2_Impl, const OFFSET: isize>() -> ISearchProtocol2_Vtbl {
        unsafe extern "system" fn CreateAccessorEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchProtocol2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcwszurl: ::windows::core::PCWSTR, pauthenticationinfo: *mut AUTHENTICATION_INFO, pincrementalaccessinfo: *mut INCREMENTAL_ACCESS_INFO, piteminfo: *mut ITEM_INFO, puserdata: *const super::Com::BLOB, ppaccessor: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateAccessorEx(::core::mem::transmute(&pcwszurl), ::core::mem::transmute_copy(&pauthenticationinfo), ::core::mem::transmute_copy(&pincrementalaccessinfo), ::core::mem::transmute_copy(&piteminfo), ::core::mem::transmute_copy(&puserdata), ::core::mem::transmute_copy(&ppaccessor)).into()
        }
        Self { base__: ISearchProtocol_Vtbl::new::<Identity, Impl, OFFSET>(), CreateAccessorEx: CreateAccessorEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchProtocol2 as ::windows::core::ComInterface>::IID || iid == &<ISearchProtocol as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait ISearchProtocolThreadContext_Impl: Sized {
    fn ThreadInit(&self) -> ::windows::core::Result<()>;
    fn ThreadShutdown(&self) -> ::windows::core::Result<()>;
    fn ThreadIdle(&self, dwtimeelaspedsincelastcallinms: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISearchProtocolThreadContext {}
impl ISearchProtocolThreadContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchProtocolThreadContext_Impl, const OFFSET: isize>() -> ISearchProtocolThreadContext_Vtbl {
        unsafe extern "system" fn ThreadInit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchProtocolThreadContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ThreadInit().into()
        }
        unsafe extern "system" fn ThreadShutdown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchProtocolThreadContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ThreadShutdown().into()
        }
        unsafe extern "system" fn ThreadIdle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchProtocolThreadContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtimeelaspedsincelastcallinms: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ThreadIdle(::core::mem::transmute_copy(&dwtimeelaspedsincelastcallinms)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ThreadInit: ThreadInit::<Identity, Impl, OFFSET>,
            ThreadShutdown: ThreadShutdown::<Identity, Impl, OFFSET>,
            ThreadIdle: ThreadIdle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchProtocolThreadContext as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISearchQueryHelper_Impl: Sized {
    fn ConnectionString(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetQueryContentLocale(&self, lcid: u32) -> ::windows::core::Result<()>;
    fn QueryContentLocale(&self) -> ::windows::core::Result<u32>;
    fn SetQueryKeywordLocale(&self, lcid: u32) -> ::windows::core::Result<()>;
    fn QueryKeywordLocale(&self) -> ::windows::core::Result<u32>;
    fn SetQueryTermExpansion(&self, expandterms: SEARCH_TERM_EXPANSION) -> ::windows::core::Result<()>;
    fn QueryTermExpansion(&self) -> ::windows::core::Result<SEARCH_TERM_EXPANSION>;
    fn SetQuerySyntax(&self, querysyntax: SEARCH_QUERY_SYNTAX) -> ::windows::core::Result<()>;
    fn QuerySyntax(&self) -> ::windows::core::Result<SEARCH_QUERY_SYNTAX>;
    fn SetQueryContentProperties(&self, pszcontentproperties: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn QueryContentProperties(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetQuerySelectColumns(&self, pszselectcolumns: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn QuerySelectColumns(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetQueryWhereRestrictions(&self, pszrestrictions: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn QueryWhereRestrictions(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetQuerySorting(&self, pszsorting: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn QuerySorting(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GenerateSQLFromUserQuery(&self, pszquery: &::windows::core::PCWSTR) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn WriteProperties(&self, itemid: i32, dwnumberofcolumns: u32, pcolumns: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalues: *const SEARCH_COLUMN_PROPERTIES, pftgathermodifiedtime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn SetQueryMaxResults(&self, cmaxresults: i32) -> ::windows::core::Result<()>;
    fn QueryMaxResults(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows::core::RuntimeName for ISearchQueryHelper {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISearchQueryHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>() -> ISearchQueryHelper_Vtbl {
        unsafe extern "system" fn ConnectionString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconnectionstring: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConnectionString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszconnectionstring, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryContentLocale<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQueryContentLocale(::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn QueryContentLocale<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryContentLocale() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryKeywordLocale<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQueryKeywordLocale(::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn QueryKeywordLocale<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryKeywordLocale() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryTermExpansion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expandterms: SEARCH_TERM_EXPANSION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQueryTermExpansion(::core::mem::transmute_copy(&expandterms)).into()
        }
        unsafe extern "system" fn QueryTermExpansion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexpandterms: *mut SEARCH_TERM_EXPANSION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryTermExpansion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pexpandterms, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuerySyntax<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querysyntax: SEARCH_QUERY_SYNTAX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuerySyntax(::core::mem::transmute_copy(&querysyntax)).into()
        }
        unsafe extern "system" fn QuerySyntax<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquerysyntax: *mut SEARCH_QUERY_SYNTAX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QuerySyntax() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pquerysyntax, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryContentProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcontentproperties: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQueryContentProperties(::core::mem::transmute(&pszcontentproperties)).into()
        }
        unsafe extern "system" fn QueryContentProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcontentproperties: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryContentProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszcontentproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuerySelectColumns<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszselectcolumns: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuerySelectColumns(::core::mem::transmute(&pszselectcolumns)).into()
        }
        unsafe extern "system" fn QuerySelectColumns<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszselectcolumns: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QuerySelectColumns() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszselectcolumns, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryWhereRestrictions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrestrictions: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQueryWhereRestrictions(::core::mem::transmute(&pszrestrictions)).into()
        }
        unsafe extern "system" fn QueryWhereRestrictions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszrestrictions: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryWhereRestrictions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszrestrictions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuerySorting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsorting: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuerySorting(::core::mem::transmute(&pszsorting)).into()
        }
        unsafe extern "system" fn QuerySorting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszsorting: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QuerySorting() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszsorting, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateSQLFromUserQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszquery: ::windows::core::PCWSTR, ppszsql: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GenerateSQLFromUserQuery(::core::mem::transmute(&pszquery)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszsql, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: i32, dwnumberofcolumns: u32, pcolumns: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalues: *const SEARCH_COLUMN_PROPERTIES, pftgathermodifiedtime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteProperties(::core::mem::transmute_copy(&itemid), ::core::mem::transmute_copy(&dwnumberofcolumns), ::core::mem::transmute_copy(&pcolumns), ::core::mem::transmute_copy(&pvalues), ::core::mem::transmute_copy(&pftgathermodifiedtime)).into()
        }
        unsafe extern "system" fn SetQueryMaxResults<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmaxresults: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQueryMaxResults(::core::mem::transmute_copy(&cmaxresults)).into()
        }
        unsafe extern "system" fn QueryMaxResults<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcmaxresults: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryMaxResults() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcmaxresults, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<ISearchQueryHelper as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait ISearchQueryHits_Impl: Sized {
    fn Init(&self, pflt: ::core::option::Option<&super::super::Storage::IndexServer::IFilter>, ulflags: u32) -> i32;
    fn NextHitMoniker(&self, pcmnk: *mut u32, papmnk: *mut *mut ::core::option::Option<super::Com::IMoniker>) -> i32;
    fn NextHitOffset(&self, pcregion: *mut u32, paregion: *mut *mut super::super::Storage::IndexServer::FILTERREGION) -> i32;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for ISearchQueryHits {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ISearchQueryHits_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHits_Impl, const OFFSET: isize>() -> ISearchQueryHits_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflt: *mut ::core::ffi::c_void, ulflags: u32) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Init(::windows::core::from_raw_borrowed(&pflt), ::core::mem::transmute_copy(&ulflags))
        }
        unsafe extern "system" fn NextHitMoniker<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcmnk: *mut u32, papmnk: *mut *mut ::core::option::Option<super::Com::IMoniker>) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NextHitMoniker(::core::mem::transmute_copy(&pcmnk), ::core::mem::transmute_copy(&papmnk))
        }
        unsafe extern "system" fn NextHitOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchQueryHits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcregion: *mut u32, paregion: *mut *mut super::super::Storage::IndexServer::FILTERREGION) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NextHitOffset(::core::mem::transmute_copy(&pcregion), ::core::mem::transmute_copy(&paregion))
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            NextHitMoniker: NextHitMoniker::<Identity, Impl, OFFSET>,
            NextHitOffset: NextHitOffset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchQueryHits as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchRoot_Impl: Sized {
    fn SetSchedule(&self, psztaskarg: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Schedule(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetRootURL(&self, pszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn RootURL(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetIsHierarchical(&self, fishierarchical: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsHierarchical(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetProvidesNotifications(&self, fprovidesnotifications: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ProvidesNotifications(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetUseNotificationsOnly(&self, fusenotificationsonly: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn UseNotificationsOnly(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnumerationDepth(&self, dwdepth: u32) -> ::windows::core::Result<()>;
    fn EnumerationDepth(&self) -> ::windows::core::Result<u32>;
    fn SetHostDepth(&self, dwdepth: u32) -> ::windows::core::Result<()>;
    fn HostDepth(&self) -> ::windows::core::Result<u32>;
    fn SetFollowDirectories(&self, ffollowdirectories: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn FollowDirectories(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetAuthenticationType(&self, authtype: AUTH_TYPE) -> ::windows::core::Result<()>;
    fn AuthenticationType(&self) -> ::windows::core::Result<AUTH_TYPE>;
    fn SetUser(&self, pszuser: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn User(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetPassword(&self, pszpassword: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Password(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISearchRoot {}
#[cfg(feature = "Win32_Foundation")]
impl ISearchRoot_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>() -> ISearchRoot_Vtbl {
        unsafe extern "system" fn SetSchedule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztaskarg: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSchedule(::core::mem::transmute(&psztaskarg)).into()
        }
        unsafe extern "system" fn Schedule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztaskarg: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Schedule() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsztaskarg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRootURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRootURL(::core::mem::transmute(&pszurl)).into()
        }
        unsafe extern "system" fn RootURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RootURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHierarchical<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fishierarchical: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsHierarchical(::core::mem::transmute_copy(&fishierarchical)).into()
        }
        unsafe extern "system" fn IsHierarchical<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfishierarchical: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsHierarchical() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfishierarchical, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProvidesNotifications<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fprovidesnotifications: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProvidesNotifications(::core::mem::transmute_copy(&fprovidesnotifications)).into()
        }
        unsafe extern "system" fn ProvidesNotifications<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprovidesnotifications: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProvidesNotifications() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprovidesnotifications, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseNotificationsOnly<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fusenotificationsonly: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUseNotificationsOnly(::core::mem::transmute_copy(&fusenotificationsonly)).into()
        }
        unsafe extern "system" fn UseNotificationsOnly<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfusenotificationsonly: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UseNotificationsOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfusenotificationsonly, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnumerationDepth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdepth: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnumerationDepth(::core::mem::transmute_copy(&dwdepth)).into()
        }
        unsafe extern "system" fn EnumerationDepth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdepth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerationDepth() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwdepth, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostDepth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdepth: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHostDepth(::core::mem::transmute_copy(&dwdepth)).into()
        }
        unsafe extern "system" fn HostDepth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdepth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HostDepth() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwdepth, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFollowDirectories<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffollowdirectories: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFollowDirectories(::core::mem::transmute_copy(&ffollowdirectories)).into()
        }
        unsafe extern "system" fn FollowDirectories<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffollowdirectories: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FollowDirectories() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pffollowdirectories, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authtype: AUTH_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthenticationType(::core::mem::transmute_copy(&authtype)).into()
        }
        unsafe extern "system" fn AuthenticationType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthtype: *mut AUTH_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AuthenticationType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pauthtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszuser: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUser(::core::mem::transmute(&pszuser)).into()
        }
        unsafe extern "system" fn User<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszuser: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.User() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszuser, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPassword<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpassword: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPassword(::core::mem::transmute(&pszpassword)).into()
        }
        unsafe extern "system" fn Password<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpassword: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Password() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpassword, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<ISearchRoot as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchScopeRule_Impl: Sized {
    fn PatternOrURL(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn IsIncluded(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsDefault(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn FollowFlags(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISearchScopeRule {}
#[cfg(feature = "Win32_Foundation")]
impl ISearchScopeRule_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchScopeRule_Impl, const OFFSET: isize>() -> ISearchScopeRule_Vtbl {
        unsafe extern "system" fn PatternOrURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchScopeRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpatternorurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PatternOrURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpatternorurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIncluded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchScopeRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisincluded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsIncluded() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisincluded, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDefault<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchScopeRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisdefault: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDefault() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisdefault, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FollowFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchScopeRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfollowflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FollowFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfollowflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PatternOrURL: PatternOrURL::<Identity, Impl, OFFSET>,
            IsIncluded: IsIncluded::<Identity, Impl, OFFSET>,
            IsDefault: IsDefault::<Identity, Impl, OFFSET>,
            FollowFlags: FollowFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchScopeRule as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISearchViewChangedSink_Impl: Sized {
    fn OnChange(&self, pdwdocid: *const i32, pchange: *const SEARCH_ITEM_CHANGE, pfinview: *const super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for ISearchViewChangedSink {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISearchViewChangedSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchViewChangedSink_Impl, const OFFSET: isize>() -> ISearchViewChangedSink_Vtbl {
        unsafe extern "system" fn OnChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISearchViewChangedSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdocid: *const i32, pchange: *const SEARCH_ITEM_CHANGE, pfinview: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnChange(::core::mem::transmute_copy(&pdwdocid), ::core::mem::transmute_copy(&pchange), ::core::mem::transmute_copy(&pfinview)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnChange: OnChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchViewChangedSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Security_Authorization\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Security_Authorization")]
pub trait ISecurityInfo_Impl: Sized {
    fn GetCurrentTrustee(&self, pptrustee: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::Result<()>;
    fn GetObjectTypes(&self, cobjecttypes: *mut u32, rgobjecttypes: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetPermissions(&self, objecttype: &::windows::core::GUID, ppermissions: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Security_Authorization")]
impl ::windows::core::RuntimeName for ISecurityInfo {}
#[cfg(feature = "Win32_Security_Authorization")]
impl ISecurityInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInfo_Impl, const OFFSET: isize>() -> ISecurityInfo_Vtbl {
        unsafe extern "system" fn GetCurrentTrustee<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptrustee: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentTrustee(::core::mem::transmute_copy(&pptrustee)).into()
        }
        unsafe extern "system" fn GetObjectTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cobjecttypes: *mut u32, rgobjecttypes: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectTypes(::core::mem::transmute_copy(&cobjecttypes), ::core::mem::transmute_copy(&rgobjecttypes)).into()
        }
        unsafe extern "system" fn GetPermissions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objecttype: ::windows::core::GUID, ppermissions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPermissions(::core::mem::transmute(&objecttype), ::core::mem::transmute_copy(&ppermissions)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentTrustee: GetCurrentTrustee::<Identity, Impl, OFFSET>,
            GetObjectTypes: GetObjectTypes::<Identity, Impl, OFFSET>,
            GetPermissions: GetPermissions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IService_Impl: Sized {
    fn InvokeService(&self, punkinner: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IService {}
impl IService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IService_Impl, const OFFSET: isize>() -> IService_Vtbl {
        unsafe extern "system" fn InvokeService<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkinner: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InvokeService(::windows::core::from_raw_borrowed(&punkinner)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InvokeService: InvokeService::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IService as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISessionProperties_Impl: Sized {
    fn GetProperties(&self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::Result<()>;
    fn SetProperties(&self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISessionProperties {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISessionProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISessionProperties_Impl, const OFFSET: isize>() -> ISessionProperties_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISessionProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProperties(::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)).into()
        }
        unsafe extern "system" fn SetProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISessionProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperties(::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            SetProperties: SetProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISessionProperties as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait ISimpleCommandCreator_Impl: Sized {
    fn CreateICommand(&self, ppiunknown: *mut ::core::option::Option<::windows::core::IUnknown>, pouterunk: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn VerifyCatalog(&self, pwszmachine: &::windows::core::PCWSTR, pwszcatalogname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetDefaultCatalog(&self, pwszcatalogname: &::windows::core::PCWSTR, cwcin: u32, pcwcout: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISimpleCommandCreator {}
impl ISimpleCommandCreator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimpleCommandCreator_Impl, const OFFSET: isize>() -> ISimpleCommandCreator_Vtbl {
        unsafe extern "system" fn CreateICommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimpleCommandCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiunknown: *mut *mut ::core::ffi::c_void, pouterunk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateICommand(::core::mem::transmute_copy(&ppiunknown), ::windows::core::from_raw_borrowed(&pouterunk)).into()
        }
        unsafe extern "system" fn VerifyCatalog<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimpleCommandCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszmachine: ::windows::core::PCWSTR, pwszcatalogname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VerifyCatalog(::core::mem::transmute(&pwszmachine), ::core::mem::transmute(&pwszcatalogname)).into()
        }
        unsafe extern "system" fn GetDefaultCatalog<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimpleCommandCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcatalogname: ::windows::core::PCWSTR, cwcin: u32, pcwcout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDefaultCatalog(::core::mem::transmute(&pwszcatalogname), ::core::mem::transmute_copy(&cwcin), ::core::mem::transmute_copy(&pcwcout)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateICommand: CreateICommand::<Identity, Impl, OFFSET>,
            VerifyCatalog: VerifyCatalog::<Identity, Impl, OFFSET>,
            GetDefaultCatalog: GetDefaultCatalog::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleCommandCreator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISourcesRowset_Impl: Sized {
    fn GetSourcesRowset(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, riid: *const ::windows::core::GUID, cpropertysets: u32, rgproperties: *mut DBPROPSET, ppsourcesrowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISourcesRowset {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISourcesRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISourcesRowset_Impl, const OFFSET: isize>() -> ISourcesRowset_Vtbl {
        unsafe extern "system" fn GetSourcesRowset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISourcesRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, cpropertysets: u32, rgproperties: *mut DBPROPSET, ppsourcesrowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSourcesRowset(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgproperties), ::core::mem::transmute_copy(&ppsourcesrowset)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSourcesRowset: GetSourcesRowset::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISourcesRowset as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IStemmer_Impl: Sized {
    fn Init(&self, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GenerateWordForms(&self, pwcinbuf: &::windows::core::PCWSTR, cwc: u32, pstemsink: ::core::option::Option<&IWordFormSink>) -> ::windows::core::Result<()>;
    fn GetLicenseToUse(&self, ppwcslicense: *const *const u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IStemmer {}
#[cfg(feature = "Win32_Foundation")]
impl IStemmer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStemmer_Impl, const OFFSET: isize>() -> IStemmer_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStemmer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Init(::core::mem::transmute_copy(&ulmaxtokensize), ::core::mem::transmute_copy(&pflicense)).into()
        }
        unsafe extern "system" fn GenerateWordForms<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStemmer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcinbuf: ::windows::core::PCWSTR, cwc: u32, pstemsink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GenerateWordForms(::core::mem::transmute(&pwcinbuf), ::core::mem::transmute_copy(&cwc), ::windows::core::from_raw_borrowed(&pstemsink)).into()
        }
        unsafe extern "system" fn GetLicenseToUse<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStemmer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwcslicense: *const *const u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLicenseToUse(::core::mem::transmute_copy(&ppwcslicense)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            GenerateWordForms: GenerateWordForms::<Identity, Impl, OFFSET>,
            GetLicenseToUse: GetLicenseToUse::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStemmer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISubscriptionItem_Impl: Sized {
    fn GetCookie(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetSubscriptionItemInfo(&self, psubscriptioniteminfo: *mut SUBSCRIPTIONITEMINFO) -> ::windows::core::Result<()>;
    fn SetSubscriptionItemInfo(&self, psubscriptioniteminfo: *const SUBSCRIPTIONITEMINFO) -> ::windows::core::Result<()>;
    fn ReadProperties(&self, ncount: u32, rgwszname: *const ::windows::core::PCWSTR, rgvalue: *mut super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn WriteProperties(&self, ncount: u32, rgwszname: *const ::windows::core::PCWSTR, rgvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn EnumProperties(&self) -> ::windows::core::Result<IEnumItemProperties>;
    fn NotifyChanged(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISubscriptionItem {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISubscriptionItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionItem_Impl, const OFFSET: isize>() -> ISubscriptionItem_Vtbl {
        unsafe extern "system" fn GetCookie<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcookie: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCookie() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriptionItemInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubscriptioniteminfo: *mut SUBSCRIPTIONITEMINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSubscriptionItemInfo(::core::mem::transmute_copy(&psubscriptioniteminfo)).into()
        }
        unsafe extern "system" fn SetSubscriptionItemInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubscriptioniteminfo: *const SUBSCRIPTIONITEMINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSubscriptionItemInfo(::core::mem::transmute_copy(&psubscriptioniteminfo)).into()
        }
        unsafe extern "system" fn ReadProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncount: u32, rgwszname: *const ::windows::core::PCWSTR, rgvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadProperties(::core::mem::transmute_copy(&ncount), ::core::mem::transmute_copy(&rgwszname), ::core::mem::transmute_copy(&rgvalue)).into()
        }
        unsafe extern "system" fn WriteProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncount: u32, rgwszname: *const ::windows::core::PCWSTR, rgvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteProperties(::core::mem::transmute_copy(&ncount), ::core::mem::transmute_copy(&rgwszname), ::core::mem::transmute_copy(&rgvalue)).into()
        }
        unsafe extern "system" fn EnumProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumitemproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumitemproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyChanged().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<ISubscriptionItem as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISubscriptionMgr_Impl: Sized {
    fn DeleteSubscription(&self, pwszurl: &::windows::core::PCWSTR, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn UpdateSubscription(&self, pwszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn UpdateAll(&self) -> ::windows::core::Result<()>;
    fn IsSubscribed(&self, pwszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetSubscriptionInfo(&self, pwszurl: &::windows::core::PCWSTR, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows::core::Result<()>;
    fn GetDefaultInfo(&self, subtype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows::core::Result<()>;
    fn ShowSubscriptionProperties(&self, pwszurl: &::windows::core::PCWSTR, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn CreateSubscription(&self, hwnd: super::super::Foundation::HWND, pwszurl: &::windows::core::PCWSTR, pwszfriendlyname: &::windows::core::PCWSTR, dwflags: u32, substype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISubscriptionMgr {}
#[cfg(feature = "Win32_Foundation")]
impl ISubscriptionMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionMgr_Impl, const OFFSET: isize>() -> ISubscriptionMgr_Vtbl {
        unsafe extern "system" fn DeleteSubscription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteSubscription(::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn UpdateSubscription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateSubscription(::core::mem::transmute(&pwszurl)).into()
        }
        unsafe extern "system" fn UpdateAll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateAll().into()
        }
        unsafe extern "system" fn IsSubscribed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, pfsubscribed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSubscribed(::core::mem::transmute(&pwszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsubscribed, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriptionInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSubscriptionInfo(::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn GetDefaultInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subtype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDefaultInfo(::core::mem::transmute_copy(&subtype), ::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn ShowSubscriptionProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowSubscriptionProperties(::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn CreateSubscription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pwszurl: ::windows::core::PCWSTR, pwszfriendlyname: ::windows::core::PCWSTR, dwflags: u32, substype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateSubscription(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&pwszurl), ::core::mem::transmute(&pwszfriendlyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&substype), ::core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<ISubscriptionMgr as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISubscriptionMgr2_Impl: Sized + ISubscriptionMgr_Impl {
    fn GetItemFromURL(&self, pwszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<ISubscriptionItem>;
    fn GetItemFromCookie(&self, psubscriptioncookie: *const ::windows::core::GUID) -> ::windows::core::Result<ISubscriptionItem>;
    fn GetSubscriptionRunState(&self, dwnumcookies: u32, pcookies: *const ::windows::core::GUID, pdwrunstate: *mut u32) -> ::windows::core::Result<()>;
    fn EnumSubscriptions(&self, dwflags: u32) -> ::windows::core::Result<IEnumSubscription>;
    fn UpdateItems(&self, dwflags: u32, dwnumcookies: u32, pcookies: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn AbortItems(&self, dwnumcookies: u32, pcookies: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn AbortAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISubscriptionMgr2 {}
#[cfg(feature = "Win32_Foundation")]
impl ISubscriptionMgr2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionMgr2_Impl, const OFFSET: isize>() -> ISubscriptionMgr2_Vtbl {
        unsafe extern "system" fn GetItemFromURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, ppsubscriptionitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemFromURL(::core::mem::transmute(&pwszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubscriptionitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemFromCookie<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubscriptioncookie: *const ::windows::core::GUID, ppsubscriptionitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemFromCookie(::core::mem::transmute_copy(&psubscriptioncookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubscriptionitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriptionRunState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnumcookies: u32, pcookies: *const ::windows::core::GUID, pdwrunstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSubscriptionRunState(::core::mem::transmute_copy(&dwnumcookies), ::core::mem::transmute_copy(&pcookies), ::core::mem::transmute_copy(&pdwrunstate)).into()
        }
        unsafe extern "system" fn EnumSubscriptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenumsubscriptions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumSubscriptions(::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumsubscriptions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateItems<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwnumcookies: u32, pcookies: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateItems(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwnumcookies), ::core::mem::transmute_copy(&pcookies)).into()
        }
        unsafe extern "system" fn AbortItems<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnumcookies: u32, pcookies: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AbortItems(::core::mem::transmute_copy(&dwnumcookies), ::core::mem::transmute_copy(&pcookies)).into()
        }
        unsafe extern "system" fn AbortAll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubscriptionMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AbortAll().into()
        }
        Self {
            base__: ISubscriptionMgr_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<ISubscriptionMgr2 as ::windows::core::ComInterface>::IID || iid == &<ISubscriptionMgr as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITableCreation_Impl: Sized + ITableDefinition_Impl {
    fn GetTableDefinition(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pccolumndescs: *mut usize, prgcolumndescs: *mut *mut DBCOLUMNDESC, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET, pcconstraintdescs: *mut u32, prgconstraintdescs: *mut *mut DBCONSTRAINTDESC, ppwszstringbuffer: *mut *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITableCreation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITableCreation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITableCreation_Impl, const OFFSET: isize>() -> ITableCreation_Vtbl {
        unsafe extern "system" fn GetTableDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITableCreation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pccolumndescs: *mut usize, prgcolumndescs: *mut *mut DBCOLUMNDESC, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET, pcconstraintdescs: *mut u32, prgconstraintdescs: *mut *mut DBCONSTRAINTDESC, ppwszstringbuffer: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTableDefinition(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pccolumndescs), ::core::mem::transmute_copy(&prgcolumndescs), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets), ::core::mem::transmute_copy(&pcconstraintdescs), ::core::mem::transmute_copy(&prgconstraintdescs), ::core::mem::transmute_copy(&ppwszstringbuffer)).into()
        }
        Self { base__: ITableDefinition_Vtbl::new::<Identity, Impl, OFFSET>(), GetTableDefinition: GetTableDefinition::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITableCreation as ::windows::core::ComInterface>::IID || iid == &<ITableDefinition as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITableDefinition_Impl: Sized {
    fn CreateTable(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, ptableid: *const super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *const DBCOLUMNDESC, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DropTable(&self, ptableid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
    fn AddColumn(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumndesc: *const DBCOLUMNDESC, ppcolumnid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
    fn DropColumn(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumnid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITableDefinition {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITableDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITableDefinition_Impl, const OFFSET: isize>() -> ITableDefinition_Vtbl {
        unsafe extern "system" fn CreateTable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITableDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *const DBCOLUMNDESC, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateTable(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&ccolumndescs), ::core::mem::transmute_copy(&rgcolumndescs), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pptableid), ::core::mem::transmute_copy(&pprowset)).into()
        }
        unsafe extern "system" fn DropTable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITableDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DropTable(::core::mem::transmute_copy(&ptableid)).into()
        }
        unsafe extern "system" fn AddColumn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITableDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumndesc: *const DBCOLUMNDESC, ppcolumnid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddColumn(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pcolumndesc), ::core::mem::transmute_copy(&ppcolumnid)).into()
        }
        unsafe extern "system" fn DropColumn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITableDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumnid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DropColumn(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pcolumnid)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTable: CreateTable::<Identity, Impl, OFFSET>,
            DropTable: DropTable::<Identity, Impl, OFFSET>,
            AddColumn: AddColumn::<Identity, Impl, OFFSET>,
            DropColumn: DropColumn::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITableDefinition as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITableDefinitionWithConstraints_Impl: Sized + ITableCreation_Impl {
    fn AddConstraint(&self, ptableid: *mut super::super::Storage::IndexServer::DBID, pconstraintdesc: *mut DBCONSTRAINTDESC) -> ::windows::core::Result<()>;
    fn CreateTableWithConstraints(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, ptableid: *mut super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *mut DBCOLUMNDESC, cconstraintdescs: u32, rgconstraintdescs: *mut DBCONSTRAINTDESC, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DropConstraint(&self, ptableid: *mut super::super::Storage::IndexServer::DBID, pconstraintid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITableDefinitionWithConstraints {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITableDefinitionWithConstraints_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITableDefinitionWithConstraints_Impl, const OFFSET: isize>() -> ITableDefinitionWithConstraints_Vtbl {
        unsafe extern "system" fn AddConstraint<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITableDefinitionWithConstraints_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, pconstraintdesc: *mut DBCONSTRAINTDESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddConstraint(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pconstraintdesc)).into()
        }
        unsafe extern "system" fn CreateTableWithConstraints<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITableDefinitionWithConstraints_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *mut DBCOLUMNDESC, cconstraintdescs: u32, rgconstraintdescs: *mut DBCONSTRAINTDESC, riid: *const ::windows::core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateTableWithConstraints(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&ccolumndescs), ::core::mem::transmute_copy(&rgcolumndescs), ::core::mem::transmute_copy(&cconstraintdescs), ::core::mem::transmute_copy(&rgconstraintdescs), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pptableid), ::core::mem::transmute_copy(&pprowset))
                .into()
        }
        unsafe extern "system" fn DropConstraint<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITableDefinitionWithConstraints_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, pconstraintid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DropConstraint(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pconstraintid)).into()
        }
        Self {
            base__: ITableCreation_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddConstraint: AddConstraint::<Identity, Impl, OFFSET>,
            CreateTableWithConstraints: CreateTableWithConstraints::<Identity, Impl, OFFSET>,
            DropConstraint: DropConstraint::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITableDefinitionWithConstraints as ::windows::core::ComInterface>::IID || iid == &<ITableDefinition as ::windows::core::ComInterface>::IID || iid == &<ITableCreation as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait ITableRename_Impl: Sized {
    fn RenameColumn(&self, ptableid: *mut super::super::Storage::IndexServer::DBID, poldcolumnid: *mut super::super::Storage::IndexServer::DBID, pnewcolumnid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
    fn RenameTable(&self, poldtableid: *mut super::super::Storage::IndexServer::DBID, poldindexid: *mut super::super::Storage::IndexServer::DBID, pnewtableid: *mut super::super::Storage::IndexServer::DBID, pnewindexid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::windows::core::RuntimeName for ITableRename {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ITableRename_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITableRename_Impl, const OFFSET: isize>() -> ITableRename_Vtbl {
        unsafe extern "system" fn RenameColumn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITableRename_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, poldcolumnid: *mut super::super::Storage::IndexServer::DBID, pnewcolumnid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RenameColumn(::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&poldcolumnid), ::core::mem::transmute_copy(&pnewcolumnid)).into()
        }
        unsafe extern "system" fn RenameTable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITableRename_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poldtableid: *mut super::super::Storage::IndexServer::DBID, poldindexid: *mut super::super::Storage::IndexServer::DBID, pnewtableid: *mut super::super::Storage::IndexServer::DBID, pnewindexid: *mut super::super::Storage::IndexServer::DBID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RenameTable(::core::mem::transmute_copy(&poldtableid), ::core::mem::transmute_copy(&poldindexid), ::core::mem::transmute_copy(&pnewtableid), ::core::mem::transmute_copy(&pnewindexid)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RenameColumn: RenameColumn::<Identity, Impl, OFFSET>,
            RenameTable: RenameTable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITableRename as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait ITokenCollection_Impl: Sized {
    fn NumberOfTokens(&self, pcount: *const u32) -> ::windows::core::Result<()>;
    fn GetToken(&self, i: u32, pbegin: *mut u32, plength: *mut u32, ppsz: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITokenCollection {}
impl ITokenCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITokenCollection_Impl, const OFFSET: isize>() -> ITokenCollection_Vtbl {
        unsafe extern "system" fn NumberOfTokens<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITokenCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NumberOfTokens(::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn GetToken<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITokenCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: u32, pbegin: *mut u32, plength: *mut u32, ppsz: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetToken(::core::mem::transmute_copy(&i), ::core::mem::transmute_copy(&pbegin), ::core::mem::transmute_copy(&plength), ::core::mem::transmute_copy(&ppsz)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NumberOfTokens: NumberOfTokens::<Identity, Impl, OFFSET>,
            GetToken: GetToken::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITokenCollection as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_DistributedTransactionCoordinator\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ITransactionJoin_Impl: Sized {
    fn GetOptionsObject(&self) -> ::windows::core::Result<super::DistributedTransactionCoordinator::ITransactionOptions>;
    fn JoinTransaction(&self, punktransactioncoord: ::core::option::Option<&::windows::core::IUnknown>, isolevel: i32, isoflags: u32, potheroptions: ::core::option::Option<&super::DistributedTransactionCoordinator::ITransactionOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ::windows::core::RuntimeName for ITransactionJoin {}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ITransactionJoin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionJoin_Impl, const OFFSET: isize>() -> ITransactionJoin_Vtbl {
        unsafe extern "system" fn GetOptionsObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionJoin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOptionsObject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoptions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JoinTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionJoin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punktransactioncoord: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, potheroptions: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.JoinTransaction(::windows::core::from_raw_borrowed(&punktransactioncoord), ::core::mem::transmute_copy(&isolevel), ::core::mem::transmute_copy(&isoflags), ::windows::core::from_raw_borrowed(&potheroptions)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOptionsObject: GetOptionsObject::<Identity, Impl, OFFSET>,
            JoinTransaction: JoinTransaction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionJoin as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_DistributedTransactionCoordinator\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
pub trait ITransactionLocal_Impl: Sized + super::DistributedTransactionCoordinator::ITransaction_Impl {
    fn GetOptionsObject(&self) -> ::windows::core::Result<super::DistributedTransactionCoordinator::ITransactionOptions>;
    fn StartTransaction(&self, isolevel: i32, isoflags: u32, potheroptions: ::core::option::Option<&super::DistributedTransactionCoordinator::ITransactionOptions>, pultransactionlevel: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
impl ::windows::core::RuntimeName for ITransactionLocal {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
impl ITransactionLocal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionLocal_Impl, const OFFSET: isize>() -> ITransactionLocal_Vtbl {
        unsafe extern "system" fn GetOptionsObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionLocal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOptionsObject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoptions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionLocal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, potheroptions: *mut ::core::ffi::c_void, pultransactionlevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartTransaction(::core::mem::transmute_copy(&isolevel), ::core::mem::transmute_copy(&isoflags), ::windows::core::from_raw_borrowed(&potheroptions), ::core::mem::transmute_copy(&pultransactionlevel)).into()
        }
        Self {
            base__: super::DistributedTransactionCoordinator::ITransaction_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOptionsObject: GetOptionsObject::<Identity, Impl, OFFSET>,
            StartTransaction: StartTransaction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionLocal as ::windows::core::ComInterface>::IID || iid == &<super::DistributedTransactionCoordinator::ITransaction as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_DistributedTransactionCoordinator\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ITransactionObject_Impl: Sized {
    fn GetTransactionObject(&self, ultransactionlevel: u32) -> ::windows::core::Result<super::DistributedTransactionCoordinator::ITransaction>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ::windows::core::RuntimeName for ITransactionObject {}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ITransactionObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionObject_Impl, const OFFSET: isize>() -> ITransactionObject_Vtbl {
        unsafe extern "system" fn GetTransactionObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultransactionlevel: u32, pptransactionobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransactionObject(::core::mem::transmute_copy(&ultransactionlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptransactionobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetTransactionObject: GetTransactionObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authorization\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITrusteeAdmin_Impl: Sized {
    fn CompareTrustees(&self, ptrustee1: *mut super::super::Security::Authorization::TRUSTEE_W, ptrustee2: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::Result<()>;
    fn CreateTrustee(&self, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::Result<()>;
    fn DeleteTrustee(&self, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::Result<()>;
    fn SetTrusteeProperties(&self, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::Result<()>;
    fn GetTrusteeProperties(&self, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITrusteeAdmin {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITrusteeAdmin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrusteeAdmin_Impl, const OFFSET: isize>() -> ITrusteeAdmin_Vtbl {
        unsafe extern "system" fn CompareTrustees<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrusteeAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrustee1: *mut super::super::Security::Authorization::TRUSTEE_W, ptrustee2: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompareTrustees(::core::mem::transmute_copy(&ptrustee1), ::core::mem::transmute_copy(&ptrustee2)).into()
        }
        unsafe extern "system" fn CreateTrustee<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrusteeAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateTrustee(::core::mem::transmute_copy(&ptrustee), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into()
        }
        unsafe extern "system" fn DeleteTrustee<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrusteeAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteTrustee(::core::mem::transmute_copy(&ptrustee)).into()
        }
        unsafe extern "system" fn SetTrusteeProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrusteeAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTrusteeProperties(::core::mem::transmute_copy(&ptrustee), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into()
        }
        unsafe extern "system" fn GetTrusteeProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrusteeAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTrusteeProperties(::core::mem::transmute_copy(&ptrustee), ::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CompareTrustees: CompareTrustees::<Identity, Impl, OFFSET>,
            CreateTrustee: CreateTrustee::<Identity, Impl, OFFSET>,
            DeleteTrustee: DeleteTrustee::<Identity, Impl, OFFSET>,
            SetTrusteeProperties: SetTrusteeProperties::<Identity, Impl, OFFSET>,
            GetTrusteeProperties: GetTrusteeProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITrusteeAdmin as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authorization\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization"))]
pub trait ITrusteeGroupAdmin_Impl: Sized {
    fn AddMember(&self, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::Result<()>;
    fn DeleteMember(&self, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::Result<()>;
    fn IsMember(&self, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pfstatus: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetMembers(&self, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pcmembers: *mut u32, prgmembers: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::Result<()>;
    fn GetMemberships(&self, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pcmemberships: *mut u32, prgmemberships: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization"))]
impl ::windows::core::RuntimeName for ITrusteeGroupAdmin {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization"))]
impl ITrusteeGroupAdmin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrusteeGroupAdmin_Impl, const OFFSET: isize>() -> ITrusteeGroupAdmin_Vtbl {
        unsafe extern "system" fn AddMember<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrusteeGroupAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMember(::core::mem::transmute_copy(&pmembershiptrustee), ::core::mem::transmute_copy(&pmembertrustee)).into()
        }
        unsafe extern "system" fn DeleteMember<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrusteeGroupAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteMember(::core::mem::transmute_copy(&pmembershiptrustee), ::core::mem::transmute_copy(&pmembertrustee)).into()
        }
        unsafe extern "system" fn IsMember<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrusteeGroupAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pfstatus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsMember(::core::mem::transmute_copy(&pmembershiptrustee), ::core::mem::transmute_copy(&pmembertrustee), ::core::mem::transmute_copy(&pfstatus)).into()
        }
        unsafe extern "system" fn GetMembers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrusteeGroupAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmembershiptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pcmembers: *mut u32, prgmembers: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMembers(::core::mem::transmute_copy(&pmembershiptrustee), ::core::mem::transmute_copy(&pcmembers), ::core::mem::transmute_copy(&prgmembers)).into()
        }
        unsafe extern "system" fn GetMemberships<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrusteeGroupAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrustee: *mut super::super::Security::Authorization::TRUSTEE_W, pcmemberships: *mut u32, prgmemberships: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMemberships(::core::mem::transmute_copy(&ptrustee), ::core::mem::transmute_copy(&pcmemberships), ::core::mem::transmute_copy(&prgmemberships)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddMember: AddMember::<Identity, Impl, OFFSET>,
            DeleteMember: DeleteMember::<Identity, Impl, OFFSET>,
            IsMember: IsMember::<Identity, Impl, OFFSET>,
            GetMembers: GetMembers::<Identity, Impl, OFFSET>,
            GetMemberships: GetMemberships::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITrusteeGroupAdmin as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUMS_Impl: Sized {
    fn SqlUmsSuspend(&self, ticks: u32);
    fn SqlUmsYield(&self, ticks: u32);
    fn SqlUmsSwitchPremptive(&self);
    fn SqlUmsSwitchNonPremptive(&self);
    fn SqlUmsFIsPremptive(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl IUMS_Vtbl {
    pub const fn new<Impl: IUMS_Impl>() -> IUMS_Vtbl {
        unsafe extern "system" fn SqlUmsSuspend<Impl: IUMS_Impl>(this: *mut ::core::ffi::c_void, ticks: u32) {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows::core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            this.SqlUmsSuspend(::core::mem::transmute_copy(&ticks))
        }
        unsafe extern "system" fn SqlUmsYield<Impl: IUMS_Impl>(this: *mut ::core::ffi::c_void, ticks: u32) {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows::core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            this.SqlUmsYield(::core::mem::transmute_copy(&ticks))
        }
        unsafe extern "system" fn SqlUmsSwitchPremptive<Impl: IUMS_Impl>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows::core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            this.SqlUmsSwitchPremptive()
        }
        unsafe extern "system" fn SqlUmsSwitchNonPremptive<Impl: IUMS_Impl>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows::core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            this.SqlUmsSwitchNonPremptive()
        }
        unsafe extern "system" fn SqlUmsFIsPremptive<Impl: IUMS_Impl>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows::core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            this.SqlUmsFIsPremptive()
        }
        Self {
            SqlUmsSuspend: SqlUmsSuspend::<Impl>,
            SqlUmsYield: SqlUmsYield::<Impl>,
            SqlUmsSwitchPremptive: SqlUmsSwitchPremptive::<Impl>,
            SqlUmsSwitchNonPremptive: SqlUmsSwitchNonPremptive::<Impl>,
            SqlUmsFIsPremptive: SqlUmsFIsPremptive::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct IUMS_ImplVtbl<T: IUMS_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: IUMS_Impl> IUMS_ImplVtbl<T> {
    const VTABLE: IUMS_Vtbl = IUMS_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl IUMS {
    pub fn new<'a, T: IUMS_Impl>(this: &'a T) -> ::windows::core::ScopedInterface<'a, Self> {
        let this = ::windows::core::ScopedHeap { vtable: &IUMS_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
        unsafe { ::windows::core::ScopedInterface::new(::std::mem::transmute(&this.vtable)) }
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IUMSInitialize_Impl: Sized {
    fn Initialize(&self, pums: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IUMSInitialize {}
impl IUMSInitialize_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUMSInitialize_Impl, const OFFSET: isize>() -> IUMSInitialize_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUMSInitialize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pums: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&pums)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUMSInitialize as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IUrlAccessor_Impl: Sized {
    fn AddRequestParameter(&self, pspec: *const super::Com::StructuredStorage::PROPSPEC, pvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetDocFormat(&self, wszdocformat: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()>;
    fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetHost(&self, wszhost: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()>;
    fn IsDirectory(&self) -> ::windows::core::Result<()>;
    fn GetSize(&self) -> ::windows::core::Result<u64>;
    fn GetLastModified(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn GetFileName(&self, wszfilename: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()>;
    fn GetSecurityDescriptor(&self, psd: *mut u8, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()>;
    fn GetRedirectedURL(&self, wszredirectedurl: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()>;
    fn GetSecurityProvider(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BindToStream(&self) -> ::windows::core::Result<super::Com::IStream>;
    fn BindToFilter(&self) -> ::windows::core::Result<super::super::Storage::IndexServer::IFilter>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IUrlAccessor {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl IUrlAccessor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: isize>() -> IUrlAccessor_Vtbl {
        unsafe extern "system" fn AddRequestParameter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pspec: *const super::Com::StructuredStorage::PROPSPEC, pvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRequestParameter(::core::mem::transmute_copy(&pspec), ::core::mem::transmute_copy(&pvar)).into()
        }
        unsafe extern "system" fn GetDocFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszdocformat: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDocFormat(::core::mem::transmute_copy(&wszdocformat), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetCLSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHost<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszhost: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHost(::core::mem::transmute_copy(&wszhost), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn IsDirectory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsDirectory().into()
        }
        unsafe extern "system" fn GetSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pllsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastModified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftlastmodified: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastModified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pftlastmodified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszfilename: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFileName(::core::mem::transmute_copy(&wszfilename), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psd: *mut u8, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSecurityDescriptor(::core::mem::transmute_copy(&psd), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetRedirectedURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszredirectedurl: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRedirectedURL(::core::mem::transmute_copy(&wszredirectedurl), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetSecurityProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pspclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSecurityProvider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pspclsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BindToStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfilter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BindToFilter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfilter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IUrlAccessor as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IUrlAccessor2_Impl: Sized + IUrlAccessor_Impl {
    fn GetDisplayUrl(&self, wszdocurl: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()>;
    fn IsDocument(&self) -> ::windows::core::Result<()>;
    fn GetCodePage(&self, wszcodepage: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IUrlAccessor2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl IUrlAccessor2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor2_Impl, const OFFSET: isize>() -> IUrlAccessor2_Vtbl {
        unsafe extern "system" fn GetDisplayUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszdocurl: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayUrl(::core::mem::transmute_copy(&wszdocurl), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn IsDocument<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsDocument().into()
        }
        unsafe extern "system" fn GetCodePage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszcodepage: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCodePage(::core::mem::transmute_copy(&wszcodepage), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        Self {
            base__: IUrlAccessor_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDisplayUrl: GetDisplayUrl::<Identity, Impl, OFFSET>,
            IsDocument: IsDocument::<Identity, Impl, OFFSET>,
            GetCodePage: GetCodePage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUrlAccessor2 as ::windows::core::ComInterface>::IID || iid == &<IUrlAccessor as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IUrlAccessor3_Impl: Sized + IUrlAccessor2_Impl {
    fn GetImpersonationSidBlobs(&self, pcwszurl: &::windows::core::PCWSTR, pcsidcount: *mut u32, ppsidblobs: *mut *mut super::Com::BLOB) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IUrlAccessor3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl IUrlAccessor3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor3_Impl, const OFFSET: isize>() -> IUrlAccessor3_Vtbl {
        unsafe extern "system" fn GetImpersonationSidBlobs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcwszurl: ::windows::core::PCWSTR, pcsidcount: *mut u32, ppsidblobs: *mut *mut super::Com::BLOB) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetImpersonationSidBlobs(::core::mem::transmute(&pcwszurl), ::core::mem::transmute_copy(&pcsidcount), ::core::mem::transmute_copy(&ppsidblobs)).into()
        }
        Self { base__: IUrlAccessor2_Vtbl::new::<Identity, Impl, OFFSET>(), GetImpersonationSidBlobs: GetImpersonationSidBlobs::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUrlAccessor3 as ::windows::core::ComInterface>::IID || iid == &<IUrlAccessor as ::windows::core::ComInterface>::IID || iid == &<IUrlAccessor2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IUrlAccessor4_Impl: Sized + IUrlAccessor3_Impl {
    fn ShouldIndexItemContent(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ShouldIndexProperty(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows::core::RuntimeName for IUrlAccessor4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IUrlAccessor4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor4_Impl, const OFFSET: isize>() -> IUrlAccessor4_Vtbl {
        unsafe extern "system" fn ShouldIndexItemContent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfindexcontent: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ShouldIndexItemContent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfindexcontent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldIndexProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlAccessor4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfindexproperty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ShouldIndexProperty(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfindexproperty, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUrlAccessor3_Vtbl::new::<Identity, Impl, OFFSET>(),
            ShouldIndexItemContent: ShouldIndexItemContent::<Identity, Impl, OFFSET>,
            ShouldIndexProperty: ShouldIndexProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUrlAccessor4 as ::windows::core::ComInterface>::IID || iid == &<IUrlAccessor as ::windows::core::ComInterface>::IID || iid == &<IUrlAccessor2 as ::windows::core::ComInterface>::IID || iid == &<IUrlAccessor3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IViewChapter_Impl: Sized {
    fn GetSpecification(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn OpenViewChapter(&self, hsource: usize, phviewchapter: *mut usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IViewChapter {}
impl IViewChapter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IViewChapter_Impl, const OFFSET: isize>() -> IViewChapter_Vtbl {
        unsafe extern "system" fn GetSpecification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IViewChapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSpecification(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprowset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenViewChapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IViewChapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsource: usize, phviewchapter: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenViewChapter(::core::mem::transmute_copy(&hsource), ::core::mem::transmute_copy(&phviewchapter)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSpecification: GetSpecification::<Identity, Impl, OFFSET>,
            OpenViewChapter: OpenViewChapter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewChapter as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IViewFilter_Impl: Sized {
    fn GetFilter(&self, haccessor: HACCESSOR, pcrows: *mut usize, pcompareops: *mut *mut u32, pcriteriadata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetFilterBindings(&self, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> ::windows::core::Result<()>;
    fn SetFilter(&self, haccessor: HACCESSOR, crows: usize, compareops: *const u32, pcriteriadata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IViewFilter {}
#[cfg(feature = "Win32_System_Com")]
impl IViewFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IViewFilter_Impl, const OFFSET: isize>() -> IViewFilter_Vtbl {
        unsafe extern "system" fn GetFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IViewFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haccessor: HACCESSOR, pcrows: *mut usize, pcompareops: *mut *mut u32, pcriteriadata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFilter(::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pcrows), ::core::mem::transmute_copy(&pcompareops), ::core::mem::transmute_copy(&pcriteriadata)).into()
        }
        unsafe extern "system" fn GetFilterBindings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IViewFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFilterBindings(::core::mem::transmute_copy(&pcbindings), ::core::mem::transmute_copy(&prgbindings)).into()
        }
        unsafe extern "system" fn SetFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IViewFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haccessor: HACCESSOR, crows: usize, compareops: *const u32, pcriteriadata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFilter(::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&compareops), ::core::mem::transmute_copy(&pcriteriadata)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFilter: GetFilter::<Identity, Impl, OFFSET>,
            GetFilterBindings: GetFilterBindings::<Identity, Impl, OFFSET>,
            SetFilter: SetFilter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewFilter as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IViewRowset_Impl: Sized {
    fn GetSpecification(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn OpenViewRowset(&self, punkouter: ::core::option::Option<&::windows::core::IUnknown>, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ::windows::core::RuntimeName for IViewRowset {}
impl IViewRowset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IViewRowset_Impl, const OFFSET: isize>() -> IViewRowset_Vtbl {
        unsafe extern "system" fn GetSpecification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IViewRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSpecification(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenViewRowset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IViewRowset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenViewRowset(::windows::core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprowset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSpecification: GetSpecification::<Identity, Impl, OFFSET>,
            OpenViewRowset: OpenViewRowset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewRowset as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IViewSort_Impl: Sized {
    fn GetSortOrder(&self, pcvalues: *mut usize, prgcolumns: *mut *mut usize, prgorders: *mut *mut u32) -> ::windows::core::Result<()>;
    fn SetSortOrder(&self, cvalues: usize, rgcolumns: *const usize, rgorders: *const u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IViewSort {}
impl IViewSort_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IViewSort_Impl, const OFFSET: isize>() -> IViewSort_Vtbl {
        unsafe extern "system" fn GetSortOrder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IViewSort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcvalues: *mut usize, prgcolumns: *mut *mut usize, prgorders: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSortOrder(::core::mem::transmute_copy(&pcvalues), ::core::mem::transmute_copy(&prgcolumns), ::core::mem::transmute_copy(&prgorders)).into()
        }
        unsafe extern "system" fn SetSortOrder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IViewSort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cvalues: usize, rgcolumns: *const usize, rgorders: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSortOrder(::core::mem::transmute_copy(&cvalues), ::core::mem::transmute_copy(&rgcolumns), ::core::mem::transmute_copy(&rgorders)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSortOrder: GetSortOrder::<Identity, Impl, OFFSET>,
            SetSortOrder: SetSortOrder::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewSort as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
pub trait IWordBreaker_Impl: Sized {
    fn Init(&self, fquery: super::super::Foundation::BOOL, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn BreakText(&self, ptextsource: *mut TEXT_SOURCE, pwordsink: ::core::option::Option<&IWordSink>, pphrasesink: ::core::option::Option<&super::super::Storage::IndexServer::IPhraseSink>) -> ::windows::core::Result<()>;
    fn ComposePhrase(&self, pwcnoun: &::windows::core::PCWSTR, cwcnoun: u32, pwcmodifier: &::windows::core::PCWSTR, cwcmodifier: u32, ulattachmenttype: u32, pwcphrase: &::windows::core::PCWSTR, pcwcphrase: *mut u32) -> ::windows::core::Result<()>;
    fn GetLicenseToUse(&self, ppwcslicense: *const *const u16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
impl ::windows::core::RuntimeName for IWordBreaker {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
impl IWordBreaker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordBreaker_Impl, const OFFSET: isize>() -> IWordBreaker_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordBreaker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fquery: super::super::Foundation::BOOL, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Init(::core::mem::transmute_copy(&fquery), ::core::mem::transmute_copy(&ulmaxtokensize), ::core::mem::transmute_copy(&pflicense)).into()
        }
        unsafe extern "system" fn BreakText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordBreaker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptextsource: *mut TEXT_SOURCE, pwordsink: *mut ::core::ffi::c_void, pphrasesink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BreakText(::core::mem::transmute_copy(&ptextsource), ::windows::core::from_raw_borrowed(&pwordsink), ::windows::core::from_raw_borrowed(&pphrasesink)).into()
        }
        unsafe extern "system" fn ComposePhrase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordBreaker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcnoun: ::windows::core::PCWSTR, cwcnoun: u32, pwcmodifier: ::windows::core::PCWSTR, cwcmodifier: u32, ulattachmenttype: u32, pwcphrase: ::windows::core::PCWSTR, pcwcphrase: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ComposePhrase(::core::mem::transmute(&pwcnoun), ::core::mem::transmute_copy(&cwcnoun), ::core::mem::transmute(&pwcmodifier), ::core::mem::transmute_copy(&cwcmodifier), ::core::mem::transmute_copy(&ulattachmenttype), ::core::mem::transmute(&pwcphrase), ::core::mem::transmute_copy(&pcwcphrase)).into()
        }
        unsafe extern "system" fn GetLicenseToUse<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordBreaker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwcslicense: *const *const u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLicenseToUse(::core::mem::transmute_copy(&ppwcslicense)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            BreakText: BreakText::<Identity, Impl, OFFSET>,
            ComposePhrase: ComposePhrase::<Identity, Impl, OFFSET>,
            GetLicenseToUse: GetLicenseToUse::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWordBreaker as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait IWordFormSink_Impl: Sized {
    fn PutAltWord(&self, pwcinbuf: &::windows::core::PCWSTR, cwc: u32) -> ::windows::core::Result<()>;
    fn PutWord(&self, pwcinbuf: &::windows::core::PCWSTR, cwc: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWordFormSink {}
impl IWordFormSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordFormSink_Impl, const OFFSET: isize>() -> IWordFormSink_Vtbl {
        unsafe extern "system" fn PutAltWord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordFormSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcinbuf: ::windows::core::PCWSTR, cwc: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutAltWord(::core::mem::transmute(&pwcinbuf), ::core::mem::transmute_copy(&cwc)).into()
        }
        unsafe extern "system" fn PutWord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordFormSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcinbuf: ::windows::core::PCWSTR, cwc: u32) -> ::windows::core::HRESULT {
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
        iid == &<IWordFormSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Storage_IndexServer\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IWordSink_Impl: Sized {
    fn PutWord(&self, cwc: u32, pwcinbuf: &::windows::core::PCWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> ::windows::core::Result<()>;
    fn PutAltWord(&self, cwc: u32, pwcinbuf: &::windows::core::PCWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> ::windows::core::Result<()>;
    fn StartAltPhrase(&self) -> ::windows::core::Result<()>;
    fn EndAltPhrase(&self) -> ::windows::core::Result<()>;
    fn PutBreak(&self, breaktype: super::super::Storage::IndexServer::WORDREP_BREAK_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::windows::core::RuntimeName for IWordSink {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IWordSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordSink_Impl, const OFFSET: isize>() -> IWordSink_Vtbl {
        unsafe extern "system" fn PutWord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cwc: u32, pwcinbuf: ::windows::core::PCWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutWord(::core::mem::transmute_copy(&cwc), ::core::mem::transmute(&pwcinbuf), ::core::mem::transmute_copy(&cwcsrclen), ::core::mem::transmute_copy(&cwcsrcpos)).into()
        }
        unsafe extern "system" fn PutAltWord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cwc: u32, pwcinbuf: ::windows::core::PCWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutAltWord(::core::mem::transmute_copy(&cwc), ::core::mem::transmute(&pwcinbuf), ::core::mem::transmute_copy(&cwcsrclen), ::core::mem::transmute_copy(&cwcsrcpos)).into()
        }
        unsafe extern "system" fn StartAltPhrase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartAltPhrase().into()
        }
        unsafe extern "system" fn EndAltPhrase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndAltPhrase().into()
        }
        unsafe extern "system" fn PutBreak<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWordSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, breaktype: super::super::Storage::IndexServer::WORDREP_BREAK_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutBreak(::core::mem::transmute_copy(&breaktype)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PutWord: PutWord::<Identity, Impl, OFFSET>,
            PutAltWord: PutAltWord::<Identity, Impl, OFFSET>,
            StartAltPhrase: StartAltPhrase::<Identity, Impl, OFFSET>,
            EndAltPhrase: EndAltPhrase::<Identity, Impl, OFFSET>,
            PutBreak: PutBreak::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWordSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait OLEDBSimpleProvider_Impl: Sized {
    fn getRowCount(&self) -> ::windows::core::Result<isize>;
    fn getColumnCount(&self) -> ::windows::core::Result<isize>;
    fn getRWStatus(&self, irow: isize, icolumn: isize) -> ::windows::core::Result<OSPRW>;
    fn getVariant(&self, irow: isize, icolumn: isize, format: OSPFORMAT) -> ::windows::core::Result<super::Com::VARIANT>;
    fn setVariant(&self, irow: isize, icolumn: isize, format: OSPFORMAT, var: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn getLocale(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn deleteRows(&self, irow: isize, crows: isize) -> ::windows::core::Result<isize>;
    fn insertRows(&self, irow: isize, crows: isize) -> ::windows::core::Result<isize>;
    fn find(&self, irowstart: isize, icolumn: isize, val: &super::Com::VARIANT, findflags: OSPFIND, comptype: OSPCOMP) -> ::windows::core::Result<isize>;
    fn addOLEDBSimpleProviderListener(&self, pospilistener: ::core::option::Option<&OLEDBSimpleProviderListener>) -> ::windows::core::Result<()>;
    fn removeOLEDBSimpleProviderListener(&self, pospilistener: ::core::option::Option<&OLEDBSimpleProviderListener>) -> ::windows::core::Result<()>;
    fn isAsync(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn getEstimatedRows(&self) -> ::windows::core::Result<isize>;
    fn stopTransfer(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for OLEDBSimpleProvider {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl OLEDBSimpleProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>() -> OLEDBSimpleProvider_Vtbl {
        unsafe extern "system" fn getRowCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcrows: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getRowCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcrows, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getColumnCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccolumns: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getColumnCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccolumns, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getRWStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize, prwstatus: *mut OSPRW) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getRWStatus(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&icolumn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prwstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getVariant<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize, format: OSPFORMAT, pvar: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getVariant(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&icolumn), ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvar, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setVariant<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize, format: OSPFORMAT, var: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setVariant(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&icolumn), ::core::mem::transmute_copy(&format), ::core::mem::transmute(&var)).into()
        }
        unsafe extern "system" fn getLocale<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlocale: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getLocale() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlocale, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn deleteRows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize, pcrowsdeleted: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.deleteRows(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcrowsdeleted, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn insertRows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize, pcrowsinserted: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.insertRows(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcrowsinserted, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn find<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irowstart: isize, icolumn: isize, val: super::Com::VARIANT, findflags: OSPFIND, comptype: OSPCOMP, pirowfound: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.find(::core::mem::transmute_copy(&irowstart), ::core::mem::transmute_copy(&icolumn), ::core::mem::transmute(&val), ::core::mem::transmute_copy(&findflags), ::core::mem::transmute_copy(&comptype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pirowfound, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addOLEDBSimpleProviderListener<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pospilistener: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addOLEDBSimpleProviderListener(::windows::core::from_raw_borrowed(&pospilistener)).into()
        }
        unsafe extern "system" fn removeOLEDBSimpleProviderListener<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pospilistener: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeOLEDBSimpleProviderListener(::windows::core::from_raw_borrowed(&pospilistener)).into()
        }
        unsafe extern "system" fn isAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbasynch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbasynch, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getEstimatedRows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirows: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getEstimatedRows() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pirows, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn stopTransfer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.stopTransfer().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<OLEDBSimpleProvider as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Search\"`, `\"implement\"`*"]
pub trait OLEDBSimpleProviderListener_Impl: Sized {
    fn aboutToChangeCell(&self, irow: isize, icolumn: isize) -> ::windows::core::Result<()>;
    fn cellChanged(&self, irow: isize, icolumn: isize) -> ::windows::core::Result<()>;
    fn aboutToDeleteRows(&self, irow: isize, crows: isize) -> ::windows::core::Result<()>;
    fn deletedRows(&self, irow: isize, crows: isize) -> ::windows::core::Result<()>;
    fn aboutToInsertRows(&self, irow: isize, crows: isize) -> ::windows::core::Result<()>;
    fn insertedRows(&self, irow: isize, crows: isize) -> ::windows::core::Result<()>;
    fn rowsAvailable(&self, irow: isize, crows: isize) -> ::windows::core::Result<()>;
    fn transferComplete(&self, xfer: OSPXFER) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for OLEDBSimpleProviderListener {}
impl OLEDBSimpleProviderListener_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: isize>() -> OLEDBSimpleProviderListener_Vtbl {
        unsafe extern "system" fn aboutToChangeCell<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.aboutToChangeCell(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&icolumn)).into()
        }
        unsafe extern "system" fn cellChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.cellChanged(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&icolumn)).into()
        }
        unsafe extern "system" fn aboutToDeleteRows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.aboutToDeleteRows(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)).into()
        }
        unsafe extern "system" fn deletedRows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.deletedRows(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)).into()
        }
        unsafe extern "system" fn aboutToInsertRows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.aboutToInsertRows(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)).into()
        }
        unsafe extern "system" fn insertedRows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.insertedRows(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)).into()
        }
        unsafe extern "system" fn rowsAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.rowsAvailable(::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)).into()
        }
        unsafe extern "system" fn transferComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xfer: OSPXFER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.transferComplete(::core::mem::transmute_copy(&xfer)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<OLEDBSimpleProviderListener as ::windows::core::ComInterface>::IID
    }
}
